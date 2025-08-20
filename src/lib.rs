use std::sync::Arc;
use tokio::sync::Mutex;

pub mod config;
pub mod errors;
pub mod holders;
pub mod models;
pub mod routers;
pub mod services;

pub use config::AppConfig;
pub use errors::{BitcoinAnalysisError, Result};
pub use holders::{BitcoinNewsHolder, BitcoinPriceHolder};
pub use models::{AmountDays, BitcoinNews, BitcoinPrice, AnalysisResult, PriceStatistics, NewsStatistics, NewsItem};
pub use services::{DataCollectorService, DataMakerDecisionService, DataProcessorService};
pub use config::load_config;

#[derive(Clone)]
pub struct AppState {
    pub collector: DataCollectorService,
    pub processor: DataProcessorService,
    pub decision: DataMakerDecisionService,
    pub amount_days: Arc<Mutex<AmountDays>>,
}
