import type { Resource } from '../../api/types'

interface ResourcePreviewProps {
  resource: Resource
}

export function ResourcePreview({ resource }: ResourcePreviewProps) {
  if (resource.youtubeId) {
    return (
      <div className="resource-preview resource-youtube">
        <iframe
          src={`https://www.youtube.com/embed/${resource.youtubeId}`}
          title={resource.title}
          allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
          allowFullScreen
        />
        <div className="resource-info">
          <span className="resource-title">{resource.title}</span>
          {resource.channel && <span className="resource-channel">{resource.channel}</span>}
        </div>
        <p className="resource-reason">{resource.reason}</p>
      </div>
    )
  }

  return (
    <a
      href={resource.url || '#'}
      target="_blank"
      rel="noopener noreferrer"
      className="resource-preview resource-article"
    >
      <span className="resource-type-badge">{resource.type}</span>
      <span className="resource-title">{resource.title}</span>
      <p className="resource-reason">{resource.reason}</p>
    </a>
  )
}
