"use client"

import { Card } from "@/components/ui/card"
import { Stethoscope, Pill, Activity, Syringe, HeartPulse, Hospital } from "lucide-react"
import type { JSX } from "react"

interface TimelineEvent {
  year: string
  title: string
  description: string
  icon: string
  highlight?: boolean
}

interface PatientTimelineProps {
  timelineData: TimelineEvent[]
}

const iconMap: { [key: string]: JSX.Element } = {
  Stethoscope: <Stethoscope className="w-6 h-6" />,
  Pill: <Pill className="w-6 h-6" />,
  Activity: <Activity className="w-6 h-6" />,
  Syringe: <Syringe className="w-6 h-6" />,
  HeartPulse: <HeartPulse className="w-6 h-6" />,
  Hospital: <Hospital className="w-6 h-6" />,
}

export default function PatientTimeline({ timelineData }: PatientTimelineProps) {
  return (
    <div className="p-6">
      <h2 className="text-xl font-bold mb-8">Patient History Timeline</h2>
      <div className="relative">
        {/* Main Timeline Line */}
        <div className="absolute left-1/2 transform -translate-x-1/2 h-full w-px bg-gradient-to-b from-primary/50 to-gray-700/50"></div>

        <div className="space-y-12">
          {timelineData.map((event, index) => (
            <div
              key={index}
              className={`relative flex items-center justify-center ${index % 2 === 0 ? "md:justify-start" : "md:justify-end"}`}
            >
              {/* Timeline Dot */}
              <div className="absolute left-1/2 transform -translate-x-1/2 w-4 h-4 rounded-full bg-gray-900 border-2 border-primary"></div>

              {/* Content Card */}
              <Card
                className={`
                relative w-full md:w-[calc(50%-2rem)] 
                ${index % 2 === 0 ? "md:mr-auto" : "md:ml-auto"}
                bg-gray-900/40 backdrop-blur-md border-gray-700/50
                ${event.highlight ? "border-primary/50" : "border-gray-700/50"}
              `}
              >
                <div className="p-6">
                  <div className="flex items-start gap-4">
                    <div
                      className={`
                      p-3 rounded-full 
                      ${event.highlight ? "bg-primary/20" : "bg-gray-800/60"}
                    `}
                    >
                      {iconMap[event.icon]}
                    </div>
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-2">
                        <span
                          className={`
                          text-2xl font-bold
                          ${event.highlight ? "text-primary" : "text-gray-400"}
                        `}
                        >
                          {event.year}
                        </span>
                        <h3 className="text-lg font-semibold text-white">{event.title}</h3>
                      </div>
                      <p className="text-gray-300 text-sm leading-relaxed">{event.description}</p>
                    </div>
                  </div>
                </div>
              </Card>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}

