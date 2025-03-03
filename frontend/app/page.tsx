import PatientDashboard from "@/components/patient-dashboard"

export default function Home() {
  return (
    <main className="min-h-screen bg-black text-white bg-[url('https://hebbkx1anhila5yf.public.blob.vercel-storage.com/image-YzCK6XSFx8JzeWclGuea8IW3Ogtf3G.png')] bg-cover bg-center bg-no-repeat">
      <div className="min-h-screen backdrop-blur-sm bg-black/30">
        <PatientDashboard />
      </div>
    </main>
  )
}
