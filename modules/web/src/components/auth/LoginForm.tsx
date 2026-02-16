import { useState } from 'react'

interface LoginFormProps {
  onSendOtp: (phone: string) => Promise<boolean>
  onOtpSent: (phone: string) => void
  loading: boolean
  error: string | null
}

export function LoginForm({ onSendOtp, onOtpSent, loading, error }: LoginFormProps) {
  const [phone, setPhone] = useState('')

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    const success = await onSendOtp(phone)
    if (success) onOtpSent(phone)
  }

  return (
    <form onSubmit={handleSubmit} className="login-form">
      <h2>Welcome to Our Meadow</h2>
      <p>Enter your phone number to get started</p>
      <input
        type="tel"
        value={phone}
        onChange={(e) => setPhone(e.target.value)}
        placeholder="+1234567890"
        required
        autoFocus
      />
      <button type="submit" disabled={loading || !phone.trim()}>
        {loading ? 'Sending...' : 'Send Code'}
      </button>
      {error && <p className="error">{error}</p>}
    </form>
  )
}
