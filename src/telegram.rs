use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct TelegramNotifier {
    #[allow(dead_code)]
    bot_token: String,
    #[allow(dead_code)]
    chat_id: String,
    #[allow(dead_code)]
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

// Module Telegram legacy - conservé pour compatibilité mais non utilisé
// Le module telegram_robust.rs est maintenant utilisé à la place
impl TelegramNotifier {
    #[allow(dead_code)]
    pub fn new(bot_token: String, chat_id: String) -> Self {
        Self {
            bot_token,
            chat_id,
            message_history: Vec::new(),
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