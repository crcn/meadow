CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Members (learner accounts, no PII stored)
CREATE TABLE members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_active_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Identifiers (hashed phone numbers → members)
-- Phone numbers are SHA256 hashed, never stored raw
CREATE TABLE identifiers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    member_id UUID NOT NULL REFERENCES members(id) ON DELETE CASCADE,
    phone_hash VARCHAR(64) NOT NULL UNIQUE,
    is_admin BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_identifiers_phone_hash ON identifiers(phone_hash);
CREATE INDEX idx_identifiers_member_id ON identifiers(member_id);

-- Traversal history per member per topic
CREATE TABLE traversal_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    member_id UUID NOT NULL REFERENCES members(id),
    topic_root_id UUID NOT NULL,
    node_id UUID NOT NULL,
    previous_node_id UUID,
    movement_type TEXT,
    is_backtrack BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_traversal_member ON traversal_history(member_id, topic_root_id);
CREATE INDEX idx_traversal_node ON traversal_history(node_id);

-- Active position per member per topic
CREATE TABLE learner_position (
    member_id UUID NOT NULL REFERENCES members(id),
    topic_root_id UUID NOT NULL,
    current_node_id UUID NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (member_id, topic_root_id)
);

-- Notes left by learners on nodes (trail markers)
CREATE TABLE node_notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    node_id UUID NOT NULL,
    member_id UUID NOT NULL REFERENCES members(id),
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_node_notes_node ON node_notes(node_id);
CREATE INDEX idx_node_notes_member ON node_notes(member_id);
