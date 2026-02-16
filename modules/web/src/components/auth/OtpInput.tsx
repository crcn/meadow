import { useState } from 'react'

interface OtpInputProps {
  phone: string
  onVerify: (phone: string, code: string) => Promise<boolean>
  loading: boolean
  error: string | null
}

export function OtpInput({ phone, onVerify, loading, error }: OtpInputProps) {
  const [code, setCode] = useState('')

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    await onVerify(phone, code)
  }

  return (
    <form onSubmit={handleSubmit} className="otp-form">
      <h2>Enter your code</h2>
      <p>We sent a code to {phone}</p>
      <input
        type="text"
        value={code}
        onChange={(e) => setCode(e.target.value)}
        placeholder="123456"
        maxLength={6}
        required
        autoFocus
      />
      <button type="submit" disabled={loading || code.length < 4}>
        {loading ? 'Verifying...' : 'Verify'}
      </button>
      {error && <p className="error">{error}</p>}
    </form>
  )
}
