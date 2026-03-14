use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate, utoipa::ToSchema)]
pub struct SupplierCreateForm {
    #[validate(length(min = 1, max = 150))]
    pub name: String,
    #[validate(email)]
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
}

pub struct SupplierDto {
    pub form: SupplierCreateForm,
    pub created_by: Uuid,
}
