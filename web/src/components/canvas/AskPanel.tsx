import { useState } from 'react'

interface AskPanelProps {
  onAsk: (question: string) => Promise<string>
}

export function AskPanel({ onAsk }: AskPanelProps) {
  const [question, setQuestion] = useState('')
  const [answer, setAnswer] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!question.trim()) return
    setLoading(true)
    try {
      const result = await onAsk(question)
      setAnswer(result)
    } catch (err) {
      setAnswer('Failed to get an answer. Please try again.')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="ask-panel">
      <h4>Ask about this node</h4>
      {answer && <div className="ask-answer">{answer}</div>}
      <form onSubmit={handleSubmit} className="ask-form">
        <input
          type="text"
          value={question}
          onChange={(e) => setQuestion(e.target.value)}
          placeholder="Ask a question..."
          disabled={loading}
        />
        <button type="submit" disabled={loading || !question.trim()}>
          {loading ? '...' : 'Ask'}
        </button>
      </form>
    </div>
  )
}
