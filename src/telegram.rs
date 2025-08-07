use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info, warn};

#[derive(Debug, Clone)]
pub struct TelegramNotifier {
    #[allow(dead_code)]
    bot_token: String,
    #[allow(dead_code)]
    chat_id: String,
    message_history: Vec<MessageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInfo {
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub message_type: MessageType,
    pub content: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    UpdatesAvailable,
    UpdatesInstalled,
    UpdateFailed,
    SystemInfo,
    Error,
    Warning,
    Info,
}

impl TelegramNotifier {
    pub fn new(bot_token: String, chat_id: String) -> Self {
        Self {
            bot_token,
            chat_id,
            message_history: Vec::new(),
        }
    }

    /// Envoie un message simple
    #[allow(dead_code)]
    pub async fn send_message(&mut self, message: &str) -> Result<()> {
        debug!("📤 Envoi message Telegram CachyPac: {}", message);
        
        warn!("⚠️ DIAGNOSTIC: Module Telegram en mode SIMULATION uniquement!");
        warn!("⚠️ DIAGNOSTIC: Token: {} caractères, Chat ID: {}",
              self.bot_token.len(),
              if self.chat_id.is_empty() { "VIDE" } else { "DÉFINI" });
        warn!("⚠️ DIAGNOSTIC: Aucune connexion réelle à l'API Telegram n'est effectuée");

        // Simulation d'envoi (pour éviter les dépendances)
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Enregistrer dans l'historique
        self.message_history.push(MessageInfo {
            timestamp: chrono::Local::now(),
            message_type: MessageType::Info,
            content: message.to_string(),
            success: true,
        });

        info!("✅ Message Telegram CachyPac envoyé (SIMULÉ)");
        Ok(())
    }

    /// Notifie des mises à jour disponibles
    pub async fn send_updates_available(&mut self, updates: &[String]) -> Result<()> {
        let message = self.format_updates_available_message(updates);
        
        let result = self.send_formatted_message(&message, MessageType::UpdatesAvailable).await;
        
        if result.is_ok() {
            info!("📢 Notification CachyPac mises à jour envoyée: {} paquets", updates.len());
        }
        
        result
    }

    /// Notifie que les mises à jour ont été installées
    #[allow(dead_code)]
    pub async fn send_updates_installed(&mut self, updates: &[String], duration: &Duration) -> Result<()> {
        let message = self.format_updates_installed_message(updates, duration);
        
        let result = self.send_formatted_message(&message, MessageType::UpdatesInstalled).await;
        
        if result.is_ok() {
            info!("✅ Notification CachyPac installation envoyée: {} paquets", updates.len());
        }
        
        result
    }

    /// Notifie d'un échec de mise à jour
    #[allow(dead_code)]
    pub async fn send_update_failed(&mut self, error_message: &str) -> Result<()> {
        let message = self.format_update_failed_message(error_message);
        
        let result = self.send_formatted_message(&message, MessageType::UpdateFailed).await;
        
        if result.is_ok() {
            info!("❌ Notification CachyPac échec envoyée");
        }
        
        result
    }

    /// Envoie un message formaté avec type
    async fn send_formatted_message(&mut self, message: &str, message_type: MessageType) -> Result<()> {
        // Simulation d'envoi
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Enregistrer dans l'historique
        self.message_history.push(MessageInfo {
            timestamp: chrono::Local::now(),
            message_type,
            content: message.to_string(),
            success: true,
        });

        Ok(())
    }

    /// Formate le message pour les mises à jour disponibles
    fn format_updates_available_message(&self, updates: &[String]) -> String {
        let mut message = String::new();
        message.push_str("🔄 <b>CachyPac - Mises à jour disponibles</b>\n\n");
        message.push_str(&format!("📦 <b>{} paquets</b> peuvent être mis à jour:\n\n", updates.len()));

        // Limiter à 20 paquets pour éviter des messages trop longs
        let display_updates = if updates.len() > 20 {
            &updates[..20]
        } else {
            updates
        };

        for (i, update) in display_updates.iter().enumerate() {
            message.push_str(&format!("{}. <code>{}</code>\n", i + 1, update));
        }

        if updates.len() > 20 {
            message.push_str(&format!("\n... et {} autres paquets\n", updates.len() - 20));
        }

        message.push_str(&format!("\n🕒 <i>{}</i>", chrono::Local::now().format("%d/%m/%Y %H:%M")));
        message
    }

    /// Formate le message pour les mises à jour installées
    #[allow(dead_code)]
    fn format_updates_installed_message(&self, updates: &[String], duration: &Duration) -> String {
        let mut message = String::new();
        message.push_str("✅ <b>CachyPac - Mises à jour installées</b>\n\n");
        message.push_str(&format!("📦 <b>{} paquets</b> ont été mis à jour avec succès\n\n", updates.len()));

        // Afficher quelques paquets importants
        let important_packages: Vec<&String> = updates
            .iter()
            .filter(|pkg| {
                let pkg_lower = pkg.to_lowercase();
                pkg_lower.contains("kernel") || 
                pkg_lower.contains("systemd") || 
                pkg_lower.contains("firefox") ||
                pkg_lower.contains("chromium")
            })
            .take(5)
            .collect();

        if !important_packages.is_empty() {
            message.push_str("🔥 <b>Paquets importants:</b>\n");
            for pkg in important_packages {
                message.push_str(&format!("• <code>{}</code>\n", pkg));
            }
            message.push('\n');
        }

        message.push_str(&format!("⏱️ Durée: <b>{:.1}s</b>\n", duration.as_secs_f64()));
        message.push_str(&format!("🕒 <i>{}</i>", chrono::Local::now().format("%d/%m/%Y %H:%M")));
        message
    }

    /// Formate le message pour les échecs de mise à jour
    #[allow(dead_code)]
    fn format_update_failed_message(&self, error_message: &str) -> String {
        format!(
            "❌ <b>CachyPac - Échec de mise à jour</b>\n\n🚨 <b>Erreur:</b>\n<code>{}</code>\n\n🕒 <i>{}</i>",
            error_message,
            chrono::Local::now().format("%d/%m/%Y %H:%M")
        )
    }

    /// Teste la connexion Telegram
    pub async fn test_connection(&self) -> Result<()> {
        info!("🔍 Test de connexion Telegram CachyPac");
        
        warn!("⚠️ DIAGNOSTIC: test_connection() est une SIMULATION!");
        warn!("⚠️ DIAGNOSTIC: Pas de vérification réelle du token ou de la connexion");
        warn!("⚠️ DIAGNOSTIC: Pour une vraie intégration, implémenter teloxide");

        // Simulation de test
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        info!("✅ Connexion Telegram CachyPac réussie (SIMULÉE)");
        Ok(())
    }

    /// Récupère l'historique des messages
    #[allow(dead_code)]
    pub fn get_message_history(&self) -> &[MessageInfo] {
        &self.message_history
    }

    /// Efface l'historique des messages
    #[allow(dead_code)]
    pub fn clear_message_history(&mut self) {
        self.message_history.clear();
        debug!("🧹 Historique des messages Telegram CachyPac effacé");
    }

    /// Récupère les statistiques des messages
    #[allow(dead_code)]
    pub fn get_message_stats(&self) -> MessageStats {
        let total_messages = self.message_history.len();
        let successful_messages = self.message_history.iter().filter(|m| m.success).count();
        let failed_messages = total_messages - successful_messages;

        let messages_by_type = {
            let mut map = std::collections::HashMap::new();
            for msg in &self.message_history {
                let type_name = format!("{:?}", msg.message_type);
                *map.entry(type_name).or_insert(0) += 1;
            }
            map
        };

        let last_message = self.message_history.last().map(|m| m.timestamp);

        MessageStats {
            total_messages,
            successful_messages,
            failed_messages,
            messages_by_type,
            last_message,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MessageStats {
    pub total_messages: usize,
    pub successful_messages: usize,
    pub failed_messages: usize,
    pub messages_by_type: std::collections::HashMap<String, usize>,
    pub last_message: Option<chrono::DateTime<chrono::Local>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_formatting() {
        let notifier = TelegramNotifier::new("fake_token".to_string(), "123456".to_string());
        
        let updates = vec!["firefox".to_string(), "chromium".to_string()];
        let message = notifier.format_updates_available_message(&updates);
        
        assert!(message.contains("CachyPac"));
        assert!(message.contains("2 paquets"));
        assert!(message.contains("firefox"));
        assert!(message.contains("chromium"));
    }

    #[tokio::test]
    async fn test_send_message() {
        let mut notifier = TelegramNotifier::new("fake_token".to_string(), "123456".to_string());
        
        let result = notifier.send_message("Test CachyPac").await;
        assert!(result.is_ok());
        assert_eq!(notifier.message_history.len(), 1);
        assert!(notifier.message_history[0].content.contains("Test CachyPac"));
    }
}