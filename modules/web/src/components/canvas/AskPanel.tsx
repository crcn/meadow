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
    } catch {
      setAnswer('Failed to get an answer. Please try again.')
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="mt-6 pt-4 border-t border-meadow-border">
      <h4 className="text-xs uppercase tracking-wide text-meadow-muted mb-3">Ask about this node</h4>
      {answer && <div className="bg-meadow-bg p-3 rounded-lg text-[13px] leading-relaxed mb-3 whitespace-pre-wrap">{answer}</div>}
      <form onSubmit={handleSubmit} className="flex gap-2">
        <input
          type="text"
          value={question}
          onChange={(e) => setQuestion(e.target.value)}
          placeholder="Ask a question..."
          disabled={loading}
          className="flex-1 text-[13px] px-2.5 py-2 border border-meadow-border rounded-lg outline-none focus:border-meadow-accent"
        />
        <button
          type="submit"
          disabled={loading || !question.trim()}
          className="text-xs px-3.5 py-2 bg-meadow-accent text-white rounded-lg cursor-pointer hover:bg-meadow-accent-hover disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? '...' : 'Ask'}
        </button>
      </form>
    </div>
  )
}
