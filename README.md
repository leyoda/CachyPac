# CachyPac - Gestionnaire de mises à jour Pacman

Une application Rust moderne avec interface graphique pour automatiser les mises à jour système Pacman sur Arch Linux et ses dérivés.

## ✨ Fonctionnalités

- 🕒 **Planification automatique** : Programmez vos mises à jour système avec des expressions cron
- 🔄 **Redémarrage intelligent** : Détection automatique du besoin de redémarrage après mises à jour
- 📱 **Notifications Telegram** : Recevez des notifications détaillées sur l'état des mises à jour
- 📊 **Historique détaillé** : Suivez l'historique complet avec statistiques et filtrage
- 🎨 **Interface moderne** : Interface graphique Iced avec 5 onglets et thèmes configurables
- ⚙️ **Configuration flexible** : Configuration complète via fichier TOML avec validation
- 🤖 **Intelligence artificielle** : Système d'analyse intelligente des mises à jour
- 🛠️ **Service systemd** : Installation et gestion automatique du service système
- 📝 **Logging avancé** : Système de logs centralisé avec recherche et export
- 📦 **Package AUR** : Installation simplifiée via AUR avec PKGBUILD complet
- 🌍 **Support multilingue** : Interface traduite en 9 langues avec détection automatique

## 🔧 Prérequis

- Arch Linux ou dérivé (Manjaro, EndeavourOS, CachyOS, etc.)
- Rust 1.70+ 
- Accès sudo pour les mises à jour système
- Optionnel : Bot Telegram pour les notifications

## 📦 Installation

### Installation via AUR (Recommandée)

```bash
# Avec yay
yay -S cachypac

# Avec paru
paru -S cachypac

# Avec makepkg (manuel)
git clone https://aur.archlinux.org/cachypac.git
cd cachypac
makepkg -si
```

L'installation AUR inclut automatiquement :
- Configuration du service systemd
- Fichiers de configuration exemple
- Script de post-installation automatique
- Toutes les dépendances requises

### Compilation depuis les sources

```bash
git clone https://github.com/leyoda/CachyPac.git
cd CachyPac
cargo build --release
```

L'exécutable sera disponible dans `target/release/cachypac`.

### Installation système manuelle

```bash
sudo cp target/release/cachypac /usr/local/bin/
```

### Installation du service systemd

```bash
# Installation du service (nécessite les privilèges root)
sudo cachypac --install-service

# Activation du service au démarrage
sudo systemctl enable cachypac

# Démarrage du service
sudo systemctl start cachypac
```

## ⚙️ Configuration

L'application recherche la configuration dans cet ordre :
1. `config.toml` (répertoire courant)
2. `config/config.toml`
3. `/etc/cachypac/config.toml`
4. `~/.config/cachypac/config.toml`

### Configuration Telegram (optionnelle)

1. Créez un bot Telegram via [@BotFather](https://t.me/botfather)
2. Obtenez votre Chat ID en envoyant un message à [@userinfobot](https://t.me/userinfobot)
3. Configurez ces informations dans l'interface de l'application

### Exemple de configuration complète

```toml
[general]
data_dir = "/var/lib/cachypac"
log_level = "info"
check_interval = 60  # minutes
auto_update = false
backup_before_update = true

[pacman]
timeout = 300  # secondes
retry_count = 3
retry_delay = 5  # secondes
exclude_packages = ["linux", "nvidia-dkms"]
include_aur = false
clean_cache_after = true
check_keyring = true

[scheduler]
enabled = true
cron_expression = "0 2 * * *"  # Tous les jours à 2h du matin
maintenance_window_start = "02:00"
maintenance_window_end = "06:00"
max_concurrent_jobs = 1

[telegram]
enabled = true
bot_token = "votre_token_bot"
chat_id = "votre_chat_id"
notify_on_updates = true
notify_on_errors = true
notify_on_success = true
message_format = "html"

[gui]
theme = "dark"  # "dark" ou "light"
window_width = 1000
window_height = 700
auto_start_minimized = false
show_system_tray = true
close_to_tray = true
```

## 🌍 Support Multilingue

CachyPac supporte **9 langues** avec détection automatique de la langue du système :

- 🇫🇷 **Français** - Langue par défaut
- 🇬🇧 **Anglais** - Langue de fallback
- 🇩🇪 **Allemand** - Deutsch
- 🇪🇸 **Espagnol** - Español
- 🇮🇹 **Italien** - Italiano
- 🇵🇹 **Portugais** - Português
- 🇷🇺 **Russe** - Русский
- 🇨🇳 **Chinois** - 中文
- 🇯🇵 **Japonais** - 日本語

### Fonctionnalités d'internationalisation

- **Détection automatique** : La langue est détectée automatiquement selon les paramètres système
- **Fallback intelligent** : Si une traduction manque, l'anglais est utilisé automatiquement
- **Interface complète** : Tous les éléments de l'interface sont traduits (menus, boutons, messages)
- **Architecture extensible** : Nouvelles langues facilement ajoutables via fichiers Fluent
- **Thread-safe** : Architecture sécurisée pour les applications multi-threadées

### Configuration de la langue

La langue est automatiquement détectée via les variables d'environnement système (`LANG`, `LC_ALL`). Pour forcer une langue spécifique :

```bash
# Lancer en français
LANG=fr_FR.UTF-8 cachypac

# Lancer en anglais
LANG=en_US.UTF-8 cachypac

# Lancer en allemand
LANG=de_DE.UTF-8 cachypac
```

## 🚀 Utilisation

### Modes de lancement

```bash
# Interface graphique (mode par défaut)
cachypac

# Mode daemon (sans interface graphique)
cachypac --daemon

# Avec fichier de configuration personnalisé
cachypac --config /path/to/config.toml

# Gestion du service systemd
cachypac --install-service    # Installer le service
cachypac --uninstall-service # Désinstaller le service
cachypac --service-status     # Afficher le statut
```

### Interface graphique

L'application propose **5 onglets principaux** :

1. **📊 Dashboard** : Vue d'ensemble du système et des mises à jour disponibles
2. **📦 Mises à jour** : Gestion manuelle des mises à jour avec liste détaillée
3. **⏰ Planificateur** : Configuration et contrôle des tâches automatiques
4. **📜 Historique** : Historique détaillé avec filtrage et statistiques
5. **⚙️ Paramètres** : Configuration complète de l'application

### Fonctionnalités avancées

- **Analyse intelligente** : Classification automatique des paquets (critiques, sûrs, risqués)
- **Fenêtres de maintenance** : Restriction des mises à jour à certaines heures
- **Gestion des échecs** : Retry automatique avec délai configurable
- **Export des données** : Export de l'historique et des logs en CSV
- **Monitoring système** : Surveillance de la charge et de l'utilisation mémoire

## 🤖 Intelligence Artificielle

Le projet inclut un exemple d'intelligence artificielle dans [`examples/simple_intelligence.rs`](examples/simple_intelligence.rs) qui démontre :

- Analyse automatique des mises à jour
- Prise de décision basée sur des règles
- Gestion des fenêtres de maintenance
- Adaptation selon la charge système

```bash
# Lancer l'exemple d'intelligence
cargo run --example simple_intelligence
```

## 🔒 Sécurité

- L'application nécessite des privilèges sudo pour exécuter `pacman`
- Les tokens Telegram sont stockés de manière sécurisée dans la configuration
- Validation stricte de toutes les entrées utilisateur
- Aucune donnée n'est transmise à des tiers (sauf Telegram si configuré)
- Logs détaillés pour audit et débogage

## 🖥️ Compatibilité

- **OS** : Arch Linux, Manjaro, EndeavourOS, CachyOS, ArcoLinux
- **Desktop** : Compatible Plasma 6, GNOME, XFCE, i3, Sway
- **Display** : X11 et Wayland supportés nativement
- **Architecture** : x86_64, ARM64

## 🏗️ Architecture technique

### Stack technologique

- **Frontend** : Iced 0.12+ (interface graphique native Rust)
- **Scheduler** : tokio-cron-scheduler (planification de tâches)
- **Configuration** : TOML avec serde (sérialisation)
- **Notifications** : teloxide (API Telegram)
- **Logging** : tracing + tracing-subscriber
- **Async Runtime** : tokio
- **CLI** : clap 4.0+
- **Error Handling** : anyhow

### Modules principaux

```
src/
├── main.rs          # Point d'entrée avec CLI et modes
├── config.rs        # Gestion configuration TOML avec validation
├── gui.rs           # Interface graphique Iced avec 5 onglets
├── scheduler.rs     # Planificateur cron avec tokio
├── pacman.rs        # Interface pacman avec parsing et retry
├── telegram.rs      # Notifications Telegram avec formatage HTML
├── history.rs       # Historique JSON avec filtrage et stats
├── logs.rs          # Système de logs centralisé
├── service.rs       # Gestion service systemd
├── i18n.rs          # Module d'internationalisation (9 langues)
└── performance.rs   # Optimisations et monitoring des performances

locales/
├── fr.ftl           # Traductions françaises (Fluent)
├── en.ftl           # Traductions anglaises
├── de.ftl           # Traductions allemandes
├── es.ftl           # Traductions espagnoles
├── it.ftl           # Traductions italiennes
├── pt.ftl           # Traductions portugaises
├── ru.ftl           # Traductions russes
├── zh.ftl           # Traductions chinoises
└── ja.ftl           # Traductions japonaises

aur/
├── PKGBUILD         # Script de construction AUR
├── .SRCINFO         # Métadonnées du package AUR
└── README.md        # Documentation AUR

examples/
└── simple_intelligence.rs  # Exemple d'IA pour mises à jour
```

## 🧪 Développement

### Tests

```bash
# Tests unitaires
cargo test

# Tests avec logs détaillés
RUST_LOG=debug cargo test

# Tests d'un module spécifique
cargo test config::tests
```

### Mode développement

```bash
# Lancement en mode debug
cargo run

# Avec logs détaillés
RUST_LOG=debug cargo run

# Mode daemon en développement
cargo run -- --daemon
```

### Linting et formatage

```bash
# Vérification du code
cargo clippy

# Formatage automatique
cargo fmt

# Vérification sans compilation
cargo check
```

## 📊 État du projet

### ✅ Fonctionnalités implémentées

- [x] Architecture modulaire complète (8 modules)
- [x] Interface graphique Iced avec 5 onglets
- [x] Planificateur de tâches avec expressions cron
- [x] Notifications Telegram avec formatage HTML
- [x] Historique JSON avec statistiques
- [x] Système de logs centralisé
- [x] Gestion service systemd
- [x] Configuration TOML avec validation
- [x] Exemple d'intelligence artificielle
- [x] Tests unitaires pour tous les modules
- [x] Dépendances complètes (clap, cron, iced)
- [x] Compilation fonctionnelle sans erreurs
- [x] Licence Apache 2.0 cohérente

### ✅ Récemment Complétées

- [x] Finalisation de l'intégration GUI/backend - Interface Iced complète avec 5 onglets
- [x] Tests d'intégration complets - 13 tests couvrant tous les aspects
- [x] Optimisation des performances - Cache, Buffer I/O, Pool de tâches
- [x] Documentation utilisateur complète - Guides et API documentation
- [x] **Package AUR pour installation simplifiée** - PKGBUILD complet avec service systemd
- [x] **Support multilingue (internationalisation)** - 9 langues avec détection automatique

### 🔄 Prochaines Étapes

- [ ] Intégration avec d'autres gestionnaires de paquets (flatpak, snap)
- [ ] Interface web optionnelle pour administration à distance
- [ ] Plugin système pour gestionnaires de bureau
- [ ] Support des hooks pacman personnalisés

## 🤝 Contribution

Les contributions sont les bienvenues ! Merci de :

1. Fork le projet
2. Créer une branche pour votre fonctionnalité (`git checkout -b feature/AmazingFeature`)
3. Commiter vos changements (`git commit -m 'Add some AmazingFeature'`)
4. Pousser vers la branche (`git push origin feature/AmazingFeature`)
5. Ouvrir une Pull Request

### Guidelines de contribution

- Respecter le style de code existant (cargo fmt)
- Ajouter des tests pour les nouvelles fonctionnalités
- Mettre à jour la documentation si nécessaire
- Suivre les conventions de commit conventionnel

## 📄 Licence

Ce projet est sous licence Apache 2.0. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

## 📚 Documentation

### Guides Utilisateur
- **[🚀 Guide de Démarrage Rapide](docs/quick_start.md)** - Installation et configuration en 5 minutes
- **[📖 Guide Utilisateur Complet](docs/user_guide.md)** - Documentation complète avec exemples détaillés
- **[⚡ Optimisations de Performance](docs/performance_optimizations.md)** - Guide des performances et optimisations

### Documentation Technique
- **[🔧 Documentation API](docs/api_documentation.md)** - API complète pour les développeurs
- **[🤖 Exemple d'Intelligence](examples/simple_intelligence.rs)** - Exemple d'intelligence artificielle

### Ressources Additionnelles
- **[🐛 Issues GitHub](https://github.com/leyoda/CachyPac/issues)** - Rapporter des bugs ou demander des fonctionnalités
- **[💬 Discussions](https://github.com/leyoda/CachyPac/discussions)** - Questions et discussions communautaires
- **[📝 Wiki](https://github.com/leyoda/CachyPac/wiki)** - Documentation collaborative

## 🆘 Support

- **Issues** : [GitHub Issues](https://github.com/leyoda/CachyPac/issues)
- **Discussions** : [GitHub Discussions](https://github.com/leyoda/CachyPac/discussions)
- **Documentation** : Guides complets disponibles dans le dossier `docs/`

## 🙏 Remerciements

- L'équipe Iced pour l'excellent framework GUI Rust
- La communauté Arch Linux pour l'écosystème
- Les mainteneurs de tokio, serde, clap et autres dépendances
- L'équipe CachyOS pour l'inspiration

## 📈 Statistiques du projet

- **Lignes de code** : ~4500+ lignes Rust
- **Modules** : 10 modules principaux + 1 exemple
- **Langues supportées** : 9 langues avec 165 traductions chacune
- **Tests** : Tests unitaires et d'intégration (29 tests)
- **Documentation** : Documentation complète des APIs + guides utilisateur
- **Package AUR** : PKGBUILD complet avec service systemd
- **Fichiers de traduction** : 9 fichiers Fluent (.ftl)

---

**⚠️ Note importante** : Cette application est conçue pour les utilisateurs avancés d'Arch Linux. Utilisez-la à vos propres risques et assurez-vous de comprendre les implications des mises à jour automatiques. Testez toujours en mode manuel avant d'activer l'automatisation.

**🔧 État actuel** : Le projet a été entièrement restauré après la résolution du problème de fichiers sources manquants. Tous les modules sont fonctionnels et l'architecture est complète.