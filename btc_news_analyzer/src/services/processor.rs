use reqwest::{Client, ClientBuilder};
use serde_json::{json, Value};
use regex::Regex;
use futures::stream::{self, StreamExt};
use std::time::Duration;
use crate::holders::{BitcoinPriceHolder, BitcoinNewsHolder};
use crate::config::AppConfig;
use crate::errors::{BitcoinAnalysisError, Result};
use crate::models::BitcoinNews;

#[derive(Clone)]
pub struct DataProcessorService {
    client: Client,
    price_holder: BitcoinPriceHolder,
    news_holder: BitcoinNewsHolder,
    config: AppConfig,
}

impl DataProcessorService {
    pub fn new(price_holder: BitcoinPriceHolder, news_holder: BitcoinNewsHolder, config: AppConfig) -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build reqwest client");
        DataProcessorService {
            client,
            price_holder,
            news_holder,
            config,
        }
    }

    pub async fn process_data(&self) -> Result<()> {
        let price_increased = self
            .price_holder
            .end_price().await?
            .zip(self.price_holder.start_price().await?)
            .map_or(false, |(end, start)| end > start);

        let news_items = self.news_holder.get().await?;
        let max_concurrent = self.config.max_concurrent_requests.unwrap_or(10);
        let tasks: Vec<_> = news_items.iter().map(|news| {
            let this = self.clone();
            async move {
                let cleaned_content = this.clean_text(&news.content)?;
                let cleaned_title = this.clean_text(&news.title)?;

                if !cleaned_content.is_empty() || !cleaned_title.is_empty() {
                    let text_to_analyze = format!("{} {}", cleaned_title, cleaned_content);
                    let is_positive = this.analyze_sentiment(&text_to_analyze).await?;
                    let mut processed_news = news.clone();
                    processed_news.content = cleaned_content;
                    processed_news.is_positive = Some(is_positive);
                    tracing::debug!("Обработана новость: {}", news.title);
                    Ok::<Option<(BitcoinNews, bool)>, BitcoinAnalysisError>(Some((processed_news, is_positive)))
                } else {
                    tracing::debug!("Пропущена новость из-за пустого контента или заголовка: {}", news.title);
                    Ok(None)
                }
            }
        }).collect();

        let results = stream::iter(tasks)
            .buffer_unordered(max_concurrent)
            .collect::<Vec<_>>()
            .await;

        self.news_holder.clear().await?;
        for result in results {
            if let Some((news, is_positive)) = result? {
                if (price_increased && is_positive) || (!price_increased && !is_positive) {
                    self.news_holder.add(news).await?;
                }
            }
        }

        let len = self.news_holder.len().await?;
        tracing::info!("Обработано {} новостей", len);
        Ok(())
    }

    fn clean_text(&self, text: &str) -> Result<String> {
        let html_regex = Regex::new(r"<[^>]+>")?;
        let url_regex = Regex::new(r"http\S+|www\.\S+")?;
        let whitespace_regex = Regex::new(r"\s+")?;

        let cleaned = html_regex.replace_all(text, " ");
        let cleaned = url_regex.replace_all(&cleaned, " ");
        let cleaned = whitespace_regex.replace_all(&cleaned, " ");

        Ok(cleaned.trim().to_string())
    }

    async fn analyze_sentiment(&self, text: &str) -> Result<bool> {
        if text.trim().is_empty() {
            return Ok(false);
        }

        let max_len = 512;
        let truncated_text: String = text
            .split_whitespace()
            .take_while(|word| max_len >= word.len() + 1)
            .collect::<Vec<&str>>()
            .join(" ");

        let payload = json!({ "inputs": truncated_text });

        let response = self
            .client
            .post(&self.config.huggingface_api_url)
            .header("Authorization", format!("Bearer {}", self.config.huggingface_api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            tracing::warn!("Hugging Face API вернул ошибку: {}", response.status());
            return Ok(self.simple_sentiment_analysis(text));
        }

        let result: Value = response.json().await?;
        match result.as_array().and_then(|arr| arr.first()).and_then(|pred| pred["label"].as_str()) {
            Some(label) => {
                tracing::debug!("Hugging Face вернул метку: {}", label);
                Ok(label.to_lowercase().contains("positive"))
            }
            None => {
                tracing::warn!("Некорректный формат ответа от Hugging Face: {:?}", result);
                Ok(self.simple_sentiment_analysis(text))
            }
        }
    }

    fn simple_sentiment_analysis(&self, text: &str) -> bool {
        let positive_words = [
            "good", "great", "excellent", "amazing", "wonderful", "fantastic",
            "positive", "bullish", "surge", "rally", "gain", "profit", "rise",
            "increase", "growth", "boom", "success", "breakthrough", "adoption",
            "institutional", "mainstream", "купить", "рост", "позитивный",
        ];

        let negative_words = [
            "bad", "terrible", "awful", "horrible", "negative", "bearish",
            "crash", "dump", "loss", "fall", "decline", "drop", "collapse",
            "ban", "regulation", "scam", "hack", "theft", "продать", "падение",
            "негативный", "кризис", "запрет",
        ];

        let negation_words = ["not", "never", "нет", "никогда"];
        let text_lower = text.to_lowercase();
        let words: Vec<&str> = text_lower.split_whitespace().collect();

        let mut positive_count = 0;
        let mut negative_count = 0;

        for (i, word) in words.iter().enumerate() {
            let is_negated = i > 0 && negation_words.contains(&words[i - 1]);
            if positive_words.contains(word) {
                positive_count += if is_negated { -1 } else { 1 };
            }
            if negative_words.contains(word) {
                negative_count += if is_negated { -1 } else { 1 };
            }
        }

        positive_count > negative_count
    }
}