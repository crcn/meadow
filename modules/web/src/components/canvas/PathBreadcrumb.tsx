import type { GraphNode } from '../../api/types'
import { NodeState } from '../../api/types'

interface PathBreadcrumbProps {
  nodes: GraphNode[]
  onNodeClick: (nodeId: string) => void
}

export function PathBreadcrumb({ nodes, onNodeClick }: PathBreadcrumbProps) {
  // Show topic root + visited nodes + current node in order
  const pathNodes = nodes.filter(
    (n) => n.state === NodeState.TOPIC_ROOT || n.state === NodeState.VISITED || n.state === NodeState.CURRENT,
  )

  if (pathNodes.length <= 1) return null

  return (
    <div className="path-breadcrumb">
      {pathNodes.map((node, i) => (
        <span key={node.id}>
          {i > 0 && <span className="breadcrumb-arrow"> → </span>}
          <button
            className={`breadcrumb-item ${node.state === NodeState.CURRENT ? 'breadcrumb-current' : ''}`}
            onClick={() => onNodeClick(node.id)}
          >
            {node.title}
          </button>
        </span>
      ))}
    </div>
  )
}
