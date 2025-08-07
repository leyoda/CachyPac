//! Tests d'intégration pour le module Telegram
//! 
//! Ces tests vérifient le bon fonctionnement de l'intégration Telegram
//! avec gestion des erreurs, retry, rate limiting et diagnostics.

use cachypac::telegram_robust::*;
use std::time::Duration;
use tokio::time::sleep;

/// Tests de configuration Telegram
#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_valid_telegram_config() {
        let config = TelegramConfig::new(
            "123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string(),
            "123456789".to_string()
        );
        assert!(config.is_ok());
    }

    #[test]
    fn test_invalid_token_format() {
        let config = TelegramConfig::new(
            "invalid_token".to_string(),
            "123456789".to_string()
        );
        assert!(matches!(config, Err(TelegramError::InvalidToken)));
    }

    #[test]
    fn test_invalid_chat_id() {
        let config = TelegramConfig::new(
            "123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string(),
            "".to_string()
        );
        assert!(matches!(config, Err(TelegramError::InvalidChatId(_))));
    }

    #[test]
    fn test_token_validation_edge_cases() {
        // Token trop court
        assert!(!TelegramConfig::is_valid_token("123:ABC"));
        
        // Token sans séparateur
        assert!(!TelegramConfig::is_valid_token("123456789ABC-DEF"));
        
        // Token avec caractères non numériques dans l'ID
        assert!(!TelegramConfig::is_valid_token("12345abc:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"));
        
        // Token valide
        assert!(TelegramConfig::is_valid_token("123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"));
    }

    #[test]
    fn test_chat_id_validation_edge_cases() {
        // Chat ID numérique valide
        assert!(TelegramConfig::is_valid_chat_id("123456789"));
        assert!(TelegramConfig::is_valid_chat_id("-123456789"));
        
        // Username valide
        assert!(TelegramConfig::is_valid_chat_id("@username"));
        
        // Chat ID invalide
        assert!(!TelegramConfig::is_valid_chat_id(""));
        assert!(!TelegramConfig::is_valid_chat_id("invalid"));
    }
}

/// Tests de validation des messages
#[cfg(test)]
mod message_validation_tests {
    use super::*;

    #[test]
    fn test_message_length_validation() {
        // Message normal
        let result = MessageValidator::validate_message("Message normal");
        assert!(result.is_ok());

        // Message vide
        let result = MessageValidator::validate_message("");
        assert!(matches!(result, Err(TelegramError::ApiError(_))));

        // Message trop long
        let long_message = "A".repeat(5000);
        let result = MessageValidator::validate_message(&long_message);
        assert!(matches!(result, Err(TelegramError::MessageTooLong(_))));

        // Message à la limite
        let limit_message = "A".repeat(4096);
        let result = MessageValidator::validate_message(&limit_message);
        assert!(result.is_ok());
    }

    #[test]
    fn test_html_sanitization() {
        let message = "Test <script>alert('xss')</script> avec <b>gras</b>";
        let result = MessageValidator::validate_message(message).unwrap();
        
        // Le script doit être échappé
        assert!(result.contains("&lt;script&gt;"));
        assert!(!result.contains("<script>"));
        
        // Le gras doit être préservé
        assert!(result.contains("<b>gras</b>"));
    }

    #[test]
    fn test_allowed_html_tags() {
        let test_cases = vec![
            ("<b>Gras</b>", true),
            ("<i>Italique</i>", true),
            ("<code>Code</code>", true),
            ("<pre>Préformaté</pre>", true),
            ("<script>Malveillant</script>", false),
            ("<div>Non autorisé</div>", false),
        ];

        for (message, should_contain_tag) in test_cases {
            let result = MessageValidator::validate_message(message).unwrap();
            
            if should_contain_tag {
                // Les balises autorisées doivent être préservées
                assert!(result.contains(&message.replace("Gras", "Gras"))
                    || result.contains(&message.replace("Italique", "Italique"))
                    || result.contains(&message.replace("Code", "Code"))
                    || result.contains(&message.replace("Préformaté", "Préformaté")));
            } else {
                // Les balises non autorisées doivent être échappées
                assert!(result.contains("&lt;") && result.contains("&gt;"));
            }
        }
    }

    #[test]
    fn test_special_characters() {
        let message = "Test avec & < > caractères spéciaux";
        let result = MessageValidator::validate_message(message).unwrap();
        
        assert!(result.contains("&amp;"));
        assert!(result.contains("&lt;"));
        assert!(result.contains("&gt;"));
    }
}

/// Tests du rate limiter
#[cfg(test)]
mod rate_limiter_tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_per_second() {
        let mut limiter = RateLimiter::new(2, 10);
        
        // Première requête OK
        assert!(limiter.check_rate_limit().await.is_ok());
        
        // Deuxième requête OK
        assert!(limiter.check_rate_limit().await.is_ok());
        
        // Troisième requête devrait déclencher le rate limit
        let result = limiter.check_rate_limit().await;
        assert!(matches!(result, Err(TelegramError::RateLimit(_))));
    }

    #[tokio::test]
    async fn test_rate_limiter_per_minute() {
        let mut limiter = RateLimiter::new(10, 2);
        
        // Première requête OK
        assert!(limiter.check_rate_limit().await.is_ok());
        
        // Deuxième requête OK
        assert!(limiter.check_rate_limit().await.is_ok());
        
        // Troisième requête devrait déclencher le rate limit par minute
        let result = limiter.check_rate_limit().await;
        assert!(matches!(result, Err(TelegramError::RateLimit(_))));
    }

    #[tokio::test]
    async fn test_rate_limiter_cleanup() {
        let mut limiter = RateLimiter::new(1, 5);
        
        // Faire une requête
        assert!(limiter.check_rate_limit().await.is_ok());
        
        // Attendre un peu pour que la requête soit "ancienne"
        sleep(Duration::from_millis(100)).await;
        
        // Vérifier que le nettoyage fonctionne
        let initial_len = limiter.requests.len();
        limiter.cleanup_old_requests(std::time::Instant::now() + Duration::from_secs(70));
        assert!(limiter.requests.len() < initial_len || initial_len == 0);
    }
}

/// Tests de retry et backoff
#[cfg(test)]
mod retry_tests {
    use super::*;

    #[test]
    fn test_backoff_calculation() {
        let config = RetryConfig::default();
        let notifier = create_test_notifier();
        
        // Premier retry
        let delay1 = notifier.calculate_backoff_delay(&config, 1);
        assert_eq!(delay1, config.initial_delay);
        
        // Deuxième retry
        let delay2 = notifier.calculate_backoff_delay(&config, 2);
        assert!(delay2 > delay1);
        
        // Troisième retry
        let delay3 = notifier.calculate_backoff_delay(&config, 3);
        assert!(delay3 > delay2);
        
        // Vérifier que le délai maximum est respecté
        let delay_max = notifier.calculate_backoff_delay(&config, 10);
        assert!(delay_max <= config.max_delay);
    }

    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.initial_delay, Duration::from_millis(500));
        assert_eq!(config.max_delay, Duration::from_secs(30));
        assert_eq!(config.backoff_multiplier, 2.0);
    }

    fn create_test_notifier() -> RobustTelegramNotifier {
        let config = TelegramConfig::new(
            "123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string(),
            "123456789".to_string()
        ).unwrap();
        
        RobustTelegramNotifier::new(config).unwrap()
    }
}

/// Tests de métriques
#[cfg(test)]
mod metrics_tests {
    use super::*;

    #[test]
    fn test_metrics_initialization() {
        let metrics = TelegramMetrics::default();
        
        assert_eq!(metrics.total_messages, 0);
        assert_eq!(metrics.successful_messages, 0);
        assert_eq!(metrics.failed_messages, 0);
        assert_eq!(metrics.average_response_time, Duration::from_millis(0));
        assert_eq!(metrics.rate_limit_hits, 0);
        assert_eq!(metrics.retry_attempts, 0);
        assert!(metrics.last_error.is_none());
    }

    #[tokio::test]
    async fn test_metrics_recording() {
        let mut notifier = create_test_notifier();
        
        // Enregistrer un succès
        notifier.record_message_success(
            "Test message",
            Duration::from_millis(100),
            0
        );
        
        let metrics = notifier.get_metrics();
        assert_eq!(metrics.total_messages, 1);
        assert_eq!(metrics.successful_messages, 1);
        assert_eq!(metrics.failed_messages, 0);
        assert_eq!(metrics.average_response_time, Duration::from_millis(100));
        
        // Enregistrer un échec
        notifier.record_message_failure(
            "Failed message",
            Duration::from_millis(200),
            2
        );
        
        let metrics = notifier.get_metrics();
        assert_eq!(metrics.total_messages, 2);
        assert_eq!(metrics.successful_messages, 1);
        assert_eq!(metrics.failed_messages, 1);
    }

    fn create_test_notifier() -> RobustTelegramNotifier {
        let config = TelegramConfig::new(
            "123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string(),
            "123456789".to_string()
        ).unwrap();
        
        RobustTelegramNotifier::new(config).unwrap()
    }
}

/// Tests de diagnostic
#[cfg(test)]
mod diagnostic_tests {
    use super::*;

    #[test]
    fn test_diagnostic_report_creation() {
        let mut report = DiagnosticReport::new();
        
        assert!(report.tests.is_empty());
        assert!(matches!(report.overall_status, OverallStatus::Unknown));
        assert!(report.recommendations.is_empty());
    }

    #[test]
    fn test_diagnostic_status_determination() {
        let mut report = DiagnosticReport::new();
        
        // Ajouter des tests avec différents résultats
        report.add_test("Test Success", TestResult::Success("OK".to_string()));
        report.add_test("Test Warning", TestResult::Warning("Attention".to_string()));
        report.add_test("Test Failure", TestResult::Failure("Erreur".to_string()));
        
        report.determine_overall_status();
        
        // Avec un échec, le statut doit être critique
        assert!(matches!(report.overall_status, OverallStatus::Critical));
        assert!(!report.recommendations.is_empty());
    }

    #[test]
    fn test_diagnostic_status_warning_only() {
        let mut report = DiagnosticReport::new();
        
        report.add_test("Test Success", TestResult::Success("OK".to_string()));
        report.add_test("Test Warning", TestResult::Warning("Attention".to_string()));
        
        report.determine_overall_status();
        
        // Avec seulement des avertissements, le statut doit être warning
        assert!(matches!(report.overall_status, OverallStatus::Warning));
    }

    #[test]
    fn test_diagnostic_status_healthy() {
        let mut report = DiagnosticReport::new();
        
        report.add_test("Test Success 1", TestResult::Success("OK 1".to_string()));
        report.add_test("Test Success 2", TestResult::Success("OK 2".to_string()));
        
        report.determine_overall_status();
        
        // Avec seulement des succès, le statut doit être healthy
        assert!(matches!(report.overall_status, OverallStatus::Healthy));
        assert!(report.recommendations.is_empty());
    }
}

/// Tests d'intégration avec mocks
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_notifier_creation() {
        let config = TelegramConfig::new(
            "123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string(),
            "123456789".to_string()
        ).unwrap();
        
        let notifier = RobustTelegramNotifier::new(config);
        assert!(notifier.is_ok());
    }

    #[tokio::test]
    async fn test_message_history_tracking() {
        let config = TelegramConfig::new(
            "123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string(),
            "123456789".to_string()
        ).unwrap();
        
        let mut notifier = RobustTelegramNotifier::new(config).unwrap();
        
        // Simuler l'enregistrement d'un message
        notifier.record_message_success(
            "Test message",
            Duration::from_millis(150),
            0
        );
        
        let history = notifier.get_message_history();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].content, "Test message");
        assert!(history[0].success);
        assert_eq!(history[0].retry_count, 0);
        assert!(history[0].response_time.is_some());
    }

    #[test]
    fn test_error_types() {
        // Test que tous les types d'erreur peuvent être créés
        let _network_error = TelegramError::Network(
            reqwest::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "test"))
        );
        let _invalid_token = TelegramError::InvalidToken;
        let _invalid_chat = TelegramError::InvalidChatId("test".to_string());
        let _message_too_long = TelegramError::MessageTooLong(5000);
        let _rate_limit = TelegramError::RateLimit(60);
        let _api_error = TelegramError::ApiError("test".to_string());
        let _timeout = TelegramError::Timeout;
        let _invalid_config = TelegramError::InvalidConfig("test".to_string());
        let _webhook_error = TelegramError::WebhookError("test".to_string());
    }
}

/// Tests de performance et stress
#[cfg(test)]
mod performance_tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_performance() {
        let mut limiter = RateLimiter::new(100, 1000);
        let start = std::time::Instant::now();
        
        // Faire plusieurs requêtes rapides
        for _ in 0..50 {
            let _ = limiter.check_rate_limit().await;
        }
        
        let duration = start.elapsed();
        // Le rate limiter ne devrait pas ajouter de délai significatif pour 50 requêtes
        assert!(duration < Duration::from_millis(100));
    }

    #[test]
    fn test_message_validation_performance() {
        let message = "Test message with <b>HTML</b> and special chars: &<>".repeat(100);
        let start = std::time::Instant::now();
        
        for _ in 0..1000 {
            let _ = MessageValidator::validate_message(&message);
        }
        
        let duration = start.elapsed();
        // La validation ne devrait pas être trop lente
        assert!(duration < Duration::from_millis(100));
    }
}

/// Tests de cas limites
#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_message_validation() {
        let result = MessageValidator::validate_message("");
        assert!(matches!(result, Err(TelegramError::ApiError(_))));
    }

    #[test]
    fn test_unicode_message_validation() {
        let unicode_message = "Test avec émojis 🚀🎉 et caractères spéciaux: àéèùç";
        let result = MessageValidator::validate_message(unicode_message);
        assert!(result.is_ok());
        
        let validated = result.unwrap();
        assert!(validated.contains("🚀🎉"));
        assert!(validated.contains("àéèùç"));
    }

    #[test]
    fn test_very_long_token() {
        let very_long_token = format!("123456789:{}", "A".repeat(1000));
        let config = TelegramConfig::new(very_long_token, "123456789".to_string());
        assert!(config.is_ok()); // Un token très long mais valide devrait être accepté
    }

    #[test]
    fn test_negative_chat_id() {
        let config = TelegramConfig::new(
            "123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string(),
            "-123456789".to_string()
        );
        assert!(config.is_ok()); // Les chat IDs négatifs sont valides (groupes)
    }

    #[tokio::test]
    async fn test_rate_limiter_edge_cases() {
        // Rate limiter avec limites très basses
        let mut limiter = RateLimiter::new(1, 1);
        
        assert!(limiter.check_rate_limit().await.is_ok());
        
        let result = limiter.check_rate_limit().await;
        assert!(matches!(result, Err(TelegramError::RateLimit(_))));
    }
}