import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import type { Appointment } from "@/types/patient"
import { Calendar, MapPin, User } from "lucide-react"

interface AppointmentsProps {
  appointments: Appointment[]
}

export default function Appointments({ appointments }: AppointmentsProps) {
  // Sort appointments by date (soonest first)
  const sortedAppointments = [...appointments].sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())

  return (
    <div className="p-6">
      <h2 className="text-xl font-bold mb-6">Upcoming Appointments</h2>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {sortedAppointments.map((appointment, index) => {
          const appointmentDate = new Date(appointment.date)
          const formattedDate = appointmentDate.toLocaleDateString("en-US", {
            weekday: "long",
            year: "numeric",
            month: "long",
            day: "numeric",
          })

          return (
            <Card key={index} className="bg-gray-800/60 backdrop-blur-sm border-gray-700/50">
              <CardHeader>
                <CardTitle className="flex justify-between">
                  <span className="text-white font-medium">{appointment.type}</span>
                  <span className="text-primary font-medium">{appointment.time}</span>
                </CardTitle>
                <CardDescription className="flex items-center gap-1">
                  <Calendar className="h-3 w-3" /> {formattedDate}
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-2 text-sm">
                  <div className="flex items-center gap-2 text-gray-300">
                    <User className="h-4 w-4 text-gray-400" />
                    <span>{appointment.provider}</span>
                  </div>
                  <div className="flex items-center gap-2 text-gray-300">
                    <MapPin className="h-4 w-4 text-gray-400" />
                    <span>{appointment.location}</span>
                  </div>
                </div>
              </CardContent>
            </Card>
          )
        })}
      </div>
    </div>
  )
}

