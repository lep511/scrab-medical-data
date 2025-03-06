use serde_json::{json, Value};
use aws_lambda_events::encodings::Body;
use lambda_runtime::tracing::{error, info};
use serde::{Deserialize, Serialize};
use crate::scrab_errors::ScrabError;

/// Root struct for the hook response
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HookResponse {
    pub hook_instance: Option<String>,
    pub hook: Option<String>,
    pub fhir_server: Option<String>,
    pub context: Option<Context>,
    pub prefetch: Option<Prefetch>,
    pub fhir_authorization: Option<Value>,
}

/// Context of the hook
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub patient_id: Option<String>,
    pub user_id: Option<String>,
}

/// Prefetch data included in the response
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Prefetch {
    pub conditions: Option<Bundle>,
    pub patient: Option<Patient>,
    pub medications: Option<Bundle>,
    // pub allergies: Option<Bundle>,
    // pub observations: Option<Bundle>,
    // pub procedures: Option<Bundle>,
    // pub careplans: Option<Bundle>,
    // pub devices: Option<Bundle>,
    // pub encounters: Option<Bundle>,
    // pub immunizations: Option<Bundle>,
}

/// Bundle of resources (e.g., conditions)
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    pub resource_type: Option<String>,
    #[serde(rename = "type")]
    pub bundle_type: Option<String>,
    pub total: Option<i32>,
    pub link: Option<Vec<Link>>,
    pub entry: Option<Vec<BundleEntry>>,
}

/// Patient resource
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    pub resource_type: Option<String>,
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Option<Vec<Extension>>,
    pub identifier: Option<Vec<Identifier>>,
    pub name: Option<Vec<HumanName>>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub address: Option<Vec<Address>>,
    pub communication: Option<Vec<Value>>,
    pub managing_organization: Option<Reference>,
}

/// Link within a bundle
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Link {
    pub relation: Option<String>,
    pub url: Option<String>,
}

/// Entry in a bundle
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntry {
    pub full_url: Option<String>,
    pub resource: Option<Resource>,
    pub response: Option<EntryResponse>,
}

/// EntryResponse metadata for an entry
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryResponse {
    pub status: Option<String>,
    pub last_modified: Option<String>,
}

/// Resource condition
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub resource_type: Option<String>,
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub identifier: Option<Vec<Identifier>>,
    pub clinical_status: Option<CodingWrapper>,
    pub verification_status: Option<CodingWrapper>,
    pub category: Option<Vec<CodingWrapper>>,
    pub code: Option<CodeableConcept>,
    pub subject: Option<Reference>,
    pub patient: Option<Reference>,
    pub reaction: Option<Vec<Reaction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onset_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onset_period: Option<Period>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abatement_date_time: Option<String>,
}

/// Metadata for a resource
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub version_id: Option<String>,
    pub last_updated: Option<String>,
    pub profile: Vec<String>,
}

/// Identifier for a resource
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Identifier {
    pub system: Option<String>,
    pub value: Option<String>,
}

/// CodingWrapper concept (e.g., coded values with text)
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CodingWrapper {
    pub coding: Option<Vec<Coding>>,
}

/// Coding within a codeable concept
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Coding {
    pub system: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
}

/// CodeableConcept 
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CodeableConcept {
    pub coding: Option<Vec<Coding>>,
    pub text: Option<String>,
}

/// Reference to another resource
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Reference {
    pub reference: Option<String>,
    pub display: Option<String>,
}

/// Reaction to allergy intolerance 
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Reaction {
    pub manifestation: Option<Vec<CodeableConcept>>,
    pub severity: Option<String>,
}

/// Period represents a time period with optional start and end timestamps
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Period {
    pub start: Option<String>,
    pub end: Option<String>,
}

/// Extension for additional data
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    pub extension: Option<Vec<Extension>>,
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_coding: Option<Coding>,
}

/// Human name
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HumanName {
    pub family: Option<String>,
    pub given: Option<Vec<String>>,
}

/// Contact point (e.g., phone, email)
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ContactPoint {
    pub system: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "use")]
    pub contact_use: Option<String>,
}

/// Address
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(rename = "use")]
    pub address_use: Option<String>,
    pub line: Option<Vec<String>>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

pub fn get_patient_id_from_context(context: &Context) -> Option<String> {
    context.patient_id.clone()
}

// Enum for clinical status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ClinicalStatus {
    Active,
    Inactive,
    Resolved,
}

// Enum for verification status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VerificationStatus {
    Unconfirmed,
    Presumed,
    Confirmed,
    Refuted,
    EnteredInError,
}

// Enum for reaction severity
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Severity {
    Mild,
    Moderate,
    Severe,
}

/// Helper function to parse JSON string into HookResponse
pub fn parse_hook_response(json_str: &str) -> Result<HookResponse, ScrabError> {
    match serde_json::from_str(json_str) {
        Ok(response) => Ok(response),
        Err(error) => {
            error!("Error parsing HookResponse: {:?}", error);
            Err(ScrabError::InvalidJson("Error to convert json".to_string()))
        }
    }
}

fn extract_patient_name(response: &HookResponse) -> Option<(String, String)> {
    if let Some(prefetch) = &response.prefetch {
        if let Some(patient) = &prefetch.patient {
            if let Some(names) = &patient.name {
                // Assuming the first name in the list is the primary name
                if let Some(human_name) = names.get(0) {
                    let family_name = human_name.family.clone().unwrap_or_default();
                    let given_names = human_name.given.clone().unwrap_or_default();
                    let given_name = given_names.join(" ");
                    return Some((given_name, family_name));
                }
            }
        }
    }
    None
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ MANAGE HOOK DATA ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Function Handle Patient View
pub async fn handle_patient_view(
    hook_data: &str, 
) -> String {
    match manage_hook_data(hook_data).await {
        Ok(body) => body,
        Err(error) => {
            error!("Error: {:?}", error);
            handle_error()
        }
    }
}

// Function CDS Services
pub fn get_cds_services() -> String {
    let body = json!({ 
        "services": [
            {
                "hook": "patient-view",
                "title": "Patient Medication",
                "description": "Patient medication description",
                "id": "medication",
                "prefetch": {
                    "conditions": "Condition?patient={{context.patientId}}",
                    "medications": "MedicationStatement?patient={{context.patientId}}"
                }
            }
        ]
    });

    // "allergies": "AllergyIntolerance?patient={{context.patientId}}",
    // "observations": "Observation?patient={{context.patientId}}"
    
    body.to_string()
}

pub fn get_cds_medication() -> String {
    let body = json!({
        "cards": [
            {
                "summary": "Medication",
                "indicator": "info",
                "source": {
                    "label": "test service"
                }
            },
            {
                "summary": "Some Warning",
                "indicator": "warning",
                "source": {
                    "label": "test service"
                }
            }
        ]
    });

    body.to_string()
}

pub fn handle_error() -> String {
       
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

pub async fn manage_hook_data(
    hook_data: &str, 
) -> Result<String, ScrabError> {

    let response: HookResponse = parse_hook_response(hook_data)?;

    info!("Hook data: {:?}", response);
    
    let iss = response.fhir_server.clone().unwrap_or_default();
    let user_id = response.context.as_ref().and_then(|ctx| ctx.user_id.clone()).unwrap_or_default();
    let patient_id = response.context.as_ref().and_then(|ctx| ctx.patient_id.clone());
    // let call_back = format!("{}/callback", smart_app);
    // let smart_app_uri = format!("{}/launch?client={}&", smart_app, user_id);
    let smart_app_uri = "https://google.com";

    let medications: &Vec<BundleEntry>  = &response
        .prefetch
        .unwrap_or_default()
        .medications
        .unwrap_or_default()
        .entry
        .unwrap_or_default();

    let body = json!({ 
        "cards": [
            {
                "summary": "Medication",
                "indicator": "info",
                "source": {
                    "label": "test service"
                },
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