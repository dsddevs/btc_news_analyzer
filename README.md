# Bitcoin News Analyzer

Веб-сервис для анализа Bitcoin на основе цен и новостей с использованием машинного обучения для анализа настроений.

## Возможности

- **Сбор данных о ценах Bitcoin** из нескольких источников (CoinGecko, Binance, CoinCap)
- **Сбор новостей** через NewsAPI
- **Анализ настроений** с помощью HuggingFace Transformers
- **RESTful API** для получения аналитических отчетов
- **Асинхронная обработка** для высокой производительности
- **Структурированное логирование** с tracing

## Быстрый старт

### 1. Установка зависимостей

```bash
# Убедитесь, что у вас установлен Rust
rustc --version

# Клонируйте репозиторий и перейдите в директорию
cd btc_news_analyzer
```

### 2. Настройка конфигурации

```bash
# Скопируйте пример файла переменных окружения
cp .env .env

# Отредактируйте .env файл и добавьте ваши API ключи:
# NEWSAPI_KEY=your_newsapi_key_from_https://newsapi.org/
# HUGGINGFACE_API_KEY=your_huggingface_token_from_https://huggingface.co/settings/tokens
```

### 3. Запуск

```bash
# Запуск в режиме разработки
cargo run

# Или сборка и запуск оптимизированной версии
cargo build --release
./target/release/btc_news_analyzer
```

Сервер будет доступен по адресу: `http://localhost:8080`

## API Endpoints

### GET `/`
Проверка здоровья сервиса
```json
{
  "status": "healthy",
  "message": "Bitcoin News Analyzer API is running",
  "version": "1.0.0"
}
```

### GET `/status`
Получение текущего статуса
```json
{
  "status": "ready",
  "current_analysis_period_days": 7,
  "available_endpoints": ["/", "/status", "/api/bitcoin-analysis"]
}
```

### POST `/api/bitcoin-analysis`
Запуск анализа Bitcoin за указанный период
```json
{
  "amount_days": 7
}
```

### GET `/analyze`
Быстрый анализ за 7 дней (без параметров)

### GET `/test-dates`
Тестовый endpoint для проверки доступных API

## Конфигурация

Основные настройки находятся в `config.toml`:

```toml
bitcoin_keywords = ["bitcoin", "cryptocurrency", "blockchain", "btc", "crypto"]
max_articles = 50
max_concurrent_requests = 10
```

### Переменные окружения

- `NEWSAPI_KEY` - API ключ для NewsAPI
- `HUGGINGFACE_API_KEY` - API ключ для HuggingFace
- `RUST_LOG` - уровень логирования (debug, info, warn, error)

## Архитектура

```
src/
├── config.rs          # Конфигурация приложения
├── errors.rs          # Обработка ошибок
├── holders/           # Хранилища данных
│   ├── news.rs        # Хранилище новостей
│   └── price.rs       # Хранилище цен
├── models.rs          # Модели данных
├── routers.rs         # HTTP маршруты
├── services/          # Бизнес-логика
│   ├── collector.rs   # Сбор данных
│   ├── processor.rs   # Обработка данных
│   └── decision.rs    # Принятие решений
└── main.rs           # Точка входа
```

## Тестирование

```bash
# Запуск всех тестов
cargo test

# Запуск с подробным выводом
cargo test -- --nocapture

# Запуск только unit тестов
cargo test --lib

# Запуск только интеграционных тестов
cargo test --test integration_tests
```

## Разработка

### Логирование

Настройте уровень логирования через переменную окружения:

```bash
# Детальное логирование
export RUST_LOG=btc_news_analyzer=debug

# Только предупреждения и ошибки
export RUST_LOG=btc_news_analyzer=warn
```

### Добавление новых источников данных

1. Реализуйте новый метод в `DataCollectorService`
2. Добавьте fallback логику в `collect_bitcoin_prices()` или `collect_bitcoin_news()`
3. Добавьте соответствующие тесты

## Производительность

- Асинхронная обработка с tokio
- Параллельный сбор данных из разных источников
- Ограничение количества одновременных запросов
- Кэширование и переиспользование HTTP клиентов

## Безопасность

- API ключи хранятся в переменных окружения
- Валидация входных данных
- Ограничения на размер запросов
- Структурированное логирование без утечки чувствительных данных

## Лицензия

MIT License

## Поддержка

Для вопросов и предложений создавайте issues в репозитории.
