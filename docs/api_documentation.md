# Documentation API CachyPac

## Table des Matières

1. [Introduction](#introduction)
2. [Architecture](#architecture)
3. [Modules Principaux](#modules-principaux)
4. [API Configuration](#api-configuration)
5. [API Pacman](#api-pacman)
6. [API Scheduler](#api-scheduler)
7. [API Telegram](#api-telegram)
8. [API Historique](#api-historique)
9. [API Logs](#api-logs)
10. [API Performance](#api-performance)
11. [Exemples d'Utilisation](#exemples-dutilisation)
12. [Intégration](#intégration)

## Introduction

Cette documentation décrit l'API interne de CachyPac, permettant aux développeurs de comprendre l'architecture et d'étendre les fonctionnalités de l'application.

### Principes de Conception

- **Modularité** : Chaque fonctionnalité est encapsulée dans un module distinct
- **Asynchrone** : Utilisation intensive de `async/await` avec Tokio
- **Type Safety** : Exploitation du système de types Rust pour la sécurité
- **Performance** : Optimisations avec cache, buffers et parallélisation
- **Testabilité** : Chaque module est testable unitairement

## Architecture

### Vue d'Ensemble

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Interface     │    │   CLI/Daemon    │    │   Service       │
│   Graphique     │    │   Mode          │    │   Manager       │
│   (Iced)        │    │                 │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
         ┌───────────────────────┼───────────────────────┐
         │                       │                       │
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Configuration │    │   Pacman        │    │   Scheduler     │
│   Manager       │    │   Manager       │    │   Manager       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
         ┌───────────────────────┼───────────────────────┐
         │                       │                       │
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Telegram      │    │   Historique    │    │   Logs          │
│   Notifier      │    │   Manager       │    │   Manager       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │   Performance   │
                    │   Manager       │
                    └─────────────────┘
```

### Flux de Données

1. **Configuration** → Chargement et validation des paramètres
2. **Pacman** → Interaction avec le gestionnaire de paquets
3. **Scheduler** → Planification et exécution des tâches
4. **Telegram** → Envoi de notifications
5. **Historique** → Enregistrement des opérations
6. **Logs** → Journalisation des événements
7. **Performance** → Optimisations et cache

## Modules Principaux

### Structure des Modules

```rust
pub mod config;      // Gestion de la configuration
pub mod gui;         // Interface graphique Iced
pub mod pacman;      // Interface avec Pacman
pub mod scheduler;   // Planification des tâches
pub mod telegram;    // Notifications Telegram
pub mod history;     // Historique des opérations
pub mod logs;        // Système de logs
pub mod service;     // Gestion des services système
pub mod performance; // Optimisations de performance
```

## API Configuration

### Structures Principales

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub pacman: PacmanConfig,
    pub scheduler: SchedulerConfig,
    pub telegram: TelegramConfig,
    pub gui: GuiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub data_dir: String,
    pub log_level: String,
    pub check_interval: u64,
    pub auto_update: bool,
    pub backup_before_update: bool,
}
```

### Méthodes Principales

#### `Config::load() -> Result<Self>`
Charge la configuration depuis les emplacements par défaut.

```rust
let config = Config::load().await?;
```

#### `Config::load_with_cache(cache_ttl: Duration) -> Result<Self>`
Charge la configuration avec mise en cache.

```rust
let config = Config::load_with_cache(Duration::from_secs(300)).await?;
```

#### `Config::validate(&self) -> Result<()>`
Valide la configuration et crée les répertoires nécessaires.

```rust
config.validate()?;
```

#### `Config::invalidate_cache()`
Invalide le cache de configuration.

```rust
Config::invalidate_cache();
```

### Exemple d'Utilisation

```rust
use cachypac::config::Config;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Charger la configuration avec cache
    let config = Config::load_with_cache(Duration::from_secs(300)).await?;
    
    // Valider la configuration
    config.validate()?;
    
    // Utiliser la configuration
    println!("Répertoire de données: {}", config.general.data_dir);
    println!("Intervalle de vérification: {} minutes", config.general.check_interval);
    
    Ok(())
}
```

## API Pacman

### Structure Principale

```rust
#[derive(Debug, Clone)]
pub struct PacmanManager {
    config: PacmanConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub current_version: String,
    pub new_version: String,
    pub repository: String,
    pub size: u64,
    pub description: String,
}
```

### Méthodes Principales

#### `PacmanManager::new(config: PacmanConfig) -> Self`
Crée une nouvelle instance du gestionnaire Pacman.

```rust
let pacman = PacmanManager::new(config.pacman.clone());
```

#### `check_updates(&self) -> Result<Vec<String>>`
Vérifie les mises à jour disponibles.

```rust
let updates = pacman.check_updates().await?;
println!("Mises à jour disponibles: {}", updates.len());
```

#### `get_package_info(&self, package: &str) -> Result<PackageInfo>`
Récupère les informations détaillées d'un paquet.

```rust
let info = pacman.get_package_info("firefox").await?;
println!("Version actuelle: {}", info.current_version);
```

#### `update_packages(&self, packages: &[String]) -> Result<UpdateResult>`
Met à jour les paquets spécifiés.

```rust
let result = pacman.update_packages(&["firefox", "systemd"]).await?;
println!("Mise à jour réussie: {}", result.success);
```

### Exemple d'Utilisation

```rust
use cachypac::pacman::PacmanManager;
use cachypac::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load().await?;
    let pacman = PacmanManager::new(config.pacman.clone());
    
    // Vérifier les mises à jour
    let updates = pacman.check_updates().await?;
    
    if !updates.is_empty() {
        println!("Mises à jour disponibles:");
        for package in &updates {
            let info = pacman.get_package_info(package).await?;
            println!("  {} : {} → {}", 
                info.name, 
                info.current_version, 
                info.new_version
            );
        }
        
        // Mettre à jour tous les paquets
        let result = pacman.update_packages(&updates).await?;
        println!("Résultat: {:?}", result);
    }
    
    Ok(())
}
```

## API Scheduler

### Structures Principales

```rust
#[derive(Debug, Clone)]
pub struct SchedulerManager {
    jobs: HashMap<String, JobInfo>,
}

#[derive(Debug, Clone)]
pub struct JobInfo {
    pub id: Uuid,
    pub name: String,
    pub cron_expression: String,
    pub enabled: bool,
    pub last_run: Option<DateTime<Local>>,
    pub next_run: Option<DateTime<Local>>,
    pub run_count: u64,
    pub last_result: Option<JobResult>,
}
```

### Méthodes Principales

#### `SchedulerManager::new() -> Self`
Crée une nouvelle instance du planificateur.

```rust
let mut scheduler = SchedulerManager::new();
```

#### `start(&mut self) -> Result<()>`
Démarre le planificateur.

```rust
scheduler.start().await?;
```

#### `create_job(&mut self, name: String, cron: String, callback: JobCallback) -> Result<Uuid>`
Crée une nouvelle tâche planifiée.

```rust
let job_id = scheduler.create_job(
    "daily_update".to_string(),
    "0 2 * * *".to_string(),
    Box::new(|_| Box::pin(async { 
        println!("Exécution de la tâche quotidienne");
        Ok(())
    }))
).await?;
```

#### `get_all_jobs(&self) -> Vec<&JobInfo>`
Récupère toutes les tâches planifiées.

```rust
let jobs = scheduler.get_all_jobs();
for job in jobs {
    println!("Tâche: {} - Prochaine exécution: {:?}", job.name, job.next_run);
}
```

### Exemple d'Utilisation

```rust
use cachypac::scheduler::SchedulerManager;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut scheduler = SchedulerManager::new();
    
    // Démarrer le planificateur
    scheduler.start().await?;
    
    // Créer une tâche quotidienne
    let _job_id = scheduler.create_job(
        "daily_check".to_string(),
        "0 2 * * *".to_string(),
        Box::new(|_| Box::pin(async {
            println!("Vérification quotidienne des mises à jour");
            // Logique de vérification ici
            Ok(())
        }))
    ).await?;
    
    // Laisser le planificateur tourner
    tokio::time::sleep(Duration::from_secs(60)).await;
    
    // Arrêter le planificateur
    scheduler.stop().await?;
    
    Ok(())
}
```

## API Telegram

### Structure Principale

```rust
#[derive(Debug, Clone)]
pub struct TelegramNotifier {
    bot_token: String,
    chat_id: String,
    message_history: Vec<MessageInfo>,
}

#[derive(Debug, Clone)]
pub struct MessageInfo {
    pub timestamp: DateTime<Local>,
    pub message_type: MessageType,
    pub content: String,
    pub success: bool,
}
```

### Méthodes Principales

#### `TelegramNotifier::new(bot_token: String, chat_id: String) -> Self`
Crée une nouvelle instance du notificateur Telegram.

```rust
let notifier = TelegramNotifier::new(
    "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
    "987654321".to_string()
);
```

#### `send_message(&mut self, message: &str) -> Result<()>`
Envoie un message simple.

```rust
notifier.send_message("Test de notification").await?;
```

#### `send_updates_available(&mut self, updates: &[String]) -> Result<()>`
Envoie une notification de mises à jour disponibles.

```rust
let updates = vec!["firefox".to_string(), "systemd".to_string()];
notifier.send_updates_available(&updates).await?;
```

#### `send_updates_installed(&mut self, updates: &[String], duration: &Duration) -> Result<()>`
Envoie une notification de mises à jour installées.

```rust
notifier.send_updates_installed(&updates, &Duration::from_secs(180)).await?;
```

### Exemple d'Utilisation

```rust
use cachypac::telegram::TelegramNotifier;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut notifier = TelegramNotifier::new(
        std::env::var("TELEGRAM_BOT_TOKEN")?,
        std::env::var("TELEGRAM_CHAT_ID")?
    );
    
    // Envoyer une notification simple
    notifier.send_message("CachyPac démarré").await?;
    
    // Simuler des mises à jour disponibles
    let updates = vec!["firefox".to_string(), "linux".to_string()];
    notifier.send_updates_available(&updates).await?;
    
    // Simuler la fin des mises à jour
    notifier.send_updates_installed(&updates, &Duration::from_secs(120)).await?;
    
    Ok(())
}
```

## API Historique

### Structures Principales

```rust
#[derive(Debug, Clone)]
pub struct UpdateHistory {
    data_dir: PathBuf,
    entries: Vec<HistoryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Local>,
    pub operation_type: OperationType,
    pub packages: Vec<String>,
    pub success: bool,
    pub duration: Duration,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    CheckUpdates,
    ManualUpdate,
    AutoUpdate,
    SystemMaintenance,
}
```

### Méthodes Principales

#### `UpdateHistory::new(data_dir: PathBuf) -> Self`
Crée une nouvelle instance de l'historique.

```rust
let mut history = UpdateHistory::new(PathBuf::from("~/.local/share/cachypac"));
```

#### `add_entry(&mut self, entry: HistoryEntry) -> Result<()>`
Ajoute une entrée à l'historique.

```rust
let entry = HistoryEntry {
    id: Uuid::new_v4(),
    timestamp: Local::now(),
    operation_type: OperationType::CheckUpdates,
    packages: vec!["firefox".to_string()],
    success: true,
    duration: Duration::from_secs(30),
    message: "Vérification réussie".to_string(),
};

history.add_entry(entry).await?;
```

#### `get_all_entries(&self) -> &[HistoryEntry]`
Récupère toutes les entrées de l'historique.

```rust
let entries = history.get_all_entries();
for entry in entries {
    println!("{}: {} - {}", 
        entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
        entry.operation_type,
        entry.message
    );
}
```

### Exemple d'Utilisation

```rust
use cachypac::history::{UpdateHistory, HistoryEntry, OperationType};
use uuid::Uuid;
use chrono::Local;
use std::time::Duration;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut history = UpdateHistory::new(PathBuf::from("./data"));
    
    // Charger l'historique existant
    history.load().await?;
    
    // Ajouter une nouvelle entrée
    let entry = HistoryEntry {
        id: Uuid::new_v4(),
        timestamp: Local::now(),
        operation_type: OperationType::ManualUpdate,
        packages: vec!["firefox".to_string(), "systemd".to_string()],
        success: true,
        duration: Duration::from_secs(120),
        message: "Mise à jour manuelle réussie".to_string(),
    };
    
    history.add_entry(entry).await?;
    
    // Sauvegarder l'historique
    history.save().await?;
    
    // Afficher les statistiques
    let stats = history.get_statistics();
    println!("Total des opérations: {}", stats.total_operations);
    println!("Opérations réussies: {}", stats.successful_operations);
    
    Ok(())
}
```

## API Logs

### Structures Principales

```rust
#[derive(Debug)]
pub struct LogManager {
    data_dir: PathBuf,
    entries: Vec<LogEntry>,
    max_entries: usize,
    auto_save: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
```

### Méthodes Principales

#### `LogManager::new(data_dir: PathBuf) -> Self`
Crée une nouvelle instance du gestionnaire de logs.

```rust
let mut log_manager = LogManager::new(PathBuf::from("./logs"));
```

#### `log_error(&mut self, module: &str, message: &str, context: Option<String>) -> Result<()>`
Enregistre un log d'erreur.

```rust
log_manager.log_error(
    "pacman", 
    "Échec de la mise à jour", 
    Some("Conflit de dépendances".to_string())
).await?;
```

#### `log_info(&mut self, module: &str, message: &str, context: Option<String>) -> Result<()>`
Enregistre un log d'information.

```rust
log_manager.log_info(
    "scheduler", 
    "Tâche exécutée avec succès", 
    None
).await?;
```

#### `get_recent_entries(&self, hours: i64) -> Vec<&LogEntry>`
Récupère les entrées récentes.

```rust
let recent_logs = log_manager.get_recent_entries(24); // Dernières 24h
for log in recent_logs {
    println!("[{}] {}: {}", log.level, log.module, log.message);
}
```

### Exemple d'Utilisation

```rust
use cachypac::logs::{LogManager, LogLevel};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut log_manager = LogManager::new(PathBuf::from("./logs"));
    
    // Charger les logs existants
    log_manager.load().await?;
    
    // Enregistrer différents types de logs
    log_manager.log_info("main", "Application démarrée", None).await?;
    log_manager.log_warn("config", "Configuration par défaut utilisée", None).await?;
    log_manager.log_error(
        "pacman", 
        "Erreur de réseau", 
        Some("Timeout après 30s".to_string())
    ).await?;
    
    // Rechercher dans les logs
    let error_logs = log_manager.search("erreur", false);
    println!("Logs d'erreur trouvés: {}", error_logs.len());
    
    // Obtenir des statistiques
    let stats = log_manager.get_statistics();
    println!("Total des logs: {}", stats.total_entries);
    println!("Erreurs: {}", stats.error_count);
    
    Ok(())
}
```

## API Performance

### Structures Principales

```rust
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub max_cache_size_mb: usize,
    pub default_cache_ttl: u64,
    pub io_buffer_size: usize,
    pub auto_flush_interval_ms: u64,
    pub max_parallel_threads: usize,
}

#[derive(Debug)]
pub struct OptimizedCache<K, V> {
    // Implémentation interne
}

#[derive(Debug)]
pub struct OptimizedBuffer<T> {
    // Implémentation interne
}
```

### Méthodes Principales

#### Cache Optimisé

```rust
// Créer un cache
let cache: OptimizedCache<String, String> = OptimizedCache::new(config);

// Insérer une valeur
cache.insert("key".to_string(), "value".to_string()).await;

// Récupérer une valeur
if let Some(value) = cache.get(&"key".to_string()).await {
    println!("Valeur trouvée: {}", value);
}

// Nettoyer les entrées expirées
cache.cleanup_expired().await;
```

#### Buffer Optimisé

```rust
// Créer un buffer
let buffer: OptimizedBuffer<String> = OptimizedBuffer::new(config);

// Ajouter des éléments
let should_flush = buffer.push("item1".to_string()).await;
if should_flush {
    let items = buffer.flush().await;
    println!("Éléments à traiter: {:?}", items);
}
```

### Exemple d'Utilisation

```rust
use cachypac::performance::{PerformanceConfig, OptimizedCache, OptimizedBuffer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = PerformanceConfig::default();
    
    // Utiliser un cache optimisé
    let cache: OptimizedCache<String, Vec<String>> = OptimizedCache::new(config.clone());
    
    // Simuler la mise en cache des résultats de pacman
    let packages = vec!["firefox".to_string(), "systemd".to_string()];
    cache.insert("available_updates".to_string(), packages.clone()).await;
    
    // Récupérer depuis le cache
    if let Some(cached_packages) = cache.get(&"available_updates".to_string()).await {
        println!("Paquets depuis le cache: {:?}", cached_packages);
    }
    
    // Utiliser un buffer pour les logs
    let buffer: OptimizedBuffer<String> = OptimizedBuffer::new(config);
    
    for i in 0..10 {
        let should_flush = buffer.push(format!("Log entry {}", i)).await;
        if should_flush {
            let entries = buffer.flush().await;
            println!("Écriture de {} entrées de log", entries.len());
        }
    }
    
    Ok(())
}
```

## Exemples d'Utilisation

### Exemple Complet : Vérification et Mise à Jour

```rust
use cachypac::{
    config::Config,
    pacman::PacmanManager,
    telegram::TelegramNotifier,
    history::{UpdateHistory, HistoryEntry, OperationType},
    logs::LogManager,
};
use uuid::Uuid;
use chrono::Local;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Charger la configuration
    let config = Config::load().await?;
    config.validate()?;
    
    // Initialiser les composants
    let pacman = PacmanManager::new(config.pacman.clone());
    let mut telegram = if config.telegram.enabled {
        Some(TelegramNotifier::new(
            config.telegram.bot_token.clone(),
            config.telegram.chat_id.clone()
        ))
    } else {
        None
    };
    
    let data_dir = std::path::PathBuf::from(&config.general.data_dir);
    let mut history = UpdateHistory::new(data_dir.clone());
    let mut log_manager = LogManager::new(data_dir);
    
    // Charger les données existantes
    history.load().await?;
    log_manager.load().await?;
    
    // Enregistrer le début de l'opération
    log_manager.log_info("main", "Début de la vérification des mises à jour", None).await?;
    
    let start_time = Instant::now();
    
    // Vérifier les mises à jour
    match pacman.check_updates().await {
        Ok(updates) => {
            let duration = start_time.elapsed();
            
            if updates.is_empty() {
                log_manager.log_info("main", "Aucune mise à jour disponible", None).await?;
                
                if let Some(ref mut notifier) = telegram {
                    notifier.send_message("✅ Système à jour - Aucune mise à jour disponible").await?;
                }
            } else {
                log_manager.log_info(
                    "main", 
                    &format!("{} mises à jour disponibles", updates.len()), 
                    None
                ).await?;
                
                // Envoyer notification Telegram
                if let Some(ref mut notifier) = telegram {
                    notifier.send_updates_available(&updates).await?;
                }
                
                // Si mise à jour automatique activée
                if config.general.auto_update {
                    log_manager.log_info("main", "Début de la mise à jour automatique", None).await?;
                    
                    match pacman.update_packages(&updates).await {
                        Ok(result) => {
                            let total_duration = start_time.elapsed();
                            
                            // Enregistrer dans l'historique
                            let entry = HistoryEntry {
                                id: Uuid::new_v4(),
                                timestamp: Local::now(),
                                operation_type: OperationType::AutoUpdate,
                                packages: updates.clone(),
                                success: result.success,
                                duration: total_duration,
                                message: if result.success {
                                    "Mise à jour automatique réussie".to_string()
                                } else {
                                    format!("Mise à jour échouée: {}", result.error.unwrap_or_default())
                                },
                            };
                            
                            history.add_entry(entry).await?;
                            
                            if result.success {
                                log_manager.log_info("main", "Mise à jour automatique réussie", None).await?;
                                
                                if let Some(ref mut notifier) = telegram {
                                    notifier.send_updates_installed(&updates, &total_duration).await?;
                                }
                            } else {
                                log_manager.log_error(
                                    "main", 
                                    "Échec de la mise à jour automatique", 
                                    result.error
                                ).await?;
                                
                                if let Some(ref mut notifier) = telegram {
                                    notifier.send_update_failed(
                                        &result.error.unwrap_or_default()
                                    ).await?;
                                }
                            }
                        }
                        Err(e) => {
                            log_manager.log_error(
                                "main", 
                                "Erreur lors de la mise à jour", 
                                Some(e.to_string())
                            ).await?;
                        }
                    }
                }
            }
            
            // Enregistrer l'opération de vérification dans l'historique
            let entry = HistoryEntry {
                id: Uuid::new_v4(),
                timestamp: Local::now(),
                operation_type: OperationType::CheckUpdates,
                packages: updates,
                success: true,
                duration,
                message: "Vérification des mises à jour réussie".to_string(),
            };
            
            history.add_entry(entry).await?;
        }
        Err(e) => {
            log_manager.log_error(
                "main", 
                "Erreur lors de la vérification des mises à jour", 
                Some(e.to_string())
            ).await?;
            
            if let Some(ref mut notifier) = telegram {
                notifier.send_update_failed(&e.to_string()).await?;
            }
        }
    }
    
    // Sauvegarder les données
    history.save().await?;
    log_manager.save().await?;
    
    log_manager.log_info("main", "Opération terminée", None).await?;
    
    Ok(())
}
```

### Exemple : Planification Avancée

```rust
use cachypac::{
    config::Config,
    scheduler::SchedulerManager,
    pacman::PacmanManager,
    telegram::TelegramNotifier,
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load().await?;
    let pacman = Arc::new(PacmanManager::new(config.pacman.clone()));
    let telegram = Arc::new(Mutex::new(
        TelegramNotifier::new(
            config.telegram.bot_token.clone(),
            config.telegram.chat_id.clone()
        )
    ));
    
    let mut scheduler = SchedulerManager::new();
    scheduler.start().await?;
    
    // Tâche quotidienne de vérification
    let pacman_clone = Arc::clone(&pacman);
    let telegram_clone = Arc::clone(&telegram);
    
    scheduler.create_job(
        "daily_check".to_string(),
        "0 2 * * *".to_string(), // 2h du matin
        Box::new(move |_| {
            let pacman = Arc::clone(&pacman_clone);
            let telegram = Arc::clone(&telegram_clone);
            
            Box::pin(async move {
                match pacman.check_updates().await {
                    Ok(updates) => {
                        if !updates.is_empty() {
                            let mut notifier = telegram.lock().await;
                            notifier.send_