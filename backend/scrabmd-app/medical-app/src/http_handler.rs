use lambda_http::{Body, Request, Error, Response};
use lambda_http::tracing::{error, info};
use crate::libs::manage_hook_data;
use serde_json::json;
use std::env;

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    info!("Event: {:?}", event);
    // info!("Body: {:?}", event.body());
    let path_fm = event.uri().path();
    // Smart App URL launch
    let smart_app_uri = env::var("SMART_APP_URI").expect("SMART_APP_URI must be set");
    
    let path: String = path_fm.rsplitn(3, '/')
        .take(2)
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect::<Vec<&str>>()
        .join("/");

    let body_string = match event.body() {
        Body::Text(text) => text,
        _ => ""
    };
    let body_resp: String;

    match path.as_str() {
        "cds-services/0001" => {
            info!("Services path cds-services-0001");
            body_resp = handle_patient_view(&body_string, &smart_app_uri).await;
        }
        _ => {
            info!("Services path cds-services-0001");
            body_resp = handle_discovery();
        }
    }

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body_resp.into())
        .map_err(Box::new)?)
}

pub async fn handle_patient_view(
    hook_data: &str, 
    smart_app_uri: &str,
) -> String {
    match manage_hook_data(hook_data, smart_app_uri).await {
        Ok(body) => body,
        Err(error) => {
            error!("Error: {:?}", error);
            handle_error()
        }
    }
}

pub fn handle_discovery() -> String {
    let body = json!({ 
        "services": [
            {
                "hook": "patient-view",
                "title": "Patient View",
                "description": "Patient view description",
                "id": "0001",
                "prefetch": {
                    "patient": "Patient/{{context.patientId}}",
                    "conditions": "Condition?patient={{context.patientId}}",
                    "allergies": "AllergyIntolerance?patient={{context.patientId}}"
                }
            }
        ]
    });
    
    body.to_string()
}

fn handle_error() -> String {
       
    let body = json!({ 
        "cards": [
            {
                "summary": "patient-view",
                "indicator": "info",
                "source": {
                    "label": "No event"
                }
            }
        ]
    });

    body.to_string()
}