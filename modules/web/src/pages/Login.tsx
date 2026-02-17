import { useState } from 'react'
import { Navigate } from 'react-router-dom'
import { LoginForm } from '../components/auth/LoginForm'
import { OtpInput } from '../components/auth/OtpInput'
import { useAuth } from '../hooks/useAuth'

export function Login() {
  const { isAuthenticated, loading, error, sendOtp, verifyOtp } = useAuth()
  const [phone, setPhone] = useState<string | null>(null)

  if (isAuthenticated) {
    return <Navigate to="/" replace />
  }

  return (
    <div className="flex items-center justify-center min-h-screen">
      {phone ? (
        <OtpInput
          phone={phone}
          onVerify={verifyOtp}
          loading={loading}
          error={error}
        />
      ) : (
        <LoginForm
          onSendOtp={sendOtp}
          onOtpSent={setPhone}
          loading={loading}
          error={error}
        />
      )}
    </div>
  )
}
