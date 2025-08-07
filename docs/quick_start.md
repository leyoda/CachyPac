# Guide de Démarrage Rapide - CachyPac

## Installation Express

### 1. Prérequis
```bash
# Vérifier que vous êtes sur Arch Linux ou un dérivé
cat /etc/os-release | grep -i arch

# Installer Rust si nécessaire
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Installation
```bash
# Cloner et compiler
git clone https://github.com/leyoda/CachyPac.git
cd CachyPac
cargo build --release

# Installer
sudo cp target/release/cachypac /usr/local/bin/
sudo chmod +x /usr/local/bin/cachypac

# Créer les répertoires
mkdir -p ~/.config/cachypac ~/.local/share/cachypac
```

### 3. Configuration Minimale
```bash
# Créer le fichier de configuration
cat > ~/.config/cachypac/config.toml << 'EOF'
[general]
data_dir = "~/.local/share/cachypac"
log_level = "info"
check_interval = 60
auto_update = false
backup_before_update = true

[pacman]
timeout = 300
retry_count = 3
exclude_packages = []
include_aur = false

[scheduler]
enabled = false

[telegram]
enabled = false

[gui]
theme = "dark"
window_width = 1000
window_height = 700
EOF
```

## Utilisation Immédiate

### Interface Graphique
```bash
# Lancer l'interface graphique
cachypac
```

### Ligne de Commande
```bash
# Vérifier les mises à jour
cachypac --check-updates

# Mode daemon (arrière-plan)
cachypac --daemon

# Aide complète
cachypac --help
```

## Configuration Telegram (Optionnel)

### 1. Créer un Bot
1. Contacter [@BotFather](https://t.me/botfather) sur Telegram
2. Utiliser `/newbot` et suivre les instructions
3. Récupérer le token du bot

### 2. Obtenir le Chat ID
1. Envoyer un message à votre bot
2. Visiter `https://api.telegram.org/bot<TOKEN>/getUpdates`
3. Récupérer votre `chat_id`

### 3. Configurer CachyPac
```toml
[telegram]
enabled = true
bot_token = "VOTRE_TOKEN_ICI"
chat_id = "VOTRE_CHAT_ID_ICI"
notify_on_updates = true
notify_on_errors = true
notify_on_success = true
```

## Service Système (Optionnel)

```bash
# Installer le service
sudo cachypac --install-service

# Activer et démarrer
sudo systemctl enable cachypac
sudo systemctl start cachypac

# Vérifier le statut
sudo systemctl status cachypac
```

## Premiers Pas

### 1. Vérification Manuelle
```bash
cachypac --check-updates
```

### 2. Interface Graphique
- Lancez `cachypac`
- Explorez les 5 onglets : Dashboard, Mises à jour, Planificateur, Historique, Paramètres
- Cliquez sur "Vérifier les mises à jour" dans le Dashboard

### 3. Configuration Avancée
- Modifiez `~/.config/cachypac/config.toml` selon vos besoins
- Redémarrez CachyPac pour appliquer les changements

## Dépannage Express

### Problème : CachyPac ne démarre pas
```bash
# Vérifier les permissions
ls -la /usr/local/bin/cachypac
chmod +x /usr/local/bin/cachypac

# Tester en mode verbose
cachypac --verbose
```

### Problème : Erreurs de configuration
```bash
# Valider la configuration
cachypac --check-config

# Utiliser la configuration par défaut
rm ~/.config/cachypac/config.toml
cachypac  # Créera une config par défaut
```

### Problème : Interface graphique lente
```toml
# Dans config.toml
[performance]
profile = "aggressive"

[gui]
refresh_interval = 10000  # 10 secondes
```

## Ressources

- **Documentation complète** : [docs/user_guide.md](user_guide.md)
- **API Documentation** : [docs/api_documentation.md](api_documentation.md)
- **GitHub** : https://github.com/leyoda/CachyPac
- **Issues** : https://github.com/leyoda/CachyPac/issues

## Commandes Essentielles

| Commande | Description |
|----------|-------------|
| `cachypac` | Interface graphique |
| `cachypac --daemon` | Mode arrière-plan |
| `cachypac --check-updates` | Vérifier les mises à jour |
| `cachypac --install-service` | Installer le service système |
| `cachypac --help` | Aide complète |

---

**Félicitations !** CachyPac est maintenant installé et configuré. Consultez le [guide utilisateur complet](user_guide.md) pour découvrir toutes les fonctionnalités avancées.