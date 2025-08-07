#!/bin/bash

# Script de désinstallation pour CachyRust

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

# Fonction principale de désinstallation
uninstall_cachy_rust() {
    echo "=================================="
    echo "  Désinstallation de CachyRust"
    echo "=================================="
    echo
    
    print_status "Suppression des fichiers système..."
    
    # Supprimer l'exécutable
    if [ -f "/usr/local/bin/cachy-rust" ]; then
        sudo rm -f /usr/local/bin/cachy-rust
        print_success "Exécutable supprimé"
    fi
    
    # Supprimer le launcher
    if [ -f "/usr/local/bin/cachy-rust-launcher.sh" ]; then
        sudo rm -f /usr/local/bin/cachy-rust-launcher.sh
        print_success "Launcher supprimé"
    fi
    
    # Supprimer le fichier .desktop
    if [ -f "/usr/share/applications/cachy-rust.desktop" ]; then
        sudo rm -f /usr/share/applications/cachy-rust.desktop
        print_success "Raccourci d'application supprimé"
    fi
    
    # Supprimer les icônes
    sudo rm -f /usr/share/icons/hicolor/*/apps/cachy-rust.png 2>/dev/null || true
    sudo rm -f /usr/share/pixmaps/cachy-rust.png 2>/dev/null || true
    print_success "Icônes supprimées"
    
    # Mettre à jour le cache des icônes
    if command -v gtk-update-icon-cache &> /dev/null; then
        sudo gtk-update-icon-cache -f -t /usr/share/icons/hicolor 2>/dev/null || true
        print_success "Cache des icônes mis à jour"
    fi
    
    # Supprimer les permissions sudo (optionnel)
    echo
    read -p "Voulez-vous supprimer les permissions sudo pour pacman ? (y/N): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        if [ -f "/etc/sudoers.d/cachy-rust" ]; then
            sudo rm -f /etc/sudoers.d/cachy-rust
            print_success "Permissions sudo supprimées"
        fi
    fi
    
    # Demander si on supprime la configuration utilisateur
    echo
    read -p "Voulez-vous supprimer la configuration utilisateur (~/.config/cachy-rust) ? (y/N): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        if [ -d "$HOME/.config/cachy-rust" ]; then
            rm -rf "$HOME/.config/cachy-rust"
            print_success "Configuration utilisateur supprimée"
        fi
    else
        print_warning "Configuration utilisateur conservée dans ~/.config/cachy-rust"
    fi
    
    echo
    print_success "Désinstallation terminée !"
    echo
    print_warning "Merci d'avoir utilisé CachyRust !"
}

# Confirmation avant désinstallation
echo "Êtes-vous sûr de vouloir désinstaller CachyRust ?"
read -p "Tapez 'oui' pour confirmer: " -r

if [[ $REPLY == "oui" ]]; then
    uninstall_cachy_rust
else
    echo "Désinstallation annulée."
    exit 0
fi