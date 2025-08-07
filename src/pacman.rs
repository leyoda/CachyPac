use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;
use tracing::{debug, info, warn};

use crate::config::PacmanConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageUpdate {
    pub name: String,
    pub current_version: String,
    pub new_version: String,
    pub repository: String,
    pub size: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PacmanManager {
    config: PacmanConfig,
}

impl PacmanManager {
    pub fn new(config: PacmanConfig) -> Self {
        Self { config }
    }

    /// Vérifie les mises à jour disponibles
    pub async fn check_updates(&self) -> Result<Vec<String>> {
        info!("🔍 Vérification des mises à jour disponibles avec CachyPac");
        
        // Vérifier si checkupdates est disponible
        let check_cmd = Command::new("which")
            .arg("checkupdates")
            .output()
            .await;
            
        if check_cmd.is_err() || !check_cmd.unwrap().status.success() {
            warn!("⚠️ DIAGNOSTIC: La commande 'checkupdates' n'est pas disponible!");
            warn!("⚠️ DIAGNOSTIC: Installez 'pacman-contrib' avec: sudo pacman -S pacman-contrib");
            return Err(anyhow::anyhow!("checkupdates non trouvé - installez pacman-contrib"));
        }
        
        debug!("✅ DIAGNOSTIC: checkupdates trouvé, continuation...");

        let mut cmd = Command::new("checkupdates");
        
        if self.config.include_aur {
            cmd.arg("--aur");
        }

        let output = timeout(
            Duration::from_secs(self.config.timeout),
            cmd.output()
        )
        .await
        .context("Timeout lors de la vérification des mises à jour")?
        .context("Impossible d'exécuter checkupdates")?;

        if !output.status.success() && output.status.code() != Some(2) {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Erreur checkupdates: {}", error));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let updates = self.parse_updates(&stdout)?;

        info!("📦 {} mises à jour trouvées", updates.len());
        Ok(updates)
    }

    /// Parse la sortie de checkupdates
    fn parse_updates(&self, output: &str) -> Result<Vec<String>> {
        let mut updates = Vec::new();

        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }

            // Format simple: nom_paquet version_actuelle -> nouvelle_version
            if let Some(package_name) = line.split_whitespace().next() {
                // Vérifier si le paquet est exclu
                if self.config.exclude_packages.contains(&package_name.to_string()) {
                    debug!("⏭️ Paquet exclu: {}", package_name);
                    continue;
                }

                updates.push(package_name.to_string());
            } else {
                warn!("⚠️ Ligne non parsée: {}", line);
            }
        }

        Ok(updates)
    }

    /// Installe les mises à jour
    pub async fn install_updates(&self, packages: Vec<String>) -> Result<()> {
        if packages.is_empty() {
            info!("✅ Aucune mise à jour à installer");
            return Ok(());
        }

        info!("🔧 Installation de {} mises à jour avec CachyPac", packages.len());

        let mut retry_count = 0;
        loop {
            match self.try_install_updates(&packages).await {
                Ok(_) => {
                    info!("✅ Mises à jour installées avec succès");
                    
                    // Nettoyer le cache si configuré
                    if self.config.clean_cache_after {
                        self.clean_cache().await?;
                    }
                    
                    return Ok(());
                }
                Err(e) => {
                    retry_count += 1;
                    if retry_count >= self.config.retry_count {
                        return Err(e);
                    }
                    
                    warn!("⚠️ Tentative {}/{} échouée: {}", retry_count, self.config.retry_count, e);
                    tokio::time::sleep(Duration::from_secs(self.config.retry_delay)).await;
                }
            }
        }
    }

    /// Tente d'installer les mises à jour
    async fn try_install_updates(&self, packages: &[String]) -> Result<()> {
        let mut cmd = Command::new("sudo");
        cmd.args(&["pacman", "-Su", "--noconfirm"]);
        
        // Ajouter les paquets spécifiques
        for package in packages {
            cmd.arg(package);
        }

        let output = timeout(
            Duration::from_secs(self.config.timeout * 2),
            cmd.output()
        )
        .await
        .context("Timeout lors de l'installation")?
        .context("Impossible d'exécuter pacman")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Erreur pacman: {}", error));
        }

        Ok(())
    }

    /// Nettoie le cache Pacman
    async fn clean_cache(&self) -> Result<()> {
        info!("🧹 Nettoyage du cache Pacman");

        let output = Command::new("sudo")
            .args(&["pacman", "-Sc", "--noconfirm"])
            .output()
            .await
            .context("Impossible de nettoyer le cache")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            warn!("⚠️ Avertissement nettoyage: {}", error);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_updates() {
        let config = PacmanConfig {
            timeout: 300,
            retry_count: 3,
            retry_delay: 5,
            exclude_packages: vec!["excluded-package".to_string()],
            include_aur: false,
            clean_cache_after: true,
            check_keyring: true,
        };

        let manager = PacmanManager::new(config);
        
        let output = "firefox 91.0-1 -> 92.0-1\nchromium 93.0-1 -> 94.0-1\nexcluded-package 1.0-1 -> 2.0-1";
        let updates = manager.parse_updates(output).unwrap();
        
        assert_eq!(updates.len(), 2);
        assert!(updates.contains(&"firefox".to_string()));
        assert!(updates.contains(&"chromium".to_string()));
        assert!(!updates.contains(&"excluded-package".to_string()));
    }
}