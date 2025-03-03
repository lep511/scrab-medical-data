"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import type { VitalSign } from "@/types/patient"
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Legend } from "recharts"

interface VitalSignsProps {
  vitalSigns: VitalSign[]
}

export default function VitalSigns({ vitalSigns }: VitalSignsProps) {
  // Format dates for better display
  const formattedData = vitalSigns.map((vs) => ({
    ...vs,
    formattedDate: new Date(vs.date).toLocaleDateString("en-US", { month: "short", day: "numeric" }),
    systolic: Number.parseInt(vs.bloodPressure.split("/")[0]),
    diastolic: Number.parseInt(vs.bloodPressure.split("/")[1]),
  }))

  // Get the most recent vital signs
  const latestVitals = vitalSigns[vitalSigns.length - 1]

  return (
    <div className="p-6">
      <div className="mb-6">
        <h2 className="text-xl font-bold mb-4">Vital Signs</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
          <Card className="bg-gray-800/60 backdrop-blur-sm border-gray-700/50">
            <CardHeader className="pb-2">
              <CardDescription className="text-gray-400">Heart Rate</CardDescription>
              <CardTitle className="text-2xl text-white">
                {latestVitals.heartRate} <span className="text-sm text-gray-400">bpm</span>
              </CardTitle>
            </CardHeader>
          </Card>
          <Card className="bg-gray-800/60 backdrop-blur-sm border-gray-700/50">
            <CardHeader className="pb-2">
              <CardDescription className="text-gray-400">Blood Pressure</CardDescription>
              <CardTitle className="text-2xl text-white">
                {latestVitals.bloodPressure} <span className="text-sm text-gray-400">mmHg</span>
              </CardTitle>
            </CardHeader>
          </Card>
          <Card className="bg-gray-800/60 backdrop-blur-sm border-gray-700/50">
            <CardHeader className="pb-2">
              <CardDescription className="text-gray-400">Temperature</CardDescription>
              <CardTitle className="text-2xl text-white">
                {latestVitals.temperature}°<span className="text-sm text-gray-400">F</span>
              </CardTitle>
            </CardHeader>
          </Card>
          <Card className="bg-gray-800/60 backdrop-blur-sm border-gray-700/50">
            <CardHeader className="pb-2">
              <CardDescription className="text-gray-400">Respiratory Rate</CardDescription>
              <CardTitle className="text-2xl text-white">
                {latestVitals.respiratoryRate} <span className="text-sm text-gray-400">bpm</span>
              </CardTitle>
            </CardHeader>
          </Card>
          <Card className="bg-gray-800/60 backdrop-blur-sm border-gray-700/50">
            <CardHeader className="pb-2">
              <CardDescription className="text-gray-400">Oxygen Saturation</CardDescription>
              <CardTitle className="text-2xl text-white">
                {latestVitals.oxygenSaturation}
                <span className="text-sm text-gray-400">%</span>
              </CardTitle>
            </CardHeader>
          </Card>
        </div>
      </div>

      <Tabs defaultValue="heartRate">
        <TabsList className="bg-gray-800 border-gray-700 mb-6">
          <TabsTrigger value="heartRate">Heart Rate</TabsTrigger>
          <TabsTrigger value="bloodPressure">Blood Pressure</TabsTrigger>
          <TabsTrigger value="temperature">Temperature</TabsTrigger>
        </TabsList>

        <TabsContent value="heartRate">
          <Card className="bg-gray-800/60 backdrop-blur-sm border-gray-700/50">
            <CardContent className="pt-6">
              <div className="h-[300px]">
                <ResponsiveContainer width="100%" height="100%">
                  <LineChart data={formattedData}>
                    <CartesianGrid strokeDasharray="3 3" stroke="#444" />
                    <XAxis dataKey="formattedDate" stroke="#aaa" />
                    <YAxis stroke="#aaa" domain={["dataMin - 5", "dataMax + 5"]} />
                    <Tooltip
                      contentStyle={{
                        backgroundColor: "rgba(51, 51, 51, 0.8)",
                        backdropFilter: "blur(8px)",
                        border: "1px solid rgba(85, 85, 85, 0.5)",
                      }}
                      labelStyle={{ color: "#fff" }}
                    />
                    <Line
                      type="monotone"
                      dataKey="heartRate"
                      stroke="#10b981"
                      strokeWidth={2}
                      dot={{ r: 4, strokeWidth: 2 }}
                      activeDot={{ r: 6, strokeWidth: 2 }}
                    />
                  </LineChart>
                </ResponsiveContainer>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="bloodPressure">
          <Card className="bg-gray-800/60 backdrop-blur-sm border-gray-700/50">
            <CardContent className="pt-6">
              <div className="h-[300px]">
                <ResponsiveContainer width="100%" height="100%">
                  <LineChart data={formattedData}>
                    <CartesianGrid strokeDasharray="3 3" stroke="#444" />
                    <XAxis dataKey="formattedDate" stroke="#aaa" />
                    <YAxis stroke="#aaa" domain={[60, 160]} />
                    <Tooltip
                      contentStyle={{
                        backgroundColor: "rgba(51, 51, 51, 0.8)",
                        backdropFilter: "blur(8px)",
                        border: "1px solid rgba(85, 85, 85, 0.5)",
                      }}
                      labelStyle={{ color: "#fff" }}
                    />
                    <Legend />
                    <Line
                      type="monotone"
                      dataKey="systolic"
                      stroke="#ef4444"
                      strokeWidth={2}
                      dot={{ r: 4, strokeWidth: 2 }}
                      activeDot={{ r: 6, strokeWidth: 2 }}
                      name="Systolic"
                    />
                    <Line
                      type="monotone"
                      dataKey="diastolic"
                      stroke="#3b82f6"
                      strokeWidth={2}
                      dot={{ r: 4, strokeWidth: 2 }}
                      activeDot={{ r: 6, strokeWidth: 2 }}
                      name="Diastolic"
                    />
                  </LineChart>
                </ResponsiveContainer>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="temperature">
          <Card className="bg-gray-800/60 backdrop-blur-sm border-gray-700/50">
            <CardContent className="pt-6">
              <div className="h-[300px]">
                <ResponsiveContainer width="100%" height="100%">
                  <LineChart data={formattedData}>
                    <CartesianGrid strokeDasharray="3 3" stroke="#444" />
                    <XAxis dataKey="formattedDate" stroke="#aaa" />
                    <YAxis stroke="#aaa" domain={[97, 100]} />
                    <Tooltip
                      contentStyle={{
                        backgroundColor: "rgba(51, 51, 51, 0.8)",
                        backdropFilter: "blur(8px)",
                        border: "1px solid rgba(85, 85, 85, 0.5)",
                      }}
                      labelStyle={{ color: "#fff" }}
                    />
                    <Line
                      type="monotone"
                      dataKey="temperature"
                      stroke="#fbbf24"
                      strokeWidth={2}
                      dot={{ r: 4, strokeWidth: 2 }}
                      activeDot={{ r: 6, strokeWidth: 2 }}
                    />
                  </LineChart>
                </ResponsiveContainer>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  )
}

