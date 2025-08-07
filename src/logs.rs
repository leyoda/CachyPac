use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Trace => write!(f, "TRACE"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFilter {
    pub level: Option<LogLevel>,
    pub module: Option<String>,
    pub date_from: Option<DateTime<Local>>,
    pub date_to: Option<DateTime<Local>>,
    pub search_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStats {
    pub total_entries: usize,
    pub error_count: usize,
    pub warn_count: usize,
    pub info_count: usize,
    pub debug_count: usize,
    pub trace_count: usize,
    pub entries_by_module: std::collections::HashMap<String, usize>,
    pub last_error: Option<DateTime<Local>>,
    pub last_warning: Option<DateTime<Local>>,
}

#[derive(Debug)]
pub struct LogManager {
    data_dir: PathBuf,
    entries: Vec<LogEntry>,
    max_entries: usize,
    auto_save: bool,
}

impl LogManager {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            data_dir,
            entries: Vec::new(),
            max_entries: 10000,
            auto_save: true,
        }
    }

    /// Configure la sauvegarde automatique
    #[allow(dead_code)]
    pub fn set_auto_save(&mut self, auto_save: bool) {
        self.auto_save = auto_save;
    }

    /// Charge les logs depuis le fichier
    pub async fn load(&mut self) -> Result<()> {
        let log_file = self.get_log_file_path();
        
        if !log_file.exists() {
            info!("Fichier de logs CachyPac non trouvé, création d'un nouveau fichier");
            return Ok(());
        }

        debug!("Chargement des logs CachyPac depuis: {:?}", log_file);
        
        let content = fs::read_to_string(&log_file)
            .await
            .context("Impossible de lire le fichier de logs CachyPac")?;

        if content.trim().is_empty() {
            debug!("Fichier de logs CachyPac vide");
            return Ok(());
        }

        self.entries = serde_json::from_str(&content)
            .context("Erreur lors du parsing des logs JSON CachyPac")?;

        self.entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        if self.entries.len() > self.max_entries {
            self.entries.truncate(self.max_entries);
        }

        info!("Logs CachyPac chargés: {} entrées", self.entries.len());
        Ok(())
    }

    /// Sauvegarde les logs dans le fichier
    pub async fn save(&self) -> Result<()> {
        let log_file = self.get_log_file_path();
        
        if let Some(parent) = log_file.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Impossible de créer le répertoire de logs CachyPac")?;
        }

        debug!("Sauvegarde des logs CachyPac vers: {:?}", log_file);

        let content = serde_json::to_string_pretty(&self.entries)
            .context("Erreur lors de la sérialisation des logs CachyPac")?;

        fs::write(&log_file, content)
            .await
            .context("Impossible d'écrire le fichier de logs CachyPac")?;

        debug!("Logs CachyPac sauvegardés: {} entrées", self.entries.len());
        Ok(())
    }

    /// Ajoute une nouvelle entrée de log
    pub async fn add_entry(&mut self, entry: LogEntry) -> Result<()> {
        self.entries.insert(0, entry);

        if self.entries.len() > self.max_entries {
            self.entries.truncate(self.max_entries);
        }

        if self.auto_save {
            self.save().await?;
        }

        Ok(())
    }

    /// Ajoute un log d'erreur
    pub async fn log_error(&mut self, module: &str, message: &str, context: Option<String>) -> Result<()> {
        let entry = LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Error,
            module: module.to_string(),
            message: message.to_string(),
            context,
        };
        
        tracing::error!("[CachyPac::{}] {}", module, message);
        self.add_entry(entry).await
    }

    /// Ajoute un log d'avertissement
    #[allow(dead_code)]
    pub async fn log_warn(&mut self, module: &str, message: &str, context: Option<String>) -> Result<()> {
        let entry = LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Warn,
            module: module.to_string(),
            message: message.to_string(),
            context,
        };
        
        warn!("[CachyPac::{}] {}", module, message);
        self.add_entry(entry).await
    }

    /// Ajoute un log d'information
    #[allow(dead_code)]
    pub async fn log_info(&mut self, module: &str, message: &str, context: Option<String>) -> Result<()> {
        let entry = LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Info,
            module: module.to_string(),
            message: message.to_string(),
            context,
        };
        
        info!("[CachyPac::{}] {}", module, message);
        self.add_entry(entry).await
    }

    /// Récupère toutes les entrées
    #[allow(dead_code)]
    pub fn get_all_entries(&self) -> &[LogEntry] {
        &self.entries
    }

    /// Récupère les entrées filtrées
    #[allow(dead_code)]
    pub fn get_filtered_entries(&self, filter: &LogFilter) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|entry| self.matches_filter(entry, filter))
            .collect()
    }

    /// Récupère les entrées récentes (dernières heures)
    pub fn get_recent_entries(&self, hours: i64) -> Vec<&LogEntry> {
        let cutoff = Local::now() - chrono::Duration::hours(hours);
        self.entries
            .iter()
            .filter(|entry| entry.timestamp > cutoff)
            .collect()
    }

    /// Récupère les erreurs récentes
    #[allow(dead_code)]
    pub fn get_recent_errors(&self, hours: i64) -> Vec<&LogEntry> {
        let cutoff = Local::now() - chrono::Duration::hours(hours);
        self.entries
            .iter()
            .filter(|entry| {
                matches!(entry.level, LogLevel::Error) && entry.timestamp > cutoff
            })
            .collect()
    }

    /// Efface tous les logs
    #[allow(dead_code)]
    pub async fn clear_all(&mut self) -> Result<()> {
        warn!("Effacement complet des logs CachyPac");
        self.entries.clear();
        if self.auto_save {
            self.save().await?;
        }
        Ok(())
    }

    /// Génère des statistiques sur les logs
    #[allow(dead_code)]
    pub fn get_statistics(&self) -> LogStats {
        let total_entries = self.entries.len();
        let mut error_count = 0;
        let mut warn_count = 0;
        let mut info_count = 0;
        let mut debug_count = 0;
        let mut trace_count = 0;
        let mut entries_by_module = std::collections::HashMap::new();
        let mut last_error = None;
        let mut last_warning = None;

        for entry in &self.entries {
            match entry.level {
                LogLevel::Error => {
                    error_count += 1;
                    if last_error.is_none() || entry.timestamp > last_error.unwrap() {
                        last_error = Some(entry.timestamp);
                    }
                }
                LogLevel::Warn => {
                    warn_count += 1;
                    if last_warning.is_none() || entry.timestamp > last_warning.unwrap() {
                        last_warning = Some(entry.timestamp);
                    }
                }
                LogLevel::Info => info_count += 1,
                LogLevel::Debug => debug_count += 1,
                LogLevel::Trace => trace_count += 1,
            }

            *entries_by_module.entry(entry.module.clone()).or_insert(0) += 1;
        }

        LogStats {
            total_entries,
            error_count,
            warn_count,
            info_count,
            debug_count,
            trace_count,
            entries_by_module,
            last_error,
            last_warning,
        }
    }

    /// Recherche dans les logs
    #[allow(dead_code)]
    pub fn search(&self, query: &str, case_sensitive: bool) -> Vec<&LogEntry> {
        let query = if case_sensitive {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        self.entries
            .iter()
            .filter(|entry| {
                let message = if case_sensitive {
                    entry.message.clone()
                } else {
                    entry.message.to_lowercase()
                };

                let context = entry.context.as_ref().map(|c| {
                    if case_sensitive {
                        c.clone()
                    } else {
                        c.to_lowercase()
                    }
                });

                message.contains(&query) || 
                context.as_ref().map_or(false, |c| c.contains(&query))
            })
            .collect()
    }

    /// Vérifie si une entrée correspond au filtre
    #[allow(dead_code)]
    fn matches_filter(&self, entry: &LogEntry, filter: &LogFilter) -> bool {
        if let Some(ref level) = filter.level {
            if !self.level_matches(&entry.level, level) {
                return false;
            }
        }

        if let Some(ref module) = filter.module {
            if !entry.module.contains(module) {
                return false;
            }
        }

        if let Some(date_from) = filter.date_from {
            if entry.timestamp < date_from {
                return false;
            }
        }

        if let Some(date_to) = filter.date_to {
            if entry.timestamp > date_to {
                return false;
            }
        }

        if let Some(ref search_text) = filter.search_text {
            let search_lower = search_text.to_lowercase();
            let message_matches = entry.message.to_lowercase().contains(&search_lower);
            let context_matches = entry.context
                .as_ref()
                .map_or(false, |c| c.to_lowercase().contains(&search_lower));
            
            if !message_matches && !context_matches {
                return false;
            }
        }

        true
    }

    /// Vérifie si deux niveaux correspondent
    #[allow(dead_code)]
    fn level_matches(&self, entry_level: &LogLevel, filter_level: &LogLevel) -> bool {
        std::mem::discriminant(entry_level) == std::mem::discriminant(filter_level)
    }

    /// Récupère le chemin du fichier de logs
    fn get_log_file_path(&self) -> PathBuf {
        self.data_dir.join("cachypac_logs.json")
    }
}

impl Clone for LogManager {
    fn clone(&self) -> Self {
        Self {
            data_dir: self.data_dir.clone(),
            entries: self.entries.clone(),
            max_entries: self.max_entries,
            auto_save: self.auto_save,
        }
    }
}

impl Default for LogFilter {
    fn default() -> Self {
        Self {
            level: None,
            module: None,
            date_from: None,
            date_to: None,
            search_text: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_log_operations() {
        let temp_dir = tempdir().unwrap();
        let mut log_manager = LogManager::new(temp_dir.path().to_path_buf());

        log_manager.log_info("cachypac", "Test message", None).await.unwrap();
        log_manager.log_error("cachypac", "Error message", Some("Context info".to_string())).await.unwrap();

        assert_eq!(log_manager.get_all_entries().len(), 2);

        let stats = log_manager.get_statistics();
        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.error_count, 1);
        assert_eq!(stats.info_count, 1);
    }

    #[test]
    fn test_log_search() {
        let temp_dir = tempdir().unwrap();
        let mut log_manager = LogManager::new(temp_dir.path().to_path_buf());

        let entry1 = LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Info,
            module: "cachypac".to_string(),
            message: "This is a CachyPac test message".to_string(),
            context: None,
        };

        let entry2 = LogEntry {
            timestamp: Local::now(),
            level: LogLevel::Error,
            module: "cachypac".to_string(),
            message: "Another message".to_string(),
            context: Some("CachyPac context".to_string()),
        };

        log_manager.entries.push(entry1);
        log_manager.entries.push(entry2);

        let results = log_manager.search("CachyPac", false);
        assert_eq!(results.len(), 2);

        let results = log_manager.search("context", false);
        assert_eq!(results.len(), 1);
    }
}