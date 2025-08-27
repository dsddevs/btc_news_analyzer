use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::BitcoinNews;
use crate::errors::{BitcoinAnalysisError, Result};

#[derive(Clone)]
pub struct BitcoinNewsHolder {
    news: Arc<Mutex<Vec<BitcoinNews>>>,
}

impl BitcoinNewsHolder {
    pub fn new() -> Self {
        BitcoinNewsHolder {
            news: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add(&self, news_item: BitcoinNews) -> Result<()> {
        let mut news = self.news.lock().await;
        news.push(news_item);
        Ok(())
    }

    pub async fn clear(&self) -> Result<()> {
        let mut news = self.news.lock().await;
        news.clear();
        Ok(())
    }

    pub async fn get(&self) -> Result<Vec<BitcoinNews>> {
        let news = self.news.lock().await;
        Ok(news.clone())
    }

    pub async fn update_sentiment(&self, index: usize, is_positive: bool) -> Result<()> {
        let mut news = self.news.lock().await;
        if let Some(item) = news.get_mut(index) {
            item.is_positive = Some(is_positive);
            Ok(())
        } else {
            Err(BitcoinAnalysisError::InvalidDataFormat(format!(
                "News item at index {} not found",
                index
            )))
        }
    }

    pub async fn len(&self) -> Result<usize> {
        let news = self.news.lock().await;
        Ok(news.len())
    }
}