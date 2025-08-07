# 🔧 Guide Service Systemd - CachyPac

## 🎯 Vérification du Service

### **1. Vérifier si le Service Existe**
```bash
# Vérification service utilisateur
systemctl --user status cachypac

# Vérification service système
sudo systemctl status cachypac

# Avec CachyPac
cargo run -- --service-status
```

### **2. Vérifier les Fichiers de Service**
```bash
# Fichier service utilisateur
ls -la ~/.config/systemd/user/cachypac.service

# Fichier service système
ls -la /etc/systemd/system/cachypac.service

# Fichier dans le projet
ls -la systemd/cachypac.service
```

## 🚀 Installation du Service

### **Option 1: Installation Automatique (Recommandée)**
```bash
# Compilation en mode release
cargo build --release

# Installation avec privilèges root
sudo cargo run --release -- --install-service
```

### **Option 2: Installation Manuelle**
```bash
# 1. Copier le fichier service
sudo cp systemd/cachypac.service /etc/systemd/system/

# 2. Modifier le chemin dans le fichier service
sudo nano /etc/systemd/system/cachypac.service
# Changer ExecStart vers le bon chemin

# 3. Recharger systemd
sudo systemctl daemon-reload

# 4. Activer le service
sudo systemctl enable cachypac

# 5. Démarrer le service
sudo systemctl start cachypac
```

### **Option 3: Service Utilisateur (Sans Root)**
```bash
# 1. Créer le répertoire
mkdir -p ~/.config/systemd/user

# 2. Copier et adapter le fichier service
cp systemd/cachypac.service ~/.config/systemd/user/
nano ~/.config/systemd/user/cachypac.service

# 3. Recharger et activer
systemctl --user daemon-reload
systemctl --user enable cachypac
systemctl --user start cachypac
```

## 📊 Vérification du Statut

### **Commandes de Vérification**
```bash
# Statut du service
sudo systemctl status cachypac

# Logs du service
sudo journalctl -u cachypac -f

# Vérifier si activé au démarrage
sudo systemctl is-enabled cachypac

# Vérifier si en cours d'exécution
sudo systemctl is-active cachypac
```

### **Avec CachyPac**
```bash
# Vérification intégrée
cargo run -- --service-status
```

## 🔄 Gestion du Service

### **Démarrage/Arrêt**
```bash
# Démarrer
sudo systemctl start cachypac

# Arrêter
sudo systemctl stop cachypac

# Redémarrer
sudo systemctl restart cachypac

# Recharger la configuration
sudo systemctl reload cachypac
```

### **Activation/Désactivation**
```bash
# Activer au démarrage
sudo systemctl enable cachypac

# Désactiver au démarrage
sudo systemctl disable cachypac
```

## 📋 Configuration du Service

### **Fichier systemd/cachypac.service**
```ini
[Unit]
Description=CachyPac - Gestionnaire de mises à jour Pacman
After=network.target

[Service]
Type=simple
User=lionel
ExecStart=/home/lionel/Devels/Rust/CachyPac/target/release/cachypac --daemon
Restart=always
RestartSec=10
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

### **Personnalisation**
- **User** : Remplacez par votre nom d'utilisateur
- **ExecStart** : Chemin vers l'exécutable CachyPac
- **Environment** : Variables d'environnement (logs, config)

## 🧪 Test Complet

### **1. Compilation Release**
```bash
cargo build --release
```

### **2. Test Manuel**
```bash
# Test du daemon
./target/release/cachypac --daemon
# Ctrl+C pour arrêter
```

### **3. Installation Service**
```bash
# Avec privilèges root
sudo ./target/release/cachypac --install-service
```

### **4. Vérification**
```bash
# Statut
sudo systemctl status cachypac

# Logs en temps réel
sudo journalctl -u cachypac -f
```

## 🎯 Programmation Automatique

### **Avec Systemd Timer (Alternative à Cron)**
```bash
# Créer un timer
sudo nano /etc/systemd/system/cachypac.timer
```

**Contenu du timer :**
```ini
[Unit]
Description=CachyPac Timer
Requires=cachypac.service

[Timer]
OnCalendar=*-*-* 02:00:00
Persistent=true

[Install]
WantedBy=timers.target
```

**Activation :**
```bash
sudo systemctl enable cachypac.timer
sudo systemctl start cachypac.timer
```

## ✅ Résultat Attendu

**Service installé et fonctionnel :**
```bash
$ sudo systemctl status cachypac
● cachypac.service - CachyPac - Gestionnaire de mises à jour Pacman
   Loaded: loaded (/etc/systemd/system/cachypac.service; enabled; vendor preset: disabled)
   Active: active (running) since Wed 2025-08-07 17:16:00 CET; 5min ago
   Main PID: 12345 (cachypac)
   Tasks: 1 (limit: 4915)
   Memory: 15.2M
   CGroup: /system.slice/cachypac.service
           └─12345 /home/lionel/Devels/Rust/CachyPac/target/release/cachypac --daemon
```

Le service CachyPac sera alors programmé et enverra des notifications Telegram automatiquement !