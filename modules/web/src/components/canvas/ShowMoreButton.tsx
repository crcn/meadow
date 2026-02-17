interface ShowMoreButtonProps {
  onClick: () => void
  loading: boolean
}

export function ShowMoreButton({ onClick, loading }: ShowMoreButtonProps) {
  return (
    <button
      className="absolute bottom-6 left-1/2 -translate-x-1/2 z-10 px-6 py-2.5 rounded-full shadow-md bg-meadow-accent text-white text-[13px] cursor-pointer hover:bg-meadow-accent-hover disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
      onClick={onClick}
      disabled={loading}
    >
      {loading ? 'Exploring...' : 'Show me more'}
    </button>
  )
}
