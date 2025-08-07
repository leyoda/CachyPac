use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::process::Command;
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub status: ServiceStatus,
    pub enabled: bool,
    pub pid: Option<u32>,
    pub memory_usage: Option<u64>,
    pub cpu_usage: Option<f32>,
    pub uptime: Option<std::time::Duration>,
    pub last_restart: Option<chrono::DateTime<chrono::Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemdUnit {
    pub unit_file: String,
    pub description: String,
    pub exec_start: String,
    pub user: String,
    pub working_directory: String,
    pub environment: Vec<String>,
    pub restart: String,
    pub restart_sec: u32,
}

#[derive(Debug)]
pub struct ServiceManager {
    service_name: String,
    systemd_unit: SystemdUnit,
    config_dir: PathBuf,
}

impl ServiceManager {
    pub fn new(service_name: String, config_dir: PathBuf) -> Self {
        let systemd_unit = SystemdUnit {
            unit_file: format!("{}.service", service_name),
            description: "CachyPac - Automated Pacman Update Manager".to_string(),
            exec_start: "/usr/local/bin/cachypac --daemon".to_string(),
            user: "cachypac".to_string(),
            working_directory: "/opt/cachypac".to_string(),
            environment: vec![
                "RUST_LOG=info".to_string(),
                "CACHYPAC_CONFIG=/etc/cachypac/config.toml".to_string(),
            ],
            restart: "always".to_string(),
            restart_sec: 10,
        };

        Self {
            service_name,
            systemd_unit,
            config_dir,
        }
    }

    /// Installe le service systemd
    pub async fn install_service(&self) -> Result<()> {
        info!("Installation du service systemd: {}", self.service_name);

        let unit_content = self.generate_systemd_unit();
        let unit_path = PathBuf::from("/etc/systemd/system").join(&self.systemd_unit.unit_file);

        if !self.is_running_as_root().await? {
            return Err(anyhow::anyhow!(
                "L'installation du service nécessite les privilèges root"
            ));
        }

        fs::write(&unit_path, unit_content)
            .await
            .context("Impossible d'écrire le fichier unit systemd")?;

        info!("Fichier unit créé: {:?}", unit_path);

        self.systemctl_daemon_reload().await?;
        self.create_system_user().await?;
        self.create_service_directories().await?;
        self.install_config_files().await?;

        info!("Service systemd installé avec succès");
        Ok(())
    }

    /// Récupère le statut du service
    pub async fn get_service_status(&self) -> Result<ServiceInfo> {
        debug!("Récupération du statut du service: {}", self.service_name);

        let output = Command::new("systemctl")
            .args(&["status", &self.service_name, "--no-pager"])
            .output()
            .await
            .context("Impossible d'exécuter systemctl status")?;

        let status_output = String::from_utf8_lossy(&output.stdout);
        let status = self.parse_service_status(&status_output)?;

        let enabled = self.is_service_enabled().await.unwrap_or(false);

        Ok(ServiceInfo {
            name: self.service_name.clone(),
            status,
            enabled,
            pid: None,
            memory_usage: None,
            cpu_usage: None,
            uptime: None,
            last_restart: None,
        })
    }

    /// Génère le contenu du fichier unit systemd
    fn generate_systemd_unit(&self) -> String {
        let mut unit_content = String::new();
        
        unit_content.push_str("[Unit]\n");
        unit_content.push_str(&format!("Description={}\n", self.systemd_unit.description));
        unit_content.push_str("After=network.target\n");
        unit_content.push_str("Wants=network.target\n\n");

        unit_content.push_str("[Service]\n");
        unit_content.push_str("Type=simple\n");
        unit_content.push_str(&format!("User={}\n", self.systemd_unit.user));
        unit_content.push_str(&format!("WorkingDirectory={}\n", self.systemd_unit.working_directory));
        unit_content.push_str(&format!("ExecStart={}\n", self.systemd_unit.exec_start));
        unit_content.push_str(&format!("Restart={}\n", self.systemd_unit.restart));
        unit_content.push_str(&format!("RestartSec={}\n", self.systemd_unit.restart_sec));

        for env in &self.systemd_unit.environment {
            unit_content.push_str(&format!("Environment={}\n", env));
        }

        unit_content.push_str("StandardOutput=journal\n");
        unit_content.push_str("StandardError=journal\n");
        unit_content.push_str("SyslogIdentifier=cachypac\n\n");

        unit_content.push_str("[Install]\n");
        unit_content.push_str("WantedBy=multi-user.target\n");

        unit_content
    }

    /// Parse le statut du service depuis la sortie systemctl
    fn parse_service_status(&self, status_output: &str) -> Result<ServiceStatus> {
        if status_output.contains("Active: active (running)") {
            Ok(ServiceStatus::Running)
        } else if status_output.contains("Active: inactive (dead)") {
            Ok(ServiceStatus::Stopped)
        } else if status_output.contains("Active: failed") {
            Ok(ServiceStatus::Failed)
        } else {
            Ok(ServiceStatus::Unknown)
        }
    }

    /// Vérifie si le service est activé
    pub async fn is_service_enabled(&self) -> Result<bool> {
        let output = Command::new("systemctl")
            .args(&["is-enabled", &self.service_name])
            .output()
            .await
            .context("Impossible d'exécuter systemctl is-enabled")?;

        Ok(output.status.success())
    }

    /// Vérifie si le processus s'exécute en tant que root
    async fn is_running_as_root(&self) -> Result<bool> {
        let output = Command::new("id")
            .args(&["-u"])
            .output()
            .await
            .context("Impossible d'exécuter id -u")?;

        let uid_str = String::from_utf8_lossy(&output.stdout);
        let uid = uid_str.trim().parse::<u32>().unwrap_or(1000);
        Ok(uid == 0)
    }

    /// Recharge la configuration systemd
    async fn systemctl_daemon_reload(&self) -> Result<()> {
        let output = Command::new("systemctl")
            .args(&["daemon-reload"])
            .output()
            .await
            .context("Impossible d'exécuter systemctl daemon-reload")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Échec du rechargement systemd: {}", error));
        }

        Ok(())
    }

    /// Crée l'utilisateur système
    async fn create_system_user(&self) -> Result<()> {
        info!("Création de l'utilisateur système: {}", self.systemd_unit.user);

        let output = Command::new("id")
            .args(&[&self.systemd_unit.user])
            .output()
            .await;

        if output.is_ok() && output.unwrap().status.success() {
            debug!("L'utilisateur {} existe déjà", self.systemd_unit.user);
            return Ok(());
        }

        let output = Command::new("useradd")
            .args(&[
                "--system",
                "--no-create-home",
                "--shell", "/bin/false",
                &self.systemd_unit.user,
            ])
            .output()
            .await
            .context("Impossible d'exécuter useradd")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Échec de la création de l'utilisateur: {}", error));
        }

        info!("Utilisateur système créé: {}", self.systemd_unit.user);
        Ok(())
    }

    /// Crée les répertoires nécessaires pour le service
    async fn create_service_directories(&self) -> Result<()> {
        let directories = vec![
            PathBuf::from(&self.systemd_unit.working_directory),
            PathBuf::from("/etc/cachypac"),
            PathBuf::from("/var/log/cachypac"),
            PathBuf::from("/var/lib/cachypac"),
        ];

        for dir in directories {
            if !dir.exists() {
                fs::create_dir_all(&dir)
                    .await
                    .context(format!("Impossible de créer le répertoire: {:?}", dir))?;
                info!("Répertoire créé: {:?}", dir);
            }
        }

        Ok(())
    }

    /// Installe les fichiers de configuration
    async fn install_config_files(&self) -> Result<()> {
        let default_config = self.config_dir.join("config.toml");
        let system_config = PathBuf::from("/etc/cachypac/config.toml");

        if default_config.exists() && !system_config.exists() {
            fs::copy(&default_config, &system_config)
                .await
                .context("Impossible de copier le fichier de configuration")?;
            info!("Fichier de configuration installé: {:?}", system_config);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_systemd_unit_generation() {
        let temp_dir = tempdir().unwrap();
        let service_manager = ServiceManager::new("test-service".to_string(), temp_dir.path().to_path_buf());
        
        let unit_content = service_manager.generate_systemd_unit();
        
        assert!(unit_content.contains("[Unit]"));
        assert!(unit_content.contains("[Service]"));
        assert!(unit_content.contains("[Install]"));
        assert!(unit_content.contains("Description=CachyPac"));
        assert!(unit_content.contains("cachypac"));
    }

    #[test]
    fn test_service_status_parsing() {
        let temp_dir = tempdir().unwrap();
        let service_manager = ServiceManager::new("test-service".to_string(), temp_dir.path().to_path_buf());
        
        let running_output = "● test-service.service - Test Service\n   Active: active (running)";
        let status = service_manager.parse_service_status(running_output).unwrap();
        assert!(matches!(status, ServiceStatus::Running));

        let stopped_output = "● test-service.service - Test Service\n   Active: inactive (dead)";
        let status = service_manager.parse_service_status(stopped_output).unwrap();
        assert!(matches!(status, ServiceStatus::Stopped));
    }
}