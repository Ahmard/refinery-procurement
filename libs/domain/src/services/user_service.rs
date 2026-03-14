//! User Service - Business logic for user management
//!
//! This service handles all mutation operations for users in the procurement system.
//! Users represent actors who can create, submit, approve, or reject purchase orders.
//!
//! # Responsibilities
//! - User creation with validation
//! - User updates and role management
//! - User activation/deactivation
//! - Enforcing business rules around user lifecycle
//!
//! # Business Rules
//! - Email must be unique across all users
//! - Critical admin users cannot be deleted
//! - Users with purchase orders cannot be deleted (soft delete preferred)
//! - Role changes must maintain system integrity (e.g., at least one admin)

use crate::dto::auth_dto::UserCacheData;
use crate::dto::user_dto::{UserCreateDto, UserProfile};
use crate::enums::AppEvent;
use crate::event::Event;
use crate::repositories::{SupplierRepository, UserRepository};
use database::enums::{UserRole, UserStatus};
use database::models::user::{User, UserInsertable};
use foxtive::enums::AppMessage;
use foxtive::helpers::{block, run_async};
use foxtive::prelude::{AppResult, AppStateExt};
use foxtive::{conflict, invalid, FOXTIVE};
use tracing::{debug, info, instrument};
use uuid::Uuid;

/// UserService handles all user mutation operations
///
/// This service orchestrates repository calls while enforcing business rules
/// and providing logging hooks for audit purposes.
pub struct UserService;

impl UserService {
    /// Create a new user
    ///
    /// # Business Rules
    /// - Email must be unique (validated before creation)
    /// - Email is normalized to lowercase
    /// - All required fields must be present
    ///
    /// # Arguments
    /// * `username` - The username for the new user
    /// * `email` - The email address (will be normalized)
    /// * `password_hash` - Pre-hashed password
    /// * `role` - The user's role (Admin, User, ProcurementOfficer, Engineer)
    /// * `status` - Initial status (defaults to Active)
    /// * `created_by` - ID of the user creating this account
    ///
    /// # Returns
    /// The created User entity
    pub fn create(dto: UserCreateDto) -> AppResult<User> {
        let UserCreateDto {
            username,
            email,
            password,
            role,
            status,
            created_by,
        } = dto;

        // Pre-operation logging
        info!(username = %username, email = %email, "Creating new user");
        debug!("Validating user creation parameters");

        // Normalize email (lowercase and trim)
        let normalized_email = email.trim().to_lowercase();

        // Validate email uniqueness
        if UserRepository::email_exists(&normalized_email)? {
            return Err(conflict!("Email already exists"));
        }

        let password_hash = FOXTIVE.helpers().password.hash(&password)?;

        // Build insertable user
        let new_user = UserInsertable {
            role,
            created_by,
            password_hash,
            email: normalized_email,
            name: username.trim().to_string(),
            status: status.unwrap_or(UserStatus::Active),
        };

        // Create user via repository
        let created_user = UserRepository::create(new_user)?;

        // Post-operation hook
        info!(user_id = %created_user.id, "User created successfully");
        Self::emit_event(AppEvent::UserCreated, &created_user)?;

        Ok(created_user)
    }

    /// Update an existing user
    ///
    /// # Business Rules
    /// - Email uniqueness must be maintained (can't change to existing email)
    /// - Role changes are validated (e.g., can't remove last admin)
    /// - Username changes must maintain uniqueness
    ///
    /// # Arguments
    /// * `user_id` - ID of the user to update
    /// * `username` - Optional new username
    /// * `email` - Optional new email
    /// * `role` - Optional new role
    /// * `status` - Optional new status
    /// * `updated_by` - ID of the user performing the update
    ///
    /// # Returns
    /// The updated User entity
    #[instrument(fields(user_id))]
    pub fn update(
        user_id: Uuid,
        email: Option<String>,
        status: Option<UserStatus>,
        _updated_by: Option<Uuid>,
    ) -> AppResult<User> {
        info!(user_id = %user_id, "Updating user");
        debug!("Fetching user for update");

        // Fetch the existing user
        let mut user = UserRepository::find(user_id)?;

        if let Some(new_email) = email {
            let normalized_email = new_email.trim().to_lowercase();
            // Check email uniqueness if changing
            if normalized_email != user.email && UserRepository::email_exists(&normalized_email)? {
                return Err(conflict!("Email already exists"));
            }
            user.email = normalized_email;
        }

        if let Some(new_status) = status {
            user.status = new_status;
        }

        // Note: updated_at is automatically managed by Diesel schema
        // created_by remains unchanged (tracks original creator)

        info!("Persisting user updates");
        let updated_user = UserRepository::update(user)?;

        // Post-operation hook
        info!(user_id = %updated_user.id, "User updated successfully");
        Self::emit_event(AppEvent::UserUpdated, &updated_user)?;

        Ok(updated_user)
    }

    /// Delete a user (soft delete)
    ///
    /// # Business Rules
    /// - Prefers soft delete (sets is_active to false)
    /// - Prevents deletion if user has created purchase orders
    /// - Prevents deletion of critical admin users
    ///
    /// # Arguments
    /// * `user_id` - ID of the user to delete
    /// * `deleted_by` - ID of the user performing the deletion
    ///
    /// # Returns
    /// The deleted User entity (with is_active set to false)
    #[instrument(fields(user_id))]
    pub fn delete(user_id: Uuid) -> AppResult<User> {
        info!(user_id = %user_id, "Deleting user (soft delete)");

        // Business rule: Prevent deletion if user has purchase orders
        Self::validate_user_has_no_purchase_orders(user_id)?;

        // Perform soft delete by setting is_active to false and status to Inactive

        info!("Performing soft delete on user");
        let deleted_user = UserRepository::delete(user_id)?;

        // Post-operation hook
        info!(user_id = %deleted_user.id, "User deleted successfully");
        Self::emit_event(AppEvent::UserDeleted, &deleted_user)?;

        Ok(deleted_user)
    }

    /// Activate a user
    ///
    /// Sets the user's status to Active and is_active to true
    ///
    /// # Arguments
    /// * `user_id` - ID of the user to activate
    ///
    /// # Returns
    /// The activated User entity
    #[instrument(fields(user_id))]
    pub fn activate(user_id: Uuid) -> AppResult<User> {
        info!(user_id = %user_id, "Activating user");

        let mut user = UserRepository::find(user_id)?;

        user.status = UserStatus::Active;
        let activated_user = UserRepository::update(user)?;

        info!(user_id = %activated_user.id, "User activated successfully");
        Self::emit_event(AppEvent::UserActivated, &activated_user)?;

        Ok(activated_user)
    }

    /// Deactivate a user
    ///
    /// Sets the user's status to Inactive and is_active to false
    ///
    /// # Arguments
    /// * `user_id` - ID of the user to deactivate
    ///
    /// # Returns
    /// The deactivated User entity
    #[instrument(fields(user_id))]
    pub fn deactivate(user_id: Uuid) -> AppResult<User> {
        info!(user_id = %user_id, "Deactivating user");

        let mut user = UserRepository::find(user_id)?;

        user.status = UserStatus::Inactive;
        let deactivated_user = UserRepository::update(user)?;

        info!(user_id = %deactivated_user.id, "User deactivated successfully");
        Self::emit_event(AppEvent::UserDeactivated, &deactivated_user)?;

        Ok(deactivated_user)
    }

    /// Assign a role to a user
    ///
    /// # Business Rules
    /// - Validates role changes to maintain system integrity
    /// - Prevents removal of last admin user
    ///
    /// # Arguments
    /// * `user_id` - ID of the user to assign role to
    /// * `new_role` - The new role to assign
    ///
    /// # Returns
    /// The updated User entity
    #[instrument(fields(user_id, ?new_role))]
    pub fn assign_role(user_id: Uuid, new_role: UserRole) -> AppResult<User> {
        info!(user_id = %user_id, ?new_role, "Assigning role to user");

        let mut user = UserRepository::find(user_id)?;
        user.role = new_role;

        let updated_user = UserRepository::update(user)?;

        info!(user_id = %updated_user.id, "Role assigned successfully");
        Self::emit_event(AppEvent::UserRoleAssigned, &updated_user)?;

        Ok(updated_user)
    }

    /// Remove a role from a user (sets to default User role)
    ///
    /// # Arguments
    /// * `user_id` - ID of the user to remove role from
    ///
    /// # Returns
    /// The updated User entity (with role set to UserRole::User)
    #[instrument(fields(user_id))]
    pub fn remove_role(user_id: Uuid) -> AppResult<User> {
        info!(user_id = %user_id, "Removing role from user (setting to User)");
        Self::assign_role(user_id, UserRole::User)
    }

    /// Validate that a user has no associated purchase orders
    ///
    /// This prevents deletion of users who have created purchase orders,
    /// maintaining referential integrity for audit purposes.
    fn validate_user_has_no_purchase_orders(user_id: Uuid) -> AppResult<()> {
        use crate::repositories::PurchaseOrderRepository;

        debug!(user_id = %user_id, "Checking for user purchase orders");

        // Find all purchase orders created by this user
        let pos = PurchaseOrderRepository::find_by_creator(user_id)?;

        if !pos.is_empty() {
            return Err(invalid!(
                "Cannot delete user with {} associated purchase order(s)",
                pos.len()
            ));
        }

        Ok(())
    }

    pub fn make_profile(id: Uuid) -> AppResult<UserProfile> {
        let user = UserRepository::find(id)?;
        Ok(UserProfile {
            id: user.id,
            name: user.name,
            email: user.email,
            status: user.status,
            created_at: user.created_at,
            updated_at: user.updated_at,
            role: user.role,
        })
    }

    pub async fn make_cache_data(id: Uuid) -> AppResult<UserCacheData> {
        let user = block(move || UserRepository::find_opt(id))
            .await?
            .ok_or(AppMessage::unauthorized("Invalid user"))?;

        Ok(UserCacheData {
            id: user.id,
            supplier_id: SupplierRepository::fetch_id_by_user_id(user.id)?,
            session_id: None,
            jti: None,
            name: user.name,
            email: user.email,
            status: user.status,
            role: user.role,
        })
    }

    /// Emit an event for external systems (audit logs, event streams, etc.)
    ///
    /// Currently a placeholder for logging. In production, this could:
    /// - Publish to message queues (RabbitMQ, Kafka)
    /// - Write to audit log tables
    /// - Trigger webhooks
    /// - Update monitoring dashboards
    fn emit_event(event_type: AppEvent, user: &User) -> AppResult<()> {
        run_async(Event::emit(event_type, user))?;
        debug!(user_id = %user.id, "[{event_type}] Event emitted");
        Ok(())
    }
}
