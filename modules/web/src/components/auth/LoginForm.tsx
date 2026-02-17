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
    <form onSubmit={handleSubmit} className="bg-meadow-surface p-10 rounded-xl shadow-sm w-full max-w-[380px] flex flex-col gap-4">
      <h2 className="text-2xl font-semibold">Welcome to Our Meadow</h2>
      <p className="text-meadow-muted text-sm">Enter your phone number to get started</p>
      <input
        type="tel"
        value={phone}
        onChange={(e) => setPhone(e.target.value)}
        placeholder="+1234567890"
        required
        autoFocus
        className="px-3.5 py-2.5 border border-meadow-border rounded-lg text-base w-full outline-none transition-colors focus:border-meadow-accent"
      />
      <button
        type="submit"
        disabled={loading || !phone.trim()}
        className="px-5 py-2.5 bg-meadow-accent text-white rounded-lg text-sm cursor-pointer transition-colors hover:bg-meadow-accent-hover disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {loading ? 'Sending...' : 'Send Code'}
      </button>
      {error && <p className="text-meadow-error text-[13px] mt-2">{error}</p>}
    </form>
  )
}
