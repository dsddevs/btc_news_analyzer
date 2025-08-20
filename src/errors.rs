// errors.rs
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum BitcoinAnalysisError {
    #[error("Ошибка HTTP запроса: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Ошибка парсинга JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Ошибка парсинга даты: {0}")]
    DateError(#[from] chrono::ParseError),

    #[error("Ошибка конфигурации: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Ошибка regex: {0}")]
    RegexError(#[from] regex::Error),

    #[error("Ошибка выполнения задачи: {0}")]
    TaskError(#[from] JoinError),

    #[error("Данные о ценах недоступны")]
    PriceDataUnavailable,

    #[error("Некорректный формат данных: {0}")]
    InvalidDataFormat(String),

    #[error("API вернул ошибку: {0}")]
    ApiError(String),

    #[error("Нет доступных источников данных: {0}")]
    NoDataSourcesAvailable(String),
}

// Определяем псевдоним Result с фиксированным типом ошибки
pub type Result<T> = std::result::Result<T, BitcoinAnalysisError>;