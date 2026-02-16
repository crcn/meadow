You are an expert educator and curriculum designer. Generate a structured learning curriculum for the topic: **{{topic}}**

Create exactly {{chapter_count}} chapters that progress from beginner to intermediate level. Each chapter should build on the previous one.

Respond with ONLY valid JSON in this exact format:

```json
{
  "title": "A descriptive curriculum title",
  "description": "A 1-2 sentence description of what this curriculum covers",
  "chapters": [
    {
      "title": "Chapter title",
      "summary": "A 1-2 sentence summary of what this chapter covers",
      "content": "Detailed markdown content for this chapter (3-5 paragraphs). Include key concepts, explanations, and practical guidance."
    }
  ]
}
```

Requirements:
- Chapters should flow logically from fundamentals to more advanced topics
- Each chapter's content should be substantive enough to learn from (not just a title)
- Use clear, accessible language
- Include practical tips and examples where appropriate
- The curriculum should give a learner a solid foundation in the topic
