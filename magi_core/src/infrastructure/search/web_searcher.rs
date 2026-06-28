use crate::domain::SearchProvider;
use reqwest::Client;
use anyhow::{Result, anyhow};
use std::env;
use async_trait::async_trait;

pub struct TavilySearcher {
    client: Client,
    api_key: Option<String>,
    sidecar_url: String,
}

impl TavilySearcher {
    pub fn new() -> Result<Self> {
        let api_key = env::var("TAVILY_API_KEY").ok();
        let sidecar_url = env::var("SEARCH_SIDECAR_URL").unwrap_or_else(|_| "http://127.0.0.1:8001".to_string());
        
        if api_key.is_none() {
            println!("[MAGI] TAVILY_API_KEY missing. Web search will fallback to Local Sidecar: {}", sidecar_url);
        }

        Ok(Self {
            client: Client::new(),
            api_key,
            sidecar_url,
        })
    }

    async fn search_via_sidecar(&self, query: &str) -> Result<String> {
        let url = format!("{}/search?q={}", self.sidecar_url, urlencoding::encode(query));
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Sidecar Error: {}", response.status()));
        }

        let sidecar_res: serde_json::Value = response.json().await?;
        let mut context = String::from("[LIVE WEB SEARCH RESULTS (LOCAL SIDECAR)]:\n");
        
        if let Some(results) = sidecar_res["results"].as_array() {
            for res in results {
                let title = res["title"].as_str().unwrap_or("No Title");
                let url = res["url"].as_str().unwrap_or("No URL");
                let content = res["content"].as_str().unwrap_or("No Content");
                context.push_str(&format!("- Title: {}\n  URL: {}\n  Summary: {}\n\n", title, url, content));
            }
        }
        Ok(context)
    }
}

#[async_trait]
impl SearchProvider for TavilySearcher {
    async fn search(&self, query: &str) -> Result<String> {
        if let Some(ref api_key) = self.api_key {
            let url = "https://api.tavily.com/search";
            let body = serde_json::json!({
                "api_key": api_key,
                "query": query,
                "search_depth": "advanced",
                "include_answer": false,
                "max_results": 3
            });

            let response = self.client.post(url).json(&body).send().await?;

            if response.status().is_success() {
                let tavily_res: serde_json::Value = response.json().await?;
                let mut context = String::from("[LIVE WEB SEARCH RESULTS (TAVILY)]:\n");
                if let Some(results) = tavily_res["results"].as_array() {
                    for res in results {
                        let title = res["title"].as_str().unwrap_or("No Title");
                        let url = res["url"].as_str().unwrap_or("No URL");
                        let content = res["content"].as_str().unwrap_or("No Content");
                        context.push_str(&format!("- Title: {}\n  URL: {}\n  Summary: {}\n\n", title, url, content));
                    }
                }
                return Ok(context);
            } else {
                eprintln!("[MAGI] Tavily API failed, falling back to Sidecar...");
            }
        }

        // Fallback to sidecar if API key is missing or API call fails
        self.search_via_sidecar(query).await
    }
}
