# Guide d'Installation CachyRust

## 🚀 Installation Complète

### Prérequis
```bash
# Arch Linux / CachyOS requis
# Installer Rust si pas déjà fait
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Dépendances système
sudo pacman -S base-devel git pkgconf
```

### Installation depuis les Sources
```bash
# 1. Cloner le projet
git clone <repository-url> CachyRust
cd CachyRust

# 2. Compilation optimisée
cargo build --release

# 3. Installation système
sudo cp target/release/cachy-rust /usr/local/bin/
sudo chmod +x /usr/local/bin/cachy-rust

# 4. Installation des ressources
sudo ./install-icons-user.sh
```

### Configuration Initiale
```bash
# Créer le répertoire de configuration
mkdir -p ~/.config/cachy-rust

# Copier la configuration exemple avec IA
cp config.intelligence.example.toml ~/.config/cachy-rust/config.toml

# Éditer selon vos besoins
nano ~/.config/cachy-rust/config.toml
```

### Configuration Service Systemd (Optionnel)
```bash
# Copier le service utilisateur
cp resources/cachy-rust-tray.service ~/.config/systemd/user/
systemctl --user daemon-reload

# Activer le démarrage automatique
systemctl --user enable cachy-rust-tray.service
systemctl --user start cachy-rust-tray.service
```

## ✅ Vérification Installation

### Tests de Base
```bash
# Aide
cachy-rust --help

# Test intelligence artificielle
cargo run --example simple_intelligence

# Interface graphique
cachy-rust

# Mode tray (expérimental)
cachy-rust --tray
```

### Vérification Configuration
```bash
# Vérifier la configuration
cat ~/.config/cachy-rust/config.toml

# Vérifier les logs
ls -la ~/.config/cachy-rust/
```

## 🧠 Activation de l'Intelligence Artificielle

### Configuration IA
Éditez `~/.config/cachy-rust/config.toml` :

```toml
[intelligence]
enabled = true                    # Activer l'IA
learning_enabled = true           # Apprentissage automatique
reboot_prediction_enabled = true  # Prédiction redémarrages
schedule_optimization_enabled = true  # Optimisation horaires
anomaly_detection_enabled = true  # Détection anomalies
min_history_entries = 10         # Minimum pour apprentissage
```

### Test des Fonctionnalités IA
```bash
# Démonstration complète
cargo run --example simple_intelligence

# Sortie attendue :
# 🧠 Démonstration Simple de l'Intelligence CachyRust
# 📦 linux: 95% de chance de redémarrage
# 🎯 Moment optimal: 2h00 du matin
# ✅ Aucune anomalie détectée
```

## 📱 Configuration Telegram (Optionnel)

### Créer un Bot Telegram
1. Contacter @BotFather sur Telegram
2. Créer un nouveau bot : `/newbot`
3. Récupérer le token

### Configuration
```toml
[telegram]
enabled = true
bot_token = "VOTRE_TOKEN_BOT"
chat_id = "VOTRE_CHAT_ID"
send_success = true
send_errors = true
send_start = true
send_reboot = true
```

## 🔧 Utilisation

### Interface Graphique
```bash
cachy-rust              # Interface complète
cachy-rust --config     # Ouvrir configuration
```

### Ligne de Commande
```bash
cachy-rust --update     # Mise à jour manuelle
cachy-rust --tray       # Mode système tray
cachy-rust --help       # Aide complète
```

### Scripts de Gestion
```bash
# Gestion du tray
./resources/cachy-rust-tray.sh start
./resources/cachy-rust-tray.sh status
./resources/cachy-rust-tray.sh stop

# Gestion du service
./manage-service.sh install
./manage-service.sh start
./manage-service.sh status
```

## 🛠️ Dépannage

### Problèmes Courants

#### Configuration Manquante
```bash
# Erreur: "missing field intelligence"
# Solution: Ajouter la section [intelligence] à config.toml
echo "
[intelligence]
enabled = false
learning_enabled = true
reboot_prediction_enabled = true
schedule_optimization_enabled = true
anomaly_detection_enabled = true
min_history_entries = 10" >> ~/.config/cachy-rust/config.toml
```

#### Mode Tray qui Plante
```bash
# Le mode tray est expérimental
# Utiliser l'interface normale :
cachy-rust

# Vérifier les logs :
cat ~/.config/cachy-rust/tray.log
```

#### Permissions
```bash
# Si erreurs de permissions :
sudo chown -R $USER:$USER ~/.config/cachy-rust/
chmod 755 ~/.config/cachy-rust/
chmod 644 ~/.config/cachy-rust/*.toml
```

### Logs de Debug
```bash
# Logs application
tail -f ~/.config/cachy-rust/tray.log

# Logs système
journalctl --user -u cachy-rust-tray.service -f

# Debug compilation
RUST_BACKTRACE=1 cargo run
```

## 📊 Vérification Fonctionnement

### Tests Complets
```bash
# 1. Compilation
cargo test
cargo build --release

# 2. Fonctionnalités de base
cachy-rust --help

# 3. Intelligence artificielle
cargo run --example simple_intelligence

# 4. Configuration
cachy-rust --config

# 5. Mise à jour (dry-run)
cachy-rust --update
```

### Métriques de Performance
```bash
# Temps de démarrage (doit être < 1s)
time cachy-rust --help

# Utilisation mémoire (doit être < 50MB)
ps aux | grep cachy-rust

# Tests unitaires (doivent tous passer)
cargo test
```

## 🎯 Prochaines Étapes

### Utilisation Quotidienne
1. **Lancer l'interface** : `cachy-rust`
2. **Configurer les notifications** Telegram
3. **Activer l'IA** pour les prédictions
4. **Planifier les mises à jour** automatiques

### Fonctionnalités Avancées
1. **Service systemd** pour démarrage automatique
2. **Mode tray** pour utilisation discrète
3. **Historique détaillé** des mises à jour
4. **Statistiques intelligentes** avec IA

---

**CachyRust est maintenant installé et prêt à optimiser vos mises à jour Arch Linux avec intelligence artificielle !**

Pour plus d'informations, consultez :
- `INTELLIGENCE.md` - Guide complet de l'IA
- `README.md` - Documentation générale
- `config.intelligence.example.toml` - Configuration exemple