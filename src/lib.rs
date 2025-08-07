//! CachyPac - Gestionnaire de mises à jour Pacman avec interface graphique
//! 
//! Cette bibliothèque fournit une interface moderne pour automatiser les mises à jour
//! système Pacman sur Arch Linux et ses dérivés.

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod config;
pub mod gui;
pub mod pacman;
pub mod scheduler;
pub mod telegram;
pub mod history;
pub mod logs;
pub mod service;
pub mod performance;
pub mod i18n;
pub mod telegram_robust;

// Ré-exports pour faciliter l'utilisation
pub use config::Config;
pub use pacman::PacmanManager;
pub use scheduler::SchedulerManager;
pub use telegram::TelegramNotifier;
pub use history::{UpdateHistory, HistoryEntry, OperationType};
pub use logs::{LogManager, LogLevel};
pub use service::ServiceManager;
pub use i18n::{I18nManager, SupportedLanguage, translate, translate_with_args, set_global_language, get_current_language, init_global_i18n};