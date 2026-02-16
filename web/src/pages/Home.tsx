import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { client } from '../api/client'
import { MY_TOPICS, ENTER_TOPIC } from '../api/operations'
import type { SessionTopic, TopicGraph } from '../api/types'
import { TopicInput } from '../components/home/TopicInput'
import { ContinueCard } from '../components/home/ContinueCard'

export function Home() {
  const navigate = useNavigate()
  const [topics, setTopics] = useState<SessionTopic[]>([])
  const [loading, setLoading] = useState(false)

  useEffect(() => {
    client
      .request<{ myTopics: SessionTopic[] }>(MY_TOPICS)
      .then((data) => setTopics(data.myTopics))
      .catch(() => {})
  }, [])

  const handleEnterTopic = async (interest: string) => {
    setLoading(true)
    try {
      const data = await client.request<{ enterTopic: TopicGraph }>(ENTER_TOPIC, { interest })
      navigate(`/topics/${data.enterTopic.topicRoot.id}`)
    } catch (e) {
      console.error('Failed to enter topic:', e)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="home-page">
      <h1>Our Meadow</h1>
      <TopicInput onSubmit={handleEnterTopic} loading={loading} />

      {topics.length > 0 && (
        <div className="continue-section">
          <h3>Continue exploring</h3>
          <div className="continue-list">
            {topics.map((topic) => (
              <ContinueCard
                key={topic.topicRoot.id}
                topic={topic}
                onClick={() => navigate(`/topics/${topic.topicRoot.id}`)}
              />
            ))}
          </div>
        </div>
      )}
    </div>
  )
}
