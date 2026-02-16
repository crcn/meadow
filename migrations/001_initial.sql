CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE curricula (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title       TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    topic       TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TYPE node_type AS ENUM ('chapter', 'page');
CREATE TYPE node_status AS ENUM ('ready', 'generating', 'error');

CREATE TABLE nodes (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    curriculum_id   UUID NOT NULL REFERENCES curricula(id) ON DELETE CASCADE,
    title           TEXT NOT NULL,
    summary         TEXT NOT NULL DEFAULT '',
    content         TEXT NOT NULL DEFAULT '',
    node_type       node_type NOT NULL DEFAULT 'chapter',
    depth           INTEGER NOT NULL DEFAULT 0,
    position        INTEGER NOT NULL DEFAULT 0,
    status          node_status NOT NULL DEFAULT 'ready',
    parent_node_id  UUID REFERENCES nodes(id) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_nodes_curriculum ON nodes(curriculum_id);
CREATE INDEX idx_nodes_parent ON nodes(parent_node_id);

CREATE TYPE edge_type AS ENUM ('next', 'deeper', 'sibling');

CREATE TABLE edges (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_node   UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    to_node     UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    edge_type   edge_type NOT NULL,
    position    INTEGER NOT NULL DEFAULT 0,
    UNIQUE(from_node, to_node, edge_type)
);

CREATE INDEX idx_edges_from ON edges(from_node);
CREATE INDEX idx_edges_to ON edges(to_node);

CREATE TABLE videos (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    node_id         UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    youtube_id      TEXT NOT NULL,
    title           TEXT NOT NULL,
    channel_name    TEXT NOT NULL DEFAULT '',
    thumbnail_url   TEXT NOT NULL DEFAULT '',
    duration_secs   INTEGER NOT NULL DEFAULT 0,
    view_count      BIGINT NOT NULL DEFAULT 0,
    rank            INTEGER NOT NULL DEFAULT 0,
    relevance_score REAL NOT NULL DEFAULT 0.0,
    ai_rationale    TEXT NOT NULL DEFAULT '',
    UNIQUE(node_id, youtube_id)
);

CREATE INDEX idx_videos_node ON videos(node_id);

CREATE TABLE navigation_state (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    curriculum_id   UUID NOT NULL UNIQUE REFERENCES curricula(id) ON DELETE CASCADE,
    current_node_id UUID REFERENCES nodes(id) ON DELETE SET NULL,
    node_stack      JSONB NOT NULL DEFAULT '[]',
    visited_nodes   JSONB NOT NULL DEFAULT '[]'
);
