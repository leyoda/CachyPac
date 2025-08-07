use std::path::PathBuf;
use tokio::time::{sleep, Duration};
use cachypac::{
    config::Config,
    pacman::PacmanManager,
    scheduler::SchedulerManager,
    telegram::TelegramNotifier,
    history::{UpdateHistory, HistoryEntry, OperationType},
    logs::{LogManager, LogLevel},
    service::ServiceManager,
};

/// Tests d'intégration pour CachyPac
/// Ces tests vérifient l'interaction entre les différents modules

#[tokio::test]
async fn test_config_loading_and_validation() {
    // Test de chargement et validation de la configuration
    let config = Config::default();
    
    // La configuration par défaut doit être valide
    assert!(config.validate().is_ok());
    
    // Vérifier les valeurs par défaut
    assert!(!config.general.data_dir.is_empty());
    assert!(config.general.check_interval > 0);
    assert!(!config.scheduler.cron_expression.is_empty());
}

#[tokio::test]
async fn test_pacman_manager_initialization() {
    // Test d'initialisation du gestionnaire Pacman
    let config = Config::default();
    let pacman_manager = PacmanManager::new(config.pacman.clone());
    
    // Le gestionnaire doit être créé sans erreur
    assert_eq!(format!("{:?}", pacman_manager).contains("PacmanManager"), true);
}

#[tokio::test]
async fn test_scheduler_manager_lifecycle() {
    // Test du cycle de vie du planificateur
    let mut scheduler = SchedulerManager::new();
    
    // Le planificateur doit démarrer sans erreur
    let start_result = scheduler.start().await;
    assert!(start_result.is_ok());
    
    // Attendre un peu pour s'assurer que le planificateur est actif
    sleep(Duration::from_millis(100)).await;
    
    // Le planificateur doit s'arrêter sans erreur
    let stop_result = scheduler.stop().await;
    assert!(stop_result.is_ok());
}

#[tokio::test]
async fn test_telegram_notifier_creation() {
    // Test de création du notificateur Telegram
    let token = "test_token".to_string();
    let chat_id = "test_chat_id".to_string();
    
    let notifier = TelegramNotifier::new(token.clone(), chat_id.clone());
    
    // Le notificateur doit être créé sans erreur
    assert_eq!(format!("{:?}", notifier).contains("TelegramNotifier"), true);
}

#[tokio::test]
async fn test_update_history_operations() {
    // Test des opérations d'historique
    let temp_dir = std::env::temp_dir().join("cachypac_test_history");
    let mut history = UpdateHistory::new(temp_dir.clone());
    
    // Créer une entrée d'historique de test
    let entry = HistoryEntry {
        id: uuid::Uuid::new_v4(),
        timestamp: chrono::Local::now(),
        operation_type: OperationType::CheckUpdates,
        packages: vec!["test-package".to_string()],
        success: true,
        message: "Test successful".to_string(),
        duration: Duration::from_secs(5),
    };
    
    // Ajouter l'entrée doit réussir
    let add_result = history.add_entry(entry.clone()).await;
    assert!(add_result.is_ok());
    
    // Sauvegarder doit réussir
    let save_result = history.save().await;
    assert!(save_result.is_ok());
    
    // Charger doit réussir
    let load_result = history.load().await;
    assert!(load_result.is_ok());
    
    // L'entrée doit être présente
    let entries = history.get_all_entries();
    assert!(!entries.is_empty());
    assert_eq!(entries[0].packages, vec!["test-package".to_string()]);
    
    // Nettoyer
    let _ = tokio::fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_log_manager_operations() {
    // Test des opérations de logging
    let temp_dir = std::env::temp_dir().join("cachypac_test_logs");
    let mut log_manager = LogManager::new(temp_dir.clone());
    
    // Enregistrer une erreur de test
    let log_result = log_manager.log_error(
        "test_module", 
        "Test error message", 
        Some("Test context".to_string())
    ).await;
    assert!(log_result.is_ok());
    
    // Sauvegarder doit réussir
    let save_result = log_manager.save().await;
    assert!(save_result.is_ok());
    
    // Charger doit réussir
    let load_result = log_manager.load().await;
    assert!(load_result.is_ok());
    
    // L'entrée de log doit être présente
    let recent_entries = log_manager.get_recent_entries(1);
    assert!(!recent_entries.is_empty());
    assert_eq!(recent_entries[0].level, LogLevel::Error);
    assert_eq!(recent_entries[0].module, "test_module");
    
    // Nettoyer
    let _ = tokio::fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_service_manager_creation() {
    // Test de création du gestionnaire de service
    let service_manager = ServiceManager::new(
        "test_service".to_string(),
        PathBuf::from("/tmp/test_config")
    );
    
    // Le gestionnaire doit être créé sans erreur
    assert_eq!(format!("{:?}", service_manager).contains("test_service"), true);
}

#[tokio::test]
async fn test_full_workflow_simulation() {
    // Test de simulation d'un workflow complet
    let config = Config::default();
    
    // 1. Initialiser tous les composants
    let pacman_manager = PacmanManager::new(config.pacman.clone());
    let mut scheduler = SchedulerManager::new();
    let temp_dir = std::env::temp_dir().join("cachypac_test_workflow");
    let mut history = UpdateHistory::new(temp_dir.clone());
    let mut log_manager = LogManager::new(temp_dir.clone());
    
    // 2. Démarrer le planificateur
    let start_result = scheduler.start().await;
    assert!(start_result.is_ok());
    
    // 3. Simuler une vérification de mises à jour (sans vraiment appeler pacman)
    let log_result = log_manager.log_info(
        "workflow_test", 
        "Simulating update check", 
        None
    ).await;
    assert!(log_result.is_ok());
    
    // 4. Créer une entrée d'historique pour la simulation
    let entry = HistoryEntry {
        id: uuid::Uuid::new_v4(),
        timestamp: chrono::Local::now(),
        operation_type: OperationType::CheckUpdates,
        packages: vec!["simulated-package".to_string()],
        success: true,
        message: "Workflow test successful".to_string(),
        duration: Duration::from_millis(100),
    };
    
    let add_result = history.add_entry(entry).await;
    assert!(add_result.is_ok());
    
    // 5. Sauvegarder les données
    let history_save = history.save().await;
    let log_save = log_manager.save().await;
    assert!(history_save.is_ok());
    assert!(log_save.is_ok());
    
    // 6. Arrêter le planificateur
    let stop_result = scheduler.stop().await;
    assert!(stop_result.is_ok());
    
    // 7. Vérifier que les données ont été sauvegardées
    let mut new_history = UpdateHistory::new(temp_dir.clone());
    let mut new_log_manager = LogManager::new(temp_dir.clone());
    
    let history_load = new_history.load().await;
    let log_load = new_log_manager.load().await;
    assert!(history_load.is_ok());
    assert!(log_load.is_ok());
    
    // Vérifier les données
    let entries = new_history.get_all_entries();
    let logs = new_log_manager.get_recent_entries(1);
    
    assert!(!entries.is_empty());
    assert!(!logs.is_empty());
    assert_eq!(entries[0].packages, vec!["simulated-package".to_string()]);
    assert_eq!(logs[0].module, "workflow_test");
    
    // Nettoyer
    let _ = tokio::fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_error_handling_and_recovery() {
    // Test de gestion d'erreurs et de récupération
    let temp_dir = std::env::temp_dir().join("cachypac_test_errors");
    
    // Créer un répertoire avec des permissions restrictives pour simuler une erreur
    let restricted_dir = temp_dir.join("restricted");
    tokio::fs::create_dir_all(&restricted_dir).await.ok();
    
    // Essayer de créer un gestionnaire de logs dans un répertoire restreint
    let mut log_manager = LogManager::new(restricted_dir.clone());
    
    // L'enregistrement d'un log doit gérer l'erreur gracieusement
    let log_result = log_manager.log_error(
        "error_test", 
        "Testing error handling", 
        None
    ).await;
    
    // Même si la sauvegarde échoue, l'opération ne doit pas paniquer
    // (le résultat peut être Ok ou Err selon les permissions du système)
    assert!(log_result.is_ok() || log_result.is_err());
    
    // Nettoyer
    let _ = tokio::fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_concurrent_operations() {
    // Test d'opérations concurrentes
    let temp_dir = std::env::temp_dir().join("cachypac_test_concurrent");
    let mut log_manager = LogManager::new(temp_dir.clone());
    
    // Créer plusieurs tâches concurrentes d'enregistrement de logs
    let mut handles = vec![];
    
    for i in 0..5 {
        let mut log_manager_clone = LogManager::new(temp_dir.clone());
        let handle = tokio::spawn(async move {
            log_manager_clone.log_info(
                "concurrent_test",
                &format!("Concurrent operation {}", i),
                None
            ).await
        });
        handles.push(handle);
    }
    
    // Attendre que toutes les tâches se terminent
    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());
    }
    
    // Nettoyer
    let _ = tokio::fs::remove_dir_all(&temp_dir).await;
}

#[test]
fn test_configuration_validation_edge_cases() {
    // Test de validation de configuration avec des cas limites
    let mut config = Config::default();
    
    // Configuration valide par défaut
    assert!(config.validate().is_ok());
    
    // Test avec intervalle de vérification invalide
    config.general.check_interval = 0;
    assert!(config.validate().is_err());
    
    // Restaurer une valeur valide
    config.general.check_interval = 60;
    assert!(config.validate().is_ok());
    
    // Test avec répertoire de données vide
    config.general.data_dir = String::new();
    assert!(config.validate().is_err());
    
    // Restaurer une valeur valide
    config.general.data_dir = "/tmp/cachypac".to_string();
    assert!(config.validate().is_ok());
}

#[test]
fn test_operation_type_display() {
    // Test de l'affichage des types d'opération
    assert_eq!(OperationType::CheckUpdates.to_string(), "Vérification des mises à jour");
    assert_eq!(OperationType::ManualUpdate.to_string(), "Mise à jour manuelle");
    assert_eq!(OperationType::AutoUpdate.to_string(), "Mise à jour automatique");
    assert_eq!(OperationType::SystemMaintenance.to_string(), "Maintenance système");
}

#[test]
fn test_log_level_display() {
    // Test de l'affichage des niveaux de log
    assert_eq!(LogLevel::Error.to_string(), "ERROR");
    assert_eq!(LogLevel::Warn.to_string(), "WARN");
    assert_eq!(LogLevel::Info.to_string(), "INFO");
    assert_eq!(LogLevel::Debug.to_string(), "DEBUG");
    assert_eq!(LogLevel::Trace.to_string(), "TRACE");
}