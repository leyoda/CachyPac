//! Exemple d'intelligence simple pour CachyPac
//! 
//! Cet exemple démontre comment utiliser les modules de CachyPac
//! pour créer un système intelligent de gestion des mises à jour.

use anyhow::Result;
use cachypac::config::Config;
use cachypac::pacman::PacmanManager;
use cachypac::scheduler::SchedulerManager;
use cachypac::telegram::TelegramNotifier;
use cachypac::history::{UpdateHistory, HistoryEntry, OperationType};
use cachypac::logs::LogManager;
use chrono::{Local, Timelike};
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn, error};
use uuid::Uuid;

/// Structure principale pour l'intelligence simple
pub struct SimpleIntelligence {
    config: Config,
    pacman: PacmanManager,
    scheduler: SchedulerManager,
    telegram: Option<TelegramNotifier>,
    history: UpdateHistory,
    logs: LogManager,
}

impl SimpleIntelligence {
    /// Crée une nouvelle instance d'intelligence simple
    pub async fn new(config_path: Option<PathBuf>) -> Result<Self> {
        // Charger la configuration
        let config = if let Some(path) = config_path {
            Config::load_from_file(&path).await?
        } else {
            Config::load().await?
        };

        // Initialiser les composants
        let pacman = PacmanManager::new(config.pacman.clone());
        let scheduler = SchedulerManager::new();
        
        let telegram = if config.telegram.enabled {
            Some(TelegramNotifier::new(
                config.telegram.bot_token.clone(),
                config.telegram.chat_id.clone(),
            ))
        } else {
            None
        };

        let data_dir = PathBuf::from(&config.general.data_dir);
        let mut history = UpdateHistory::new(data_dir.clone());
        history.load().await?;

        let mut logs = LogManager::new(data_dir);
        logs.load().await?;

        Ok(Self {
            config,
            pacman,
            scheduler,
            telegram,
            history,
            logs,
        })
    }

    /// Lance l'intelligence simple
    pub async fn run(&mut self) -> Result<()> {
        info!("🚀 Démarrage de l'intelligence simple CachyPac");
        
        self.logs.log_info("simple_intelligence", "Démarrage de l'intelligence simple CachyPac", None).await?;

        // Boucle principale d'intelligence
        loop {
            match self.intelligence_cycle().await {
                Ok(_) => {
                    info!("✅ Cycle d'intelligence CachyPac terminé avec succès");
                }
                Err(e) => {
                    error!("❌ Erreur dans le cycle d'intelligence CachyPac: {}", e);
                    self.logs.log_error("simple_intelligence", &format!("Erreur dans le cycle: {}", e), None).await?;
                }
            }

            // Attendre avant le prochain cycle
            let interval = Duration::from_secs(self.config.general.check_interval * 60);
            info!("⏰ Attente de {} minutes avant le prochain cycle CachyPac", self.config.general.check_interval);
            sleep(interval).await;
        }
    }

    /// Exécute un cycle d'intelligence
    async fn intelligence_cycle(&mut self) -> Result<()> {
        info!("🔄 Début du cycle d'intelligence CachyPac");

        // 1. Vérifier les mises à jour disponibles
        let start_time = std::time::Instant::now();
        let updates = self.pacman.check_updates().await?;
        let check_duration = start_time.elapsed();

        self.logs.log_info(
            "simple_intelligence", 
            &format!("Vérification CachyPac des mises à jour terminée: {} paquets trouvés", updates.len()),
            Some(format!("Durée: {:?}", check_duration))
        ).await?;

        if updates.is_empty() {
            info!("✅ Aucune mise à jour disponible");
            return Ok(());
        }

        info!("📦 {} mises à jour disponibles", updates.len());

        // 2. Analyser les mises à jour avec intelligence
        let analysis = self.analyze_updates(&updates).await?;
        
        // 3. Prendre une décision intelligente
        let decision = self.make_intelligent_decision(&analysis).await?;

        // 4. Exécuter la décision
        match decision {
            IntelligentDecision::InstallAll => {
                self.install_all_updates(&updates).await?;
            }
            IntelligentDecision::InstallSafe(safe_packages) => {
                self.install_safe_updates(&safe_packages).await?;
            }
            IntelligentDecision::Postpone(reason) => {
                self.postpone_updates(&updates, &reason).await?;
            }
            IntelligentDecision::NotifyOnly => {
                self.notify_updates_available(&updates).await?;
            }
        }

        info!("✅ Cycle d'intelligence CachyPac terminé");
        Ok(())
    }

    /// Analyse les mises à jour avec intelligence
    async fn analyze_updates(&mut self, updates: &[String]) -> Result<UpdateAnalysis> {
        info!("🧠 Analyse intelligente CachyPac des mises à jour");

        let mut critical_packages = Vec::new();
        let mut safe_packages = Vec::new();
        let mut risky_packages = Vec::new();

        // Listes de paquets critiques et risqués (exemple simple)
        let critical_keywords = ["kernel", "systemd", "glibc", "gcc", "mesa"];
        let risky_keywords = ["nvidia", "xorg", "wayland", "pulseaudio", "pipewire"];

        for package in updates {
            let package_lower = package.to_lowercase();
            
            if critical_keywords.iter().any(|&keyword| package_lower.contains(keyword)) {
                critical_packages.push(package.clone());
            } else if risky_keywords.iter().any(|&keyword| package_lower.contains(keyword)) {
                risky_packages.push(package.clone());
            } else {
                safe_packages.push(package.clone());
            }
        }

        // Vérifier l'historique des échecs récents
        let recent_failures = self.history.get_recent_failures(24);
        let has_recent_failures = !recent_failures.is_empty();

        // Vérifier si c'est dans une fenêtre de maintenance
        let in_maintenance_window = self.is_in_maintenance_window().await?;

        let analysis = UpdateAnalysis {
            total_updates: updates.len(),
            critical_packages,
            safe_packages,
            risky_packages,
            has_recent_failures,
            in_maintenance_window,
            system_load: self.get_system_load().await.unwrap_or(0.0),
        };

        self.logs.log_info(
            "simple_intelligence",
            "Analyse CachyPac des mises à jour terminée",
            Some(format!("Critique: {}, Sûr: {}, Risqué: {}", 
                analysis.critical_packages.len(),
                analysis.safe_packages.len(), 
                analysis.risky_packages.len()))
        ).await?;

        Ok(analysis)
    }

    /// Prend une décision intelligente basée sur l'analyse
    async fn make_intelligent_decision(&mut self, analysis: &UpdateAnalysis) -> Result<IntelligentDecision> {
        info!("🤔 Prise de décision intelligente CachyPac");

        // Règles de décision simples
        let decision = if analysis.has_recent_failures {
            // Si des échecs récents, être prudent
            IntelligentDecision::NotifyOnly
        } else if !analysis.in_maintenance_window && !analysis.critical_packages.is_empty() {
            // Paquets critiques en dehors de la fenêtre de maintenance
            IntelligentDecision::Postpone("Paquets critiques détectés en dehors de la fenêtre de maintenance".to_string())
        } else if analysis.system_load > 80.0 {
            // Charge système élevée
            IntelligentDecision::Postpone("Charge système élevée".to_string())
        } else if analysis.safe_packages.len() == analysis.total_updates {
            // Que des paquets sûrs
            IntelligentDecision::InstallAll
        } else if !analysis.safe_packages.is_empty() && analysis.in_maintenance_window {
            // Installer seulement les paquets sûrs
            IntelligentDecision::InstallSafe(analysis.safe_packages.clone())
        } else {
            // Par défaut, notifier seulement
            IntelligentDecision::NotifyOnly
        };

        let decision_str = match &decision {
            IntelligentDecision::InstallAll => "Installer toutes les mises à jour",
            IntelligentDecision::InstallSafe(_) => "Installer seulement les mises à jour sûres",
            IntelligentDecision::Postpone(reason) => &format!("Reporter: {}", reason),
            IntelligentDecision::NotifyOnly => "Notifier seulement",
        };

        info!("💡 Décision CachyPac prise: {}", decision_str);
        self.logs.log_info("simple_intelligence", &format!("Décision: {}", decision_str), None).await?;

        Ok(decision)
    }

    /// Installe toutes les mises à jour
    async fn install_all_updates(&mut self, updates: &[String]) -> Result<()> {
        info!("🔧 Installation CachyPac de toutes les mises à jour");

        let start_time = std::time::Instant::now();
        let result = self.pacman.install_updates(updates.to_vec()).await;
        let duration = start_time.elapsed();

        let success = result.is_ok();
        let message = if success {
            "Installation CachyPac de toutes les mises à jour réussie".to_string()
        } else {
            format!("Échec de l'installation CachyPac: {}", result.unwrap_err())
        };

        // Enregistrer dans l'historique
        let entry = HistoryEntry {
            id: Uuid::new_v4(),
            timestamp: Local::now(),
            operation_type: OperationType::AutoUpdate,
            packages: updates.to_vec(),
            success,
            message: message.clone(),
            duration,
        };

        self.history.add_entry(entry).await?;

        // Notifier si configuré
        if let Some(ref mut telegram) = self.telegram {
            if success {
                telegram.send_updates_installed(updates, &duration).await?;
            } else {
                telegram.send_update_failed(&message).await?;
            }
        }

        if success {
            info!("✅ Installation CachyPac terminée avec succès");
        } else {
            error!("❌ Échec de l'installation CachyPac");
        }

        Ok(())
    }

    /// Installe seulement les mises à jour sûres
    async fn install_safe_updates(&mut self, safe_packages: &[String]) -> Result<()> {
        info!("🔧 Installation CachyPac des mises à jour sûres uniquement");

        let start_time = std::time::Instant::now();
        let result = self.pacman.install_updates(safe_packages.to_vec()).await;
        let duration = start_time.elapsed();

        let success = result.is_ok();
        let message = if success {
            format!("Installation CachyPac de {} mises à jour sûres réussie", safe_packages.len())
        } else {
            format!("Échec de l'installation CachyPac des mises à jour sûres: {}", result.unwrap_err())
        };

        // Enregistrer dans l'historique
        let entry = HistoryEntry {
            id: Uuid::new_v4(),
            timestamp: Local::now(),
            operation_type: OperationType::AutoUpdate,
            packages: safe_packages.to_vec(),
            success,
            message: message.clone(),
            duration,
        };

        self.history.add_entry(entry).await?;

        // Notifier
        if let Some(ref mut telegram) = self.telegram {
            if success {
                telegram.send_updates_installed(safe_packages, &duration).await?;
            } else {
                telegram.send_update_failed(&message).await?;
            }
        }

        Ok(())
    }

    /// Reporte les mises à jour
    async fn postpone_updates(&mut self, updates: &[String], reason: &str) -> Result<()> {
        info!("⏸️ Report CachyPac des mises à jour: {}", reason);

        self.logs.log_info(
            "simple_intelligence",
            &format!("Mises à jour CachyPac reportées: {}", reason),
            Some(format!("{} paquets concernés", updates.len()))
        ).await?;

        // Notifier du report
        if let Some(ref mut telegram) = self.telegram {
            telegram.send_message(&format!(
                "🔄 CachyPac - Mises à jour reportées\n\n📦 {} paquets disponibles\n⚠️ Raison: {}\n\n{}",
                updates.len(),
                reason,
                updates.join(", ")
            )).await?;
        }

        Ok(())
    }

    /// Notifie seulement les mises à jour disponibles
    async fn notify_updates_available(&mut self, updates: &[String]) -> Result<()> {
        info!("📢 Notification CachyPac des mises à jour disponibles");

        if let Some(ref mut telegram) = self.telegram {
            telegram.send_updates_available(updates).await?;
        }

        self.logs.log_info(
            "simple_intelligence",
            &format!("Notification CachyPac envoyée pour {} mises à jour", updates.len()),
            None
        ).await?;

        Ok(())
    }

    /// Vérifie si on est dans une fenêtre de maintenance
    async fn is_in_maintenance_window(&self) -> Result<bool> {
        // Implémentation simple: fenêtre de maintenance entre 2h et 6h du matin
        let now = Local::now();
        let hour = now.hour();
        Ok(hour >= 2 && hour < 6)
    }

    /// Récupère la charge système
    async fn get_system_load(&self) -> Result<f32> {
        // Implémentation simple: lire /proc/loadavg
        match tokio::fs::read_to_string("/proc/loadavg").await {
            Ok(content) => {
                let parts: Vec<&str> = content.split_whitespace().collect();
                if let Some(load_str) = parts.first() {
                    if let Ok(load) = load_str.parse::<f32>() {
                        return Ok(load * 100.0); // Convertir en pourcentage approximatif
                    }
                }
                Ok(0.0)
            }
            Err(_) => Ok(0.0),
        }
    }
}

/// Structure d'analyse des mises à jour
#[derive(Debug)]
struct UpdateAnalysis {
    total_updates: usize,
    critical_packages: Vec<String>,
    safe_packages: Vec<String>,
    risky_packages: Vec<String>,
    has_recent_failures: bool,
    in_maintenance_window: bool,
    system_load: f32,
}

/// Énumération des décisions intelligentes
#[derive(Debug)]
enum IntelligentDecision {
    InstallAll,
    InstallSafe(Vec<String>),
    Postpone(String),
    NotifyOnly,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialiser le logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("🚀 Démarrage de l'exemple d'intelligence simple CachyPac");

    // Créer et lancer l'intelligence simple
    let mut intelligence = SimpleIntelligence::new(None).await?;
    intelligence.run().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_update_analysis() {
        let temp_dir = tempdir().unwrap();
        let config = Config::default();
        
        // Créer une instance de test (sans les composants réseau)
        let pacman = PacmanManager::new(config.pacman.clone());
        let scheduler = SchedulerManager::new();
        let history = UpdateHistory::new(temp_dir.path().to_path_buf());
        let logs = LogManager::new(temp_dir.path().to_path_buf());
        
        let mut intelligence = SimpleIntelligence {
            config,
            pacman,
            scheduler,
            telegram: None,
            history,
            logs,
        };

        let updates = vec![
            "firefox".to_string(),
            "kernel-lts".to_string(),
            "nvidia-dkms".to_string(),
            "vim".to_string(),
        ];

        let analysis = intelligence.analyze_updates(&updates).await.unwrap();
        
        assert_eq!(analysis.total_updates, 4);
        assert!(!analysis.critical_packages.is_empty()); // kernel-lts
        assert!(!analysis.risky_packages.is_empty());    // nvidia-dkms
        assert!(!analysis.safe_packages.is_empty());     // firefox, vim
    }
}