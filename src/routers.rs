use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::AppState;

#[derive(Deserialize)]
pub struct AnalysisRequest {
    pub amount_days: u32,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub current_analysis_period_days: u32,
    pub available_endpoints: Vec<String>,
}

// Основной обработчик анализа Bitcoin
pub async fn bitcoin_analysis(
    State(state): State<AppState>,
    Json(req): Json<AnalysisRequest>,
) -> Result<Json<Value>, StatusCode> {
    // Валидация входных данных
    if req.amount_days == 0 || req.amount_days > 365 {
        tracing::warn!("Некорректное количество дней: {}", req.amount_days);
        return Err(StatusCode::BAD_REQUEST);
    }

    tracing::info!("Начинаем анализ Bitcoin за {} дней", req.amount_days);

    // Обновляем количество дней
    {
        let mut amount_days = state.amount_days.lock().await;
        amount_days.days = req.amount_days;
    }

    // Собираем данные
    if let Err(e) = state.collector.collect_data().await {
        tracing::error!("Ошибка сбора данных: {}", e);
        return Ok(Json(json!({
            "status": "error",
            "message": format!("Ошибка сбора данных: {}", e),
            "error_type": "data_collection_error"
        })));
    }

    // Обрабатываем данные
    if let Err(e) = state.processor.process_data().await {
        tracing::error!("Ошибка обработки данных: {}", e);
        return Ok(Json(json!({
            "status": "error",
            "message": format!("Ошибка обработки данных: {}", e),
            "error_type": "data_processing_error"
        })));
    }

    // Принимаем решение
    match state.decision.make_decision().await {
        Ok(analysis_result) => {
            tracing::info!("Анализ успешно завершен");
            Ok(Json(serde_json::to_value(analysis_result).unwrap()))
        },
        Err(e) => {
            tracing::error!("Ошибка принятия решения: {}", e);
            Ok(Json(json!({
                "status": "error",
                "message": format!("Ошибка принятия решения: {}", e),
                "error_type": "decision_making_error"
            })))
        }
    }
}

// Проверка здоровья сервиса
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        message: "Bitcoin News Analyzer API is running".to_string(),
        version: "1.0.0".to_string(),
    })
}

// Получение статуса сервиса
pub async fn get_status(State(state): State<AppState>) -> Json<StatusResponse> {
    let days = {
        let amount_days = state.amount_days.lock().await;
        amount_days.days
    };

    Json(StatusResponse {
        status: "ready".to_string(),
        current_analysis_period_days: days,
        available_endpoints: vec![
            "/".to_string(),
            "/status".to_string(),
            "/api/bitcoin-analysis".to_string(),
        ],
    })
}

// Простой анализ без параметров (по умолчанию 7 дней)
pub async fn simple_analysis(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let req = AnalysisRequest { amount_days: 7 };
    bitcoin_analysis(State(state), Json(req)).await
}

// Тестовый эндпоинт для проверки актуальных дат
pub async fn test_dates() -> Json<Value> {
    let now = chrono::Utc::now().date_naive();
    let week_ago = now.checked_sub_days(chrono::Days::new(7)).unwrap();
    let two_weeks_ago = now.checked_sub_days(chrono::Days::new(14)).unwrap();
    let month_ago = now.checked_sub_days(chrono::Days::new(30)).unwrap();

    Json(json!({
        "current_date": now.format("%Y-%m-%d").to_string(),
        "week_ago": week_ago.format("%Y-%m-%d").to_string(),
        "two_weeks_ago": two_weeks_ago.format("%Y-%m-%d").to_string(),
        "month_ago": month_ago.format("%Y-%m-%d").to_string(),
        "available_apis": {
            "coingecko_current": "https://api.coingecko.com/api/v3/coins/bitcoin/market_chart?vs_currency=usd&days=7&interval=daily".to_string(),
            "binance_klines": "https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1d&limit=7",
            "coincap_history": format!("https://api.coincap.io/v2/assets/bitcoin/history?interval=d1&start={}&end={}",
                week_ago.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() * 1000,
                now.and_hms_opt(23, 59, 59).unwrap().and_utc().timestamp() * 1000)
        },
        "note": "Все данные актуальны на текущую дату"
    }))
}

// Создание маршрутов
pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/status", get(get_status))
        .route("/test-dates", get(test_dates))
        .route("/api/bitcoin-analysis", post(bitcoin_analysis))
        .route("/analyze", get(simple_analysis))
        .with_state(state)
}
