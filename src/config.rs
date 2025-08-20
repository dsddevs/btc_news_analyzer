use anyhow::Result;
use config::Config;
use std::env;

#[derive(Clone, serde::Deserialize)]
pub struct AppConfig {
    pub coindesk_api_url: String,
    pub newsapi_url: String,
    pub newsapi_key: String,
    pub huggingface_api_url: String,
    pub huggingface_api_key: String,
    pub bitcoin_keywords: Vec<String>,
    pub max_articles: Option<usize>,
    pub max_concurrent_requests: Option<usize>,
}

impl AppConfig {
    /// Валидация конфигурации
    pub fn validate(&self) -> Result<()> {
        if self.bitcoin_keywords.is_empty() {
            return Err(anyhow::anyhow!("Bitcoin keywords cannot be empty"));
        }
        
        if let Some(max_articles) = self.max_articles {
            if max_articles == 0 || max_articles > 1000 {
                return Err(anyhow::anyhow!("max_articles must be between 1 and 1000"));
            }
        }
        
        if let Some(max_concurrent) = self.max_concurrent_requests {
            if max_concurrent == 0 || max_concurrent > 50 {
                return Err(anyhow::anyhow!("max_concurrent_requests must be between 1 and 50"));
            }
        }
        
        Ok(())
    }
}

pub fn load_config() -> Result<AppConfig> {
    // Загружаем .env файл
    dotenvy::dotenv().ok();
    
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .add_source(config::Environment::with_prefix("BTC_ANALYZER"))
        .build()?;

    let mut config: AppConfig = settings.try_deserialize()?;

    config.newsapi_key = env::var("NEWSAPI_KEY")
        .map_err(|_| anyhow::anyhow!("NEWSAPI_KEY environment variable is required"))?;
    
    config.huggingface_api_key = env::var("HUGGINGFACE_API_KEY")
        .map_err(|_| anyhow::anyhow!("HUGGINGFACE_API_KEY environment variable is required"))?;

    config.validate()?;
    
    Ok(config)
}
