import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import type { Treatment } from "@/types/patient"
import { Calendar, User } from "lucide-react"

interface TreatmentHistoryProps {
  treatments: Treatment[]
}

export default function TreatmentHistory({ treatments }: TreatmentHistoryProps) {
  // Sort treatments by date (newest first)
  const sortedTreatments = [...treatments].sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())

  return (
    <div className="p-6">
      <h2 className="text-xl font-bold mb-6">Treatment History</h2>

      <div className="relative">
        {/* Timeline line */}
        <div className="absolute left-4 top-0 bottom-0 w-px bg-gray-700"></div>

        <div className="space-y-8 relative">
          {sortedTreatments.map((treatment, index) => {
            const treatmentDate = new Date(treatment.date)
            const formattedDate = treatmentDate.toLocaleDateString("en-US", {
              year: "numeric",
              month: "long",
              day: "numeric",
            })

            return (
              <div key={index} className="relative pl-12">
                {/* Timeline dot */}
                <div className="absolute left-0 top-1.5 h-8 w-8 rounded-full bg-gray-800 border-4 border-gray-700 flex items-center justify-center">
                  <div className="h-2 w-2 rounded-full bg-primary"></div>
                </div>

                <Card className="bg-gray-800/60 backdrop-blur-sm border-gray-700/50">
                  <CardHeader className="pb-2">
                    <div className="flex justify-between items-start">
                      <div>
                        <CardTitle className="text-white font-medium">{treatment.type}</CardTitle>
                        <CardDescription className="flex items-center gap-1 mt-1">
                          <Calendar className="h-3 w-3" /> {formattedDate}
                        </CardDescription>
                      </div>
                      <div className="flex items-center gap-1 text-sm text-gray-300">
                        <User className="h-3 w-3" /> {treatment.provider}
                      </div>
                    </div>
                  </CardHeader>
                  <CardContent>
                    <p className="text-gray-300">{treatment.notes}</p>
                  </CardContent>
                </Card>
              </div>
            )
          })}
        </div>
      </div>
    </div>
  )
}

