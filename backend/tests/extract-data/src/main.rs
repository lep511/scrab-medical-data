use serde_json::Value;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use serde_json::json;
use tracing::{span, Level};

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
    pub allergies: Option<Bundle>,
    // pub medications: Option<Bundle>,
    // pub observations: Option<Bundle>,
    // pub procedures: Option<Bundle>,
    // pub immunizations: Option<Bundle>,
    // pub careplans: Option<Bundle>,
    // pub devices: Option<Bundle>,
    // pub encounters: Option<Bundle>,
    // pub immunizations: Option<Bundle>,
    // pub medications: Option<Bundle>,
    // pub observations: Option<Bundle>,
    // pub procedures: Option<Bundle>,
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
    pub extension: Option<Vec<ExtensionItem>>,
}
/// Extension item
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionItem {
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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~ MEDICAL RECORD ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MedicalRecord {
    pub id: String,
    pub name: String,
    pub age: u32,
    pub gender: String,
    pub ethnicity: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub emergency_contact: String,
    pub allergies: Vec<String>,
    pub chronic_conditions: Vec<String>,
    pub current_medications: Vec<Medication>,
    pub vital_signs: Vec<VitalSign>,
    pub treatments: Vec<Treatment>,
    pub appointments: Vec<Appointment>,
    pub timeline: Vec<TimelineEvent>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Medication {
    pub name: String,
    pub dosage: String,
    pub frequency: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VitalSign {
    pub date: String,
    pub heart_rate: u32,
    pub blood_pressure: String,
    pub temperature: f32,
    pub respiratory_rate: u32,
    pub oxygen_saturation: u32,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Treatment {
    pub date: String,
    #[serde(rename = "type")]
    pub t_type: String,
    pub provider: String,
    pub notes: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Appointment {
    pub date: String,
    pub time: String,
    pub provider: String,
    #[serde(rename = "type")]
    pub a_type: String,
    pub location: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TimelineEvent {
    pub year: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub highlight: bool,
}

// Trait for setting default values and selective updates
pub trait DefaultValueSetter {
    /// Create a new instance with default values
    fn new_default() -> Self;

    /// Selectively update fields with new values
    fn set_fields(&mut self, updater: impl Fn(&mut Self));
}

impl DefaultValueSetter for MedicalRecord {
    fn new_default() -> Self {
        let timeline = vec![
            TimelineEvent {
                year: "n/a".to_string(),
                title: "n/a".to_string(),
                description: "n/a".to_string(),
                icon: "Activity".to_string(),
                highlight: false,
            },
        ];

        let appointments = vec![
            Appointment {
                date: "n/a".to_string(),
                time: "n/a".to_string(),
                provider: "n/a".to_string(),
                a_type: "n/a".to_string(),
                location: "n/a".to_string(),
            },
        ];

        let treatments = vec![
            Treatment {
                date: "n/a".to_string(),
                t_type: "n/a".to_string(),
                provider: "n/a".to_string(),
                notes: "n/a".to_string(),
            },
        ];

        let vital_signs = vec![
            VitalSign {
                date: "n/a".to_string(),
                heart_rate: 0,
                blood_pressure: "n/a".to_string(),
                temperature: 0.0,
                respiratory_rate: 0,
                oxygen_saturation: 0,
            },
        ];

        let current_medications = vec![
            Medication {
                name: "n/a".to_string(),
                dosage: "n/a".to_string(),
                frequency: "n/a".to_string(),
            },
        ];

        Self {
            id: "n/a".to_string(),
            name: "n/a".to_string(),
            age: 0,
            gender: "n/a".to_string(),
            ethnicity: "n/a".to_string(),
            address: "n/a".to_string(),
            phone: "n/a".to_string(),
            email: "n/a".to_string(),
            emergency_contact: "n/a".to_string(),
            allergies: vec!["n/a".to_string()],
            chronic_conditions: vec!["n/a".to_string()],
            current_medications: current_medications,
            vital_signs: vital_signs,
            treatments: treatments,
            appointments: appointments,
            timeline: timeline,
        }
    }

    fn set_fields(&mut self, updater: impl Fn(&mut Self)) {
        updater(self);
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~ MAIN PAGE PARAMETERS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct MainPageParams {
    pub iss: String,
    pub access_token: String,
    pub patient_id: String,
}

/// Helper function to parse JSON string into Patient
pub fn parse_patient_response(json_str: &str) -> Result<Patient, Box<dyn std::error::Error>> {
    match serde_json::from_str(json_str) {
        Ok(patient) => Ok(patient),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn extract_ethnicity(patient: &Patient) -> String {
    let url = "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity";
    let mut ethnicity = String::from("n/a");
    
    if let Some(extension) = patient.extension.as_ref() {
        for ext in extension {
            println!("value: {:?}", ext);
            if let Some(ext_item) = ext.extension.as_ref() {
                for item in ext_item {
                    if let Some(ext_url) = item.url.as_ref() {
                        if let Some(ext_ext) = ext.extension.as_ref() {
                            for e in ext_ext {
                                if let Some(value) = e.value_coding.as_ref() {
                                    if let Some(display) = value.display.as_ref() {
                                        ethnicity = display.to_string();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if ethnicity == "n/a" {
        "n/a".to_string()
    } else {
        ethnicity
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let span = span!(Level::TRACE, "my_span");
    let patient_data = json!({
        "resourceType": "Patient",
        "id": "2694ec73-6151-4197-aabc-62ae04ed8962",
        "meta": {
            "lastUpdated": "2025-02-19T18:54:59.385335+00:00",
            "profile": [
                "http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient"
            ]
        },
        "text": {
            "status": "generated",
            "div": "<div xmlns=\"http://www.w3.org/1999/xhtml\">Generated by <a href=\"https://github.com/synthetichealth/synthea\">Synthea</a>.Version identifier: master-branch-latest\n .   Person seed: -1409190165185284208  Population seed: 1736238559394</div>"
        },
        "extension": [
            {
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2106-3",
                            "display": "White"
                        }
                    },
                    {
                        "url": "text",
                        "valueString": "White"
                    }
                ],
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-race"
            },
            {
                "extension": [
                    {
                        "url": "ombCategory",
                        "valueCoding": {
                            "system": "urn:oid:2.16.840.1.113883.6.238",
                            "code": "2135-2",
                            "display": "Hispanic or Latino"
                        }
                    },
                    {
                        "url": "text",
                        "valueString": "Hispanic or Latino"
                    }
                ],
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-ethnicity"
            },
            {
                "url": "http://hl7.org/fhir/StructureDefinition/patient-mothersMaidenName",
                "valueString": "Andrea7 Centeno914"
            },
            {
                "url": "http://hl7.org/fhir/us/core/StructureDefinition/us-core-birthsex",
                "valueCode": "M"
            },
            {
                "url": "http://hl7.org/fhir/StructureDefinition/patient-birthPlace",
                "valueAddress": {
                    "city": "Wesley",
                    "state": "Saint Andrew Parish",
                    "country": "DM"
                }
            },
            {
                "url": "http://synthetichealth.github.io/synthea/disability-adjusted-life-years",
                "valueDecimal": 0.23608689104906505
            },
            {
                "url": "http://synthetichealth.github.io/synthea/quality-adjusted-life-years",
                "valueDecimal": 45.76391310895094
            }
        ],
        "identifier": [
            {
                "system": "https://github.com/synthetichealth/synthea",
                "value": "2286ee8d-2dc3-2284-6b5b-1b7756957399"
            },
            {
                "type": {
                    "coding": [
                        {
                            "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                            "code": "MR",
                            "display": "Medical Record Number"
                        }
                    ],
                    "text": "Medical Record Number"
                },
                "system": "http://hospital.smarthealthit.org",
                "value": "2286ee8d-2dc3-2284-6b5b-1b7756957399"
            },
            {
                "type": {
                    "coding": [
                        {
                            "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                            "code": "SS",
                            "display": "Social Security Number"
                        }
                    ],
                    "text": "Social Security Number"
                },
                "system": "http://hl7.org/fhir/sid/us-ssn",
                "value": "999-17-9365"
            },
            {
                "type": {
                    "coding": [
                        {
                            "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                            "code": "DL",
                            "display": "Driver's license number"
                        }
                    ],
                    "text": "Driver's license number"
                },
                "system": "urn:oid:2.16.840.1.113883.4.3.25",
                "value": "S99912129"
            },
            {
                "type": {
                    "coding": [
                        {
                            "system": "http://terminology.hl7.org/CodeSystem/v2-0203",
                            "code": "PPN",
                            "display": "Passport Number"
                        }
                    ],
                    "text": "Passport Number"
                },
                "system": "http://hl7.org/fhir/sid/passport-USA",
                "value": "X28688481X"
            }
        ],
        "name": [
            {
                "use": "official",
                "family": "Villaseñor332",
                "given": [
                    "Daniel959",
                    "Hernán834"
                ],
                "prefix": [
                    "Mr."
                ]
            }
        ],
        "telecom": [
            {
                "system": "phone",
                "value": "555-756-9253",
                "use": "home"
            }
        ],
        "gender": "male",
        "birthDate": "1978-12-18",
        "address": [
            {
                "extension": [
                    {
                        "extension": [
                            {
                                "url": "latitude",
                                "valueDecimal": 42.56295642081726
                            },
                            {
                                "url": "longitude",
                                "valueDecimal": -70.85325008175316
                            }
                        ],
                        "url": "http://hl7.org/fhir/StructureDefinition/geolocation"
                    }
                ],
                "line": [
                    "272 Gibson Center"
                ],
                "city": "Beverly",
                "state": "MA",
                "postalCode": "01915",
                "country": "US"
            }
        ],
        "maritalStatus": {
            "coding": [
                {
                    "system": "http://terminology.hl7.org/CodeSystem/v3-MaritalStatus",
                    "code": "D",
                    "display": "Divorced"
                }
            ],
            "text": "Divorced"
        },
        "multipleBirthBoolean": false,
        "communication": [
            {
                "language": {
                    "coding": [
                        {
                            "system": "urn:ietf:bcp:47",
                            "code": "es",
                            "display": "Spanish"
                        }
                    ],
                    "text": "Spanish"
                }
            }
        ]
    });

    let patient: Patient = match parse_patient_response(&patient_data.to_string()) {
        Ok(patient) => patient,
        Err(e) => {
            error!("Error parsing patient data: {:?}", e);
            return Err(e);
        }
    };

    let ethnicity = extract_ethnicity(&patient);
    println!("Ethnicity: {}", ethnicity);

    Ok(())
}
