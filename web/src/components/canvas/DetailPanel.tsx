import { useState } from 'react'
import type { GraphNode } from '../../api/types'
import { ResourcePreview } from './ResourcePreview'
import { AskPanel } from './AskPanel'

interface DetailPanelProps {
  node: GraphNode
  onClose: () => void
  onLeaveNote: (nodeId: string, body: string) => Promise<void>
  onAsk: (question: string) => Promise<string>
}

export function DetailPanel({ node, onClose, onLeaveNote, onAsk }: DetailPanelProps) {
  const [noteBody, setNoteBody] = useState('')
  const [submittingNote, setSubmittingNote] = useState(false)

  const handleLeaveNote = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!noteBody.trim()) return
    setSubmittingNote(true)
    await onLeaveNote(node.id, noteBody)
    setNoteBody('')
    setSubmittingNote(false)
  }

  return (
    <div className="detail-panel">
      <div className="detail-header">
        <h3>{node.title}</h3>
        <button className="detail-close" onClick={onClose}>×</button>
      </div>

      <p className="detail-description">{node.description}</p>

      {node.resources.length > 0 && (
        <div className="detail-resources">
          <h4>Resources</h4>
          {node.resources.map((r, i) => (
            <ResourcePreview key={i} resource={r} />
          ))}
        </div>
      )}

      {node.notes.length > 0 && (
        <div className="detail-notes">
          <h4>Trail Notes</h4>
          {node.notes.map((note) => (
            <div key={note.id} className="trail-note">
              <p>{note.body}</p>
              <span className="note-time">{new Date(note.createdAt).toLocaleDateString()}</span>
            </div>
          ))}
        </div>
      )}

      <form onSubmit={handleLeaveNote} className="note-form">
        <input
          type="text"
          value={noteBody}
          onChange={(e) => setNoteBody(e.target.value)}
          placeholder="Leave a trail note..."
          disabled={submittingNote}
        />
        <button type="submit" disabled={submittingNote || !noteBody.trim()}>
          Note
        </button>
      </form>

      <AskPanel onAsk={onAsk} />
    </div>
  )
}
