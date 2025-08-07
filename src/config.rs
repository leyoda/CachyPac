use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tokio::fs;
use tracing::{debug, info, warn};

/// Cache pour la configuration avec TTL
#[derive(Debug)]
struct ConfigCache {
    config: Config,
    loaded_at: Instant,
    ttl: Duration,
}

impl ConfigCache {
    fn new(config: Config, ttl: Duration) -> Self {
        Self {
            config,
            loaded_at: Instant::now(),
            ttl,
        }
    }

    fn is_expired(&self) -> bool {
        self.loaded_at.elapsed() > self.ttl
    }
}

/// Cache global pour les configurations
static CONFIG_CACHE: std::sync::LazyLock<Arc<RwLock<Option<ConfigCache>>>> =
    std::sync::LazyLock::new(|| Arc::new(RwLock::new(None)));

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacmanConfig {
    pub timeout: u64,
    pub retry_count: u32,
    pub retry_delay: u64,
    pub exclude_packages: Vec<String>,
    pub include_aur: bool,
    pub clean_cache_after: bool,
    pub check_keyring: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    pub enabled: bool,
    pub cron_expression: String,
    pub maintenance_window_start: String,
    pub maintenance_window_end: String,
    pub max_concurrent_jobs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub enabled: bool,
    pub bot_token: String,
    pub chat_id: String,
    pub notify_on_updates: bool,
    pub notify_on_errors: bool,
    pub notify_on_success: bool,
    pub message_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiConfig {
    pub theme: String,
    pub window_width: u32,
    pub window_height: u32,
    pub auto_start_minimized: bool,
    pub show_system_tray: bool,
    pub close_to_tray: bool,
}

impl Config {
    /// Charge la configuration depuis le fichier par défaut avec cache
    pub async fn load() -> Result<Self> {
        Self::load_with_cache(Duration::from_secs(300)).await // Cache 5 minutes
    }

    /// Charge la configuration avec cache personnalisé
    pub async fn load_with_cache(cache_ttl: Duration) -> Result<Self> {
        // Vérifier le cache d'abord
        {
            let cache = CONFIG_CACHE.read().unwrap();
            if let Some(cached) = cache.as_ref() {
                if !cached.is_expired() {
                    debug!("🚀 Configuration chargée depuis le cache");
                    return Ok(cached.config.clone());
                }
            }
        }

        // Charger depuis le disque
        let config = Self::load_from_disk().await?;
        
        // Mettre à jour le cache
        {
            let mut cache = CONFIG_CACHE.write().unwrap();
            *cache = Some(ConfigCache::new(config.clone(), cache_ttl));
            debug!("💾 Configuration mise en cache pour {:?}", cache_ttl);
        }

        Ok(config)
    }

    /// Charge la configuration depuis le disque (sans cache)
    async fn load_from_disk() -> Result<Self> {
        let config_paths = vec![
            PathBuf::from("config.toml"),
            PathBuf::from("config/config.toml"),
            PathBuf::from("/etc/cachypac/config.toml"),
            dirs::config_dir().map(|d| d.join("cachypac/config.toml")).unwrap_or_else(|| PathBuf::from("~/.config/cachypac/config.toml")),
        ];

        for path in config_paths {
            if path.exists() {
                info!("📁 Chargement de la configuration depuis: {:?}", path);
                return Self::load_from_file(&path).await;
            }
        }

        warn!("⚠️ Aucun fichier de configuration trouvé, utilisation des valeurs par défaut");
        Ok(Self::default())
    }

    /// Charge la configuration depuis un fichier spécifique
    pub async fn load_from_file(path: &PathBuf) -> Result<Self> {
        debug!("📖 Lecture du fichier de configuration: {:?}", path);
        
        let content = fs::read_to_string(path)
            .await
            .context(format!("Impossible de lire le fichier de configuration: {:?}", path))?;

        let config: Config = toml::from_str(&content)
            .context("Erreur lors du parsing de la configuration TOML")?;

        info!("✅ Configuration chargée avec succès");
        Ok(config)
    }

    /// Sauvegarde la configuration dans un fichier
    pub async fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        debug!("💾 Sauvegarde de la configuration vers: {:?}", path);
        
        // Créer le répertoire parent si nécessaire
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await
                    .context(format!("Impossible de créer le répertoire: {:?}", parent))?;
            }
        }
        
        let content = toml::to_string_pretty(self)
            .context("Erreur lors de la sérialisation TOML")?;
        
        fs::write(path, content).await
            .context(format!("Impossible d'écrire le fichier de configuration: {:?}", path))?;
        
        // Invalider le cache après sauvegarde
        Self::invalidate_cache();
        
        info!("✅ Configuration sauvegardée avec succès vers: {:?}", path);
        Ok(())
    }

    /// Sauvegarde la configuration dans le fichier par défaut
    pub async fn save(&self) -> Result<()> {
        let config_path = PathBuf::from("config.toml");
        self.save_to_file(&config_path).await
    }

    /// Invalide le cache de configuration
    #[allow(dead_code)]
    pub fn invalidate_cache() {
        let mut cache = CONFIG_CACHE.write().unwrap();
        *cache = None;
        debug!("🗑️ Cache de configuration invalidé");
    }

    /// Vérifie si la configuration est en cache
    #[allow(dead_code)]
    pub fn is_cached() -> bool {
        let cache = CONFIG_CACHE.read().unwrap();
        cache.as_ref().map_or(false, |c| !c.is_expired())
    }

    /// Valide la configuration
    pub fn validate(&self) -> Result<()> {
        // Validation des paramètres généraux
        if self.general.data_dir.is_empty() {
            return Err(anyhow::anyhow!("Le répertoire de données ne peut pas être vide"));
        }
        
        if self.general.check_interval == 0 {
            return Err(anyhow::anyhow!("L'intervalle de vérification doit être supérieur à 0"));
        }

        // Validation du répertoire de données
        let data_dir = PathBuf::from(&self.general.data_dir);
        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir)
                .context(format!("Impossible de créer le répertoire de données: {:?}", data_dir))?;
            debug!("📁 Répertoire de données créé: {:?}", data_dir);
        }

        // Validation Telegram
        if self.telegram.enabled {
            if self.telegram.bot_token.is_empty() {
                return Err(anyhow::anyhow!("Token Telegram manquant"));
            }
            if self.telegram.chat_id.is_empty() {
                return Err(anyhow::anyhow!("Chat ID Telegram manquant"));
            }
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("cachypac");

        Self {
            general: GeneralConfig {
                data_dir: data_dir.to_string_lossy().to_string(),
                log_level: "info".to_string(),
                check_interval: 60,
                auto_update: false,
                backup_before_update: true,
            },
            pacman: PacmanConfig {
                timeout: 300,
                retry_count: 3,
                retry_delay: 5,
                exclude_packages: vec![],
                include_aur: false,
                clean_cache_after: true,
                check_keyring: true,
            },
            scheduler: SchedulerConfig {
                enabled: false,
                cron_expression: "0 2 * * *".to_string(),
                maintenance_window_start: "02:00".to_string(),
                maintenance_window_end: "06:00".to_string(),
                max_concurrent_jobs: 1,
            },
            telegram: TelegramConfig {
                enabled: false,
                bot_token: String::new(),
                chat_id: String::new(),
                notify_on_updates: true,
                notify_on_errors: true,
                notify_on_success: true,
                message_format: "html".to_string(),
            },
            gui: GuiConfig {
                theme: "dark".to_string(),
                window_width: 1000,
                window_height: 700,
                auto_start_minimized: false,
                show_system_tray: true,
                close_to_tray: true,
            },
        }
    }
}