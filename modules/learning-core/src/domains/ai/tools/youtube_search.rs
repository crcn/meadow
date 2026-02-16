use ai_client::tool::{Tool, ToolDefinition};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct YoutubeSearchTool {
    api_key: String,
}

impl YoutubeSearchTool {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[derive(Debug, Deserialize)]
pub struct YoutubeSearchArgs {
    pub query: String,
}

#[derive(Debug, Serialize)]
pub struct YoutubeSearchResult {
    pub videos: Vec<YoutubeVideo>,
}

#[derive(Debug, Serialize)]
pub struct YoutubeVideo {
    pub video_id: String,
    pub title: String,
    pub channel: String,
    pub description_snippet: String,
}

#[derive(Debug, thiserror::Error)]
pub enum YoutubeError {
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("api error: {0}")]
    Api(String),
}

#[derive(Deserialize)]
struct YTSearchResponse {
    items: Vec<YTSearchItem>,
}

#[derive(Deserialize)]
struct YTSearchItem {
    id: YTVideoId,
    snippet: YTSnippet,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct YTVideoId {
    video_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct YTSnippet {
    title: String,
    channel_title: String,
    description: String,
}

#[async_trait]
impl Tool for YoutubeSearchTool {
    const NAME: &'static str = "youtube_search";
    type Error = YoutubeError;
    type Args = YoutubeSearchArgs;
    type Output = YoutubeSearchResult;

    async fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search YouTube for educational videos. Returns video IDs, titles, channels, and description snippets.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query for finding YouTube videos"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/youtube/v3/search")
            .query(&[
                ("part", "snippet"),
                ("q", &args.query),
                ("type", "video"),
                ("maxResults", "5"),
                ("relevanceLanguage", "en"),
                ("key", &self.api_key),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(YoutubeError::Api(text));
        }

        let api_response: YTSearchResponse = response.json().await?;

        Ok(YoutubeSearchResult {
            videos: api_response
                .items
                .into_iter()
                .filter_map(|item| {
                    Some(YoutubeVideo {
                        video_id: item.id.video_id?,
                        title: item.snippet.title,
                        channel: item.snippet.channel_title,
                        description_snippet: item.snippet.description,
                    })
                })
                .collect(),
        })
    }
}
