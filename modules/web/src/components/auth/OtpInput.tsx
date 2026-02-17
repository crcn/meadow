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
    <form onSubmit={handleSubmit} className="bg-meadow-surface p-10 rounded-xl shadow-sm w-full max-w-[380px] flex flex-col gap-4">
      <h2 className="text-2xl font-semibold">Enter your code</h2>
      <p className="text-meadow-muted text-sm">We sent a code to {phone}</p>
      <input
        type="text"
        value={code}
        onChange={(e) => setCode(e.target.value)}
        placeholder="123456"
        maxLength={6}
        required
        autoFocus
        className="px-3.5 py-2.5 border border-meadow-border rounded-lg text-base w-full outline-none transition-colors focus:border-meadow-accent"
      />
      <button
        type="submit"
        disabled={loading || code.length < 4}
        className="px-5 py-2.5 bg-meadow-accent text-white rounded-lg text-sm cursor-pointer transition-colors hover:bg-meadow-accent-hover disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {loading ? 'Verifying...' : 'Verify'}
      </button>
      {error && <p className="text-meadow-error text-[13px] mt-2">{error}</p>}
    </form>
  )
}
