use ai_client::tool::{Tool, ToolDefinition};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct YoutubeDetailsTool {
    api_key: String,
}

impl YoutubeDetailsTool {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[derive(Debug, Deserialize)]
pub struct YoutubeDetailsArgs {
    pub video_id: String,
}

#[derive(Debug, Serialize)]
pub struct YoutubeDetailsResult {
    pub title: String,
    pub channel: String,
    pub description: String,
    pub view_count: String,
    pub duration: String,
    pub default_audio_language: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum YoutubeDetailsError {
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("not found")]
    NotFound,
    #[error("api error: {0}")]
    Api(String),
}

#[derive(Deserialize)]
struct YTVideoResponse {
    items: Vec<YTVideoItem>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct YTVideoItem {
    snippet: YTVideoSnippet,
    content_details: YTContentDetails,
    statistics: YTStatistics,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct YTVideoSnippet {
    title: String,
    channel_title: String,
    description: String,
    default_audio_language: Option<String>,
}

#[derive(Deserialize)]
struct YTContentDetails {
    duration: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct YTStatistics {
    view_count: Option<String>,
}

#[async_trait]
impl Tool for YoutubeDetailsTool {
    const NAME: &'static str = "youtube_details";
    type Error = YoutubeDetailsError;
    type Args = YoutubeDetailsArgs;
    type Output = YoutubeDetailsResult;

    async fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Get full details for a YouTube video including description, view count, and duration. Use this to judge video quality after searching.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "video_id": {
                        "type": "string",
                        "description": "YouTube video ID"
                    }
                },
                "required": ["video_id"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/youtube/v3/videos")
            .query(&[
                ("part", "snippet,contentDetails,statistics"),
                ("id", &args.video_id),
                ("key", &self.api_key),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(YoutubeDetailsError::Api(text));
        }

        let api_response: YTVideoResponse = response.json().await?;
        let item = api_response.items.into_iter().next().ok_or(YoutubeDetailsError::NotFound)?;

        Ok(YoutubeDetailsResult {
            title: item.snippet.title,
            channel: item.snippet.channel_title,
            description: item.snippet.description,
            view_count: item.statistics.view_count.unwrap_or_default(),
            duration: item.content_details.duration,
            default_audio_language: item.snippet.default_audio_language,
        })
    }
}
