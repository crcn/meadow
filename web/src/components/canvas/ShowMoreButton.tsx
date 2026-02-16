interface ShowMoreButtonProps {
  onClick: () => void
  loading: boolean
}

export function ShowMoreButton({ onClick, loading }: ShowMoreButtonProps) {
  return (
    <button className="show-more-button" onClick={onClick} disabled={loading}>
      {loading ? 'Exploring...' : 'Show me more'}
    </button>
  )
}
