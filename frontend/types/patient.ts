export interface Patient {
  id: string
  name: string
  age: number
  gender: string
  bloodType: string
  address: string
  phone: string
  email: string
  emergencyContact: string
  allergies: string[]
  chronicConditions: string[]
  currentMedications: Medication[]
  vitalSigns: VitalSign[]
  treatments: Treatment[]
  appointments: Appointment[]
  timeline: TimelineEvent[]
}

export interface Medication {
  name: string
  dosage: string
  frequency: string
}

export interface VitalSign {
  date: string
  heartRate: number
  bloodPressure: string
  temperature: number
  respiratoryRate: number
  oxygenSaturation: number
}

export interface Treatment {
  date: string
  type: string
  provider: string
  notes: string
}

export interface Appointment {
  date: string
  time: string
  provider: string
  type: string
  location: string
}

export interface TimelineEvent {
  year: string
  title: string
  description: string
  icon: string
  highlight?: boolean
}
