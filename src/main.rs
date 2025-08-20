use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use btc_news_analyzer::{
    AppState, AmountDays, BitcoinNewsHolder, BitcoinPriceHolder,
    DataCollectorService, DataMakerDecisionService, DataProcessorService,
    load_config
};
use btc_news_analyzer::routers::create_routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Настройка структурированного логирования
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("btc_news_analyzer=info,warn"));
    
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true))
        .init();

    let config = load_config()?;
    let price_holder = BitcoinPriceHolder::new();
    let news_holder = BitcoinNewsHolder::new();
    let amount_days = Arc::new(Mutex::new(AmountDays { days: 7 })); // По умолчанию 7 дней

    let state = AppState {
        collector: DataCollectorService::new(
            price_holder.clone(),
            news_holder.clone(),
            amount_days.clone(),
            config.clone(),
        ),
        processor: DataProcessorService::new(
            price_holder.clone(),
            news_holder.clone(),
            config.clone(),
        ),
        decision: DataMakerDecisionService::new(price_holder, news_holder, amount_days.clone()),
        amount_days,
    };

    let app = create_routes(state);
    println!("Сервер запущен на http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
