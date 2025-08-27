use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::BitcoinPrice;
use crate::errors::Result;

#[derive(Clone)]
pub struct BitcoinPriceHolder {
    prices: Arc<Mutex<Vec<BitcoinPrice>>>,
}

impl BitcoinPriceHolder {
    pub fn new() -> Self {
        BitcoinPriceHolder {
            prices: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add(&self, price: BitcoinPrice) -> Result<()> {
        let mut prices = self.prices.lock().await;
        prices.push(price);
        Ok(())
    }

    pub async fn clear(&self) -> Result<()> {
        let mut prices = self.prices.lock().await;
        prices.clear();
        Ok(())
    }

    pub async fn get(&self) -> Result<Vec<BitcoinPrice>> {
        let prices = self.prices.lock().await;
        Ok(prices.clone())
    }

    pub async fn start_price(&self) -> Result<Option<f64>> {
        let prices = self.prices.lock().await;
        Ok(prices.first().map(|p| p.price))
    }

    pub async fn end_price(&self) -> Result<Option<f64>> {
        let prices = self.prices.lock().await;
        Ok(prices.last().map(|p| p.price))
    }

    pub async fn len(&self) -> Result<usize> {
        let prices = self.prices.lock().await;
        Ok(prices.len())
    }
}