import type { SessionTopic } from '../../api/types'

interface ContinueCardProps {
  topic: SessionTopic
  onClick: () => void
}

export function ContinueCard({ topic, onClick }: ContinueCardProps) {
  const lastActive = new Date(topic.lastActive)
  const timeAgo = getTimeAgo(lastActive)

  return (
    <button className="continue-card" onClick={onClick}>
      <span className="continue-card-name">{topic.topicRoot.name}</span>
      <span className="continue-card-time">{timeAgo}</span>
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
