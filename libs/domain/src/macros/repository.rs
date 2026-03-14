#[macro_export]
macro_rules! impl_base_crud {
    ($repo_name:ident, $table:ident, $model:ident, $insertable:ident, $name:expr) => {
        impl $repo_name {
            pub const ENTITY_NAME: &'static str = $name;

            /// Create a single record
            pub fn create(new_item: $insertable) -> AppResult<$model> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::create_with_conn(new_item, &mut FOXTIVE.db_conn()?)
            }

            /// Create a single record with provided connection (for transactions)
            pub fn create_with_conn<C>(new_item: $insertable, conn: &mut C) -> AppResult<$model>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::prelude::IntoAppResult;

                diesel::insert_into($table::table)
                    .values(&new_item)
                    .get_result(conn)
                    .into_app_result()
            }

            /// Create multiple records in a single transaction
            pub fn create_many(items: Vec<$insertable>) -> AppResult<Vec<$model>> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::create_many_with_conn(items, &mut FOXTIVE.db_conn()?)
            }

            /// Create multiple records with provided connection
            pub fn create_many_with_conn<C>(
                items: Vec<$insertable>,
                conn: &mut C,
            ) -> AppResult<Vec<$model>>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::prelude::IntoAppResult;

                diesel::insert_into($table::table)
                    .values(&items)
                    .get_results(conn)
                    .into_app_result()
            }

            /// Update a record
            pub fn update(item: $model) -> AppResult<$model> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::update_with_conn(item, &mut FOXTIVE.db_conn()?)
            }

            /// Update a record with provided connection
            pub fn update_with_conn<C>(item: $model, conn: &mut C) -> AppResult<$model>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::prelude::IntoAppResult;

                diesel::update($table::table.find(item.id))
                    .set(&item)
                    .get_result(conn)
                    .into_app_result()
            }

            /// Check if a record exists by ID
            pub fn exists(id: uuid::Uuid) -> AppResult<bool> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::exists_with_conn(id, &mut FOXTIVE.db_conn()?)
            }

            /// Check if a record exists by ID with provided connection
            pub fn exists_with_conn<C>(id: uuid::Uuid, conn: &mut C) -> AppResult<bool>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::dsl::exists;
                use diesel::prelude::*;
                use foxtive::prelude::IntoAppResult;

                diesel::select(exists($table::table.find(id)))
                    .get_result(conn)
                    .into_app_result()
            }

            /// Count total records
            pub fn count() -> AppResult<i64> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::count_with_conn(&mut FOXTIVE.db_conn()?)
            }

            /// Count total records with provided connection
            pub fn count_with_conn<C>(conn: &mut C) -> AppResult<i64>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::dsl::count_star;
                use diesel::prelude::*;
                use foxtive::prelude::IntoAppResult;

                $table::table
                    .select(count_star())
                    .first(conn)
                    .into_app_result()
            }

            /// Execute multiple operations in a transaction
            pub fn with_transaction<F, R>(f: F) -> AppResult<R>
            where
                F: FnOnce(&mut diesel::PgConnection) -> AppResult<R>,
            {
                use diesel::Connection;
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                let mut conn = FOXTIVE.db_conn()?;
                conn.transaction(|conn| f(conn))
            }

            /// Execute operations with an existing transaction
            pub fn in_transaction<F, R>(conn: &mut diesel::PgConnection, f: F) -> AppResult<R>
            where
                F: FnOnce(&mut diesel::PgConnection) -> AppResult<R>,
            {
                f(conn)
            }
        }
    };
}

// Hard delete specific operations
#[macro_export]
macro_rules! impl_hard_delete_ops {
    ($repo_name:ident, $table:ident, $model:ident, $name:expr) => {
        impl $repo_name {
            /// Find a record by ID
            pub fn find(id: uuid::Uuid) -> AppResult<$model> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::find_with_conn(id, &mut FOXTIVE.db_conn()?)
            }

            /// Find a record by ID with provided connection
            pub fn find_with_conn<C>(id: uuid::Uuid, conn: &mut C) -> AppResult<$model>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::database::ext::OptionalResultExt;

                $table::table.find(id).first(conn).required($name)
            }

            /// Find a record by ID, returning None if not found
            pub fn find_opt(id: uuid::Uuid) -> AppResult<Option<$model>> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::find_opt_with_conn(id, &mut FOXTIVE.db_conn()?)
            }

            /// Find a record by ID with provided connection, returning None if not found
            pub fn find_opt_with_conn<C>(id: uuid::Uuid, conn: &mut C) -> AppResult<Option<$model>>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::database::ext::OptionalResultExt;

                $table::table.find(id).first(conn).optional()
            }

            /// Get all records
            pub fn all() -> AppResult<Vec<$model>> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::all_with_conn(&mut FOXTIVE.db_conn()?)
            }

            /// Get all records with connection
            pub fn all_with_conn<C>(conn: &mut C) -> AppResult<Vec<$model>>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::prelude::IntoAppResult;

                $table::table.load(conn).into_app_result()
            }

            /// Delete a record by entity (hard delete - permanently removes from database)
            pub fn delete_entity(entity: $model) -> AppResult<()> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::delete_entity_with_conn(entity, &mut FOXTIVE.db_conn()?)
            }

            /// Delete a record by entity with provided connection
            pub fn delete_entity_with_conn<C>(entity: $model, conn: &mut C) -> AppResult<()>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::prelude::IntoAppResult;

                diesel::delete($table::table.find(entity.id))
                    .execute(conn)
                    .into_app_result()
                    .map(|_| ())
            }

            /// Delete a record by ID (hard delete - permanently removes from database)
            pub fn delete(id: uuid::Uuid) -> AppResult<()> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::delete_with_conn(id, &mut FOXTIVE.db_conn()?)
            }

            /// Delete a record by ID with provided connection
            pub fn delete_with_conn<C>(id: uuid::Uuid, conn: &mut C) -> AppResult<()>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                let entity = Self::find_with_conn(id, conn)?;
                Self::delete_entity_with_conn(entity, conn)
            }
        }
    };
}

// Soft delete specific operations
#[macro_export]
macro_rules! impl_soft_delete_ops {
    ($repo_name:ident, $table:ident, $model:ident, $name:expr) => {
        impl $repo_name {
            /// Find a record by ID (excluding soft-deleted records)
            pub fn find(id: uuid::Uuid) -> AppResult<$model> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::find_with_conn(id, &mut FOXTIVE.db_conn()?)
            }

            /// Find a record by ID with provided connection
            pub fn find_with_conn<C>(id: uuid::Uuid, conn: &mut C) -> AppResult<$model>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::database::ext::OptionalResultExt;

                $table::table
                    .find(id)
                    .filter($table::deleted_at.is_null())
                    .first(conn)
                    .required($name)
            }

            /// Find a record by ID, returning None if not found or soft-deleted
            pub fn find_opt(id: uuid::Uuid) -> AppResult<Option<$model>> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::find_opt_with_conn(id, &mut FOXTIVE.db_conn()?)
            }

            /// Find a record by ID with provided connection, returning None if not found or soft-deleted
            pub fn find_opt_with_conn<C>(id: uuid::Uuid, conn: &mut C) -> AppResult<Option<$model>>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::database::ext::OptionalResultExt;

                $table::table
                    .find(id)
                    .filter($table::deleted_at.is_null())
                    .first(conn)
                    .optional()
            }

            /// Find a record by ID, including soft-deleted records
            pub fn find_with_deleted(id: uuid::Uuid) -> AppResult<$model> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::find_with_deleted_with_conn(id, &mut FOXTIVE.db_conn()?)
            }

            /// Find a record by ID with connection, including soft-deleted records
            pub fn find_with_deleted_with_conn<C>(id: uuid::Uuid, conn: &mut C) -> AppResult<$model>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::database::ext::OptionalResultExt;

                $table::table.find(id).first(conn).required($name)
            }

            /// Find a record by ID including soft-deleted, returning None if not found
            pub fn find_opt_with_deleted(id: uuid::Uuid) -> AppResult<Option<$model>> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::find_opt_with_deleted_with_conn(id, &mut FOXTIVE.db_conn()?)
            }

            /// Find a record by ID with connection including soft-deleted, returning None if not found
            pub fn find_opt_with_deleted_with_conn<C>(
                id: uuid::Uuid,
                conn: &mut C,
            ) -> AppResult<Option<$model>>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::database::ext::OptionalResultExt;

                $table::table.find(id).first(conn).optional()
            }

            /// Get all records (excluding soft-deleted)
            pub fn all() -> AppResult<Vec<$model>> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::all_with_conn(&mut FOXTIVE.db_conn()?)
            }

            /// Get all records with connection
            pub fn all_with_conn<C>(conn: &mut C) -> AppResult<Vec<$model>>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::prelude::IntoAppResult;

                $table::table
                    .filter($table::deleted_at.is_null())
                    .load(conn)
                    .into_app_result()
            }

            /// Soft delete a record by entity
            pub fn delete_entity(entity: $model) -> AppResult<$model> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::delete_entity_with_conn(entity, &mut FOXTIVE.db_conn()?)
            }

            /// Soft delete a record by entity with provided connection
            pub fn delete_entity_with_conn<C>(mut entity: $model, conn: &mut C) -> AppResult<$model>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                entity.deleted_at = Some(chrono::Utc::now().naive_utc());
                Self::update_with_conn(entity, conn)
            }

            /// Soft delete a record by ID
            pub fn delete(id: uuid::Uuid) -> AppResult<$model> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::delete_with_conn(id, &mut FOXTIVE.db_conn()?)
            }

            /// Soft delete a record by ID with provided connection
            pub fn delete_with_conn<C>(id: uuid::Uuid, conn: &mut C) -> AppResult<$model>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                let entity = Self::find_with_conn(id, conn)?;
                Self::delete_entity_with_conn(entity, conn)
            }
        }
    };
}

// Optional hard delete operations for soft delete entities
#[macro_export]
macro_rules! impl_hard_delete_for_soft_delete {
    ($repo_name:ident, $table:ident, $model:ident) => {
        impl $repo_name {
            /// Hard delete a record by entity (permanently removes from database)
            pub fn hard_delete_entity(entity: $model) -> AppResult<()> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::hard_delete_entity_with_conn(entity, &mut FOXTIVE.db_conn()?)
            }

            /// Hard delete a record by entity with provided connection
            pub fn hard_delete_entity_with_conn<C>(entity: $model, conn: &mut C) -> AppResult<()>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                use diesel::RunQueryDsl;
                use foxtive::prelude::IntoAppResult;

                diesel::delete($table::table.find(entity.id))
                    .execute(conn)
                    .into_app_result()
                    .map(|_| ())
            }

            /// Hard delete a record by ID (permanently removes from database)
            pub fn hard_delete(id: uuid::Uuid) -> AppResult<()> {
                use foxtive::FOXTIVE;
                use foxtive::prelude::AppStateExt;

                Self::hard_delete_with_conn(id, &mut FOXTIVE.db_conn()?)
            }

            /// Hard delete a record by ID with provided connection
            pub fn hard_delete_with_conn<C>(id: uuid::Uuid, conn: &mut C) -> AppResult<()>
            where
                C: diesel::Connection<Backend = diesel::pg::Pg>
                    + diesel::connection::LoadConnection,
            {
                // For hard delete on soft delete entities, we need to find with deleted records
                let entity = Self::find_with_deleted_with_conn(id, conn)?;
                Self::hard_delete_entity_with_conn(entity, conn)
            }
        }
    };
}

// Base batch operations
#[macro_export]
macro_rules! impl_base_batch_ops {
    ($repo_name:ident, $model:ident, $insertable:ident) => {
        impl $repo_name {
            /// Batch create with automatic transaction
            pub fn batch_create(items: Vec<$insertable>) -> AppResult<Vec<$model>> {
                Self::with_transaction(|conn| Self::create_many_with_conn(items, conn))
            }

            /// Batch update with automatic transaction
            pub fn batch_update(items: Vec<$model>) -> AppResult<Vec<$model>> {
                Self::with_transaction(|conn| {
                    let mut results = Vec::with_capacity(items.len());
                    for item in items {
                        results.push(Self::update_with_conn(item, conn)?);
                    }
                    Ok(results)
                })
            }
        }
    };
}

// Hard delete batch operations
#[macro_export]
macro_rules! impl_hard_delete_batch_ops {
    ($repo_name:ident, $model:ident) => {
        impl $repo_name {
            /// Batch delete entities with automatic transaction
            pub fn batch_delete_entities(entities: Vec<$model>) -> AppResult<()> {
                Self::with_transaction(|conn| {
                    for entity in entities {
                        Self::delete_entity_with_conn(entity, conn)?;
                    }
                    Ok(())
                })
            }

            /// Batch delete by IDs with automatic transaction
            pub fn batch_delete(ids: Vec<uuid::Uuid>) -> AppResult<()> {
                Self::with_transaction(|conn| {
                    for id in ids {
                        Self::delete_with_conn(id, conn)?;
                    }
                    Ok(())
                })
            }
        }
    };
}

// Soft delete batch operations
#[macro_export]
macro_rules! impl_soft_delete_batch_ops {
    ($repo_name:ident, $model:ident) => {
        impl $repo_name {
            /// Batch soft delete entities with automatic transaction
            /// Returns the updated (soft deleted) entities
            pub fn batch_delete_entities(entities: Vec<$model>) -> AppResult<Vec<$model>> {
                Self::with_transaction(|conn| {
                    let mut results = Vec::with_capacity(entities.len());
                    for entity in entities {
                        results.push(Self::delete_entity_with_conn(entity, conn)?);
                    }
                    Ok(results)
                })
            }

            /// Batch soft delete by IDs with automatic transaction
            /// Returns the updated (soft deleted) entities
            pub fn batch_delete(ids: Vec<uuid::Uuid>) -> AppResult<Vec<$model>> {
                Self::with_transaction(|conn| {
                    let mut results = Vec::with_capacity(ids.len());
                    for id in ids {
                        results.push(Self::delete_with_conn(id, conn)?);
                    }
                    Ok(results)
                })
            }

            /// Batch hard delete entities with automatic transaction
            pub fn batch_hard_delete_entities(entities: Vec<$model>) -> AppResult<()> {
                Self::with_transaction(|conn| {
                    for entity in entities {
                        Self::hard_delete_entity_with_conn(entity, conn)?;
                    }
                    Ok(())
                })
            }

            /// Batch hard delete by IDs with automatic transaction
            pub fn batch_hard_delete(ids: Vec<uuid::Uuid>) -> AppResult<()> {
                Self::with_transaction(|conn| {
                    for id in ids {
                        Self::hard_delete_with_conn(id, conn)?;
                    }
                    Ok(())
                })
            }
        }
    };
}

// Main convenience macros - defined AFTER all the component macros
#[macro_export]
macro_rules! impl_crud_repo {
    // For entities WITH soft delete
    ($repo_name:ident, $table:ident, $model:ident, $insertable:ident, $name:expr, soft_delete) => {
        $crate::impl_base_crud!($repo_name, $table, $model, $insertable, $name);
        $crate::impl_soft_delete_ops!($repo_name, $table, $model, $name);
        $crate::impl_hard_delete_for_soft_delete!($repo_name, $table, $model);
        $crate::impl_batch_repo!($repo_name, $table, $model, $insertable, $name, soft_delete);
    };

    // For entities WITHOUT soft delete
    ($repo_name:ident, $table:ident, $model:ident, $insertable:ident, $name:expr) => {
        $crate::impl_base_crud!($repo_name, $table, $model, $insertable, $name);
        $crate::impl_hard_delete_ops!($repo_name, $table, $model, $name);
        $crate::impl_batch_repo!($repo_name, $table, $model, $insertable, $name);
    };
}

#[macro_export]
macro_rules! impl_batch_repo {
    // For entities WITH soft delete
    ($repo_name:ident, $table:ident, $model:ident, $insertable:ident, $name:expr, soft_delete) => {
        $crate::impl_base_batch_ops!($repo_name, $model, $insertable);
        $crate::impl_soft_delete_batch_ops!($repo_name, $model);
    };

    // For entities WITHOUT soft delete
    ($repo_name:ident, $table:ident, $model:ident, $insertable:ident, $name:expr) => {
        $crate::impl_base_batch_ops!($repo_name, $model, $insertable);
        $crate::impl_hard_delete_batch_ops!($repo_name, $model);
    };
}