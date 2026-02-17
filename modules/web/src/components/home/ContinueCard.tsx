import type { SessionTopic } from '../../api/types'

interface ContinueCardProps {
  topic: SessionTopic
  onClick: () => void
}

export function ContinueCard({ topic, onClick }: ContinueCardProps) {
  const lastActive = new Date(topic.lastActive)
  const timeAgo = getTimeAgo(lastActive)

  return (
    <button
      className="flex justify-between items-center px-4.5 py-3.5 bg-meadow-surface border border-meadow-border rounded-lg cursor-pointer text-left w-full hover:bg-meadow-bg hover:border-meadow-accent transition-colors"
      onClick={onClick}
    >
      <span className="font-medium text-meadow-text">{topic.topicRoot.name}</span>
      <span className="text-xs text-meadow-muted">{timeAgo}</span>
    </button>
  )
}

function getTimeAgo(date: Date): string {
  const seconds = Math.floor((Date.now() - date.getTime()) / 1000)
  if (seconds < 60) return 'just now'
  const minutes = Math.floor(seconds / 60)
  if (minutes < 60) return `${minutes}m ago`
  const hours = Math.floor(minutes / 60)
  if (hours < 24) return `${hours}h ago`
  const days = Math.floor(hours / 24)
  return `${days}d ago`
}
