use axum::response::Json;
use derive_new::new;
use driver::company_driver::get_companies_data;
use serde::Serialize;

pub async fn ping() -> &'static str {
    "pong"
}

#[derive(Serialize, new, Debug)]
pub struct CompanyJson {
    id: String,
    name: String,
    founded_year: i32,
    employee_count: i32
}

type CompaniesJson = Vec<CompanyJson>;

pub async fn get_companies() -> Json<CompaniesJson> {
    let companies_data = get_companies_data().await;
    let companies_json = companies_data
        .iter()
        .map(|company| {
            CompanyJson::new(
                company.id.to_string(),
                company.name.to_string(),
                company.founded_year,
                company.employee_count
            )
        })
        .collect::<CompaniesJson>();
    Json(companies_json)
}
