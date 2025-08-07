//! Exemple d'utilisation du diagnostic Telegram
//! 
//! Cet exemple montre comment utiliser le module telegram_robust pour :
//! - Configurer une connexion Telegram
//! - Exécuter un diagnostic complet
//! - Envoyer des messages avec retry automatique
//! - Gérer les erreurs et métriques

use cachypac::telegram_robust::*;
use std::env;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialiser le logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("🚀 Démarrage de l'exemple de diagnostic Telegram CachyPac");

    // Configuration depuis les variables d'environnement ou valeurs de test
    let bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .unwrap_or_else(|_| "123456789:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string());
    
    let chat_id = env::var("TELEGRAM_CHAT_ID")
        .unwrap_or_else(|_| "123456789".to_string());

    println!("📋 Configuration Telegram:");
    println!("   Token: {}...{}", &bot_token[..10], &bot_token[bot_token.len()-10..]);
    println!("   Chat ID: {}", chat_id);
    println!();

    // Étape 1: Créer la configuration
    info!("🔧 Création de la configuration Telegram");
    let config = match TelegramConfig::new(bot_token, chat_id) {
        Ok(config) => {
            println!("✅ Configuration créée avec succès");
            config
        }
        Err(e) => {
            error!("❌ Erreur de configuration: {}", e);
            println!("💡 Vérifiez que votre token et chat_id sont valides");
            return Err(e.into());
        }
    };

    // Étape 2: Créer le notificateur
    info!("🔨 Création du notificateur Telegram");
    let mut notifier = match RobustTelegramNotifier::new(config) {
        Ok(notifier) => {
            println!("✅ Notificateur créé avec succès");
            notifier
        }
        Err(e) => {
            error!("❌ Erreur création notificateur: {}", e);
            return Err(e.into());
        }
    };

    // Étape 3: Exécuter le diagnostic complet
    println!("\n🔍 === DIAGNOSTIC TELEGRAM COMPLET ===");
    
    let diagnostic_result = notifier.run_diagnostics().await;
    
    match diagnostic_result {
        Ok(report) => {
            print_diagnostic_report(&report);
        }
        Err(e) => {
            error!("❌ Erreur lors du diagnostic: {}", e);
            println!("💡 Le diagnostic a échoué, mais nous continuons avec les tests de base");
        }
    }

    // Étape 4: Tests de validation des messages
    println!("\n📝 === TESTS DE VALIDATION DES MESSAGES ===");
    test_message_validation();

    // Étape 5: Tests du rate limiter
    println!("\n⚡ === TESTS DU RATE LIMITER ===");
    test_rate_limiter().await;

    // Étape 6: Test d'envoi de message (si configuration réelle)
    if env::var("TELEGRAM_BOT_TOKEN").is_ok() && env::var("TELEGRAM_CHAT_ID").is_ok() {
        println!("\n📤 === TEST D'ENVOI DE MESSAGE RÉEL ===");
        test_real_message_sending(&mut notifier).await;
    } else {
        println!("\n📤 === SIMULATION D'ENVOI DE MESSAGE ===");
        println!("💡 Pour tester l'envoi réel, définissez TELEGRAM_BOT_TOKEN et TELEGRAM_CHAT_ID");
        test_simulated_message_sending(&mut notifier).await;
    }

    // Étape 7: Afficher les métriques finales
    println!("\n📊 === MÉTRIQUES FINALES ===");
    print_metrics(notifier.get_metrics());

    // Étape 8: Afficher l'historique des messages
    println!("\n📜 === HISTORIQUE DES MESSAGES ===");
    print_message_history(notifier.get_message_history());

    println!("\n🎉 Diagnostic Telegram terminé avec succès!");
    Ok(())
}

fn print_diagnostic_report(report: &DiagnosticReport) {
    println!("📊 Rapport de diagnostic généré le: {}", report.timestamp.format("%d/%m/%Y %H:%M:%S"));
    println!("🎯 Statut global: {:?}", report.overall_status);
    println!();

    println!("🧪 Résultats des tests:");
    for (test_name, result) in &report.tests {
        match result {
            TestResult::Success(msg) => {
                println!("  ✅ {}: {}", test_name, msg);
            }
            TestResult::Warning(msg) => {
                println!("  ⚠️  {}: {}", test_name, msg);
            }
            TestResult::Failure(msg) => {
                println!("  ❌ {}: {}", test_name, msg);
            }
        }
    }

    if !report.recommendations.is_empty() {
        println!("\n💡 Recommandations:");
        for (i, recommendation) in report.recommendations.iter().enumerate() {
            println!("  {}. {}", i + 1, recommendation);
        }
    }

    println!();
}

fn test_message_validation() {
    let long_message = "CachyPac ".repeat(100);
    let too_long_message = "A".repeat(5000);
    
    let test_cases = vec![
        ("Message simple", "Bonjour CachyPac!"),
        ("Message avec HTML", "<b>CachyPac</b> - <i>Mise à jour</i> disponible"),
        ("Message avec émojis", "🚀 CachyPac 🎉 Mise à jour terminée!"),
        ("Message avec caractères spéciaux", "Mise à jour: firefox & chromium < 100MB"),
        ("Message long", long_message.as_str()),
        ("Message trop long", too_long_message.as_str()),
        ("Message vide", ""),
    ];

    for (name, message) in test_cases {
        match MessageValidator::validate_message(message) {
            Ok(validated) => {
                println!("  ✅ {}: Message validé ({} caractères)", name, validated.len());
            }
            Err(e) => {
                println!("  ❌ {}: {}", name, e);
            }
        }
    }
}

async fn test_rate_limiter() {
    let mut limiter = RateLimiter::new(3, 10);
    
    println!("  🧪 Test des limites par seconde (max: 3)");
    for i in 1..=5 {
        match limiter.check_rate_limit().await {
            Ok(()) => {
                println!("    ✅ Requête {} autorisée", i);
            }
            Err(TelegramError::RateLimit(wait_time)) => {
                println!("    ⚠️  Requête {} bloquée - Rate limit atteint (attente: {}s)", i, wait_time);
            }
            Err(e) => {
                println!("    ❌ Requête {} erreur: {}", i, e);
            }
        }
    }
}

async fn test_real_message_sending(notifier: &mut RobustTelegramNotifier) {
    let test_message = "🔍 Test de diagnostic CachyPac - Message envoyé depuis l'exemple Rust";
    
    println!("  📤 Envoi du message de test...");
    match notifier.send_message_with_retry(test_message).await {
        Ok(()) => {
            println!("  ✅ Message envoyé avec succès!");
        }
        Err(e) => {
            println!("  ❌ Échec d'envoi: {}", e);
            
            // Suggestions basées sur le type d'erreur
            match e {
                TelegramError::InvalidToken => {
                    println!("  💡 Vérifiez votre token bot Telegram");
                }
                TelegramError::InvalidChatId(_) => {
                    println!("  💡 Vérifiez votre chat ID Telegram");
                }
                TelegramError::Network(_) => {
                    println!("  💡 Vérifiez votre connexion Internet");
                }
                TelegramError::RateLimit(wait_time) => {
                    println!("  💡 Rate limit atteint, attendez {} secondes", wait_time);
                }
                _ => {
                    println!("  💡 Consultez la documentation Telegram Bot API");
                }
            }
        }
    }
}

async fn test_simulated_message_sending(_notifier: &mut RobustTelegramNotifier) {
    println!("  🎭 Simulation d'envoi de messages...");
    
    // Simuler différents types de messages
    let messages = vec![
        "📦 CachyPac: 5 mises à jour disponibles",
        "✅ CachyPac: Mises à jour installées avec succès",
        "❌ CachyPac: Échec de mise à jour - Vérifiez les logs",
        "🔄 CachyPac: Redémarrage du système requis",
    ];

    for (i, message) in messages.iter().enumerate() {
        println!("    📝 Message {}: {}", i + 1, message);
        
        // Note: record_message_success est privée, on simule juste l'affichage
        println!("    ⏱️  Temps de réponse simulé: {}ms", 150 + i as u64 * 50);
        
        println!("    ✅ Message {} simulé avec succès", i + 1);
    }
}

fn print_metrics(metrics: &TelegramMetrics) {
    println!("📈 Statistiques d'utilisation:");
    println!("  • Messages totaux: {}", metrics.total_messages);
    println!("  • Messages réussis: {}", metrics.successful_messages);
    println!("  • Messages échoués: {}", metrics.failed_messages);
    println!("  • Temps de réponse moyen: {:?}", metrics.average_response_time);
    println!("  • Rate limits atteints: {}", metrics.rate_limit_hits);
    println!("  • Tentatives de retry: {}", metrics.retry_attempts);
    
    if let Some(ref last_error) = metrics.last_error {
        println!("  • Dernière erreur: {}", last_error);
    }

    // Calculer le taux de succès
    if metrics.total_messages > 0 {
        let success_rate = (metrics.successful_messages as f64 / metrics.total_messages as f64) * 100.0;
        println!("  • Taux de succès: {:.1}%", success_rate);
    }
}

fn print_message_history(history: &[MessageInfo]) {
    if history.is_empty() {
        println!("  📭 Aucun message dans l'historique");
        return;
    }

    println!("  📬 {} message(s) dans l'historique:", history.len());
    
    for (i, msg) in history.iter().enumerate().take(5) { // Afficher seulement les 5 derniers
        let status = if msg.success { "✅" } else { "❌" };
        let response_time = msg.response_time
            .map(|d| format!("{}ms", d.as_millis()))
            .unwrap_or_else(|| "N/A".to_string());
        
        println!("    {}. {} [{}] {} ({}ms, {} retry)", 
                 i + 1, 
                 status,
                 msg.timestamp.format("%H:%M:%S"),
                 msg.content.chars().take(50).collect::<String>(),
                 response_time,
                 msg.retry_count);
    }
    
    if history.len() > 5 {
        println!("    ... et {} autres messages", history.len() - 5);
    }
}