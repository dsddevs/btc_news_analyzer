use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;

use crate::holders::{BitcoinPriceHolder, BitcoinNewsHolder};
use crate::models::{AmountDays, PriceStatistics, NewsStatistics, NewsItem, AnalysisResult};
use crate::errors::{BitcoinAnalysisError, Result};

#[derive(Clone)]
pub struct DataMakerDecisionService {
    price_holder: BitcoinPriceHolder,
    news_holder: BitcoinNewsHolder,
    amount_days: Arc<Mutex<AmountDays>>,
}

impl DataMakerDecisionService {
    pub fn new(
        price_holder: BitcoinPriceHolder,
        news_holder: BitcoinNewsHolder,
        amount_days: Arc<Mutex<AmountDays>>,
    ) -> Self {
        DataMakerDecisionService {
            price_holder,
            news_holder,
            amount_days,
        }
    }

    pub async fn make_decision(&self) -> Result<AnalysisResult> {
        let days = {
            let amount_days = self.amount_days.lock().await;
            amount_days.days
        };

        // Получаем данные о ценах
        let prices = self.price_holder.get().await?;
        let start_price = self.price_holder.start_price().await?.ok_or(BitcoinAnalysisError::PriceDataUnavailable)?;
        let end_price = self.price_holder.end_price().await?.ok_or(BitcoinAnalysisError::PriceDataUnavailable)?;

        // Рассчитываем статистику цен
        let price_statistics = self.calculate_price_statistics(&prices, start_price, end_price)?;

        // Получаем и анализируем новости
        let news_items = self.news_holder.get().await?;
        let news_statistics = self.calculate_news_statistics(&news_items);
        let key_news = self.format_key_news(&news_items);

        // Определяем общий настрой рынка
        let market_sentiment = self.determine_market_sentiment(&price_statistics, &news_statistics);
        let confidence_level = self.determine_confidence_level(&price_statistics, &news_statistics);

        // Создаем краткое резюме
        let summary = self.generate_summary(&price_statistics, &news_statistics, &market_sentiment);

        Ok(AnalysisResult {
            analysis_period_days: days,
            timestamp: Utc::now().to_rfc3339(),
            status: "success".to_string(),
            price_statistics,
            news_statistics,
            key_news,
            market_sentiment,
            confidence_level,
            summary,
        })
    }

    fn calculate_price_statistics(&self, prices: &[crate::models::BitcoinPrice], start_price: f64, end_price: f64) -> Result<PriceStatistics> {
        if prices.is_empty() {
            return Err(BitcoinAnalysisError::PriceDataUnavailable);
        }

        let price_values: Vec<f64> = prices.iter().map(|p| p.price).collect();
        let highest_price = price_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let lowest_price = price_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let average_price = price_values.iter().sum::<f64>() / price_values.len() as f64;

        let price_change_absolute = end_price - start_price;
        let price_change_percentage = (price_change_absolute / start_price) * 100.0;

        // Рассчитываем волатильность (стандартное отклонение)
        let variance = price_values.iter()
            .map(|&price| (price - average_price).powi(2))
            .sum::<f64>() / price_values.len() as f64;
        let volatility = variance.sqrt();

        // Определяем тренд
        let trend = if price_change_percentage > 2.0 {
            "bullish".to_string()
        } else if price_change_percentage < -2.0 {
            "bearish".to_string()
        } else {
            "sideways".to_string()
        };

        Ok(PriceStatistics {
            start_price,
            end_price,
            price_change_absolute,
            price_change_percentage,
            highest_price,
            lowest_price,
            average_price,
            volatility,
            trend,
        })
    }

    fn calculate_news_statistics(&self, news_items: &[crate::models::BitcoinNews]) -> NewsStatistics {
        let total_analyzed = news_items.len();
        let positive_count = news_items.iter().filter(|n| n.is_positive == Some(true)).count();
        let negative_count = news_items.iter().filter(|n| n.is_positive == Some(false)).count();
        let neutral_count = total_analyzed - positive_count - negative_count;

        let positive_percentage = if total_analyzed > 0 {
            (positive_count as f64 / total_analyzed as f64) * 100.0
        } else {
            0.0
        };

        let negative_percentage = if total_analyzed > 0 {
            (negative_count as f64 / total_analyzed as f64) * 100.0
        } else {
            0.0
        };

        // Рассчитываем общий sentiment score (-1.0 до 1.0)
        let sentiment_score = if total_analyzed > 0 {
            (positive_count as f64 - negative_count as f64) / total_analyzed as f64
        } else {
            0.0
        };

        NewsStatistics {
            total_analyzed,
            positive_count,
            negative_count,
            neutral_count,
            positive_percentage,
            negative_percentage,
            sentiment_score,
        }
    }

    fn format_key_news(&self, news_items: &[crate::models::BitcoinNews]) -> Vec<NewsItem> {
        news_items.iter()
            .take(5) // Берем только топ-5 новостей
            .map(|news| {
                let sentiment = match news.is_positive {
                    Some(true) => "positive",
                    Some(false) => "negative",
                    None => "neutral",
                };

                // Простая оценка уверенности на основе длины контента
                let confidence = if news.content.len() > 100 {
                    0.8
                } else if news.content.len() > 50 {
                    0.6
                } else {
                    0.4
                };

                NewsItem {
                    title: news.title.clone(),
                    sentiment: sentiment.to_string(),
                    confidence,
                    published_at: news.published_at.clone(),
                    url: news.url.clone(),
                }
            })
            .collect()
    }

    fn determine_market_sentiment(&self, price_stats: &PriceStatistics, news_stats: &NewsStatistics) -> String {
        let price_weight = 0.6;
        let news_weight = 0.4;

        let price_score = if price_stats.price_change_percentage > 5.0 {
            1.0
        } else if price_stats.price_change_percentage > 2.0 {
            0.5
        } else if price_stats.price_change_percentage < -5.0 {
            -1.0
        } else if price_stats.price_change_percentage < -2.0 {
            -0.5
        } else {
            0.0
        };

        let combined_score = price_score * price_weight + news_stats.sentiment_score * news_weight;

        match combined_score {
            x if x > 0.6 => "very_bullish",
            x if x > 0.2 => "bullish",
            x if x < -0.6 => "very_bearish",
            x if x < -0.2 => "bearish",
            _ => "neutral",
        }.to_string()
    }

    fn determine_confidence_level(&self, price_stats: &PriceStatistics, news_stats: &NewsStatistics) -> String {
        let has_sufficient_news = news_stats.total_analyzed >= 3;
        let price_change_significant = price_stats.price_change_percentage.abs() > 1.0;
        let low_volatility = price_stats.volatility < price_stats.average_price * 0.05;

        if has_sufficient_news && price_change_significant && low_volatility {
            "high"
        } else if has_sufficient_news || price_change_significant {
            "medium"
        } else {
            "low"
        }.to_string()
    }

    fn generate_summary(&self, price_stats: &PriceStatistics, news_stats: &NewsStatistics, market_sentiment: &str) -> String {
        let price_direction = if price_stats.price_change_percentage > 0.0 { "выросла" } else { "упала" };
        let sentiment_description = match market_sentiment {
            "very_bullish" => "крайне позитивные",
            "bullish" => "позитивные",
            "neutral" => "нейтральные",
            "bearish" => "негативные",
            "very_bearish" => "крайне негативные",
            _ => "смешанные",
        };

        format!(
            "За анализируемый период цена Bitcoin {} на {:.2}% (с ${:.2} до ${:.2}). \
            Проанализировано {} новостей, из которых {}% позитивных и {}% негативных. \
            Общие настроения рынка: {}. Волатильность составила ${:.2}.",
            price_direction,
            price_stats.price_change_percentage.abs(),
            price_stats.start_price,
            price_stats.end_price,
            news_stats.total_analyzed,
            news_stats.positive_percentage.round(),
            news_stats.negative_percentage.round(),
            sentiment_description,
            price_stats.volatility
        )
    }
}

