# Internationalisation CachyRust

## Vue d'ensemble

CachyRust supporte l'internationalisation (i18n) avec deux langues :
- **Français** (fr) - Langue par défaut si `LANG` commence par "fr"
- **Anglais** (en) - Langue par défaut dans tous les autres cas

## Architecture

### Module i18n (`src/i18n.rs`)

Le système d'internationalisation est basé sur :

1. **Enum `Language`** : Définit les langues supportées
2. **Enum `TranslationKey`** : Clés de traduction typées
3. **Struct `I18n`** : Gestionnaire principal des traductions
4. **API globale** : Fonctions `t()` et `tf()` pour l'usage simple

### Détection automatique de langue

```rust
// Détection basée sur la variable d'environnement LANG
let language = Language::detect_system_language();

// Exemples :
// LANG=fr_FR.UTF-8 → Language::French
// LANG=en_US.UTF-8 → Language::English
// LANG=de_DE.UTF-8 → Language::English (fallback)
```

## Configuration

### Fichier de configuration

La langue est stockée dans `config.toml` :

```toml
[gui]
language = "fr"  # ou "en"
theme = "dark"
window_width = 1000
window_height = 700
```

### Changement de langue

```rust
// Via la configuration
config.update_language(Language::English);
config.save().await?;

// Via l'API globale
i18n::set_language(Language::French);
```

## Utilisation

### API Simple

```rust
use crate::i18n::{t, tf, TranslationKey};

// Traduction simple
let title = t(TranslationKey::AppTitle);

// Traduction avec formatage
let message = tf(TranslationKey::UpdateCompleted, &["5"]);
// Résultat FR: "✅ Mise à jour terminée avec succès: 5 paquets"
// Résultat EN: "✅ Update completed successfully: 5 packages"
```

### Initialisation

```rust
// Dans main.rs
i18n::init_with_language(config.gui.language);

// Ou détection automatique
i18n::init();
```

## Clés de traduction

### Catégories principales

- **Interface** : `AppTitle`, `SystemStatus`, `UpdatesAvailable`
- **Onglets** : `TabDashboard`, `TabConfiguration`, `TabHistory`
- **Configuration** : `ConfigScheduler`, `EnableAutoUpdates`, `UpdateTime`
- **Statuts** : `StatusSuccess`, `StatusFailed`, `StatusInProgress`
- **Messages** : `UpdateStarted`, `UpdateCompleted`, `RebootRequired`
- **Erreurs** : `ErrorGeneric`, `ErrorPacmanLocked`, `ErrorTelegramConfig`
- **Boutons** : `ButtonSave`, `ButtonCancel`, `ButtonTest`

### Formatage avec paramètres

Utilisez `{0}`, `{1}`, etc. dans les traductions :

```rust
// Clé de traduction
TranslationKey::UpdateCompleted => "✅ Mise à jour terminée: {0} paquets"

// Usage
tf(TranslationKey::UpdateCompleted, &["5"])
// → "✅ Mise à jour terminée: 5 paquets"
```

## Ajout de nouvelles traductions

### 1. Ajouter la clé

```rust
// Dans src/i18n.rs
pub enum TranslationKey {
    // ... existantes
    NewFeature,
}
```

### 2. Ajouter les traductions

```rust
// Français
fr.insert(TranslationKey::NewFeature, "Nouvelle fonctionnalité");

// Anglais  
en.insert(TranslationKey::NewFeature, "New feature");
```

### 3. Utiliser la traduction

```rust
let text = t(TranslationKey::NewFeature);
```

## Ajout d'une nouvelle langue

### 1. Étendre l'enum Language

```rust
pub enum Language {
    French,
    English,
    German, // Nouvelle langue
}
```

### 2. Implémenter les méthodes

```rust
impl Language {
    pub fn detect_system_language() -> Self {
        if let Ok(lang) = std::env::var("LANG") {
            let lang = lang.to_lowercase();
            if lang.starts_with("fr") {
                return Language::French;
            } else if lang.starts_with("de") {
                return Language::German; // Nouveau
            }
        }
        Language::English
    }

    pub fn to_code(&self) -> &'static str {
        match self {
            Language::French => "fr",
            Language::English => "en",
            Language::German => "de", // Nouveau
        }
    }
}
```

### 3. Ajouter les traductions

```rust
fn load_german_translations(&mut self) {
    let mut de = HashMap::new();
    de.insert(TranslationKey::AppTitle, "CachyRust - Pacman Update Manager");
    // ... autres traductions
    self.translations.insert(Language::German, de);
}
```

## Tests

### Tests unitaires

```bash
cargo test i18n
```

Les tests couvrent :
- Détection de langue
- Codes de langue
- Traductions de base
- Formatage avec paramètres

### Test manuel

```bash
# Test avec langue française
LANG=fr_FR.UTF-8 ./target/release/cachy-rust

# Test avec langue anglaise
LANG=en_US.UTF-8 ./target/release/cachy-rust
```

## Fallback et robustesse

### Système de fallback

1. **Traduction manquante** → Fallback vers l'anglais
2. **Langue non supportée** → Anglais par défaut
3. **I18n non initialisé** → Message d'erreur `[I18N_NOT_INITIALIZED]`

### Gestion d'erreurs

```rust
// Traduction sûre avec fallback automatique
let text = t(TranslationKey::SomeKey); // Ne panic jamais

// Vérification d'initialisation
if i18n::current_language() == Language::English {
    // Système initialisé
}
```

## Performance

### Optimisations

- **Chargement unique** : Traductions chargées une seule fois
- **HashMap lookup** : Accès O(1) aux traductions
- **Static storage** : Pas d'allocation dynamique
- **Thread-safe** : Utilisation de `OnceLock` et `Mutex`

### Mémoire

- ~50KB pour toutes les traductions (2 langues)
- Pas d'impact sur le temps de démarrage
- Changement de langue instantané

## Intégration GUI

### Configuration dans l'interface

L'utilisateur peut changer la langue via :
1. Onglet Configuration → Interface → Langue
2. Redémarrage automatique pour appliquer
3. Sauvegarde dans `config.toml`

### Mise à jour dynamique

Pour une mise à jour sans redémarrage (future amélioration) :

```rust
// Changer la langue
i18n::set_language(new_language);

// Forcer le rafraîchissement de l'interface
app.refresh_ui();
```

## Bonnes pratiques

### 1. Clés descriptives

```rust
// ✅ Bon
TranslationKey::UpdateInProgress

// ❌ Éviter
TranslationKey::Msg1
```

### 2. Formatage cohérent

```rust
// ✅ Utiliser des placeholders numérotés
"Mise à jour: {0} paquets en {1} secondes"

// ❌ Éviter la concaténation
"Mise à jour: " + packages + " paquets"
```

### 3. Contexte culturel

```rust
// Français : format 24h
"Heure: 14:30"

// Anglais : format 12h possible
"Time: 2:30 PM"
```

### 4. Longueur des textes

Prévoir que les traductions peuvent être plus longues :
- Allemand : souvent +30% vs anglais
- Français : souvent +20% vs anglais

## Maintenance

### Vérification des traductions manquantes

```bash
# Rechercher les clés non traduites
grep -r "MISSING TRANSLATION" logs/
```

### Mise à jour des traductions

1. Ajouter la nouvelle clé dans `TranslationKey`
2. Ajouter dans `load_french_translations()`
3. Ajouter dans `load_english_translations()`
4. Tester avec `cargo test i18n`
5. Tester manuellement avec les deux langues

## Roadmap

### Améliorations futures

- [ ] **Pluralisation** : Gestion des formes plurielles
- [ ] **Formatage de dates** : Selon la locale
- [ ] **Formatage des nombres** : Séparateurs décimaux
- [ ] **RTL Support** : Langues droite-à-gauche
- [ ] **Changement à chaud** : Sans redémarrage
- [ ] **Langues supplémentaires** : Allemand, Espagnol, Italien

### Fichiers de traduction externes

Possibilité future d'externaliser les traductions :

```
translations/
├── fr.toml
├── en.toml
└── de.toml
```

## Conclusion

Le système d'internationalisation de CachyRust est :
- ✅ **Robuste** : Fallback automatique, gestion d'erreurs
- ✅ **Performant** : Lookup O(1), pas d'allocation
- ✅ **Extensible** : Facile d'ajouter langues et traductions
- ✅ **Type-safe** : Clés typées, pas de strings magiques
- ✅ **Testé** : Tests unitaires complets

L'implémentation actuelle couvre tous les besoins de base et peut être étendue selon les demandes des utilisateurs.