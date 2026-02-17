import { useState } from 'react'

interface TopicInputProps {
  onSubmit: (interest: string) => void
  loading: boolean
}

export function TopicInput({ onSubmit, loading }: TopicInputProps) {
  const [interest, setInterest] = useState('')

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (interest.trim()) onSubmit(interest.trim())
  }

  return (
    <form onSubmit={handleSubmit} className="flex gap-2 mb-12">
      <input
        type="text"
        value={interest}
        onChange={(e) => setInterest(e.target.value)}
        placeholder="What do you want to learn about?"
        autoFocus
        className="flex-1 px-3.5 py-2.5 border border-meadow-border rounded-lg text-base outline-none transition-colors focus:border-meadow-accent"
      />
      <button
        type="submit"
        disabled={loading || !interest.trim()}
        className="px-5 py-2.5 bg-meadow-accent text-white rounded-lg text-sm cursor-pointer transition-colors hover:bg-meadow-accent-hover disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {loading ? 'Exploring...' : 'Explore'}
      </button>
    </form>
  )
}
