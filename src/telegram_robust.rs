//! Module Telegram robuste avec gestion complète des erreurs
//! 
//! Ce module fournit une implémentation production-ready pour l'intégration Telegram
//! avec gestion des erreurs, retry automatique, rate limiting, et diagnostics complets.

use anyhow::Result;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

/// Erreurs spécifiques à Telegram
#[derive(Debug, Error, Clone)]
pub enum TelegramError {
    #[error("Erreur réseau: {0}")]
    Network(String),
    #[error("Token bot invalide")]
    InvalidToken,
    #[error("Chat ID invalide: {0}")]
    InvalidChatId(String),
    #[error("Message trop long: {0} caractères (max: 4096)")]
    MessageTooLong(usize),
    #[error("Rate limit atteint, retry dans {0}s")]
    RateLimit(u64),
    #[error("API Telegram erreur: {0}")]
    ApiError(String),
    #[error("Timeout de connexion")]
    Timeout,
    #[error("Configuration invalide: {0}")]
    InvalidConfig(String),
    #[error("Webhook erreur: {0}")]
    WebhookError(String),
}

/// Configuration Telegram
#[derive(Debug, Clone)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
    pub api_base_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub rate_limit_per_second: u32,
    pub rate_limit_per_minute: u32,
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
            rate_limit_per_second: 30,
            rate_limit_per_minute: 20,
        })
    }

    fn validate_credentials(token: &str, chat_id: &str) -> Result<(), TelegramError> {
        // Validation format token (123456789:ABC-DEF...)
        if !Self::is_valid_token(token) {
            return Err(TelegramError::InvalidToken);
        }

        // Validation chat_id
        if !Self::is_valid_chat_id(chat_id) {
            return Err(TelegramError::InvalidChatId(chat_id.to_string()));
        }

        Ok(())
    }

    fn is_valid_token(token: &str) -> bool {
        let parts: Vec<&str> = token.split(':').collect();
        parts.len() == 2 && 
        parts[0].chars().all(|c| c.is_ascii_digit()) &&
        parts[1].len() >= 35
    }

    fn is_valid_chat_id(chat_id: &str) -> bool {
        chat_id.parse::<i64>().is_ok() || chat_id.starts_with('@')
    }

    pub async fn test_authentication(&self) -> Result<BotInfo, TelegramError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(self.timeout_seconds))
            .build()
            .map_err(|e| TelegramError::Network(e.to_string()))?;
            
        let url = format!("{}/bot{}/getMe", self.api_base_url, self.bot_token);
        
        let response = client.get(&url).send().await
            .map_err(|e| TelegramError::Network(e.to_string()))?;
        
        if response.status() == 401 {
            return Err(TelegramError::InvalidToken);
        }

        let api_response: TelegramApiResponse<BotInfo> = response.json().await
            .map_err(|e| TelegramError::Network(e.to_string()))?;
        
        if !api_response.ok {
            return Err(TelegramError::ApiError(
                api_response.description.unwrap_or_default()
            ));
        }

        Ok(api_response.result.unwrap())
    }
}

/// Informations du bot
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

/// Réponse API Telegram générique
#[derive(Debug, Deserialize)]
pub struct TelegramApiResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub description: Option<String>,
    pub error_code: Option<i32>,
}

/// Rate limiter pour respecter les limites Telegram
#[derive(Debug)]
pub struct RateLimiter {
    requests: VecDeque<Instant>,
    max_requests_per_second: u32,
    max_requests_per_minute: u32,
    last_request: Option<Instant>,
}

impl RateLimiter {
    pub fn new(per_second: u32, per_minute: u32) -> Self {
        Self {
            requests: VecDeque::new(),
            max_requests_per_second: per_second,
            max_requests_per_minute: per_minute,
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
            warn!("⚠️ Rate limit par seconde atteint, attente {:?}", wait_time);
            sleep(wait_time).await;
            return Err(TelegramError::RateLimit(1));
        }

        // Vérifier limite par minute
        if self.requests.len() >= self.max_requests_per_minute as usize {
            if let Some(&oldest) = self.requests.front() {
                let elapsed = now.duration_since(oldest);
                if elapsed < Duration::from_secs(60) {
                    let wait_time = Duration::from_secs(60) - elapsed;
                    warn!("⚠️ Rate limit par minute atteint, attente {:?}", wait_time);
                    sleep(wait_time).await;
                    return Err(TelegramError::RateLimit(wait_time.as_secs()));
                }
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

/// Configuration pour les tentatives automatiques
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

/// Validateur de messages
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
        
        // Vérification basique des balises
        for tag in allowed_tags.iter() {
            if message.contains(tag) {
                let open_tag = tag.replace("/", "");
                let close_tag = format!("</{}>", &open_tag[1..open_tag.len()-1]);
                
                if !tag.starts_with("</") && !message.contains(&close_tag) {
                    return Err(TelegramError::ApiError(
                        format!("Balise non fermée: {}", tag)
                    ));
                }
            }
        }
        
        Ok(())
    }
}

/// Notificateur Telegram robuste
#[derive(Debug)]
pub struct RobustTelegramNotifier {
    config: TelegramConfig,
    client: Client,
    rate_limiter: RateLimiter,
    message_history: Vec<MessageInfo>,
    metrics: TelegramMetrics,
}

/// Informations sur un message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInfo {
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub message_type: MessageType,
    pub content: String,
    pub success: bool,
    pub response_time: Option<Duration>,
    pub retry_count: u32,
}

/// Types de messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    UpdatesAvailable,
    UpdatesInstalled,
    UpdateFailed,
    SystemInfo,
    Error,
    Warning,
    Info,
    Diagnostic,
}

/// Métriques Telegram
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

impl Default for TelegramMetrics {
    fn default() -> Self {
        Self {
            total_messages: 0,
            successful_messages: 0,
            failed_messages: 0,
            average_response_time: Duration::from_millis(0),
            last_error: None,
            rate_limit_hits: 0,
            retry_attempts: 0,
        }
    }
}

impl RobustTelegramNotifier {
    pub fn new(config: TelegramConfig) -> Result<Self, TelegramError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .user_agent("CachyPac/1.0")
            .build()
            .map_err(|e| TelegramError::Network(e.to_string()))?;

        let rate_limiter = RateLimiter::new(
            config.rate_limit_per_second,
            config.rate_limit_per_minute
        );

        Ok(Self {
            config,
            client,
            rate_limiter,
            message_history: Vec::new(),
            metrics: TelegramMetrics::default(),
        })
    }

    /// Envoie un message avec retry automatique
    pub async fn send_message_with_retry(&mut self, message: &str) -> Result<(), TelegramError> {
        let config = RetryConfig::default();
        let mut last_error = None;
        let start_time = Instant::now();

        // Valider le message
        let validated_message = MessageValidator::validate_message(message)?;

        for attempt in 1..=config.max_attempts {
            match self.send_message_internal(&validated_message).await {
                Ok(()) => {
                    let response_time = start_time.elapsed();
                    
                    // Enregistrer le succès
                    self.record_message_success(&validated_message, response_time, attempt - 1);
                    
                    if attempt > 1 {
                        info!("✅ Message envoyé après {} tentatives en {:?}", attempt, response_time);
                    }
                    return Ok(());
                }
                Err(e) => {
                    last_error = Some(e.clone());
                    self.metrics.retry_attempts += 1;
                    
                    if let TelegramError::RateLimit(_) = e {
                        self.metrics.rate_limit_hits += 1;
                    }
                    
                    if attempt < config.max_attempts {
                        let delay = self.calculate_backoff_delay(&config, attempt);
                        warn!("❌ Tentative {}/{} échouée: {}. Retry dans {:?}", 
                              attempt, config.max_attempts, e, delay);
                        sleep(delay).await;
                    }
                }
            }
        }

        // Enregistrer l'échec
        let response_time = start_time.elapsed();
        self.record_message_failure(&validated_message, response_time, config.max_attempts - 1);
        
        Err(last_error.unwrap())
    }

    async fn send_message_internal(&mut self, message: &str) -> Result<(), TelegramError> {
        // Vérifier le rate limiting
        self.rate_limiter.check_rate_limit().await?;
        
        let url = format!("{}/bot{}/sendMessage", self.config.api_base_url, self.config.bot_token);
        let payload = json!({
            "chat_id": self.config.chat_id,
            "text": message,
            "parse_mode": "HTML",
            "disable_web_page_preview": true
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| TelegramError::Network(e.to_string()))?;

        self.handle_api_response(response).await
    }

    async fn handle_api_response(&self, response: Response) -> Result<(), TelegramError> {
        let status = response.status();
        
        if status.is_success() {
            debug!("✅ Message Telegram envoyé avec succès");
            return Ok(());
        }

        let error_text = response.text().await.unwrap_or_default();
        
        match status.as_u16() {
            400 => Err(TelegramError::ApiError(format!("Requête invalide: {}", error_text))),
            401 => Err(TelegramError::InvalidToken),
            403 => Err(TelegramError::ApiError("Bot bloqué ou permissions insuffisantes".to_string())),
            404 => Err(TelegramError::InvalidChatId(self.config.chat_id.clone())),
            429 => {
                // Extraire le temps d'attente si disponible
                let retry_after = 60; // Valeur par défaut
                Err(TelegramError::RateLimit(retry_after))
            }
            _ => Err(TelegramError::ApiError(format!("Erreur HTTP {}: {}", status, error_text))),
        }
    }

    fn calculate_backoff_delay(&self, config: &RetryConfig, attempt: u32) -> Duration {
        let delay_ms = config.initial_delay.as_millis() as f64 
            * config.backoff_multiplier.powi((attempt - 1) as i32);
        
        let delay = Duration::from_millis(delay_ms as u64);
        std::cmp::min(delay, config.max_delay)
    }

    fn record_message_success(&mut self, message: &str, response_time: Duration, retry_count: u32) {
        self.metrics.total_messages += 1;
        self.metrics.successful_messages += 1;
        
        // Mettre à jour le temps de réponse moyen
        let total_time = self.metrics.average_response_time.as_millis() as u64 * (self.metrics.successful_messages - 1)
            + response_time.as_millis() as u64;
        self.metrics.average_response_time = Duration::from_millis(total_time / self.metrics.successful_messages);

        self.message_history.push(MessageInfo {
            timestamp: chrono::Local::now(),
            message_type: MessageType::Info,
            content: message.to_string(),
            success: true,
            response_time: Some(response_time),
            retry_count,
        });
    }

    fn record_message_failure(&mut self, message: &str, response_time: Duration, retry_count: u32) {
        self.metrics.total_messages += 1;
        self.metrics.failed_messages += 1;

        self.message_history.push(MessageInfo {
            timestamp: chrono::Local::now(),
            message_type: MessageType::Error,
            content: message.to_string(),
            success: false,
            response_time: Some(response_time),
            retry_count,
        });
    }

    /// Exécute un diagnostic complet
    pub async fn run_diagnostics(&self) -> Result<DiagnosticReport, TelegramError> {
        let mut report = DiagnosticReport::new();
        
        info!("🔍 Démarrage du diagnostic Telegram CachyPac");
        
        // Test 1: Connectivité Internet
        report.add_test("Connectivité Internet", self.test_internet_connectivity().await);
        
        // Test 2: API Telegram
        report.add_test("API Telegram", self.test_telegram_api().await);
        
        // Test 3: Authentification Bot
        report.add_test("Authentification Bot", self.test_bot_authentication().await);
        
        // Test 4: Permissions Chat
        report.add_test("Permissions Chat", self.test_chat_permissions().await);
        
        // Test 5: Validation Messages
        report.add_test("Validation Messages", self.test_message_validation().await);
        
        // Déterminer le statut global
        report.determine_overall_status();
        
        info!("📊 Diagnostic terminé: {:?}", report.overall_status);
        Ok(report)
    }

    async fn test_internet_connectivity(&self) -> TestResult {
        match self.client.get("https://www.google.com").timeout(Duration::from_secs(10)).send().await {
            Ok(_) => TestResult::Success("Connectivité Internet OK".to_string()),
            Err(e) => TestResult::Failure(format!("Pas de connexion Internet: {}", e)),
        }
    }

    async fn test_telegram_api(&self) -> TestResult {
        match self.client.get("https://api.telegram.org").timeout(Duration::from_secs(10)).send().await {
            Ok(response) if response.status().is_success() => {
                TestResult::Success("API Telegram accessible".to_string())
            }
            Ok(response) => TestResult::Warning(
                format!("API Telegram répond mais avec erreur: {}", response.status())
            ),
            Err(e) => TestResult::Failure(format!("API Telegram inaccessible: {}", e)),
        }
    }

    async fn test_bot_authentication(&self) -> TestResult {
        match self.config.test_authentication().await {
            Ok(bot_info) => TestResult::Success(
                format!("Bot authentifié: {} (ID: {})", bot_info.first_name, bot_info.id)
            ),
            Err(TelegramError::InvalidToken) => TestResult::Failure("Token bot invalide".to_string()),
            Err(e) => TestResult::Failure(format!("Erreur authentification: {}", e)),
        }
    }

    async fn test_chat_permissions(&self) -> TestResult {
        // Test basique d'envoi de message de diagnostic
        let test_message = "🔍 Test de diagnostic CachyPac";
        
        let url = format!("{}/bot{}/sendMessage", self.config.api_base_url, self.config.bot_token);
        let payload = json!({
            "chat_id": self.config.chat_id,
            "text": test_message,
            "parse_mode": "HTML"
        });

        match self.client.post(&url).json(&payload).send().await {
            Ok(response) if response.status().is_success() => {
                TestResult::Success("Permissions chat OK - Message test envoyé".to_string())
            }
            Ok(response) if response.status() == 403 => {
                TestResult::Failure("Bot bloqué ou permissions insuffisantes".to_string())
            }
            Ok(response) if response.status() == 404 => {
                TestResult::Failure(format!("Chat ID invalide: {}", self.config.chat_id))
            }
            Ok(response) => TestResult::Warning(
                format!("Réponse inattendue: {}", response.status())
            ),
            Err(e) => TestResult::Failure(format!("Erreur test permissions: {}", e)),
        }
    }

    async fn test_message_validation(&self) -> TestResult {
        let long_message = "A".repeat(4000);
        let test_cases = vec![
            ("Message simple", "Test simple"),
            ("Message HTML", "<b>Test</b> avec <i>HTML</i>"),
            ("Message long", long_message.as_str()),
            ("Message avec émojis", "🚀 Test émojis 🎉"),
        ];

        let mut results = Vec::new();
        
        for (name, message) in test_cases {
            match MessageValidator::validate_message(message) {
                Ok(_) => results.push(format!("✅ {}", name)),
                Err(e) => results.push(format!("❌ {}: {}", name, e)),
            }
        }

        if results.iter().all(|r| r.starts_with("✅")) {
            TestResult::Success(format!("Validation OK: {}", results.join(", ")))
        } else {
            TestResult::Warning(format!("Validation partielle: {}", results.join(", ")))
        }
    }

    /// Récupère les métriques actuelles
    pub fn get_metrics(&self) -> &TelegramMetrics {
        &self.metrics
    }

    /// Récupère l'historique des messages
    pub fn get_message_history(&self) -> &[MessageInfo] {
        &self.message_history
    }
}

/// Rapport de diagnostic
#[derive(Debug)]
pub struct DiagnosticReport {
    pub tests: Vec<(String, TestResult)>,
    pub overall_status: OverallStatus,
    pub recommendations: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Local>,
}

impl DiagnosticReport {
    pub fn new() -> Self {
        Self {
            tests: Vec::new(),
            overall_status: OverallStatus::Unknown,
            recommendations: Vec::new(),
            timestamp: chrono::Local::now(),
        }
    }

    pub fn add_test(&mut self, name: &str, result: TestResult) {
        self.tests.push((name.to_string(), result));
    }

    pub fn determine_overall_status(&mut self) {
        let mut has_failure = false;
        let mut has_warning = false;

        for (_, result) in &self.tests {
            match result {
                TestResult::Failure(_) => has_failure = true,
                TestResult::Warning(_) => has_warning = true,
                TestResult::Success(_) => {}
            }
        }

        self.overall_status = if has_failure {
            OverallStatus::Critical
        } else if has_warning {
            OverallStatus::Warning
        } else {
            OverallStatus::Healthy
        };

        // Générer des recommandations
        self.generate_recommendations();
    }

    fn generate_recommendations(&mut self) {
        for (test_name, result) in &self.tests {
            match result {
                TestResult::Failure(msg) => {
                    match test_name.as_str() {
                        "Connectivité Internet" => {
                            self.recommendations.push("Vérifiez votre connexion Internet".to_string());
                        }
                        "Authentification Bot" => {
                            self.recommendations.push("Vérifiez votre token bot Telegram".to_string());
                        }
                        "Permissions Chat" => {
                            self.recommendations.push("Vérifiez que le bot a accès au chat".to_string());
                        }
                        _ => {
                            self.recommendations.push(format!("Résoudre: {}", msg));
                        }
                    }
                }
                TestResult::Warning(msg) => {
                    self.recommendations.push(format!("Attention: {}", msg));
                }
                TestResult::Success(_) => {}
            }
        }
    }
}

/// Résultat d'un test de diagnostic
#[derive(Debug)]
pub enum TestResult {
    Success(String),
    Warning(String),
    Failure(String),
}

/// Statut global du système
#[derive(Debug)]
pub enum OverallStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_validation() {
        assert!(TelegramConfig::is_valid_token("123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"));
        assert!(!TelegramConfig::is_valid_token("invalid_token"));
        assert!(!TelegramConfig::is_valid_token("123456789"));
    }

    #[test]
    fn test_chat_id_validation() {
        assert!(TelegramConfig::is_valid_chat_id("123456789"));
        assert!(TelegramConfig::is_valid_chat_id("@username"));
        assert!(!TelegramConfig::is_valid_chat_id(""));
    }

    #[test]
    fn test_message_validation() {
        assert!(MessageValidator::validate_message("Test simple").is_ok());
        assert!(MessageValidator::validate_message("<b>Test HTML</b>").is_ok());
        
        let long_message = "A".repeat(5000);
        assert!(matches!(
            MessageValidator::validate_message(&long_message),
            Err(TelegramError::MessageTooLong(_))
        ));
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(2, 5);
        
        // Première requête OK
        assert!(limiter.check_rate_limit().await.is_ok());
        
        // Deuxième requête OK
        assert!(limiter.check_rate_limit().await.is_ok());
        
        // Troisième requête devrait déclencher le rate limit
        assert!(matches!(
            limiter.check_rate_limit().await,
            Err(TelegramError::RateLimit(_))
        ));
    }
}