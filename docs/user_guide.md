# Guide Utilisateur CachyPac

## Table des Matières

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Configuration](#configuration)
4. [Utilisation](#utilisation)
5. [Interface Graphique](#interface-graphique)
6. [Mode Daemon](#mode-daemon)
7. [Notifications Telegram](#notifications-telegram)
8. [Planification](#planification)
9. [Historique et Logs](#historique-et-logs)
10. [Dépannage](#dépannage)
11. [FAQ](#faq)

## Introduction

CachyPac est une application moderne développée en Rust pour automatiser et simplifier la gestion des mises à jour Pacman sur Arch Linux et ses dérivés. Elle offre une interface graphique intuitive, des notifications intelligentes, et une planification flexible des mises à jour.

### Fonctionnalités Principales

- 🖥️ **Interface graphique moderne** avec Iced
- 🤖 **Mode daemon** pour fonctionnement en arrière-plan
- 📱 **Notifications Telegram** en temps réel
- ⏰ **Planification automatique** des mises à jour
- 📊 **Historique détaillé** des opérations
- 🔧 **Configuration flexible** via fichiers TOML
- 🚀 **Performances optimisées** avec cache et buffers
- 🛡️ **Sécurité renforcée** avec validation des paquets

## Installation

### Prérequis

- **Système d'exploitation :** Arch Linux, Manjaro, EndeavourOS, ou autre dérivé d'Arch
- **Rust :** Version 1.70 ou supérieure
- **Pacman :** Gestionnaire de paquets Arch Linux
- **Git :** Pour cloner le repository

### Installation depuis les Sources

1. **Cloner le repository :**
```bash
git clone https://github.com/leyoda/CachyPac.git
cd CachyPac
```

2. **Compiler l'application :**
```bash
cargo build --release
```

3. **Installer le binaire :**
```bash
sudo cp target/release/cachypac /usr/local/bin/
sudo chmod +x /usr/local/bin/cachypac
```

4. **Créer les répertoires de configuration :**
```bash
sudo mkdir -p /etc/cachypac
mkdir -p ~/.config/cachypac
mkdir -p ~/.local/share/cachypac
```

### Installation via AUR (Bientôt Disponible)

```bash
yay -S cachypac
# ou
paru -S cachypac
```

### Vérification de l'Installation

```bash
cachypac --version
```

## Configuration

### Fichier de Configuration Principal

CachyPac recherche sa configuration dans l'ordre suivant :
1. `./config.toml` (répertoire courant)
2. `./config/config.toml`
3. `/etc/cachypac/config.toml`
4. `~/.config/cachypac/config.toml`

### Configuration de Base

Créez le fichier `~/.config/cachypac/config.toml` :

```toml
[general]
data_dir = "~/.local/share/cachypac"
log_level = "info"
check_interval = 60  # minutes
auto_update = false
backup_before_update = true

[pacman]
timeout = 300  # secondes
retry_count = 3
retry_delay = 5  # secondes
exclude_packages = ["linux", "linux-headers"]
include_aur = false
clean_cache_after = true
check_keyring = true

[scheduler]
enabled = true
cron_expression = "0 2 * * *"  # 2h du matin tous les jours
maintenance_window_start = "02:00"
maintenance_window_end = "06:00"
max_concurrent_jobs = 1

[telegram]
enabled = false
bot_token = ""
chat_id = ""
notify_on_updates = true
notify_on_errors = true
notify_on_success = true
message_format = "html"

[gui]
theme = "dark"
window_width = 1000
window_height = 700
auto_start_minimized = false
show_system_tray = true
close_to_tray = true
```

### Configuration Avancée

#### Optimisations de Performance

```toml
[performance]
profile = "balanced"  # conservative, balanced, aggressive

[performance.io]
buffer_size = 8192
flush_interval = 5000  # ms
compression_level = 6

[performance.network]
connection_pool_size = 10
request_timeout = 30000  # ms
cache_ttl = 300  # seconds

[performance.memory]
max_cache_size = "100MB"
gc_interval = 60000  # ms
lazy_loading = true
```

#### Filtres de Paquets

```toml
[pacman.filters]
# Exclure des paquets spécifiques
exclude_packages = [
    "linux",
    "linux-headers", 
    "nvidia",
    "nvidia-utils"
]

# Inclure seulement certains groupes
include_groups = ["base", "base-devel"]

# Exclure des dépôts
exclude_repos = ["testing"]
```

## Utilisation

### Ligne de Commande

#### Commandes de Base

```bash
# Lancer l'interface graphique
cachypac

# Lancer en mode daemon
cachypac --daemon

# Utiliser un fichier de configuration spécifique
cachypac --config /path/to/config.toml

# Afficher l'aide
cachypac --help
```

#### Gestion du Service

```bash
# Installer le service systemd
sudo cachypac --install-service

# Vérifier le statut du service
cachypac --service-status

# Désinstaller le service
sudo cachypac --uninstall-service
```

#### Exemples d'Utilisation

```bash
# Vérification manuelle des mises à jour
cachypac --check-updates

# Mise à jour avec confirmation
cachypac --update --confirm

# Mise à jour automatique (attention !)
cachypac --update --auto

# Nettoyage du cache
cachypac --clean-cache

# Afficher l'historique
cachypac --history --last 10
```

## Interface Graphique

### Vue d'Ensemble

L'interface graphique de CachyPac est organisée en 5 onglets principaux :

#### 1. Dashboard
- **Vue d'ensemble** du système
- **Statut des mises à jour** disponibles
- **Informations système** (version, dernière mise à jour)
- **Actions rapides** (vérifier, mettre à jour)

#### 2. Mises à jour
- **Liste des paquets** à mettre à jour
- **Détails des paquets** (version actuelle/nouvelle, taille)
- **Sélection individuelle** des paquets
- **Boutons d'action** (tout sélectionner, mettre à jour)

#### 3. Planificateur
- **Configuration des tâches** automatiques
- **Calendrier visuel** des planifications
- **Gestion des fenêtres** de maintenance
- **Historique des exécutions** planifiées

#### 4. Historique
- **Journal complet** des opérations
- **Filtres avancés** (date, type, statut)
- **Détails des opérations** (durée, paquets affectés)
- **Export des données** (JSON, CSV)

#### 5. Paramètres
- **Configuration générale** de l'application
- **Paramètres Pacman** (timeout, retry, exclusions)
- **Configuration Telegram** (bot, notifications)
- **Préférences interface** (thème, taille fenêtre)

### Navigation et Raccourcis

| Raccourci | Action |
|-----------|--------|
| `Ctrl+R` | Actualiser les données |
| `Ctrl+U` | Vérifier les mises à jour |
| `Ctrl+S` | Sauvegarder la configuration |
| `Ctrl+Q` | Quitter l'application |
| `F5` | Actualiser la vue courante |
| `F11` | Mode plein écran |

## Mode Daemon

### Configuration du Service

Le mode daemon permet à CachyPac de fonctionner en arrière-plan et d'effectuer des vérifications automatiques.

#### Installation du Service

```bash
sudo cachypac --install-service
sudo systemctl enable cachypac
sudo systemctl start cachypac
```

#### Configuration du Service

Le fichier de service systemd est créé dans `/etc/systemd/system/cachypac.service` :

```ini
[Unit]
Description=CachyPac - Gestionnaire de mises à jour Pacman
After=network.target

[Service]
Type=simple
User=cachypac
Group=cachypac
ExecStart=/usr/local/bin/cachypac --daemon
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

#### Surveillance du Service

```bash
# Vérifier le statut
sudo systemctl status cachypac

# Voir les logs
sudo journalctl -u cachypac -f

# Redémarrer le service
sudo systemctl restart cachypac
```

### Fonctionnement en Arrière-plan

En mode daemon, CachyPac :
- Vérifie les mises à jour selon l'intervalle configuré
- Envoie des notifications Telegram si configuré
- Enregistre toutes les activités dans les logs
- Respecte les fenêtres de maintenance
- Peut effectuer des mises à jour automatiques si activé

## Notifications Telegram

### Configuration du Bot

1. **Créer un bot Telegram :**
   - Contacter [@BotFather](https://t.me/botfather) sur Telegram
   - Utiliser la commande `/newbot`
   - Suivre les instructions pour nommer votre bot
   - Récupérer le token du bot

2. **Obtenir votre Chat ID :**
   - Envoyer un message à votre bot
   - Visiter `https://api.telegram.org/bot<TOKEN>/getUpdates`
   - Récupérer votre `chat_id` dans la réponse JSON

3. **Configurer CachyPac :**
```toml
[telegram]
enabled = true
bot_token = "123456789:ABCdefGHIjklMNOpqrsTUVwxyz"
chat_id = "987654321"
notify_on_updates = true
notify_on_errors = true
notify_on_success = true
message_format = "html"
```

### Types de Notifications

#### Mises à jour Disponibles
```
🔄 CachyPac - Mises à jour disponibles

📦 Paquets à mettre à jour : 15
📊 Taille totale : 245 MB

Paquets principaux :
• firefox 120.0-1 → 121.0-1
• linux 6.6.8-1 → 6.6.9-1
• systemd 254.8-1 → 254.9-1

⏰ Dernière vérification : 2024-01-15 14:30:25
```

#### Mise à jour Terminée
```
✅ CachyPac - Mise à jour terminée

📦 Paquets mis à jour : 15
⏱️ Durée : 3m 45s
💾 Données téléchargées : 245 MB

✅ Mise à jour réussie
🔄 Redémarrage recommandé

⏰ Terminé le : 2024-01-15 14:35:10
```

#### Erreur de Mise à jour
```
❌ CachyPac - Erreur de mise à jour

🚫 Erreur : Conflit de dépendances
📦 Paquet concerné : package-name
🔍 Détails : dependency conflict detected

💡 Action recommandée :
Vérifier manuellement avec pacman -Syu

⏰ Erreur survenue le : 2024-01-15 14:32:15
```

### Personnalisation des Messages

```toml
[telegram.templates]
update_available = """
🔄 <b>CachyPac</b> - Mises à jour disponibles

📦 Paquets : {package_count}
📊 Taille : {total_size}

{package_list}

⏰ {timestamp}
"""

update_success = """
✅ <b>CachyPac</b> - Mise à jour terminée

📦 Paquets : {package_count}
⏱️ Durée : {duration}
💾 Téléchargé : {download_size}

⏰ {timestamp}
"""
```

## Planification

### Configuration Cron

CachyPac utilise des expressions cron pour la planification :

```toml
[scheduler]
enabled = true
cron_expression = "0 2 * * *"  # Tous les jours à 2h
maintenance_window_start = "02:00"
maintenance_window_end = "06:00"
```

### Exemples d'Expressions Cron

| Expression | Description |
|------------|-------------|
| `0 2 * * *` | Tous les jours à 2h00 |
| `0 2 * * 1` | Tous les lundis à 2h00 |
| `0 2 1 * *` | Le 1er de chaque mois à 2h00 |
| `0 */6 * * *` | Toutes les 6 heures |
| `30 1 * * 0` | Tous les dimanches à 1h30 |

### Fenêtres de Maintenance

Les fenêtres de maintenance permettent de limiter les mises à jour automatiques à certaines heures :

```toml
[scheduler]
maintenance_window_start = "02:00"
maintenance_window_end = "06:00"
# Les mises à jour automatiques ne se feront qu'entre 2h et 6h
```

### Gestion des Tâches

#### Via l'Interface Graphique
- Créer, modifier, supprimer des tâches planifiées
- Visualiser le calendrier des exécutions
- Activer/désactiver temporairement des tâches

#### Via la Configuration
```toml
[scheduler.jobs.daily_update]
enabled = true
cron = "0 2 * * *"
action = "check_and_notify"
auto_update = false

[scheduler.jobs.weekly_cleanup]
enabled = true
cron = "0 3 * * 0"
action = "clean_cache"
```

## Historique et Logs

### Historique des Opérations

CachyPac maintient un historique détaillé de toutes les opérations :

#### Types d'Opérations
- **CheckUpdates** : Vérification des mises à jour
- **ManualUpdate** : Mise à jour manuelle
- **AutoUpdate** : Mise à jour automatique
- **SystemMaintenance** : Maintenance système

#### Informations Enregistrées
- **Timestamp** : Date et heure précises
- **Durée** : Temps d'exécution
- **Paquets** : Liste des paquets concernés
- **Statut** : Succès ou échec
- **Message** : Détails de l'opération

### Système de Logs

#### Niveaux de Log
- **ERROR** : Erreurs critiques
- **WARN** : Avertissements
- **INFO** : Informations générales
- **DEBUG** : Informations de débogage
- **TRACE** : Informations très détaillées

#### Localisation des Logs
- **Fichier principal** : `~/.local/share/cachypac/cachypac_logs.json`
- **Logs système** : `journalctl -u cachypac`
- **Logs interface** : Affichés dans l'onglet Historique

#### Rotation des Logs
```toml
[logging]
max_file_size = "10MB"
max_files = 5
compress_old = true
retention_days = 30
```

### Analyse et Export

#### Filtres Avancés
- Par période (dernière heure, jour, semaine, mois)
- Par type d'opération
- Par statut (succès, échec)
- Par module (pacman, scheduler, telegram)

#### Export des Données
```bash
# Export JSON
cachypac --export-history --format json --output history.json

# Export CSV
cachypac --export-history --format csv --output history.csv

# Export avec filtres
cachypac --export-history --from "2024-01-01" --to "2024-01-31"
```

## Dépannage

### Problèmes Courants

#### 1. CachyPac ne démarre pas

**Symptômes :**
- Erreur au lancement
- Interface graphique ne s'affiche pas

**Solutions :**
```bash
# Vérifier les dépendances
ldd /usr/local/bin/cachypac

# Vérifier les permissions
ls -la /usr/local/bin/cachypac

# Tester en mode verbose
cachypac --verbose

# Vérifier la configuration
cachypac --check-config
```

#### 2. Erreurs de Permission

**Symptômes :**
- "Permission denied" lors des mises à jour
- Impossible d'écrire dans les répertoires

**Solutions :**
```bash
# Vérifier les permissions des répertoires
ls -la ~/.local/share/cachypac/
ls -la ~/.config/cachypac/

# Corriger les permissions
chmod -R 755 ~/.local/share/cachypac/
chmod -R 644 ~/.config/cachypac/

# Pour le mode daemon
sudo usermod -a -G wheel cachypac
```

#### 3. Problèmes de Réseau

**Symptômes :**
- Timeout lors des vérifications
- Échec des notifications Telegram

**Solutions :**
```bash
# Tester la connectivité
ping -c 3 archlinux.org
curl -I https://api.telegram.org

# Vérifier les proxies
echo $http_proxy
echo $https_proxy

# Configuration proxy dans CachyPac
[network]
proxy = "http://proxy.example.com:8080"
timeout = 60
```

#### 4. Interface Graphique Lente

**Symptômes :**
- Interface qui rame
- Temps de réponse élevé

**Solutions :**
```toml
# Optimiser les performances
[performance]
profile = "aggressive"

[performance.memory]
max_cache_size = "200MB"
lazy_loading = true

[gui]
refresh_interval = 5000  # ms
animation_enabled = false
```

### Diagnostic Avancé

#### Logs de Débogage
```bash
# Activer le mode debug
export RUST_LOG=cachypac=debug
cachypac

# Logs détaillés
export RUST_LOG=trace
cachypac --daemon

# Logs spécifiques
export RUST_LOG=cachypac::pacman=debug,cachypac::telegram=info
```

#### Profiling de Performance
```bash
# Mesurer les performances
cachypac --profile --output profile.json

# Analyser la mémoire
valgrind --tool=massif cachypac

# Tracer les appels système
strace -o trace.log cachypac
```

#### Vérification de l'Intégrité
```bash
# Vérifier les fichiers de configuration
cachypac --validate-config

# Tester les composants
cachypac --test-components

# Vérifier la base de données
cachypac --check-database
```

## FAQ

### Questions Générales

**Q: CachyPac peut-il remplacer complètement pacman ?**
R: Non, CachyPac est un frontend pour pacman. Il utilise pacman en arrière-plan et ne le remplace pas.

**Q: Est-ce sûr d'utiliser les mises à jour automatiques ?**
R: Les mises à jour automatiques peuvent être risquées. Il est recommandé de les utiliser uniquement pour les mises à jour de sécurité ou sur des systèmes de test.

**Q: CachyPac fonctionne-t-il avec les AUR helpers ?**
R: Partiellement. CachyPac peut détecter les paquets AUR mais ne peut pas les mettre à jour directement. Utilisez votre AUR helper habituel.

**Q: Puis-je utiliser CachyPac sur plusieurs machines ?**
R: Oui, vous pouvez partager la configuration via des fichiers de configuration centralisés ou des variables d'environnement.

### Questions Techniques

**Q: Comment sauvegarder ma configuration ?**
R: Copiez le répertoire `~/.config/cachypac/` et `~/.local/share/cachypac/`.

**Q: CachyPac supporte-t-il les hooks pacman ?**
R: Oui, CachyPac respecte et exécute les hooks pacman configurés.

**Q: Puis-je personnaliser les notifications ?**
R: Oui, via les templates dans la configuration Telegram et les webhooks personnalisés.

**Q: Comment contribuer au projet ?**
R: Visitez le repository GitHub : https://github.com/leyoda/CachyPac

### Dépannage Rapide

**Q: L'interface est en anglais, comment changer la langue ?**
R: Actuellement, seul le français est supporté. L'internationalisation est prévue pour une version future.

**Q: CachyPac consomme beaucoup de mémoire**
R: Ajustez les paramètres de cache dans la configuration performance.

**Q: Les notifications Telegram ne fonctionnent pas**
R: Vérifiez le token du bot, le chat_id, et la connectivité réseau.

**Q: Comment désinstaller CachyPac ?**
R: 
```bash
sudo systemctl stop cachypac
sudo systemctl disable cachypac
sudo rm /usr/local/bin/cachypac
sudo rm /etc/systemd/system/cachypac.service
rm -rf ~/.config/cachypac/
rm -rf ~/.local/share/cachypac/
```

---

## Support et Communauté

- **GitHub** : https://github.com/leyoda/CachyPac
- **Issues** : https://github.com/leyoda/CachyPac/issues
- **Wiki** : https://github.com/leyoda/CachyPac/wiki
- **Discussions** : https://github.com/leyoda/CachyPac/discussions

---

*Ce guide est maintenu à jour avec chaque version de CachyPac. Pour la version la plus récente, consultez la documentation en ligne.*