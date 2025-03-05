use crate::oidc_request::get_mdata;
use crate::http_page::get_main_page;
use crate::libs::{MedicalRecord, MainPageParams, Patient};
use crate::libs::{
    DefaultValueSetter, Medication, VitalSign, Treatment, TimelineEvent
};
use lambda_runtime::tracing::{error, info};
use chrono::{NaiveDate, Utc};
use crate::scrab_errors::ScrabError;

pub struct PatientDetails {
    pub full_name: String,
    pub address: String,
    pub gender: String,
    pub phone: String,
    pub age: u32,
}

pub async fn main_console_page(
    params: &MainPageParams,
) -> Result<String, ScrabError> {

    let query = format!("Patient/{}", params.patient_id);
    info!("Query: {}", query);

    let patient_data = match get_mdata(
        &params.iss,
        &query,
        &params.access_token,
    ).await {
        Ok(data) => data,
        Err(e) => {
            error!("Error getting patient data: {:?}", e);
            return Err(ScrabError::GenericError("Error getting metadata".to_string()));
        }
    };
        
    let patient: Patient = match parse_patient_response(&patient_data) {
        Ok(patient) => patient,
        Err(e) => {
            error!("Error parsing patient data: {:?}", e);
            return Err(ScrabError::GenericError("Error parsing patient data".to_string()));
        }
    };

    let patient_details: PatientDetails = extract_patient_details(&patient);

    // Create a new record with all default 'n/a' values
    let mut record = MedicalRecord::new_default();

    // Selectively update specific fields
    record.set_fields(|r| {
        r.id = params.patient_id.clone();
        r.name = patient_details.full_name.clone();
        r.age = patient_details.age;
        r.gender = patient_details.gender.clone();
        r.address = patient_details.address.clone();
        r.phone = patient_details.phone.clone();
        r.allergies = vec!["Penicillin".to_string()];
        
        r.current_medications.push(Medication {
            name: "Aspirin".to_string(),
            dosage: "100mg".to_string(),
            frequency: "Daily".to_string(),
        });
        
        r.vital_signs.push(VitalSign {
            date: "2024-03-04".to_string(),
            temperature: 37.0,
            blood_pressure: "120/80".to_string(),
            heart_rate: 72,
            respiratory_rate: 16,
            oxygen_saturation: 98,
        });

        r.treatments.push(Treatment {
            date: "2024-03-04".to_string(),
            t_type: "Medication Adjustment".to_string(),
            provider: "Medication Adjustment".to_string(),
            notes: "Increased Lisinopril to 10mg".to_string(),
        });

        r.timeline.push(TimelineEvent {
            year: "2019".to_string(),
            title: "New Hospital Admission".to_string(),
            description: "Brief hospital stay due to severe pneumonia.".to_string(),
            icon: "Hospital".to_string(),
            highlight: false,
        });
    });

    let patients_json = match serde_json::to_string_pretty(&record) {
        Ok(json) => json,
        Err(e) => {
            error!("Error serializing record to JSON: {:?}", e);
            return Err(ScrabError::GenericError("Error serializing record to JSON".to_string()));
        }
    };

    let html = get_main_page(&patients_json);
    
    Ok(html)
}

/// Helper function to parse JSON string into Patient
pub fn parse_patient_response(json_str: &str) -> Result<Patient, Box<dyn std::error::Error>> {
    match serde_json::from_str(json_str) {
        Ok(patient) => Ok(patient),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn capitalize_first_char(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first_char) => first_char.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// Calculates age based on a birth date string in the format "YYYY-MM-DD"
pub fn calculate_age(birth_date_str: &str) -> u32 {
    // Try to parse the birth date string
    let Ok(birth_date) = NaiveDate::parse_from_str(birth_date_str, "%Y-%m-%d") else {
        return 0;
    };
    
    // Get current date
    let current_date = Utc::now().date_naive();
    
    // Calculate age, return 0 if birth date is in the future
    current_date.years_since(birth_date).unwrap_or(0)
}

fn extract_patient_details(patient: &Patient) -> PatientDetails {
    // Extract first given name
    let first_given_name = patient.name.as_ref()
        .and_then(|names| names.first())
        .and_then(|name| name.given.as_ref())
        .and_then(|names| names.first())
        .cloned()
        .unwrap_or_else(|| "n/a".to_string());

    // Extract family name
    let family_name = patient.name.as_ref()
        .and_then(|names| names.first())
        .and_then(|name| name.family.clone())
        .unwrap_or_else(|| "n/a".to_string());

    // Extract full address
    let full_address = patient.address.as_ref()
        .and_then(|addresses| addresses.first())
        .map(|address| {
            let line = address.line.as_ref()
                .and_then(|lines| lines.first())
                .cloned()
                .unwrap_or_else(|| "n/a".to_string());
            
            let city = address.city.clone().unwrap_or_else(|| "n/a".to_string());
            let state = address.state.clone().unwrap_or_else(|| "n/a".to_string());
            let postal_code = address.postal_code.clone().unwrap_or_else(|| "n/a".to_string());

            format!("{}, {}, {} {}", line, city, state, postal_code)
        })
        .unwrap_or_else(|| "n/a".to_string());

    // Extract gender
    let mut gender = patient.gender.clone().unwrap_or_else(|| "n/a".to_string());
    if gender != "n/a" {gender = capitalize_first_char(&gender);}

    // Extract birth date
    let birth_date = patient.birth_date.clone().unwrap_or_else(|| "n/a".to_string());
    let mut age = 0;
    if birth_date != "n/a" {
        age = calculate_age(&birth_date);
    }

    PatientDetails {
        full_name: format!("{} {}", first_given_name, family_name),
        address: full_address,
        gender,
        phone: "n/a".to_string(),
        age: age,
    }
}
