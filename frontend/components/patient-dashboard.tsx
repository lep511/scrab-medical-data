"use client"

import { useState, useEffect } from "react"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Card } from "@/components/ui/card"
import PatientInfo from "@/components/patient-info"
import TreatmentHistory from "@/components/treatment-history"
import VitalSigns from "@/components/vital-signs"
import Appointments from "@/components/appointments"
import PatientTimeline from "@/components/patient-timeline"
import type { Patient } from "@/types/patient"

export default function PatientDashboard() {
  const [patient, setPatient] = useState<Patient | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    fetch("/data/patient-data.json")
      .then((response) => {
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`)
        }
        return response.json()
      })
      .then((data) => {
        setPatient(data)
        setLoading(false)
      })
      .catch((err) => {
        console.error("Error loading patient data:", err)
        setError(`Failed to load patient data: ${err.message}`)
        setLoading(false)
      })
  }, [])

  if (loading) {
    return <div className="text-white text-center py-10">Loading patient data...</div>
  }

  if (error) {
    return <div className="text-red-500 text-center py-10">{error}</div>
  }

  if (!patient) {
    return <div className="text-white text-center py-10">No patient data available</div>
  }

  return (
    <div className="container mx-auto py-8 px-4">
      <header className="mb-8">
        <h1 className="text-3xl font-bold text-white">Patient Dashboard</h1>
        <div className="h-px bg-gray-600 w-full mt-4"></div>
      </header>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
        <Card className="bg-gray-900/40 backdrop-blur-md border-gray-700/50 col-span-1 lg:col-span-3">
          <PatientInfo patient={patient} />
        </Card>
      </div>

      <Tabs defaultValue="vitals" className="w-full">
        <TabsList className="bg-gray-800/50 backdrop-blur-sm border-gray-700/50 mb-6">
          <TabsTrigger value="vitals">Vital Signs</TabsTrigger>
          <TabsTrigger value="timeline">Timeline</TabsTrigger>
          <TabsTrigger value="treatments">Treatment History</TabsTrigger>
          <TabsTrigger value="appointments">Appointments</TabsTrigger>
        </TabsList>

        <TabsContent value="vitals">
          <Card className="bg-gray-900/40 backdrop-blur-md border-gray-700/50">
            <VitalSigns vitalSigns={patient.vitalSigns} />
          </Card>
        </TabsContent>

        <TabsContent value="timeline">
          <Card className="bg-gray-900/40 backdrop-blur-md border-gray-700/50">
            <PatientTimeline timelineData={patient.timeline} />
          </Card>
        </TabsContent>

        <TabsContent value="treatments">
          <Card className="bg-gray-900/40 backdrop-blur-md border-gray-700/50">
            <TreatmentHistory treatments={patient.treatments} />
          </Card>
        </TabsContent>

        <TabsContent value="appointments">
          <Card className="bg-gray-900/40 backdrop-blur-md border-gray-700/50">
            <Appointments appointments={patient.appointments} />
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  )
}

