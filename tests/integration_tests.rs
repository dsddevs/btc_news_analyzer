use btc_news_analyzer::*;
use chrono::NaiveDate;
use tokio_test;

#[tokio::test]
async fn test_bitcoin_price_holder() {
    let holder = BitcoinPriceHolder::new();
    
    // Тест добавления цены
    let price = BitcoinPrice {
        date: NaiveDate::from_ymd_opt(2025, 8, 20).unwrap(),
        price: 67000.0,
    };
    
    holder.add(price.clone()).await.unwrap();
    assert_eq!(holder.len().await.unwrap(), 1);
    
    // Тест получения цен
    let prices = holder.get().await.unwrap();
    assert_eq!(prices.len(), 1);
    assert_eq!(prices[0].price, 67000.0);
    
    // Тест start_price и end_price
    assert_eq!(holder.start_price().await.unwrap(), Some(67000.0));
    assert_eq!(holder.end_price().await.unwrap(), Some(67000.0));
    
    // Тест очистки
    holder.clear().await.unwrap();
    assert_eq!(holder.len().await.unwrap(), 0);
}

#[tokio::test]
async fn test_bitcoin_news_holder() {
    let holder = BitcoinNewsHolder::new();
    
    // Тест добавления новости
    let news = BitcoinNews {
        title: "Bitcoin reaches new heights".to_string(),
        content: "Bitcoin price surges to $70,000".to_string(),
        is_positive: Some(true),
        url: Some("https://example.com".to_string()),
        published_at: Some("2025-08-20T12:00:00Z".to_string()),
    };
    
    holder.add(news.clone()).await.unwrap();
    assert_eq!(holder.len().await.unwrap(), 1);
    
    // Тест получения новостей
    let news_items = holder.get().await.unwrap();
    assert_eq!(news_items.len(), 1);
    assert_eq!(news_items[0].title, "Bitcoin reaches new heights");
    assert_eq!(news_items[0].is_positive, Some(true));
    
    // Тест обновления настроения
    holder.update_sentiment(0, false).await.unwrap();
    let updated_news = holder.get().await.unwrap();
    assert_eq!(updated_news[0].is_positive, Some(false));
    
    // Тест очистки
    holder.clear().await.unwrap();
    assert_eq!(holder.len().await.unwrap(), 0);
}

#[test]
fn test_config_validation() {
    let mut config = AppConfig {
        coindesk_api_url: "https://api.coindesk.com/v1/bpi/historical/close.json".to_string(),
        newsapi_url: "https://newsapi.org/v2/everything".to_string(),
        newsapi_key: "test_key".to_string(),
        huggingface_api_url: "https://api-inference.huggingface.co/models/test".to_string(),
        huggingface_api_key: "test_key".to_string(),
        bitcoin_keywords: vec!["bitcoin".to_string(), "crypto".to_string()],
        max_articles: Some(50),
        max_concurrent_requests: Some(10),
    };
    
    // Валидная конфигурация должна проходить
    assert!(config.validate().is_ok());
    
    // Пустые ключевые слова должны вызывать ошибку
    config.bitcoin_keywords = vec![];
    assert!(config.validate().is_err());
    
    // Восстанавливаем ключевые слова
    config.bitcoin_keywords = vec!["bitcoin".to_string()];
    
    // Неверное количество статей
    config.max_articles = Some(0);
    assert!(config.validate().is_err());
    
    config.max_articles = Some(2000);
    assert!(config.validate().is_err());
    
    // Неверное количество одновременных запросов
    config.max_articles = Some(50);
    config.max_concurrent_requests = Some(0);
    assert!(config.validate().is_err());
    
    config.max_concurrent_requests = Some(100);
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn test_multiple_prices_ordering() {
    let holder = BitcoinPriceHolder::new();
    
    // Добавляем цены в разном порядке
    let price1 = BitcoinPrice {
        date: NaiveDate::from_ymd_opt(2025, 8, 18).unwrap(),
        price: 65000.0,
    };
    let price2 = BitcoinPrice {
        date: NaiveDate::from_ymd_opt(2025, 8, 20).unwrap(),
        price: 67000.0,
    };
    let price3 = BitcoinPrice {
        date: NaiveDate::from_ymd_opt(2025, 8, 19).unwrap(),
        price: 66000.0,
    };
    
    holder.add(price1).await.unwrap();
    holder.add(price2).await.unwrap();
    holder.add(price3).await.unwrap();
    
    assert_eq!(holder.len().await.unwrap(), 3);
    
    // Первая и последняя цены должны быть правильными
    assert_eq!(holder.start_price().await.unwrap(), Some(65000.0));
    assert_eq!(holder.end_price().await.unwrap(), Some(66000.0));
}
