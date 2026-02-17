import type { Resource } from '../../api/types'

interface ResourcePreviewProps {
  resource: Resource
  onUpvote: () => void
}

function extractYoutubeId(url: string | null): string | null {
  if (!url) return null
  const match = url.match(/(?:youtube\.com\/watch\?v=|youtu\.be\/)([a-zA-Z0-9_-]{11})/)
  return match ? match[1] : null
}

export function ResourcePreview({ resource, onUpvote }: ResourcePreviewProps) {
  const videoId = resource.youtubeId || extractYoutubeId(resource.url)

  if (videoId) {
    return (
      <div className="mb-4 border border-meadow-border rounded-lg overflow-hidden">
        <iframe
          className="w-full border-none aspect-video"
          src={`https://www.youtube.com/embed/${videoId}`}
          title={resource.title}
          allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
          allowFullScreen
        />
        <div className="px-3 py-2 flex items-start justify-between gap-2">
          <div className="flex flex-col min-w-0">
            <span className="font-medium text-[13px]">{resource.title}</span>
            {resource.channel && <span className="text-[11px] text-meadow-muted">{resource.channel}</span>}
          </div>
          <button
            onClick={onUpvote}
            className="shrink-0 flex items-center gap-1 text-[11px] px-1.5 py-0.5 rounded text-meadow-muted hover:text-meadow-accent hover:bg-meadow-bg transition-colors"
            title="Upvote this resource"
          >
            <span>&#9650;</span>
            {resource.votes > 0 && <span>{resource.votes}</span>}
          </button>
        </div>
        <p className="text-xs text-meadow-muted px-3 pb-2 leading-snug">{resource.reason}</p>
      </div>
    )
  }

  return (
    <div className="block p-3 mb-4 border border-meadow-border rounded-lg">
      <div className="flex items-start justify-between gap-2">
        <a
          href={resource.url || '#'}
          target="_blank"
          rel="noopener noreferrer"
          className="min-w-0 no-underline text-meadow-text hover:text-meadow-accent transition-colors"
        >
          <span className="inline-block text-[10px] uppercase bg-meadow-bg px-1.5 py-0.5 rounded text-meadow-muted mb-1">{resource.type}</span>
          <span className="block font-medium text-[13px]">{resource.title}</span>
        </a>
        <button
          onClick={onUpvote}
          className="shrink-0 flex items-center gap-1 text-[11px] px-1.5 py-0.5 rounded text-meadow-muted hover:text-meadow-accent hover:bg-meadow-bg transition-colors"
          title="Upvote this resource"
        >
          <span>&#9650;</span>
          {resource.votes > 0 && <span>{resource.votes}</span>}
        </button>
      </div>
      <p className="text-xs text-meadow-muted leading-snug mt-1">{resource.reason}</p>
    </div>
  )
}
