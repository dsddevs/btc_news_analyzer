use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinPrice {
    pub date: NaiveDate,
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinNews {
    pub title: String,
    pub content: String,
    pub is_positive: Option<bool>,
    pub url: Option<String>,
    pub published_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AmountDays {
    pub days: u32,
}

#[derive(Debug, Serialize)]
pub struct PriceStatistics {
    pub start_price: f64,
    pub end_price: f64,
    pub price_change_absolute: f64,
    pub price_change_percentage: f64,
    pub highest_price: f64,
    pub lowest_price: f64,
    pub average_price: f64,
    pub volatility: f64,
    pub trend: String, // "bullish", "bearish", "sideways"
}

#[derive(Debug, Serialize)]
pub struct NewsStatistics {
    pub total_analyzed: usize,
    pub positive_count: usize,
    pub negative_count: usize,
    pub neutral_count: usize,
    pub positive_percentage: f64,
    pub negative_percentage: f64,
    pub sentiment_score: f64, // -1.0 to 1.0
}

#[derive(Debug, Serialize)]
pub struct NewsItem {
    pub title: String,
    pub sentiment: String, // "positive", "negative", "neutral"
    pub confidence: f64,
    pub published_at: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AnalysisResult {
    pub analysis_period_days: u32,
    pub timestamp: String,
    pub status: String,
    pub price_statistics: PriceStatistics,
    pub news_statistics: NewsStatistics,
    pub key_news: Vec<NewsItem>,
    pub market_sentiment: String, // "very_bullish", "bullish", "neutral", "bearish", "very_bearish"
    pub confidence_level: String, // "high", "medium", "low"
    pub summary: String,
}