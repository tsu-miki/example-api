use std::{error::Error, fs::File, io::BufReader, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CompanyData {
    pub id: String,
    pub name: String,
    pub industry: String,
    pub prefecture: String,
    pub city: String,
    pub street: String,
    pub phone: String,
    pub email: String,
    pub founded_year: i32,
    pub employee_count: i32,
    pub capital_million_jpy: i32,
    pub is_listed: bool,
    pub website: String,
}

type CompaniesData = Vec<CompanyData>;

pub async fn get_companies_data() -> CompaniesData {
    let companies_data = read_companies_from_file("./driver/data/companies.json").unwrap();
    companies_data
}

fn read_companies_from_file<P: AsRef<Path>>(path: P) -> Result<CompaniesData, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let json = serde_json::from_reader(reader)?;
    Ok(json)
}
