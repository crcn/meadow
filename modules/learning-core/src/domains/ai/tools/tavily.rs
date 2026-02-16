use ai_client::tool::{Tool, ToolDefinition};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct TavilySearchTool {
    api_key: String,
}

impl TavilySearchTool {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[derive(Debug, Deserialize)]
pub struct TavilySearchArgs {
    pub query: String,
}

#[derive(Debug, Serialize)]
pub struct TavilySearchResult {
    pub results: Vec<TavilyResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TavilyResult {
    pub title: String,
    pub url: String,
    pub content: String,
}

#[derive(Debug, thiserror::Error)]
pub enum TavilyError {
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("api error: {0}")]
    Api(String),
}

#[derive(Deserialize)]
struct TavilyApiResponse {
    results: Vec<TavilyApiResult>,
}

#[derive(Deserialize)]
struct TavilyApiResult {
    title: String,
    url: String,
    content: String,
}

#[async_trait]
impl Tool for TavilySearchTool {
    const NAME: &'static str = "tavily_search";
    type Error = TavilyError;
    type Args = TavilySearchArgs;
    type Output = TavilySearchResult;

    async fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search the web for articles, guides, tutorials, and learning resources. Returns titles, URLs, and content snippets.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query for finding learning resources"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.tavily.com/search")
            .json(&serde_json::json!({
                "api_key": self.api_key,
                "query": args.query,
                "search_depth": "basic",
                "max_results": 5,
                "include_answer": false
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(TavilyError::Api(text));
        }

        let api_response: TavilyApiResponse = response.json().await?;

        Ok(TavilySearchResult {
            results: api_response
                .results
                .into_iter()
                .map(|r| TavilyResult {
                    title: r.title,
                    url: r.url,
                    content: r.content,
                })
                .collect(),
        })
    }
}
