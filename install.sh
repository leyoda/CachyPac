#!/bin/bash

# Script d'installation pour CachyRust
# Gestionnaire de mises à jour Pacman avec interface graphique

set -e

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Fonction d'affichage
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Vérifications préalables
check_requirements() {
    print_status "Vérification des prérequis..."
    
    # Vérifier si on est sur Arch Linux ou dérivé
    if ! command -v pacman &> /dev/null; then
        print_error "pacman n'est pas installé. Cette application est conçue pour Arch Linux et ses dérivés."
        exit 1
    fi
    
    # Vérifier Rust
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo n'est pas installé. Installez-le avec: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    # Vérifier sudo
    if ! command -v sudo &> /dev/null; then
        print_error "sudo n'est pas installé."
        exit 1
    fi
    
    print_success "Tous les prérequis sont satisfaits"
}

# Compilation de l'application
build_application() {
    print_status "Compilation de CachyRust..."
    
    if ! cargo build --release; then
        print_error "Échec de la compilation"
        exit 1
    fi
    
    print_success "Compilation réussie"
}

# Installation des fichiers
install_files() {
    print_status "Installation des fichiers..."
    
    # Créer les répertoires nécessaires
    sudo mkdir -p /usr/local/bin
    sudo mkdir -p /usr/share/applications
    sudo mkdir -p /usr/share/pixmaps
    sudo mkdir -p /usr/share/icons/hicolor/{16x16,32x32,48x48,64x64,128x128}/apps
    sudo mkdir -p /etc/systemd/user
    
    # Copier l'exécutable et le launcher
    sudo cp target/release/cachy-rust /usr/local/bin/
    sudo chmod +x /usr/local/bin/cachy-rust
    
    if [ -f "resources/cachy-rust-launcher.sh" ]; then
        sudo cp resources/cachy-rust-launcher.sh /usr/local/bin/
        sudo chmod +x /usr/local/bin/cachy-rust-launcher.sh
        print_success "Launcher installé (supprime les warnings graphiques)"
    fi
    
    # Installer le script de gestion du system tray
    if [ -f "resources/cachy-rust-tray.sh" ]; then
        sudo cp resources/cachy-rust-tray.sh /usr/local/bin/
        sudo chmod +x /usr/local/bin/cachy-rust-tray.sh
        print_success "Script de gestion du system tray installé"
    fi
    
    # Copier les icônes (nouveau design moderne)
    if [ -f "resources/cachy-rust-modern-16x16.png" ]; then
        sudo cp resources/cachy-rust-modern-16x16.png /usr/share/icons/hicolor/16x16/apps/cachy-rust.png
        sudo cp resources/cachy-rust-modern-32x32.png /usr/share/icons/hicolor/32x32/apps/cachy-rust.png
        sudo cp resources/cachy-rust-modern-48x48.png /usr/share/icons/hicolor/48x48/apps/cachy-rust.png
        sudo cp resources/cachy-rust-modern-64x64.png /usr/share/icons/hicolor/64x64/apps/cachy-rust.png
        sudo cp resources/cachy-rust-modern-128x128.png /usr/share/icons/hicolor/128x128/apps/cachy-rust.png
        sudo cp resources/cachy-rust-modern-128x128.png /usr/share/pixmaps/cachy-rust.png
        print_success "Icônes modernes installées"
    elif [ -f "resources/cachy-rust-16.png" ]; then
        # Fallback vers les anciennes icônes
        sudo cp resources/cachy-rust-16.png /usr/share/icons/hicolor/16x16/apps/cachy-rust.png
        sudo cp resources/cachy-rust-32.png /usr/share/icons/hicolor/32x32/apps/cachy-rust.png
        sudo cp resources/cachy-rust-48.png /usr/share/icons/hicolor/48x48/apps/cachy-rust.png
        sudo cp resources/cachy-rust-64.png /usr/share/icons/hicolor/64x64/apps/cachy-rust.png
        sudo cp resources/cachy-rust-128.png /usr/share/icons/hicolor/128x128/apps/cachy-rust.png
        sudo cp resources/cachy-rust-128.png /usr/share/pixmaps/cachy-rust.png
        print_success "Icônes installées (version classique)"
    else
        print_warning "Icônes non trouvées, utilisation de l'icône par défaut"
    fi
    
    # Installer les fichiers .desktop
    if [ -f "resources/cachy-rust.desktop" ]; then
        sudo cp resources/cachy-rust.desktop /usr/share/applications/
        print_success "Fichier .desktop principal installé"
    else
        # Créer le fichier .desktop de base
        EXEC_PATH="/usr/local/bin/cachy-rust"
        if [ -f "/usr/local/bin/cachy-rust-launcher.sh" ]; then
            EXEC_PATH="/usr/local/bin/cachy-rust-launcher.sh"
        fi
        
        cat > cachy-rust.desktop << EOF
[Desktop Entry]
Name=CachyRust
Comment=Gestionnaire de mises à jour Pacman avec interface graphique moderne
Exec=$EXEC_PATH
Icon=cachy-rust
Terminal=false
Type=Application
Categories=System;Settings;PackageManager;
Keywords=pacman;update;system;arch;cachy;rust;
StartupNotify=true
EOF
        
        sudo cp cachy-rust.desktop /usr/share/applications/
        print_success "Fichier .desktop de base créé"
    fi
    
    # Installer le service systemd pour le system tray
    if [ -f "resources/cachy-rust-tray.service" ]; then
        sudo cp resources/cachy-rust-tray.service /etc/systemd/user/
        print_success "Service systemd pour system tray installé"
    fi
    
    # Mettre à jour le cache des icônes
    if command -v gtk-update-icon-cache &> /dev/null; then
        sudo gtk-update-icon-cache -f -t /usr/share/icons/hicolor
        print_success "Cache des icônes mis à jour"
    fi
    
    print_success "Fichiers installés"
}

# Configuration utilisateur
setup_user_config() {
    print_status "Configuration utilisateur..."
    
    # Créer le répertoire de configuration
    CONFIG_DIR="$HOME/.config/cachy-rust"
    mkdir -p "$CONFIG_DIR"
    
    # Copier la configuration d'exemple si elle n'existe pas
    if [ ! -f "$CONFIG_DIR/config.toml" ]; then
        if [ -f "config.example.toml" ]; then
            cp config.example.toml "$CONFIG_DIR/config.toml"
            print_success "Configuration d'exemple copiée vers $CONFIG_DIR/config.toml"
        else
            print_warning "Fichier de configuration d'exemple non trouvé"
        fi
    else
        print_warning "Configuration existante trouvée, pas de modification"
    fi
}

# Configuration des permissions sudo
setup_sudo_permissions() {
    print_status "Configuration des permissions sudo..."
    
    # Demander si l'utilisateur veut configurer sudo sans mot de passe pour pacman
    echo
    read -p "Voulez-vous configurer sudo pour permettre l'exécution de pacman sans mot de passe ? (y/N): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        SUDOERS_FILE="/etc/sudoers.d/cachy-rust"
        echo "$USER ALL=(ALL) NOPASSWD: /usr/bin/pacman" | sudo tee "$SUDOERS_FILE" > /dev/null
        sudo chmod 440 "$SUDOERS_FILE"
        print_success "Permissions sudo configurées"
    else
        print_warning "Vous devrez entrer votre mot de passe sudo à chaque mise à jour"
    fi
}

# Configuration de l'autostart KDE
setup_kde_integration() {
    print_status "Configuration de l'intégration KDE..."
    
    # Demander si l'utilisateur veut l'indicateur système
    echo
    read -p "Voulez-vous activer l'indicateur système (system tray) au démarrage ? (Y/n): " -n 1 -r
    echo
    
    if [[ ! $REPLY =~ ^[Nn]$ ]]; then
        # Créer le répertoire d'autostart
        AUTOSTART_DIR="$HOME/.config/autostart"
        mkdir -p "$AUTOSTART_DIR"
        
        # Copier le fichier d'autostart
        if [ -f "resources/cachy-rust-autostart.desktop" ]; then
            cp resources/cachy-rust-autostart.desktop "$AUTOSTART_DIR/"
            print_success "Autostart configuré pour KDE"
        fi
        
        # Activer le service systemd utilisateur
        systemctl --user enable cachy-rust-tray.service 2>/dev/null || print_warning "Impossible d'activer le service systemd utilisateur"
        
        print_success "Indicateur système configuré"
        echo "  - L'indicateur apparaîtra dans la barre des tâches au prochain démarrage"
        echo "  - Vous pouvez le démarrer maintenant avec: cachy-rust-tray.sh start"
    else
        print_warning "Indicateur système non configuré"
        echo "  - Vous pouvez l'activer plus tard avec: cachy-rust-tray.sh start"
    fi
}

# Affichage des informations post-installation
show_post_install_info() {
    echo
    print_success "Installation terminée !"
    echo
    echo "Pour utiliser CachyRust :"
    echo "  - Lancez 'cachy-rust' depuis le terminal"
    echo "  - Ou trouvez 'CachyRust' dans votre menu d'applications"
    echo "  - Ou utilisez l'indicateur système dans la barre des tâches"
    echo
    echo "Configuration :"
    echo "  - Fichier de config: ~/.config/cachy-rust/config.toml"
    echo "  - Historique: ~/.config/cachy-rust/history.toml"
    echo
    echo "Gestion de l'indicateur système :"
    echo "  - Démarrer: cachy-rust-tray.sh start"
    echo "  - Arrêter: cachy-rust-tray.sh stop"
    echo "  - Statut: cachy-rust-tray.sh status"
    echo
    echo "Pour configurer Telegram (optionnel) :"
    echo "  1. Créez un bot via @BotFather sur Telegram"
    echo "  2. Obtenez votre Chat ID via @userinfobot"
    echo "  3. Configurez ces informations dans l'interface"
    echo
    print_warning "IMPORTANT: Testez d'abord en mode manuel avant d'activer les mises à jour automatiques"
}

# Fonction principale
main() {
    echo "=================================="
    echo "   Installation de CachyRust"
    echo "=================================="
    echo
    
    check_requirements
    build_application
    install_files
    setup_user_config
    setup_sudo_permissions
    setup_kde_integration
    show_post_install_info
}

# Exécution du script principal
main "$@"