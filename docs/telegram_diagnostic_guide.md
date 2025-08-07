# 🔍 Guide de Diagnostic Telegram - CachyPac

## 📋 Analyse des Problèmes Potentiels

### 🚨 Problèmes Identifiés dans l'Implémentation Actuelle

1. **❌ Pas d'implémentation HTTP réelle** - Utilise seulement des simulations
2. **❌ Pas de gestion d'erreurs réseau** - Aucune vérification de connectivité
3. **❌ Pas de validation des tokens** - Aucune vérification d'authentification
4. **❌ Pas de gestion des limites de débit** - Rate limiting non implémenté
5. **❌ Pas de retry automatique** - Aucune logique de nouvelle tentative
6. **❌ Pas de validation des formats** - Messages non validés
7. **❌ Pas de gestion des webhooks** - Configuration manquante

---

## 🔧 Solutions Détaillées

### 1. 🌐 Connectivité Réseau

#### Problème
```rust
// ❌ Implémentation actuelle - simulation seulement
pub async fn send_message(&mut self, message: &str) -> Result<()> {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    // Pas de vraie requête HTTP
    Ok(())
}
```

#### Solution Complète
```rust
use reqwest::{Client, Error as ReqwestError};
use serde_json::json;
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum TelegramError {
    #[error("Erreur réseau: {0}")]
    Network(#[from] ReqwestError),
    #[error("Token invalide")]
    InvalidToken,
    #[error("Chat ID invalide: {0}")]
    InvalidChatId(String),
    #[error("Message trop long: {0} caractères (max: 4096)")]
    MessageTooLong(usize),
    #[error("Rate limit atteint, retry dans {0}s")]
    RateLimit(u64),
    #[error("API Telegram erreur: {0}")]
    ApiError(String),
}

impl TelegramNotifier {
    pub fn new(bot_token: String, chat_id: String) -> Result<Self, TelegramError> {
        // Validation du token
        if !Self::is_valid_token(&bot_token) {
            return Err(TelegramError::InvalidToken);
        }
        
        // Validation du chat_id
        if !Self::is_valid_chat_id(&chat_id) {
            return Err(TelegramError::InvalidChatId(chat_id));
        }

        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("CachyPac/1.0")
            .build()
            .map_err(TelegramError::Network)?;

        Ok(Self {
            bot_token,
            chat_id,
            client,
            message_history: Vec::new(),
            rate_limiter: RateLimiter::new(),
        })
    }

    // Validation du token (format: 123456789:ABC-DEF...)
    fn is_valid_token(token: &str) -> bool {
        let parts: Vec<&str> = token.split(':').collect();
        parts.len() == 2 && 
        parts[0].chars().all(|c| c.is_ascii_digit()) &&
        parts[1].len() >= 35
    }

    // Validation du chat_id
    fn is_valid_chat_id(chat_id: &str) -> bool {
        chat_id.parse::<i64>().is_ok() || chat_id.starts_with('@')
    }

    pub async fn send_message(&mut self, message: &str) -> Result<(), TelegramError> {
        // Validation de la longueur
        if message.len() > 4096 {
            return Err(TelegramError::MessageTooLong(message.len()));
        }

        // Vérification du rate limiting
        self.rate_limiter.check_rate_limit().await?;

        // Test de connectivité
        self.test_connectivity().await?;

        // Envoi du message
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);
        
        let payload = json!({
            "chat_id": self.chat_id,
            "text": message,
            "parse_mode": "HTML",
            "disable_web_page_preview": true
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        self.handle_response(response, message).await
    }

    async fn test_connectivity(&self) -> Result<(), TelegramError> {
        let url = format!("https://api.telegram.org/bot{}/getMe", self.bot_token);
        
        let response = self.client
            .get(&url)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(TelegramError::InvalidToken);
        }

        Ok(())
    }
}
```

### 2. 🔐 Authentification API

#### Problème
```rust
// ❌ Aucune validation du token
pub fn new(bot_token: String, chat_id: String) -> Self {
    Self { bot_token, chat_id, message_history: Vec::new() }
}
```

#### Solution
```rust
#[derive(Debug, Clone)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
    pub api_base_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

impl TelegramConfig {
    pub fn new(bot_token: String, chat_id: String) -> Result<Self, TelegramError> {
        Self::validate_credentials(&bot_token, &chat_id)?;
        
        Ok(Self {
            bot_token,
            chat_id,
            api_base_url: "https://api.telegram.org".to_string(),
            timeout_seconds: 30,
            max_retries: 3,
        })
    }

    fn validate_credentials(token: &str, chat_id: &str) -> Result<(), TelegramError> {
        // Validation format token
        if !token.contains(':') || token.len() < 45 {
            return Err(TelegramError::InvalidToken);
        }

        // Validation chat_id
        if chat_id.is_empty() {
            return Err(TelegramError::InvalidChatId(chat_id.to_string()));
        }

        Ok(())
    }

    pub async fn test_authentication(&self) -> Result<BotInfo, TelegramError> {
        let client = Client::new();
        let url = format!("{}/bot{}/getMe", self.api_base_url, self.bot_token);
        
        let response = client.get(&url).send().await?;
        
        if response.status() == 401 {
            return Err(TelegramError::InvalidToken);
        }

        let bot_info: BotInfo = response.json().await?;
        Ok(bot_info)
    }
}

#[derive(Debug, Deserialize)]
pub struct BotInfo {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: Option<String>,
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_inline_queries: Option<bool>,
}
```

### 3. 🔄 Gestion des Webhooks

#### Solution Complète
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub certificate: Option<String>,
    pub ip_address: Option<String>,
    pub max_connections: Option<u32>,
    pub allowed_updates: Option<Vec<String>>,
    pub drop_pending_updates: Option<bool>,
    pub secret_token: Option<String>,
}

impl TelegramNotifier {
    pub async fn set_webhook(&self, config: WebhookConfig) -> Result<(), TelegramError> {
        let url = format!("{}/bot{}/setWebhook", self.config.api_base_url, self.bot_token);
        
        let mut payload = json!({
            "url": config.url,
            "max_connections": config.max_connections.unwrap_or(40),
            "drop_pending_updates": config.drop_pending_updates.unwrap_or(false)
        });

        if let Some(secret) = config.secret_token {
            payload["secret_token"] = json!(secret);
        }

        if let Some(allowed) = config.allowed_updates {
            payload["allowed_updates"] = json!(allowed);
        }

        let response = self.client.post(&url).json(&payload).send().await?;
        self.handle_api_response(response).await?;
        
        info!("✅ Webhook configuré: {}", config.url);
        Ok(())
    }

    pub async fn delete_webhook(&self) -> Result<(), TelegramError> {
        let url = format!("{}/bot{}/deleteWebhook", self.config.api_base_url, self.bot_token);
        
        let response = self.client.post(&url).send().await?;
        self.handle_api_response(response).await?;
        
        info!("🗑️ Webhook supprimé");
        Ok(())
    }

    pub async fn get_webhook_info(&self) -> Result<WebhookInfo, TelegramError> {
        let url = format!("{}/bot{}/getWebhookInfo", self.config.api_base_url, self.bot_token);
        
        let response = self.client.get(&url).send().await?;
        let webhook_info: WebhookInfo = self.handle_api_response(response).await?;
        
        Ok(webhook_info)
    }
}

#[derive(Debug, Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: u32,
    pub ip_address: Option<String>,
    pub last_error_date: Option<i64>,
    pub last_error_message: Option<String>,
    pub last_synchronization_error_date: Option<i64>,
    pub max_connections: Option<u32>,
    pub allowed_updates: Option<Vec<String>>,
}
```

### 4. 🚦 Gestion des Limites de Débit

#### Solution Rate Limiting
```rust
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Debug)]
pub struct RateLimiter {
    requests: VecDeque<Instant>,
    max_requests_per_second: u32,
    max_requests_per_minute: u32,
    last_request: Option<Instant>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            requests: VecDeque::new(),
            max_requests_per_second: 30,  // Limite Telegram
            max_requests_per_minute: 20,   // Limite conservative
            last_request: None,
        }
    }

    pub async fn check_rate_limit(&mut self) -> Result<(), TelegramError> {
        let now = Instant::now();
        
        // Nettoyer les anciennes requêtes
        self.cleanup_old_requests(now);
        
        // Vérifier limite par seconde
        let recent_requests = self.requests.iter()
            .filter(|&&req_time| now.duration_since(req_time) < Duration::from_secs(1))
            .count();
            
        if recent_requests >= self.max_requests_per_second as usize {
            let wait_time = Duration::from_millis(1100);
            warn!("⚠️ Rate limit atteint, attente {}ms", wait_time.as_millis());
            sleep(wait_time).await;
            return Err(TelegramError::RateLimit(1));
        }

        // Vérifier limite par minute
        if self.requests.len() >= self.max_requests_per_minute as usize {
            let oldest = self.requests.front().unwrap();
            let wait_time = Duration::from_secs(60) - now.duration_since(*oldest);
            
            if wait_time > Duration::from_secs(0) {
                warn!("⚠️ Rate limit minute atteint, attente {}s", wait_time.as_secs());
                sleep(wait_time).await;
                return Err(TelegramError::RateLimit(wait_time.as_secs()));
            }
        }

        // Enregistrer la requête
        self.requests.push_back(now);
        self.last_request = Some(now);
        
        Ok(())
    }

    fn cleanup_old_requests(&mut self, now: Instant) {
        while let Some(&front) = self.requests.front() {
            if now.duration_since(front) > Duration::from_secs(60) {
                self.requests.pop_front();
            } else {
                break;
            }
        }
    }
}
```

### 5. 🔄 Retry Automatique

#### Solution avec Backoff Exponentiel
```rust
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

impl TelegramNotifier {
    pub async fn send_message_with_retry(&mut self, message: &str) -> Result<(), TelegramError> {
        let config = RetryConfig::default();
        let mut last_error = None;

        for attempt in 1..=config.max_attempts {
            match self.send_message_internal(message).await {
                Ok(()) => {
                    if attempt > 1 {
                        info!("✅ Message envoyé après {} tentatives", attempt);
                    }
                    return Ok(());
                }
                Err(e) => {
                    last_error = Some(e.clone());
                    
                    if attempt < config.max_attempts {
                        let delay = self.calculate_backoff_delay(&config, attempt);
                        warn!("❌ Tentative {}/{} échouée: {}. Retry dans {:?}", 
                              attempt, config.max_attempts, e, delay);
                        sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap())
    }

    fn calculate_backoff_delay(&self, config: &RetryConfig, attempt: u32) -> Duration {
        let delay_ms = config.initial_delay.as_millis() as f64 
            * config.backoff_multiplier.powi((attempt - 1) as i32);
        
        let delay = Duration::from_millis(delay_ms as u64);
        std::cmp::min(delay, config.max_delay)
    }

    async fn send_message_internal(&mut self, message: &str) -> Result<(), TelegramError> {
        // Implémentation réelle avec gestion d'erreurs
        self.rate_limiter.check_rate_limit().await?;
        
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);
        let payload = json!({
            "chat_id": self.chat_id,
            "text": message,
            "parse_mode": "HTML"
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    TelegramError::Network(e)
                } else if e.is_connect() {
                    TelegramError::Network(e)
                } else {
                    TelegramError::Network(e)
                }
            })?;

        self.handle_response(response, message).await
    }
}
```

### 6. ✅ Validation des Formats de Messages

#### Solution Complète
```rust
#[derive(Debug, Clone)]
pub struct MessageValidator;

impl MessageValidator {
    pub fn validate_message(message: &str) -> Result<String, TelegramError> {
        // Vérifier la longueur
        if message.is_empty() {
            return Err(TelegramError::ApiError("Message vide".to_string()));
        }
        
        if message.len() > 4096 {
            return Err(TelegramError::MessageTooLong(message.len()));
        }

        // Nettoyer et valider le HTML
        let cleaned = Self::sanitize_html(message);
        
        // Vérifier les entités HTML
        Self::validate_html_entities(&cleaned)?;
        
        Ok(cleaned)
    }

    fn sanitize_html(message: &str) -> String {
        message
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            // Permettre seulement les balises autorisées
            .replace("&lt;b&gt;", "<b>")
            .replace("&lt;/b&gt;", "</b>")
            .replace("&lt;i&gt;", "<i>")
            .replace("&lt;/i&gt;", "</i>")
            .replace("&lt;code&gt;", "<code>")
            .replace("&lt;/code&gt;", "</code>")
            .replace("&lt;pre&gt;", "<pre>")
            .replace("&lt;/pre&gt;", "</pre>")
    }

    fn validate_html_entities(message: &str) -> Result<(), TelegramError> {
        let allowed_tags = ["<b>", "</b>", "<i>", "</i>", "<code>", "</code>", "<pre>", "</pre>"];
        
        // Vérifier que toutes les balises sont fermées
        let mut stack = Vec::new();
        let mut chars = message.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '<' {
                let mut tag = String::new();
                tag.push(ch);
                
                while let Some(&next_ch) = chars.peek() {
                    chars.next();
                    tag.push(next_ch);
                    if next_ch == '>' {
                        break;
                    }
                }
                
                if !allowed_tags.contains(&tag.as_str()) {
                    return Err(TelegramError::ApiError(
                        format!("Balise HTML non autorisée: {}", tag)
                    ));
                }
                
                if tag.starts_with("</") {
                    if let Some(open_tag) = stack.pop() {
                        let expected_close = format!("</{}>", &open_tag[1..open_tag.len()-1]);
                        if tag != expected_close {
                            return Err(TelegramError::ApiError(
                                format!("Balise mal fermée: {} attendu {}", tag, expected_close)
                            ));
                        }
                    }
                } else {
                    stack.push(tag);
                }
            }
        }
        
        if !stack.is_empty() {
            return Err(TelegramError::ApiError(
                format!("Balises non fermées: {:?}", stack)
            ));
        }
        
        Ok(())
    }
}
```

### 7. 📊 Monitoring et Diagnostics

#### Solution de Monitoring
```rust
#[derive(Debug, Clone)]
pub struct TelegramMetrics {
    pub total_messages: u64,
    pub successful_messages: u64,
    pub failed_messages: u64,
    pub average_response_time: Duration,
    pub last_error: Option<String>,
    pub rate_limit_hits: u64,
    pub retry_attempts: u64,
}

impl TelegramNotifier {
    pub async fn run_diagnostics(&self) -> Result<DiagnosticReport, TelegramError> {
        let mut report = DiagnosticReport::new();
        
        // Test 1: Connectivité
        report.add_test("Connectivité Internet", self.test_internet_connectivity().await);
        
        // Test 2: API Telegram
        report.add_test("API Telegram", self.test_telegram_api().await);
        
        // Test 3: Authentification
        report.add_test("Authentification Bot", self.test_bot_authentication().await);
        
        // Test 4: Permissions Chat
        report.add_test("Permissions Chat", self.test_chat_permissions().await);
        
        // Test 5: Envoi Message Test
        report.add_test("Envoi Message", self.test_send_message().await);
        
        // Test 6: Rate Limiting
        report.add_test("Rate Limiting", self.test_rate_limiting().await);
        
        Ok(report)
    }

    async fn test_internet_connectivity(&self) -> TestResult {
        match self.client.get("https://www.google.com").send().await {
            Ok(_) => TestResult::Success("Connectivité OK".to_string()),
            Err(e) => TestResult::Failure(format!("Pas de connexion Internet: {}", e)),
        }
    }

    async fn test_telegram_api(&self) -> TestResult {
        match self.client.get("https://api.telegram.org").send().await {
            Ok(response) if response.status().is_success() => {
                TestResult::Success("API Telegram accessible".to_string())
            }
            Ok(response) => TestResult::Failure(
                format!("API Telegram erreur: {}", response.status())
            ),
            Err(e) => TestResult::Failure(format!("API Telegram inaccessible: {}", e)),
        }
    }

    async fn test_bot_authentication(&self) -> TestResult {
        let url = format!("https://api.telegram.org/bot{}/getMe", self.bot_token);
        
        match self.client.get(&url).send().await {
            Ok(response) if response.status() == 200 => {
                TestResult::Success("Token bot valide".to_string())
            }
            Ok(response) if response.status() == 401 => {
                TestResult::Failure("Token bot invalide".to_string())
            }
            Ok(response) => TestResult::Failure(
                format!("Erreur authentification: {}", response.status())
            ),
            Err(e) => TestResult::Failure(format!("Erreur réseau: {}", e)),
        }
    }
}

#[derive(Debug)]
pub struct DiagnosticReport {
    pub tests: Vec<(String, TestResult)>,
    pub overall_status: OverallStatus,
    pub recommendations: Vec<String>,
}

#[derive(Debug)]
pub enum TestResult {
    Success(String),
    Warning(String),
    Failure(String),
}

#[derive(Debug)]
pub enum OverallStatus {
    Healthy,
    Warning,
    Critical,
}
```

---

## 🛠️ Implémentation Complète

Maintenant je vais créer une version complète et robuste du module Telegram avec toutes ces améliorations.

### Dépendances Requises

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
thiserror = "1.0"
tracing = "0.1"
chrono = { version = "0.4", features = ["serde"] }
```

### Tests d'Intégration

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_full_telegram_workflow() {
        let config = TelegramConfig::new(
            "123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string(),
            "123456789".to_string()
        ).unwrap();
        
        let mut notifier = TelegramNotifier::new(config).unwrap();
        
        // Test diagnostics
        let report = notifier.run_diagnostics().await.unwrap();
        assert!(matches!(report.overall_status, OverallStatus::Healthy));
        
        // Test envoi message
        let result = notifier.send_message_with_retry("Test CachyPac").await;
        assert!(result.is_ok());
    }
}
```

Ce guide fournit une solution complète pour tous les problèmes potentiels de l'intégration Telegram avec CachyPac.