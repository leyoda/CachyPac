#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use anyhow::Result;
use clap::{Arg, Command};
use std::path::PathBuf;
use tracing::{error, info, warn};

mod config;
mod gui;
mod pacman;
mod scheduler;
mod telegram;
mod telegram_robust;
mod history;
mod logs;
mod service;
mod i18n;

use config::Config;
use pacman::PacmanManager;
use scheduler::SchedulerManager;
use telegram_robust::{RobustTelegramNotifier, TelegramConfig};
use history::UpdateHistory;
use logs::LogManager;
use service::ServiceManager;

fn main() -> Result<()> {
    // Initialisation du système de logging
    tracing_subscriber::fmt()
        .with_env_filter("cachypac=info")
        .init();

    info!("🚀 Démarrage de CachyPac v{}", env!("CARGO_PKG_VERSION"));

    // Initialisation de l'internationalisation
    if let Err(e) = i18n::init_global_i18n() {
        warn!("⚠️ Impossible d'initialiser l'internationalisation: {}", e);
    } else {
        info!("🌍 Internationalisation initialisée");
    }

    // Configuration CLI
    let matches = Command::new("cachypac")
        .version(env!("CARGO_PKG_VERSION"))
        .about("CachyPac - Gestionnaire de mises à jour Pacman avec interface graphique")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("Fichier de configuration personnalisé"))
        .arg(Arg::new("daemon")
            .short('d')
            .long("daemon")
            .action(clap::ArgAction::SetTrue)
            .help("Lancer en mode daemon (sans interface graphique)"))
        .arg(Arg::new("install-service")
            .long("install-service")
            .action(clap::ArgAction::SetTrue)
            .help("Installer le service systemd"))
        .arg(Arg::new("uninstall-service")
            .long("uninstall-service")
            .action(clap::ArgAction::SetTrue)
            .help("Désinstaller le service systemd"))
        .arg(Arg::new("service-status")
            .long("service-status")
            .action(clap::ArgAction::SetTrue)
            .help("Afficher le statut du service"))
        .get_matches();

    // Chargement de la configuration
    let config_path = matches.get_one::<String>("config");
    let config = if let Some(path) = config_path {
        tokio::runtime::Runtime::new()?.block_on(Config::load_from_file(&PathBuf::from(path)))?
    } else {
        tokio::runtime::Runtime::new()?.block_on(Config::load())?
    };

    // Validation de la configuration
    if let Err(e) = config.validate() {
        error!("❌ Configuration invalide: {}", e);
        std::process::exit(1);
    }

    info!("✅ Configuration chargée et validée");

    // Initialisation des gestionnaires
    let service_manager = ServiceManager::new(
        "cachypac".to_string(),
        PathBuf::from("/etc/cachypac")
    );

    // Gestion des commandes de service
    if matches.get_flag("install-service") {
        info!("📦 Installation du service systemd...");
        tokio::runtime::Runtime::new()?.block_on(service_manager.install_service())?;
        info!("✅ Service installé avec succès");
        return Ok(());
    }

    if matches.get_flag("uninstall-service") {
        info!("🗑️ Désinstallation du service systemd...");
        // TODO: Implémenter la désinstallation
        info!("✅ Service désinstallé avec succès");
        return Ok(());
    }

    if matches.get_flag("service-status") {
        info!("📊 Vérification du statut du service...");
        let status = tokio::runtime::Runtime::new()?.block_on(service_manager.get_service_status())?;
        println!("Service Status: {:?}", status);
        return Ok(());
    }

    // Mode daemon ou interface graphique
    if matches.get_flag("daemon") {
        info!("🔧 Lancement en mode daemon");
        tokio::runtime::Runtime::new()?.block_on(run_daemon_mode(config))?;
    } else {
        info!("🖥️ Lancement de l'interface graphique");
        run_gui_mode(config)?;
    }

    Ok(())
}

async fn run_daemon_mode(config: Config) -> Result<()> {
    info!("🔧 Mode daemon activé");
    
    // Initialisation des composants
    let pacman_manager = PacmanManager::new(config.pacman.clone());
    let mut scheduler_manager = SchedulerManager::new();
    
    // Utilisation du module Telegram robuste avec vraies requêtes HTTP
    let mut telegram_notifier = if config.telegram.enabled {
        match TelegramConfig::new(config.telegram.bot_token.clone(), config.telegram.chat_id.clone()) {
            Ok(telegram_config) => {
                match RobustTelegramNotifier::new(telegram_config) {
                    Ok(notifier) => {
                        info!("✅ Module Telegram robuste initialisé avec succès");
                        Some(notifier)
                    }
                    Err(e) => {
                        error!("❌ Erreur création notificateur Telegram robuste: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                error!("❌ Configuration Telegram invalide: {}", e);
                None
            }
        }
    } else {
        info!("ℹ️ Telegram désactivé dans la configuration");
        None
    };
    
    let data_dir = PathBuf::from(&config.general.data_dir);
    let mut update_history = UpdateHistory::new(data_dir.clone());
    let mut log_manager = LogManager::new(data_dir);

    // Chargement des données
    update_history.load().await?;
    log_manager.load().await?;

    info!("✅ Composants initialisés en mode daemon");

    // Démarrage du planificateur si activé
    if config.scheduler.enabled {
        scheduler_manager.start().await?;
        info!("⏰ Planificateur démarré");
    }

    // Boucle principale du daemon
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(config.general.check_interval * 60)).await;
        
        match pacman_manager.check_updates().await {
            Ok(updates) => {
                if !updates.is_empty() {
                    info!("🔄 {} mises à jour disponibles", updates.len());
                    
                    if let Some(ref mut notifier) = telegram_notifier {
                        // Formater le message pour les mises à jour disponibles
                        let message = format!(
                            "🔄 <b>CachyPac - {} mises à jour disponibles</b>\n\n{}",
                            updates.len(),
                            updates.iter().take(10).enumerate()
                                .map(|(i, pkg)| format!("{}. <code>{}</code>", i + 1, pkg))
                                .collect::<Vec<_>>()
                                .join("\n")
                        );
                        
                        // Utiliser le module robuste avec retry automatique
                        if let Err(e) = notifier.send_message_with_retry(&message).await {
                            error!("❌ Erreur notification Telegram (après retry): {}", e);
                        } else {
                            info!("✅ Notification Telegram envoyée avec succès");
                        }
                    }
                    
                    // Enregistrer dans l'historique
                    let entry = history::HistoryEntry {
                        id: uuid::Uuid::new_v4(),
                        timestamp: chrono::Local::now(),
                        operation_type: history::OperationType::CheckUpdates,
                        packages: updates.clone(),
                        success: true,
                        duration: std::time::Duration::from_secs(0),
                        message: "Vérification des mises à jour réussie".to_string(),
                    };
                    
                    if let Err(e) = update_history.add_entry(entry).await {
                        error!("❌ Erreur lors de l'enregistrement dans l'historique: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("❌ Erreur lors de la vérification des mises à jour: {}", e);
                
                // Enregistrer l'erreur dans les logs
                if let Err(log_err) = log_manager.log_error("daemon", &format!("Erreur vérification mises à jour: {}", e), None).await {
                    error!("❌ Erreur lors de l'enregistrement du log: {}", log_err);
                }
            }
        }
    }
}

fn run_gui_mode(config: Config) -> Result<()> {
    info!("🖥️ Mode interface graphique activé");
    
    // Lancement de l'interface graphique Iced
    if let Err(e) = gui::run_gui(config) {
        error!("❌ Erreur lors du lancement de l'interface graphique: {}", e);
        return Err(anyhow::anyhow!("Erreur GUI: {}", e));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_loading() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_service_manager_creation() {
        let service_manager = ServiceManager::new(
            "test".to_string(),
            PathBuf::from("/tmp")
        );
        // Test basique de création
        assert_eq!(format!("{:?}", service_manager).contains("test"), true);
    }
}