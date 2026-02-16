import { gql } from 'graphql-request'

// ─── Fragments ───────────────────────────────────────────────────────

const TOPIC_GRAPH_FRAGMENT = gql`
  fragment TopicGraphFields on TopicGraph {
    topicRoot {
      id
      name
      description
    }
    currentNodeId
    nodes {
      id
      title
      description
      resources {
        type
        youtubeId
        url
        title
        channel
        reason
      }
      notes {
        id
        body
        createdAt
      }
      state
      movement
      isWildcard
      visitCount
    }
    edges {
      id
      sourceId
      targetId
      movement
      state
      weight
    }
  }
`

// ─── Queries ─────────────────────────────────────────────────────────

export const MY_TOPICS = gql`
  query MyTopics {
    myTopics {
      topicRoot {
        id
        name
        description
      }
      currentNodeId
      lastActive
    }
  }
`

export const TOPIC_GRAPH = gql`
  query TopicGraph($topicRootId: ID!) {
    topicGraph(topicRootId: $topicRootId) {
      ...TopicGraphFields
    }
  }
  ${TOPIC_GRAPH_FRAGMENT}
`

// ─── Mutations ───────────────────────────────────────────────────────

export const SEND_OTP = gql`
  mutation SendOtp($phone: String!) {
    sendOtp(phone: $phone) {
      success
    }
  }
`

export const VERIFY_OTP = gql`
  mutation VerifyOtp($phone: String!, $code: String!) {
    verifyOtp(phone: $phone, code: $code) {
      memberId
      token
    }
  }
`

export const ENTER_TOPIC = gql`
  mutation EnterTopic($interest: String!) {
    enterTopic(interest: $interest) {
      ...TopicGraphFields
    }
  }
  ${TOPIC_GRAPH_FRAGMENT}
`

export const TRAVERSE = gql`
  mutation Traverse($topicRootId: ID!, $fromNodeId: ID!, $toNodeId: ID!, $movement: Movement!) {
    traverse(topicRootId: $topicRootId, fromNodeId: $fromNodeId, toNodeId: $toNodeId, movement: $movement) {
      ...TopicGraphFields
    }
  }
  ${TOPIC_GRAPH_FRAGMENT}
`

export const BACK_UP = gql`
  mutation BackUp($topicRootId: ID!) {
    backUp(topicRootId: $topicRootId) {
      ...TopicGraphFields
    }
  }
  ${TOPIC_GRAPH_FRAGMENT}
`

export const SHOW_MORE = gql`
  mutation ShowMore($topicRootId: ID!, $nodeId: ID!, $nodeTitle: String!) {
    showMore(topicRootId: $topicRootId, nodeId: $nodeId, nodeTitle: $nodeTitle) {
      ...TopicGraphFields
    }
  }
  ${TOPIC_GRAPH_FRAGMENT}
`

export const LEAVE_NOTE = gql`
  mutation LeaveNote($nodeId: ID!, $body: String!) {
    leaveNote(nodeId: $nodeId, body: $body) {
      ...TopicGraphFields
    }
  }
  ${TOPIC_GRAPH_FRAGMENT}
`

export const ASK_ABOUT_NODE = gql`
  mutation AskAboutNode($topicRootId: ID!, $nodeId: ID!, $question: String!) {
    askAboutNode(topicRootId: $topicRootId, nodeId: $nodeId, question: $question) {
      answer
    }
  }
`
