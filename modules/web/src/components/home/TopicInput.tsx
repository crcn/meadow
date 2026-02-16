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
    <form onSubmit={handleSubmit} className="topic-input">
      <input
        type="text"
        value={interest}
        onChange={(e) => setInterest(e.target.value)}
        placeholder="What do you want to learn about?"
        autoFocus
      />
      <button type="submit" disabled={loading || !interest.trim()}>
        {loading ? 'Exploring...' : 'Explore'}
      </button>
    </form>
  )
}
