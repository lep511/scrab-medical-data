import { Badge } from "@/components/ui/badge"
import type { Patient } from "@/types/patient"
import { User, Phone, Mail, AlertTriangle, Pill } from "lucide-react"

interface PatientInfoProps {
  patient: Patient
}

export default function PatientInfo({ patient }: PatientInfoProps) {
  return (
    <div className="p-6">
      <div className="flex flex-col md:flex-row justify-between gap-6">
        <div className="flex-1">
          <div className="flex items-center gap-4 mb-4">
            <div className="h-16 w-16 rounded-full bg-gray-700 flex items-center justify-center">
              <User className="h-8 w-8 text-gray-300" />
            </div>
            <div>
              <h2 className="text-2xl font-bold">{patient.name}</h2>
              <div className="flex gap-2 text-gray-300 text-sm">
                <span>ID: {patient.id}</span>
                <span>•</span>
                <span>{patient.age} years</span>
                <span>•</span>
                <span>{patient.gender}</span>
                <span>•</span>
                <span>Blood Type: {patient.bloodType}</span>
              </div>
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
            <div className="flex items-center gap-2 text-white">
              <Phone className="h-4 w-4" />
              <span>{patient.phone}</span>
            </div>
            <div className="flex items-center gap-2 text-white">
              <Mail className="h-4 w-4" />
              <span>{patient.email}</span>
            </div>
          </div>

          <div className="mb-6">
            <h3 className="text-sm uppercase text-gray-300 mb-2">Emergency Contact</h3>
            <p className="text-white">{patient.emergencyContact}</p>
          </div>
        </div>

        <div className="flex-1">
          <div className="mb-6">
            <h3 className="text-sm uppercase text-gray-300 mb-2 flex items-center gap-2">
              <AlertTriangle className="h-4 w-4" /> Allergies
            </h3>
            <div className="flex flex-wrap gap-2">
              {patient.allergies.map((allergy) => (
                <Badge key={allergy} variant="destructive">
                  {allergy}
                </Badge>
              ))}
            </div>
          </div>

          <div className="mb-6">
            <h3 className="text-sm uppercase text-gray-300 mb-2">Chronic Conditions</h3>
            <div className="flex flex-wrap gap-2">
              {patient.chronicConditions.map((condition) => (
                <Badge key={condition} variant="secondary" className="bg-gray-700/60 backdrop-blur-sm">
                  {condition}
                </Badge>
              ))}
            </div>
          </div>
        </div>

        <div className="flex-1">
          <h3 className="text-sm uppercase text-gray-300 mb-2 flex items-center gap-2">
            <Pill className="h-4 w-4" /> Current Medications
          </h3>
          <div className="space-y-2">
            {patient.currentMedications.map((med, index) => (
              <div key={index} className="p-3 bg-gray-800/60 backdrop-blur-sm rounded-md">
                <div className="font-medium text-white">{med.name}</div>
                <div className="text-sm text-gray-400">
                  {med.dosage} • {med.frequency}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}

