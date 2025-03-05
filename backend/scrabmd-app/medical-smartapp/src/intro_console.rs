use crate::oidc_request::get_mdata;
use crate::oidc_database::{SessionData, get_session_data};
use crate::http_page::get_main_page;
use crate::libs::{MedicalRecord, MainPageParams};
use serde_json::{Value, json};

pub async fn main_console_page(
    params: &MainPageParams,
) -> Result<String, Box<dyn std::error::Error>> {

    let patients_json = json!({});
    let html = get_main_page(
        &patients_json,
    );

    Ok(html)
}