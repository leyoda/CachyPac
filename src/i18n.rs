//! Module d'internationalisation pour CachyPac
//! 
//! Ce module gère la localisation de l'application de manière simple et efficace
//! en évitant les problèmes de thread safety avec FluentBundle.

use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use sys_locale::get_locale;

/// Langues supportées par CachyPac
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SupportedLanguage {
    French,
    English,
    German,
    Spanish,
    Italian,
    Portuguese,
    Russian,
    Chinese,
    Japanese,
}

impl SupportedLanguage {
    /// Convertit depuis un code de langue
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "fr" | "fr_fr" | "french" => Some(SupportedLanguage::French),
            "en" | "en_us" | "en_gb" | "english" => Some(SupportedLanguage::English),
            "de" | "de_de" | "german" => Some(SupportedLanguage::German),
            "es" | "es_es" | "spanish" => Some(SupportedLanguage::Spanish),
            "it" | "it_it" | "italian" => Some(SupportedLanguage::Italian),
            "pt" | "pt_pt" | "pt_br" | "portuguese" => Some(SupportedLanguage::Portuguese),
            "ru" | "ru_ru" | "russian" => Some(SupportedLanguage::Russian),
            "zh" | "zh_cn" | "zh_tw" | "chinese" => Some(SupportedLanguage::Chinese),
            "ja" | "ja_jp" | "japanese" => Some(SupportedLanguage::Japanese),
            _ => None,
        }
    }

    /// Retourne le nom de la langue dans sa propre langue
    #[allow(dead_code)]
    pub fn native_name(&self) -> &'static str {
        match self {
            SupportedLanguage::French => "Français",
            SupportedLanguage::English => "English",
            SupportedLanguage::German => "Deutsch",
            SupportedLanguage::Spanish => "Español",
            SupportedLanguage::Italian => "Italiano",
            SupportedLanguage::Portuguese => "Português",
            SupportedLanguage::Russian => "Русский",
            SupportedLanguage::Chinese => "中文",
            SupportedLanguage::Japanese => "日本語",
        }
    }

    /// Retourne le code de langue
    #[allow(dead_code)]
    pub fn code(&self) -> &'static str {
        match self {
            SupportedLanguage::French => "fr",
            SupportedLanguage::English => "en",
            SupportedLanguage::German => "de",
            SupportedLanguage::Spanish => "es",
            SupportedLanguage::Italian => "it",
            SupportedLanguage::Portuguese => "pt",
            SupportedLanguage::Russian => "ru",
            SupportedLanguage::Chinese => "zh",
            SupportedLanguage::Japanese => "ja",
        }
    }

    /// Retourne toutes les langues supportées
    pub fn all() -> Vec<SupportedLanguage> {
        vec![
            SupportedLanguage::French,
            SupportedLanguage::English,
            SupportedLanguage::German,
            SupportedLanguage::Spanish,
            SupportedLanguage::Italian,
            SupportedLanguage::Portuguese,
            SupportedLanguage::Russian,
            SupportedLanguage::Chinese,
            SupportedLanguage::Japanese,
        ]
    }
}

/// Gestionnaire d'internationalisation simplifié
pub struct I18nManager {
    translations: HashMap<SupportedLanguage, HashMap<String, String>>,
    current_language: SupportedLanguage,
    fallback_language: SupportedLanguage,
}

impl std::fmt::Debug for I18nManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("I18nManager")
            .field("current_language", &self.current_language)
            .field("fallback_language", &self.fallback_language)
            .field("available_languages", &self.translations.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl I18nManager {
    /// Crée une nouvelle instance du gestionnaire I18n
    pub fn new() -> Result<Self> {
        let mut manager = Self {
            translations: HashMap::new(),
            current_language: SupportedLanguage::French, // Par défaut
            fallback_language: SupportedLanguage::English,
        };

        // Charger toutes les langues supportées
        manager.load_all_languages()?;

        // Détecter la langue du système
        manager.detect_system_language();

        Ok(manager)
    }

    /// Charge toutes les langues supportées
    fn load_all_languages(&mut self) -> Result<()> {
        tracing::warn!("⚠️ DIAGNOSTIC I18N: Chargement des langues...");
        tracing::warn!("⚠️ DIAGNOSTIC I18N: Les fichiers .ftl dans locales/ NE SONT PAS utilisés!");
        tracing::warn!("⚠️ DIAGNOSTIC I18N: Utilisation de traductions codées en dur uniquement");
        
        for language in SupportedLanguage::all() {
            if let Err(e) = self.load_language(&language) {
                tracing::warn!("Impossible de charger la langue {:?}: {}", language, e);
                // Continuer avec les autres langues
            }
        }
        Ok(())
    }

    /// Charge une langue spécifique
    fn load_language(&mut self, language: &SupportedLanguage) -> Result<()> {
        // DIAGNOSTIC: Les fichiers Fluent existent mais ne sont pas chargés
        let ftl_path = format!("locales/{}.ftl", language.code());
        tracing::debug!("⚠️ DIAGNOSTIC I18N: Fichier {} existe mais NON chargé", ftl_path);
        
        // Charger les traductions depuis les ressources intégrées
        let translations = self.get_embedded_translations(language);
        self.translations.insert(language.clone(), translations);
        Ok(())
    }

    /// Retourne les traductions intégrées pour une langue
    fn get_embedded_translations(&self, language: &SupportedLanguage) -> HashMap<String, String> {
        let mut translations = HashMap::new();
        
        match language {
            SupportedLanguage::French => {
                translations.insert("app-title".to_string(), "CachyPac - Gestionnaire de Paquets Intelligent".to_string());
                translations.insert("menu-dashboard".to_string(), "Dashboard".to_string());
                translations.insert("menu-updates".to_string(), "Mises à jour".to_string());
                translations.insert("menu-packages".to_string(), "Paquets".to_string());
                translations.insert("menu-scheduler".to_string(), "Planificateur".to_string());
                translations.insert("menu-history".to_string(), "Historique".to_string());
                translations.insert("menu-settings".to_string(), "Paramètres".to_string());
                translations.insert("menu-about".to_string(), "À propos".to_string());
                translations.insert("updates-check".to_string(), "Vérifier les mises à jour".to_string());
                translations.insert("updates-install-all".to_string(), "Installer toutes les mises à jour".to_string());
                translations.insert("packages-search".to_string(), "Rechercher des paquets...".to_string());
                translations.insert("settings-language".to_string(), "Langue".to_string());
                translations.insert("button-ok".to_string(), "OK".to_string());
                translations.insert("button-cancel".to_string(), "Annuler".to_string());
                translations.insert("status-idle".to_string(), "En attente".to_string());
                translations.insert("error-network".to_string(), "Erreur réseau".to_string());
                translations.insert("success-install".to_string(), "Installation réussie".to_string());
            },
            SupportedLanguage::English => {
                translations.insert("app-title".to_string(), "CachyPac - Intelligent Package Manager".to_string());
                translations.insert("menu-dashboard".to_string(), "Dashboard".to_string());
                translations.insert("menu-updates".to_string(), "Updates".to_string());
                translations.insert("menu-packages".to_string(), "Packages".to_string());
                translations.insert("menu-scheduler".to_string(), "Scheduler".to_string());
                translations.insert("menu-history".to_string(), "History".to_string());
                translations.insert("menu-settings".to_string(), "Settings".to_string());
                translations.insert("menu-about".to_string(), "About".to_string());
                translations.insert("updates-check".to_string(), "Check for updates".to_string());
                translations.insert("updates-install-all".to_string(), "Install all updates".to_string());
                translations.insert("packages-search".to_string(), "Search packages...".to_string());
                translations.insert("settings-language".to_string(), "Language".to_string());
                translations.insert("button-ok".to_string(), "OK".to_string());
                translations.insert("button-cancel".to_string(), "Cancel".to_string());
                translations.insert("status-idle".to_string(), "Idle".to_string());
                translations.insert("error-network".to_string(), "Network error".to_string());
                translations.insert("success-install".to_string(), "Installation successful".to_string());
            },
            SupportedLanguage::German => {
                translations.insert("app-title".to_string(), "CachyPac - Intelligenter Paketmanager".to_string());
                translations.insert("menu-updates".to_string(), "Updates".to_string());
                translations.insert("menu-packages".to_string(), "Pakete".to_string());
                translations.insert("menu-history".to_string(), "Verlauf".to_string());
                translations.insert("menu-settings".to_string(), "Einstellungen".to_string());
                translations.insert("menu-about".to_string(), "Über".to_string());
                translations.insert("updates-check".to_string(), "Nach Updates suchen".to_string());
                translations.insert("updates-install-all".to_string(), "Alle Updates installieren".to_string());
                translations.insert("packages-search".to_string(), "Pakete suchen...".to_string());
                translations.insert("settings-language".to_string(), "Sprache".to_string());
                translations.insert("button-ok".to_string(), "OK".to_string());
                translations.insert("button-cancel".to_string(), "Abbrechen".to_string());
                translations.insert("status-idle".to_string(), "Bereit".to_string());
                translations.insert("error-network".to_string(), "Netzwerkfehler".to_string());
                translations.insert("success-install".to_string(), "Installation erfolgreich".to_string());
            },
            SupportedLanguage::Spanish => {
                translations.insert("app-title".to_string(), "CachyPac - Gestor de Paquetes Inteligente".to_string());
                translations.insert("menu-updates".to_string(), "Actualizaciones".to_string());
                translations.insert("menu-packages".to_string(), "Paquetes".to_string());
                translations.insert("menu-history".to_string(), "Historial".to_string());
                translations.insert("menu-settings".to_string(), "Configuración".to_string());
                translations.insert("menu-about".to_string(), "Acerca de".to_string());
                translations.insert("updates-check".to_string(), "Buscar actualizaciones".to_string());
                translations.insert("updates-install-all".to_string(), "Instalar todas las actualizaciones".to_string());
                translations.insert("packages-search".to_string(), "Buscar paquetes...".to_string());
                translations.insert("settings-language".to_string(), "Idioma".to_string());
                translations.insert("button-ok".to_string(), "Aceptar".to_string());
                translations.insert("button-cancel".to_string(), "Cancelar".to_string());
                translations.insert("status-idle".to_string(), "Inactivo".to_string());
                translations.insert("error-network".to_string(), "Error de red".to_string());
                translations.insert("success-install".to_string(), "Instalación exitosa".to_string());
            },
            _ => {
                // Pour les autres langues, utiliser l'anglais comme base
                return self.get_embedded_translations(&SupportedLanguage::English);
            }
        }
        
        translations
    }

    /// Détecte la langue du système
    fn detect_system_language(&mut self) {
        if let Some(locale) = get_locale() {
            let language_code = locale.split('_').next().unwrap_or("en");
            if let Some(language) = SupportedLanguage::from_code(language_code) {
                if self.translations.contains_key(&language) {
                    self.current_language = language;
                    tracing::info!("Langue détectée: {:?}", self.current_language);
                }
            }
        }
    }

    /// Change la langue courante
    #[allow(dead_code)]
    pub fn set_language(&mut self, language: SupportedLanguage) -> Result<()> {
        if self.translations.contains_key(&language) {
            self.current_language = language;
            tracing::info!("Langue changée vers: {:?}", self.current_language);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Langue non supportée: {:?}", language))
        }
    }

    /// Retourne la langue courante
    #[allow(dead_code)]
    pub fn current_language(&self) -> &SupportedLanguage {
        &self.current_language
    }

    /// Traduit un message
    pub fn translate(&self, key: &str) -> String {
        // Essayer avec la langue courante
        if let Some(translations) = self.translations.get(&self.current_language) {
            if let Some(translation) = translations.get(key) {
                return translation.clone();
            }
        }
        
        // Fallback vers la langue de secours
        if let Some(translations) = self.translations.get(&self.fallback_language) {
            if let Some(translation) = translations.get(key) {
                return translation.clone();
            }
        }
        
        // Si aucune traduction trouvée, retourner la clé
        tracing::warn!("Traduction manquante pour la clé: {}", key);
        format!("[{}]", key)
    }

    /// Traduit un message simple (alias pour translate)
    pub fn t(&self, key: &str) -> String {
        self.translate(key)
    }

    /// Retourne toutes les langues disponibles
    #[allow(dead_code)]
    pub fn available_languages(&self) -> Vec<&SupportedLanguage> {
        self.translations.keys().collect()
    }

    /// Vérifie si une langue est disponible
    #[allow(dead_code)]
    pub fn is_language_available(&self, language: &SupportedLanguage) -> bool {
        self.translations.contains_key(language)
    }
}

impl Default for I18nManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            tracing::error!("Impossible d'initialiser I18nManager: {}", e);
            // Créer une instance minimale en cas d'erreur
            Self {
                translations: HashMap::new(),
                current_language: SupportedLanguage::French,
                fallback_language: SupportedLanguage::English,
            }
        })
    }
}

/// Instance globale du gestionnaire I18n (thread-safe)
static GLOBAL_I18N: OnceLock<Arc<Mutex<I18nManager>>> = OnceLock::new();

/// Initialise l'instance globale I18n
pub fn init_global_i18n() -> Result<()> {
    let manager = I18nManager::new()?;
    GLOBAL_I18N.set(Arc::new(Mutex::new(manager)))
        .map_err(|_| anyhow::anyhow!("I18n déjà initialisé"))?;
    Ok(())
}

/// Fonctions utilitaires globales
#[allow(dead_code)]
pub fn get_i18n() -> Option<Arc<Mutex<I18nManager>>> {
    GLOBAL_I18N.get().cloned()
}

pub fn translate(key: &str) -> String {
    if let Some(i18n) = GLOBAL_I18N.get() {
        if let Ok(manager) = i18n.lock() {
            return manager.t(key);
        }
    }
    format!("[{}]", key) // Fallback si I18n n'est pas initialisé
}

#[allow(dead_code)]
pub fn translate_with_args(key: &str, _args: &HashMap<String, String>) -> String {
    // Version simplifiée sans support des arguments pour éviter les complications
    translate(key)
}

#[allow(dead_code)]
pub fn set_global_language(language: SupportedLanguage) -> Result<()> {
    if let Some(i18n) = GLOBAL_I18N.get() {
        let mut manager = i18n.lock().unwrap();
        manager.set_language(language)?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("I18n non initialisé"))
    }
}

#[allow(dead_code)]
pub fn get_current_language() -> SupportedLanguage {
    if let Some(i18n) = GLOBAL_I18N.get() {
        if let Ok(manager) = i18n.lock() {
            return manager.current_language().clone();
        }
    }
    SupportedLanguage::French // Fallback par défaut
}

/// Macro pour simplifier les traductions
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::i18n::translate($key)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_language_conversion() {
        assert_eq!(SupportedLanguage::from_code("fr"), Some(SupportedLanguage::French));
        assert_eq!(SupportedLanguage::from_code("en"), Some(SupportedLanguage::English));
        assert_eq!(SupportedLanguage::from_code("invalid"), None);
    }

    #[test]
    fn test_language_native_names() {
        assert_eq!(SupportedLanguage::French.native_name(), "Français");
        assert_eq!(SupportedLanguage::English.native_name(), "English");
        assert_eq!(SupportedLanguage::German.native_name(), "Deutsch");
    }

    #[test]
    fn test_i18n_manager_creation() {
        let manager = I18nManager::new();
        assert!(manager.is_ok());
        
        let manager = manager.unwrap();
        assert!(!manager.available_languages().is_empty());
    }

    #[test]
    fn test_translation() {
        let manager = I18nManager::new().unwrap();
        let translation = manager.translate("app-title");
        assert!(!translation.is_empty());
        assert!(!translation.starts_with('['));
    }

    #[test]
    fn test_language_switching() {
        let mut manager = I18nManager::new().unwrap();
        let result = manager.set_language(SupportedLanguage::English);
        assert!(result.is_ok());
        assert_eq!(manager.current_language(), &SupportedLanguage::English);
    }
}