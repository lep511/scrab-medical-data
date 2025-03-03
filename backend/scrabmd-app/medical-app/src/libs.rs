use serde_json::Value;
use serde::{Deserialize, Serialize};

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
}

/// Period represents a time period with optional start and end timestamps
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Period {
    pub start: Option<String>,
    pub end: Option<String>,
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