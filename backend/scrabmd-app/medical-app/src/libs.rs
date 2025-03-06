use serde_json::{json, Value, from_str};
use lambda_http::tracing::{error, warn, info};
use crate::llm_engine::manage_medication;
use crate::scrab_errors::ScrabError;
use std::env;

fn extract_medications(json_str: &str) -> Vec<String> {
    // Parse the JSON string into a serde_json::Value
    let v: Value = match from_str(json_str) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Failed to parse JSON: {}", e);
            return vec![];
        }
    };

    // Navigate to the "prefetch.medications.entry" array
    let entries = match v.get("prefetch")
        .and_then(|p| p.get("medications"))
        .and_then(|m| m.get("entry"))
        .and_then(|e| e.as_array()) {
        Some(array) => array,
        None => {
            eprintln!("No medications entry array found");
            return vec![];
        }
    };

    // Extract details from each MedicationStatement resource
    let medications: Vec<String> = entries.iter().filter_map(|entry| {
        let resource = entry.get("resource")?;
        
        let name = resource.get("medicationCodeableConcept")
            .and_then(|mc| mc.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("Unknown");

        let status = resource.get("status")
            .and_then(|s| s.as_str())
            .unwrap_or("Unknown");

        let effective = if let Some(date_time) = resource.get("effectiveDateTime").and_then(|dt| dt.as_str()) {
            date_time.to_string()
        } else if let Some(period) = resource.get("effectivePeriod").and_then(|p| p.as_object()) {
            let start = period.get("start").and_then(|s| s.as_str()).unwrap_or("Unknown");
            let end = period.get("end").and_then(|e| e.as_str()).unwrap_or("Unknown");
            format!("from {} to {}", start, end)
        } else {
            "Unknown".to_string()
        };

        Some(format!("{} (status: {}, effective: {})", name, status, effective))
    }).collect();

    medications
}

pub async fn manage_hook_data(
    hook_data: &str, 
) -> Result<String, ScrabError> {

    // Extract SMART_API_URL from enviroment
    let smart_app_uri = env::var("SMART_API_URL").unwrap_or_default();

    // let medications = extract_medications(json_str);
    // for med in medications {
    //     println!("{}", med);
    // }

    // Convert hook_data to json
    let response: Value = from_str(hook_data).unwrap_or_default();
    let empty_vec: Vec<Value> = Vec::new();

    // Extract medications
    let medications = response
        .get("prefetch")
        .and_then(|prefetch| prefetch.get("medications"))
        .and_then(|medications| medications.get("entry"))
        .and_then(|entry| entry.as_array())
        .unwrap_or(&empty_vec);

    // Convert to json string to_string_pretty
    let medications_value = serde_json::to_string_pretty(&medications).unwrap_or_default();
    let mut medications_result: String;

    if medications_value != "[]" {
        warn!("No medications found");
        medications_result = "".to_string();
    } else {
        medications_result = match manage_medication(&medications_value).await {
            Ok(result) => result,
            Err(e) => {
                error!("Error managing medications: {:?}", e);
                "".to_string()
            }
        };
    }

    let body = json!({ 
        "cards": [
            {
                "summary": "Medication",
                "indicator": "info",
                "source": {
                    "label": "test service"
                },
                "detail": medications_result,
                "links": [
                    {
                        "label": "My App",
                        "url": smart_app_uri,
                        "type": "smart"
                    }
                ]
            },
            {
                "summary": "Some Warning",
                "indicator": "warning",
                "source": {
                    "label": "test service"
                },
                "suggestions": [
                    {
                        "uuid": "XXX",
                        "label": "Some suggestion"
                    }
                ]
            }
        ]
    });

    Ok(body.to_string())
}