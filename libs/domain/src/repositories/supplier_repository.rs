use crate::impl_crud_repo;
use database::models::supplier::{Supplier, SupplierInsertable};
use database::schema::suppliers;
use diesel::dsl::exists;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
};
use foxtive::database::ext::OptionalResultExt;
use foxtive::database::pagination::Paginate;
use foxtive::http::QueryParams;
use foxtive::prelude::{AppResult, AppStateExt, IntoAppResult};
use foxtive::results::{AppOptionalResult, AppPaginationResult};
use foxtive::FOXTIVE;
use uuid::Uuid;

pub struct SupplierRepository;

impl_crud_repo!(
    SupplierRepository,
    suppliers,
    Supplier,
    SupplierInsertable,
    "supplier"
);

impl SupplierRepository {
    pub fn list(query: QueryParams) -> AppPaginationResult<Supplier> {
        let mut builder = suppliers::table
            .filter(suppliers::deleted_at.is_null())
            .into_boxed();

        if query.search().is_some() {
            builder = builder.filter(
                suppliers::name
                    .ilike(query.search_query_like())
                    .or(suppliers::contact_email.ilike(query.search_query_like()))
                    .or(suppliers::contact_phone.ilike(query.search_query_like()))
                    .or(suppliers::status.ilike(query.search_query_like())),
            );
        }

        builder
            .paginate(query.curr_page())
            .per_page(query.per_page())
            .load_and_count_pages(&mut *FOXTIVE.db_conn()?)
    }

    pub fn fetch_id_by_user_id(id: Uuid) -> AppOptionalResult<Uuid> {
        suppliers::table
            .select(suppliers::id)
            .filter(suppliers::user_id.eq(id))
            .first(&mut FOXTIVE.db_conn()?)
            .optional()
    }

    /// Find supplier by name (exact match)
    pub fn find_by_name(name: &str) -> AppResult<Option<Supplier>> {
        suppliers::table
            .filter(suppliers::name.eq(name))
            .first(&mut FOXTIVE.db_conn()?)
            .optional()
    }

    pub fn exist_by_name(name: &str) -> AppResult<bool> {
        diesel::select(exists(
            suppliers::table
                .filter(suppliers::name.eq(name))
                .filter(suppliers::deleted_at.is_null()),
        ))
        .first(&mut FOXTIVE.db_conn()?)
        .into_app_result()
    }

    /// Search suppliers by name (case-insensitive partial match)
    pub fn search_by_name(search_term: &str) -> AppResult<Vec<Supplier>> {
        let pattern = format!("%{}%", search_term);
        suppliers::table
            .filter(suppliers::name.ilike(pattern))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find supplier by contact email
    pub fn find_by_email(email: &str) -> AppResult<Option<Supplier>> {
        suppliers::table
            .filter(suppliers::contact_email.eq(email))
            .first(&mut FOXTIVE.db_conn()?)
            .optional()
    }
}
