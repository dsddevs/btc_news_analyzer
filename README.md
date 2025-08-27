# Bitcoin News Analyzer Pro 🚀

*Professional-grade Bitcoin analysis powered by AI sentiment analysis and multi-source price tracking*

## 🎯 What Makes This Special

**Bitcoin News Analyzer Pro** is an enterprise-ready web service that combines real-time Bitcoin price data with AI-powered news sentiment analysis to deliver actionable market insights. Built with Rust for maximum performance and reliability.

### 🏆 Key Features

- **🔄 Multi-Source Price Aggregation** - Real-time data from CoinGecko, Binance, and CoinCap APIs
- **🧠 AI Sentiment Analysis** - Advanced natural language processing using HuggingFace Transformers
- **⚡ High-Performance Architecture** - Asynchronous processing with Tokio for enterprise-scale throughput
- **📊 RESTful API** - Clean, well-documented endpoints for seamless integration
- **🔍 Comprehensive Analytics** - Historical trend analysis and predictive insights
- **📝 Enterprise Logging** - Structured tracing and monitoring for production environments

## 💼 Business Value

- **Risk Assessment**: Quantify market sentiment to make informed trading decisions
- **Automated Monitoring**: Continuous analysis without manual oversight
- **Integration Ready**: REST API designed for seamless integration with existing systems
- **Scalable Architecture**: Handle high-volume requests with optimized concurrent processing

## 🚀 Quick Start Guide

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- API Keys (see Configuration section)

### Installation

```bash
# Clone the repository
git clone https://github.com/dsddevs/btc_news_analyzer.git
cd btc_news_analyzer

# Set up environment variables
cp .env.example .env
# Edit .env with your API keys (see Configuration below)

# Run the service
cargo run --release
```

**Service will be available at:** `http://localhost:3000`

## 🔧 Configuration

### Required API Keys

Create a `.env` file with the following:

```env
# Get your free key at https://newsapi.org/
NEWSAPI_KEY=your_newsapi_key_here

# Get your token at https://huggingface.co/settings/tokens
HUGGINGFACE_API_KEY=your_huggingface_token_here

# Optional: Set logging level
RUST_LOG=info
```

### Advanced Configuration

Customize analysis parameters in `config.toml`:

```toml
# Keywords for news filtering
bitcoin_keywords = ["bitcoin", "cryptocurrency", "blockchain", "btc", "crypto"]

# Analysis scope
max_articles = 50
max_concurrent_requests = 10

# Performance tuning
cache_duration_minutes = 15
request_timeout_seconds = 30
```

## 📡 API Reference

### Health Check
```http
GET /
```
**Response:**
```json
{
  "status": "healthy",
  "message": "Bitcoin News Analyzer API is running",
  "version": "1.0.0"
}
```

### Service Status
```http
GET /status
```
**Response:**
```json
{
  "status": "ready",
  "current_analysis_period_days": 7,
  "available_endpoints": ["/", "/status", "/api/bitcoin-analysis"]
}
```

### Bitcoin Analysis (Main Endpoint)
```http
POST /api/bitcoin-analysis
Content-Type: application/json

{
  "amount_days": 7
}
```

**Response Example:**
```json
{
  "analysis_period": "7 days",
  "price_data": {
    "current_price": 43500.00,
    "price_change_percentage": 2.45,
    "trend": "bullish"
  },
  "sentiment_analysis": {
    "overall_sentiment": "positive",
    "confidence_score": 0.78,
    "articles_analyzed": 45
  },
  "recommendation": "buy_signal",
  "timestamp": "2025-01-20T10:30:00Z"
}
```

### Quick Analysis
```http
GET /analyze
```
Performs instant 7-day analysis (no parameters required)

## 🏗️ Architecture Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   HTTP Router   │────│  Data Collector │────│   External APIs │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │               ┌───────┴───────┐
         ▼                       ▼               │ • CoinGecko   │
┌─────────────────┐    ┌─────────────────┐       │ • Binance     │
│   Data Models   │    │  AI Processor   │       │ • NewsAPI     │
└─────────────────┘    └─────────────────┘       │ • HuggingFace │
         │                       │               └───────────────┘
         │                       │
         ▼                       ▼
┌─────────────────┐    ┌─────────────────┐
│ Decision Engine │────│ Response Builder│
└─────────────────┘    └─────────────────┘
```

## 🧪 Testing & Quality Assurance

```bash
# Run comprehensive test suite
cargo test

# Performance benchmarks
cargo bench

# Code coverage
cargo tarpaulin --out Html

# Linting and formatting
cargo clippy
cargo fmt
```

## 🚀 Production Deployment

### Docker Deployment
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/btc_news_analyzer /usr/local/bin/
EXPOSE 3000
CMD ["btc_news_analyzer"]
```

### Performance Tuning

- **Concurrent Processing**: Configurable request limits for optimal throughput
- **Connection Pooling**: Reusable HTTP clients for external APIs
- **Memory Optimization**: Efficient data structures with minimal allocations
- **Caching Strategy**: Intelligent caching to reduce API calls and improve response times

## 📊 Monitoring & Observability

- **Structured Logging**: JSON-formatted logs for easy parsing
- **Metrics Integration**: Compatible with Prometheus and Grafana
- **Health Checks**: Built-in endpoints for load balancer integration
- **Error Tracking**: Comprehensive error reporting and alerting

## 🔒 Security Features

- **API Key Management**: Secure environment variable storage
- **Input Validation**: Comprehensive request sanitization
- **Rate Limiting**: Configurable request throttling
- **Audit Logging**: Complete request/response tracking

## 💰 Commercial Licensing

This software is available under flexible licensing options:

- **Apache-2 License**: Free for open-source and personal projects
- **Commercial License**: Available for enterprise deployments
- **Support & Consulting**: Professional services available

## 📞 Enterprise Support

Need help integrating Bitcoin News Analyzer Pro into your system?

- **Technical Consultation**: Architecture and integration planning
- **Custom Development**: Tailored features and modifications
- **24/7 Support**: Priority support for production deployments
- **SLA Guarantees**: Uptime and performance commitments

### Development Setup
```bash
# Install development dependencies
cargo install cargo-watch cargo-tarpaulin

# Run in development mode with auto-reload
cargo watch -x run

# Pre-commit checks
./scripts/pre-commit.sh
```

## 📫 Contact
telegram: @dsddevs