use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MedicalRecord {
    pub id: String,
    pub name: String,
    pub age: u8,
    pub gender: String,
    pub blood_type: String,
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
    pub heart_rate: u8,
    pub blood_pressure: String,
    pub temperature: f32,
    pub respiratory_rate: u8,
    pub oxygen_saturation: u8,
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