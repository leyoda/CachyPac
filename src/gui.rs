use std::collections::HashMap;
use iced::{
    widget::{button, column, container, progress_bar, row, scrollable, text, text_input, Column},
    Application, Command, Element, Length, Settings, Theme,
};
use iced_aw::tab_bar;
use tracing::{error, info};

use crate::{
    config::Config,
    pacman::PacmanManager,
    scheduler::SchedulerManager,
    telegram::TelegramNotifier,
    history::UpdateHistory,
    logs::LogManager,
    i18n::translate,
};

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(TabId),
    CheckUpdates,
    InstallUpdates,
    UpdatesChecked(Result<Vec<String>, String>),
    UpdatesInstalled(Result<(), String>),
    ConfigChanged(String, String),
    SaveConfig,
    LoadHistory,
    HistoryLoaded(Result<Vec<String>, String>),
    TestTelegram,
    TelegramTested(Result<(), String>),
    DiagnosticTelegram,
    TelegramDiagnosticCompleted(Result<String, String>),
    ToggleScheduler,
    #[allow(dead_code)]
    RefreshLogs,
    LogsRefreshed(Result<Vec<String>, String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabId {
    Dashboard,
    Updates,
    Scheduler,
    History,
    Settings,
}

impl TabId {
    const ALL: [TabId; 5] = [
        TabId::Dashboard,
        TabId::Updates,
        TabId::Scheduler,
        TabId::History,
        TabId::Settings,
    ];
}

impl std::fmt::Display for TabId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            TabId::Dashboard => format!("📊 {}", translate("menu-dashboard")),
            TabId::Updates => format!("📦 {}", translate("menu-updates")),
            TabId::Scheduler => format!("⏰ {}", translate("menu-scheduler")),
            TabId::History => format!("📜 {}", translate("menu-history")),
            TabId::Settings => format!("⚙️ {}", translate("menu-settings")),
        };
        write!(f, "{}", text)
    }
}

#[derive(Debug)]
pub struct CachyPacApp {
    config: Config,
    current_tab: TabId,
    available_updates: Vec<String>,
    is_checking_updates: bool,
    is_installing_updates: bool,
    status_message: String,
    progress: f32,
    config_inputs: HashMap<String, String>,
    history_entries: Vec<String>,
    log_entries: Vec<String>,
    pacman_manager: PacmanManager,
    #[allow(dead_code)]
    scheduler_manager: SchedulerManager,
    telegram_notifier: Option<TelegramNotifier>,
    update_history: UpdateHistory,
    log_manager: LogManager,
}

impl Application for CachyPacApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = Config;

    fn new(config: Config) -> (Self, Command<Message>) {
        let mut config_inputs = HashMap::new();
        config_inputs.insert("data_dir".to_string(), config.general.data_dir.clone());
        config_inputs.insert("check_interval".to_string(), config.general.check_interval.to_string());
        config_inputs.insert("cron_expression".to_string(), config.scheduler.cron_expression.clone());
        config_inputs.insert("telegram_token".to_string(), config.telegram.bot_token.clone());
        config_inputs.insert("telegram_chat_id".to_string(), config.telegram.chat_id.clone());

        let pacman_manager = PacmanManager::new(config.pacman.clone());
        let scheduler_manager = SchedulerManager::new();
        let telegram_notifier = if config.telegram.enabled && !config.telegram.bot_token.is_empty() {
            Some(TelegramNotifier::new(config.telegram.bot_token.clone(), config.telegram.chat_id.clone()))
        } else {
            None
        };
        
        let data_dir = std::path::PathBuf::from(&config.general.data_dir);
        let update_history = UpdateHistory::new(data_dir.clone());
        let log_manager = LogManager::new(data_dir);

        let app = Self {
            config,
            current_tab: TabId::Dashboard,
            available_updates: Vec::new(),
            is_checking_updates: false,
            is_installing_updates: false,
            status_message: "CachyPac prêt".to_string(),
            progress: 0.0,
            config_inputs,
            history_entries: Vec::new(),
            log_entries: Vec::new(),
            pacman_manager,
            scheduler_manager,
            telegram_notifier,
            update_history,
            log_manager,
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        translate("app-title")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TabSelected(tab) => {
                self.current_tab = tab;
                Command::none()
            }
            Message::CheckUpdates => {
                if !self.is_checking_updates {
                    self.is_checking_updates = true;
                    self.status_message = "Vérification des mises à jour...".to_string();
                    self.progress = 0.0;
                    info!("🔍 Vérification des mises à jour demandée");
                    
                    let pacman_manager = self.pacman_manager.clone();
                    Command::perform(
                        async move { pacman_manager.check_updates().await },
                        |result| Message::UpdatesChecked(result.map_err(|e| e.to_string())),
                    )
                } else {
                    Command::none()
                }
            }
            Message::UpdatesChecked(result) => {
                self.is_checking_updates = false;
                match result {
                    Ok(updates) => {
                        self.available_updates = updates;
                        self.status_message = format!("{} mises à jour disponibles", self.available_updates.len());
                        self.progress = 1.0;
                        info!("✅ {} mises à jour trouvées", self.available_updates.len());
                    }
                    Err(error) => {
                        self.status_message = format!("Erreur: {}", error);
                        self.progress = 0.0;
                        error!("❌ Erreur lors de la vérification: {}", error);
                    }
                }
                Command::none()
            }
            Message::InstallUpdates => {
                if !self.is_installing_updates && !self.available_updates.is_empty() {
                    self.is_installing_updates = true;
                    self.status_message = "Installation des mises à jour...".to_string();
                    self.progress = 0.0;
                    info!("⬇️ Installation des mises à jour demandée");
                    
                    let pacman_manager = self.pacman_manager.clone();
                    let updates = self.available_updates.clone();
                    Command::perform(
                        async move { pacman_manager.install_updates(updates).await },
                        |result| Message::UpdatesInstalled(result.map_err(|e| e.to_string())),
                    )
                } else {
                    Command::none()
                }
            }
            Message::UpdatesInstalled(result) => {
                self.is_installing_updates = false;
                match result {
                    Ok(_) => {
                        self.status_message = "Mises à jour installées avec succès".to_string();
                        self.available_updates.clear();
                        self.progress = 1.0;
                        info!("✅ Mises à jour installées avec succès");
                    }
                    Err(error) => {
                        self.status_message = format!("Erreur d'installation: {}", error);
                        self.progress = 0.0;
                        error!("❌ Erreur d'installation: {}", error);
                    }
                }
                Command::none()
            }
            Message::ConfigChanged(key, value) => {
                self.config_inputs.insert(key, value);
                Command::none()
            }
            Message::SaveConfig => {
                // Mettre à jour la configuration avec les nouvelles valeurs
                if let Some(data_dir) = self.config_inputs.get("data_dir") {
                    self.config.general.data_dir = data_dir.clone();
                }
                if let Some(interval) = self.config_inputs.get("check_interval") {
                    if let Ok(val) = interval.parse() {
                        self.config.general.check_interval = val;
                    }
                }
                if let Some(cron) = self.config_inputs.get("cron_expression") {
                    self.config.scheduler.cron_expression = cron.clone();
                }
                if let Some(token) = self.config_inputs.get("telegram_token") {
                    self.config.telegram.bot_token = token.clone();
                }
                if let Some(chat_id) = self.config_inputs.get("telegram_chat_id") {
                    self.config.telegram.chat_id = chat_id.clone();
                }

                self.status_message = "Configuration sauvegardée".to_string();
                info!("💾 Configuration sauvegardée");
                Command::none()
            }
            Message::LoadHistory => {
                let mut history = self.update_history.clone();
                Command::perform(
                    async move {
                        match history.load().await {
                            Ok(_) => {
                                let entries = history.get_all_entries()
                                    .iter()
                                    .map(|e| format!("{}: {} - {}",
                                        e.timestamp.format("%Y-%m-%d %H:%M"),
                                        e.operation_type,
                                        if e.success { "Succès" } else { "Échec" }
                                    ))
                                    .collect();
                                Ok(entries)
                            }
                            Err(e) => Err(e.to_string())
                        }
                    },
                    Message::HistoryLoaded,
                )
            }
            Message::HistoryLoaded(result) => {
                match result {
                    Ok(entries) => {
                        self.history_entries = entries;
                        info!("📜 Historique chargé: {} entrées", self.history_entries.len());
                    }
                    Err(error) => {
                        error!("❌ Erreur lors du chargement de l'historique: {}", error);
                    }
                }
                Command::none()
            }
            Message::TestTelegram => {
                if let Some(notifier) = &self.telegram_notifier {
                    let notifier = notifier.clone();
                    Command::perform(
                        async move { notifier.test_connection().await },
                        |result| Message::TelegramTested(result.map_err(|e| e.to_string())),
                    )
                } else {
                    // Diagnostic détaillé du problème
                    let problem = if self.config.telegram.bot_token.is_empty() {
                        "Token Telegram manquant"
                    } else if self.config.telegram.chat_id.is_empty() {
                        "Chat ID Telegram manquant"
                    } else if !self.config.telegram.enabled {
                        "Telegram désactivé dans la configuration"
                    } else {
                        "Configuration Telegram invalide"
                    };
                    
                    self.status_message = format!("❌ Telegram non configuré: {}", problem);
                    error!("❌ Test Telegram impossible: {}", problem);
                    Command::none()
                }
            }
            Message::DiagnosticTelegram => {
                // Diagnostic basique avec l'ancien module
                if self.config.telegram.enabled && !self.config.telegram.bot_token.is_empty() && !self.config.telegram.chat_id.is_empty() {
                    let config = self.config.telegram.clone();
                    Command::perform(
                        async move {
                            let mut diagnostics = Vec::new();
                            
                            // Test 1: Validation du token
                            if config.bot_token.len() < 45 || !config.bot_token.contains(':') {
                                diagnostics.push("❌ Format du token invalide");
                            } else {
                                diagnostics.push("✅ Format du token valide");
                            }
                            
                            // Test 2: Validation du chat_id
                            if config.chat_id.parse::<i64>().is_ok() || config.chat_id.starts_with('@') {
                                diagnostics.push("✅ Format du chat_id valide");
                            } else {
                                diagnostics.push("❌ Format du chat_id invalide");
                            }
                            
                            // Test 3: Configuration générale
                            if config.enabled {
                                diagnostics.push("✅ Telegram activé");
                            } else {
                                diagnostics.push("❌ Telegram désactivé");
                            }
                            
                            let result = format!("🔍 Diagnostic Telegram CachyPac:\n\n{}\n\n💡 Pour un diagnostic complet, utilisez: cargo run --example telegram_diagnostic",
                                diagnostics.join("\n"));
                            
                            Ok(result)
                        },
                        Message::TelegramDiagnosticCompleted,
                    )
                } else {
                    self.status_message = "❌ Configuration Telegram incomplète pour le diagnostic".to_string();
                    Command::none()
                }
            }
            Message::TelegramTested(result) => {
                match result {
                    Ok(_) => {
                        self.status_message = "✅ Test Telegram réussi - Message envoyé!".to_string();
                        info!("✅ Test Telegram réussi");
                    }
                    Err(error) => {
                        self.status_message = format!("❌ Erreur Telegram: {}", error);
                        error!("❌ Erreur Telegram: {}", error);
                    }
                }
                Command::none()
            }
            Message::TelegramDiagnosticCompleted(result) => {
                match result {
                    Ok(details) => {
                        self.status_message = "🔍 Diagnostic Telegram terminé - Voir les détails".to_string();
                        info!("🔍 Diagnostic Telegram: {}", details);
                        // Ici on pourrait afficher les détails dans une popup ou un onglet dédié
                    }
                    Err(error) => {
                        self.status_message = format!("❌ Erreur diagnostic: {}", error);
                        error!("❌ Erreur diagnostic Telegram: {}", error);
                    }
                }
                Command::none()
            }
            Message::ToggleScheduler => {
                self.config.scheduler.enabled = !self.config.scheduler.enabled;
                self.status_message = if self.config.scheduler.enabled {
                    "Planificateur activé".to_string()
                } else {
                    "Planificateur désactivé".to_string()
                };
                info!("🔄 Planificateur {}", if self.config.scheduler.enabled { "activé" } else { "désactivé" });
                Command::none()
            }
            Message::RefreshLogs => {
                let mut log_manager = self.log_manager.clone();
                Command::perform(
                    async move {
                        match log_manager.load().await {
                            Ok(_) => {
                                let entries = log_manager.get_recent_entries(24)
                                    .iter()
                                    .map(|e| format!("{}: [{}] {} - {}",
                                        e.timestamp.format("%H:%M:%S"),
                                        e.level,
                                        e.module,
                                        e.message
                                    ))
                                    .collect();
                                Ok(entries)
                            }
                            Err(e) => Err(e.to_string())
                        }
                    },
                    Message::LogsRefreshed,
                )
            }
            Message::LogsRefreshed(result) => {
                match result {
                    Ok(entries) => {
                        self.log_entries = entries;
                        info!("📝 Logs rafraîchis: {} entrées", self.log_entries.len());
                    }
                    Err(error) => {
                        error!("❌ Erreur lors du rafraîchissement des logs: {}", error);
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let tabs = TabId::ALL
            .iter()
            .fold(tab_bar::TabBar::new(Message::TabSelected), |tabs, tab| {
                tabs.push(*tab, iced_aw::TabLabel::Text(tab.to_string()))
            })
            .set_active_tab(&self.current_tab);

        let content = match self.current_tab {
            TabId::Dashboard => self.dashboard_view(),
            TabId::Updates => self.updates_view(),
            TabId::Scheduler => self.scheduler_view(),
            TabId::History => self.history_view(),
            TabId::Settings => self.settings_view(),
        };

        let status_bar = container(
            row![
                text(&self.status_message).size(14),
                progress_bar(0.0..=1.0, self.progress).width(Length::Fixed(200.0))
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center)
        )
        .padding(10)
        .style(iced::theme::Container::Box);

        column![tabs, content, status_bar]
            .spacing(0)
            .into()
    }

    fn theme(&self) -> Theme {
        if self.config.gui.theme == "dark" {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

impl CachyPacApp {
    fn dashboard_view(&self) -> Element<Message> {
        let stats = column![
            text("📊 CachyPac Dashboard").size(24),
            text("").size(10),
            row![
                text("🔄 Mises à jour disponibles:").size(16),
                text(self.available_updates.len().to_string()).size(16).style(iced::theme::Text::Color(iced::Color::from_rgb(0.0, 0.8, 0.0)))
            ].spacing(10),
            row![
                text("⚙️ Planificateur:").size(16),
                text(if self.config.scheduler.enabled { "Activé" } else { "Désactivé" }).size(16)
                    .style(if self.config.scheduler.enabled { 
                        iced::theme::Text::Color(iced::Color::from_rgb(0.0, 0.8, 0.0))
                    } else { 
                        iced::theme::Text::Color(iced::Color::from_rgb(0.8, 0.0, 0.0))
                    })
            ].spacing(10),
            row![
                text("📱 Notifications Telegram:").size(16),
                text(if self.config.telegram.enabled { "Activées" } else { "Désactivées" }).size(16)
                    .style(if self.config.telegram.enabled { 
                        iced::theme::Text::Color(iced::Color::from_rgb(0.0, 0.8, 0.0))
                    } else { 
                        iced::theme::Text::Color(iced::Color::from_rgb(0.8, 0.0, 0.0))
                    })
            ].spacing(10),
        ].spacing(15);

        let actions = row![
            button("🔍 Vérifier les mises à jour")
                .on_press(Message::CheckUpdates)
                .style(iced::theme::Button::Primary),
            button("📜 Charger l'historique")
                .on_press(Message::LoadHistory)
                .style(iced::theme::Button::Secondary),
        ].spacing(10);

        container(
            column![stats, text("").size(20), actions]
                .spacing(10)
                .align_items(iced::Alignment::Start)
        )
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn updates_view(&self) -> Element<Message> {
        let header = text("📦 Gestion des Mises à Jour").size(24);
        
        let content = if self.available_updates.is_empty() {
            column![
                text("✅ Aucune mise à jour disponible").size(16),
                text("").size(10),
                button("🔍 Vérifier les mises à jour")
                    .on_press(Message::CheckUpdates)
                    .style(iced::theme::Button::Primary)
            ]
        } else {
            let updates_list = self.available_updates
                .iter()
                .fold(Column::new().spacing(5), |col, update| {
                    col.push(text(format!("• {}", update)).size(14))
                });

            column![
                text(format!("📋 {} mises à jour disponibles:", self.available_updates.len())).size(16),
                text("").size(5),
                scrollable(updates_list).height(Length::Fixed(200.0)),
                text("").size(10),
                row![
                    button("⬇️ Installer toutes les mises à jour")
                        .on_press(Message::InstallUpdates)
                        .style(iced::theme::Button::Primary),
                    button("🔍 Revérifier")
                        .on_press(Message::CheckUpdates)
                        .style(iced::theme::Button::Secondary)
                ].spacing(10)
            ]
        };

        container(
            column![header, text("").size(10), content]
                .spacing(10)
                .align_items(iced::Alignment::Start)
        )
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn scheduler_view(&self) -> Element<Message> {
        let header = text("⏰ Planificateur de Tâches").size(24);
        
        let status_text = if self.config.scheduler.enabled { "✅ Activé" } else { "❌ Désactivé" };
        let status_color = if self.config.scheduler.enabled { 
            iced::Color::from_rgb(0.0, 0.8, 0.0)
        } else { 
            iced::Color::from_rgb(0.8, 0.0, 0.0)
        };

        let info = column![
            row![
                text("Statut:").size(16),
                text(status_text).size(16).style(iced::theme::Text::Color(status_color))
            ].spacing(10),
            row![
                text("Expression cron:").size(16),
                text(&self.config.scheduler.cron_expression).size(16)
            ].spacing(10),
            row![
                text("Fenêtre de maintenance:").size(16),
                text(format!("{} - {}", 
                    self.config.scheduler.maintenance_window_start,
                    self.config.scheduler.maintenance_window_end
                )).size(16)
            ].spacing(10),
        ].spacing(15);

        let toggle_button = button(
            if self.config.scheduler.enabled { "❌ Désactiver" } else { "✅ Activer" }
        )
        .on_press(Message::ToggleScheduler)
        .style(if self.config.scheduler.enabled { 
            iced::theme::Button::Destructive 
        } else { 
            iced::theme::Button::Positive 
        });

        container(
            column![header, text("").size(10), info, text("").size(20), toggle_button]
                .spacing(10)
                .align_items(iced::Alignment::Start)
        )
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn history_view(&self) -> Element<Message> {
        let header = text("📜 Historique des Opérations").size(24);
        
        let content = if self.history_entries.is_empty() {
            column![
                text("Aucun historique disponible pour le moment").size(16),
                text("").size(10),
                button("📜 Charger l'historique")
                    .on_press(Message::LoadHistory)
                    .style(iced::theme::Button::Primary)
            ]
        } else {
            let history_list = self.history_entries
                .iter()
                .fold(Column::new().spacing(5), |col, entry| {
                    col.push(text(entry).size(14))
                });

            column![
                text(format!("📋 {} entrées d'historique:", self.history_entries.len())).size(16),
                text("").size(5),
                scrollable(history_list).height(Length::Fixed(300.0)),
                text("").size(10),
                button("🔄 Actualiser")
                    .on_press(Message::LoadHistory)
                    .style(iced::theme::Button::Secondary)
            ]
        };

        container(
            column![header, text("").size(10), content]
                .spacing(10)
                .align_items(iced::Alignment::Start)
        )
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn settings_view(&self) -> Element<Message> {
        let header = text("⚙️ Paramètres CachyPac").size(24);
        
        let data_dir_input = row![
            text("Répertoire de données:").width(Length::Fixed(200.0)),
            text_input("", self.config_inputs.get("data_dir").unwrap_or(&String::new()))
                .on_input(|value| Message::ConfigChanged("data_dir".to_string(), value))
                .width(Length::Fixed(300.0))
        ].spacing(10).align_items(iced::Alignment::Center);

        let interval_input = row![
            text("Intervalle (minutes):").width(Length::Fixed(200.0)),
            text_input("", self.config_inputs.get("check_interval").unwrap_or(&String::new()))
                .on_input(|value| Message::ConfigChanged("check_interval".to_string(), value))
                .width(Length::Fixed(300.0))
        ].spacing(10).align_items(iced::Alignment::Center);

        let cron_input = row![
            text("Expression cron:").width(Length::Fixed(200.0)),
            text_input("", self.config_inputs.get("cron_expression").unwrap_or(&String::new()))
                .on_input(|value| Message::ConfigChanged("cron_expression".to_string(), value))
                .width(Length::Fixed(300.0))
        ].spacing(10).align_items(iced::Alignment::Center);

        let telegram_token_input = row![
            text("Token Telegram:").width(Length::Fixed(200.0)),
            text_input("", self.config_inputs.get("telegram_token").unwrap_or(&String::new()))
                .on_input(|value| Message::ConfigChanged("telegram_token".to_string(), value))
                .width(Length::Fixed(300.0))
                .secure(true)
        ].spacing(10).align_items(iced::Alignment::Center);

        let telegram_chat_input = row![
            text("Chat ID Telegram:").width(Length::Fixed(200.0)),
            text_input("", self.config_inputs.get("telegram_chat_id").unwrap_or(&String::new()))
                .on_input(|value| Message::ConfigChanged("telegram_chat_id".to_string(), value))
                .width(Length::Fixed(300.0))
        ].spacing(10).align_items(iced::Alignment::Center);

        let buttons = row![
            button("💾 Sauvegarder")
                .on_press(Message::SaveConfig)
                .style(iced::theme::Button::Primary),
            button("📱 Tester Telegram")
                .on_press(Message::TestTelegram)
                .style(iced::theme::Button::Secondary),
            button("🔍 Diagnostic Telegram")
                .on_press(Message::DiagnosticTelegram)
                .style(iced::theme::Button::Secondary),
        ].spacing(10);

        container(
            column![
                header,
                text("").size(10),
                data_dir_input,
                interval_input,
                cron_input,
                telegram_token_input,
                telegram_chat_input,
                text("").size(20),
                buttons
            ]
            .spacing(15)
            .align_items(iced::Alignment::Start)
        )
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

pub fn run_gui(config: Config) -> iced::Result {
    CachyPacApp::run(Settings::with_flags(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let config = Config::default();
        let (app, _) = CachyPacApp::new(config.clone());
        
        assert_eq!(app.current_tab, TabId::Dashboard);
        assert_eq!(app.available_updates.len(), 0);
        assert!(!app.is_checking_updates);
        assert!(!app.is_installing_updates);
        assert!(app.status_message.contains("CachyPac"));
    }

    #[test]
    fn test_tab_display() {
        assert_eq!(TabId::Dashboard.to_string(), "📊 Dashboard");
        assert_eq!(TabId::Updates.to_string(), "📦 Mises à jour");
        assert_eq!(TabId::Scheduler.to_string(), "⏰ Planificateur");
        assert_eq!(TabId::History.to_string(), "📜 Historique");
        assert_eq!(TabId::Settings.to_string(), "⚙️ Paramètres");
    }
}