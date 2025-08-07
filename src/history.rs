use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Local>,
    pub operation_type: OperationType,
    pub packages: Vec<String>,
    pub success: bool,
    pub message: String,
    pub duration: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    ManualUpdate,
    AutoUpdate,
    CheckUpdates,
    CleanCache,
    DatabaseUpdate,
    SystemMaintenance,
    PackageInstall,
    PackageRemove,
}

impl std::fmt::Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::ManualUpdate => write!(f, "Mise à jour manuelle"),
            OperationType::AutoUpdate => write!(f, "Mise à jour automatique"),
            OperationType::CheckUpdates => write!(f, "Vérification des mises à jour"),
            OperationType::CleanCache => write!(f, "Nettoyage du cache"),
            OperationType::DatabaseUpdate => write!(f, "Mise à jour de la base de données"),
            OperationType::SystemMaintenance => write!(f, "Maintenance système"),
            OperationType::PackageInstall => write!(f, "Installation de paquet"),
            OperationType::PackageRemove => write!(f, "Suppression de paquet"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryFilter {
    pub operation_type: Option<OperationType>,
    pub success_only: Option<bool>,
    pub date_from: Option<DateTime<Local>>,
    pub date_to: Option<DateTime<Local>>,
    pub package_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryStats {
    pub total_operations: usize,
    pub successful_operations: usize,
    pub failed_operations: usize,
    pub total_packages_updated: usize,
    pub last_successful_update: Option<DateTime<Local>>,
    pub last_failed_update: Option<DateTime<Local>>,
    pub average_duration: std::time::Duration,
    pub operations_by_type: std::collections::HashMap<String, usize>,
}

#[derive(Debug)]
pub struct UpdateHistory {
    data_dir: PathBuf,
    entries: Vec<HistoryEntry>,
    max_entries: usize,
}

impl UpdateHistory {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            data_dir,
            entries: Vec::new(),
            max_entries: 1000,
        }
    }

    /// Charge l'historique depuis le fichier
    pub async fn load(&mut self) -> Result<()> {
        let history_file = self.get_history_file_path();
        
        if !history_file.exists() {
            info!("Fichier d'historique CachyPac non trouvé, création d'un nouvel historique");
            return Ok(());
        }

        debug!("Chargement de l'historique CachyPac depuis: {:?}", history_file);
        
        let content = fs::read_to_string(&history_file)
            .await
            .context("Impossible de lire le fichier d'historique CachyPac")?;

        if content.trim().is_empty() {
            debug!("Fichier d'historique CachyPac vide");
            return Ok(());
        }

        self.entries = serde_json::from_str(&content)
            .context("Erreur lors du parsing de l'historique JSON CachyPac")?;

        self.entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        if self.entries.len() > self.max_entries {
            self.entries.truncate(self.max_entries);
        }

        info!("Historique CachyPac chargé: {} entrées", self.entries.len());
        Ok(())
    }

    /// Sauvegarde l'historique dans le fichier
    pub async fn save(&self) -> Result<()> {
        let history_file = self.get_history_file_path();
        
        if let Some(parent) = history_file.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Impossible de créer le répertoire d'historique CachyPac")?;
        }

        debug!("Sauvegarde de l'historique CachyPac vers: {:?}", history_file);

        let content = serde_json::to_string_pretty(&self.entries)
            .context("Erreur lors de la sérialisation de l'historique CachyPac")?;

        fs::write(&history_file, content)
            .await
            .context("Impossible d'écrire le fichier d'historique CachyPac")?;

        debug!("Historique CachyPac sauvegardé: {} entrées", self.entries.len());
        Ok(())
    }

    /// Ajoute une nouvelle entrée à l'historique
    pub async fn add_entry(&mut self, entry: HistoryEntry) -> Result<()> {
        debug!("Ajout d'une entrée à l'historique CachyPac: {:?}", entry.operation_type);

        self.entries.insert(0, entry);

        if self.entries.len() > self.max_entries {
            self.entries.truncate(self.max_entries);
        }

        self.save().await?;
        Ok(())
    }

    /// Récupère toutes les entrées
    pub fn get_all_entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    /// Récupère les entrées filtrées
    #[allow(dead_code)]
    pub fn get_filtered_entries(&self, filter: &HistoryFilter) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|entry| self.matches_filter(entry, filter))
            .collect()
    }

    /// Récupère les entrées récentes (dernières 24h)
    #[allow(dead_code)]
    pub fn get_recent_entries(&self, hours: i64) -> Vec<&HistoryEntry> {
        let cutoff = Local::now() - chrono::Duration::hours(hours);
        self.entries
            .iter()
            .filter(|entry| entry.timestamp > cutoff)
            .collect()
    }

    /// Récupère les entrées d'échec récentes
    #[allow(dead_code)]
    pub fn get_recent_failures(&self, hours: i64) -> Vec<&HistoryEntry> {
        let cutoff = Local::now() - chrono::Duration::hours(hours);
        self.entries
            .iter()
            .filter(|entry| !entry.success && entry.timestamp > cutoff)
            .collect()
    }

    /// Génère des statistiques sur l'historique
    #[allow(dead_code)]
    pub fn get_statistics(&self) -> HistoryStats {
        let total_operations = self.entries.len();
        let successful_operations = self.entries.iter().filter(|e| e.success).count();
        let failed_operations = total_operations - successful_operations;

        let total_packages_updated = self.entries
            .iter()
            .filter(|e| e.success && matches!(e.operation_type, OperationType::ManualUpdate | OperationType::AutoUpdate))
            .map(|e| e.packages.len())
            .sum();

        let last_successful_update = self.entries
            .iter()
            .filter(|e| e.success && matches!(e.operation_type, OperationType::ManualUpdate | OperationType::AutoUpdate))
            .map(|e| e.timestamp)
            .max();

        let last_failed_update = self.entries
            .iter()
            .filter(|e| !e.success && matches!(e.operation_type, OperationType::ManualUpdate | OperationType::AutoUpdate))
            .map(|e| e.timestamp)
            .max();

        let average_duration = if total_operations > 0 {
            let total_duration: std::time::Duration = self.entries
                .iter()
                .map(|e| e.duration)
                .sum();
            total_duration / total_operations as u32
        } else {
            std::time::Duration::from_secs(0)
        };

        let mut operations_by_type = std::collections::HashMap::new();
        for entry in &self.entries {
            let type_name = format!("{:?}", entry.operation_type);
            *operations_by_type.entry(type_name).or_insert(0) += 1;
        }

        HistoryStats {
            total_operations,
            successful_operations,
            failed_operations,
            total_packages_updated,
            last_successful_update,
            last_failed_update,
            average_duration,
            operations_by_type,
        }
    }

    /// Efface tout l'historique
    #[allow(dead_code)]
    pub async fn clear_all(&mut self) -> Result<()> {
        warn!("Effacement complet de l'historique CachyPac");
        self.entries.clear();
        self.save().await?;
        Ok(())
    }

    /// Vérifie si une entrée correspond au filtre
    #[allow(dead_code)]
    fn matches_filter(&self, entry: &HistoryEntry, filter: &HistoryFilter) -> bool {
        if let Some(ref op_type) = filter.operation_type {
            if std::mem::discriminant(&entry.operation_type) != std::mem::discriminant(op_type) {
                return false;
            }
        }

        if let Some(success_only) = filter.success_only {
            if entry.success != success_only {
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

        if let Some(ref package_name) = filter.package_name {
            if !entry.packages.iter().any(|pkg| pkg.contains(package_name)) {
                return false;
            }
        }

        true
    }

    /// Récupère le chemin du fichier d'historique
    fn get_history_file_path(&self) -> PathBuf {
        self.data_dir.join("cachypac_history.json")
    }
}

impl Clone for UpdateHistory {
    fn clone(&self) -> Self {
        Self {
            data_dir: self.data_dir.clone(),
            entries: self.entries.clone(),
            max_entries: self.max_entries,
        }
    }
}

impl Default for HistoryFilter {
    fn default() -> Self {
        Self {
            operation_type: None,
            success_only: None,
            date_from: None,
            date_to: None,
            package_name: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_history_operations() {
        let temp_dir = tempdir().unwrap();
        let mut history = UpdateHistory::new(temp_dir.path().to_path_buf());

        let entry = HistoryEntry {
            id: Uuid::new_v4(),
            timestamp: Local::now(),
            operation_type: OperationType::ManualUpdate,
            packages: vec!["firefox".to_string(), "chromium".to_string()],
            success: true,
            message: "Test CachyPac update".to_string(),
            duration: std::time::Duration::from_secs(30),
        };

        history.add_entry(entry.clone()).await.unwrap();
        assert_eq!(history.get_all_entries().len(), 1);

        let stats = history.get_statistics();
        assert_eq!(stats.total_operations, 1);
        assert_eq!(stats.successful_operations, 1);
        assert_eq!(stats.total_packages_updated, 2);
    }
}