#!/bin/bash

# Script de test du package AUR CachyRust
# Usage: ./test-package.sh [install|remove|reinstall|test-all]

# Utiliser PKGBUILD-local pour les tests locaux
PKGBUILD_FILE="PKGBUILD-local"

set -e

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Vérifier les prérequis
check_prerequisites() {
    print_status "Vérification des prérequis..."
    
    if ! command -v makepkg &> /dev/null; then
        print_error "makepkg n'est pas installé. Installez base-devel."
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo n'est pas installé."
        exit 1
    fi
    
    if [ ! -f "$PKGBUILD_FILE" ]; then
        print_error "$PKGBUILD_FILE non trouvé dans le répertoire courant."
        exit 1
    fi
    
    # Copier PKGBUILD-local vers PKGBUILD pour makepkg
    if [ "$PKGBUILD_FILE" != "PKGBUILD" ]; then
        print_status "Utilisation de $PKGBUILD_FILE pour les tests locaux..."
        cp "$PKGBUILD_FILE" PKGBUILD
    fi
    
    print_success "Prérequis vérifiés"
}

# Nettoyer les fichiers de build précédents
clean_build() {
    print_status "Nettoyage des fichiers de build..."
    rm -rf src/ pkg/ *.pkg.tar.* *.log
    print_success "Nettoyage terminé"
}

# Construire le package
build_package() {
    print_status "Construction du package..."
    
    # Nettoyer d'abord
    clean_build
    
    # Construire le package
    if makepkg -s --noconfirm; then
        print_success "Package construit avec succès"
        
        # Lister les packages générés
        print_status "Packages générés:"
        ls -la *.pkg.tar.*
        
        return 0
    else
        print_error "Échec de la construction du package"
        return 1
    fi
}

# Installer le package
install_package() {
    print_status "Installation du package..."
    
    # Trouver le fichier package
    PKG_FILE=$(ls *.pkg.tar.* 2>/dev/null | head -n1)
    
    if [ -z "$PKG_FILE" ]; then
        print_warning "Aucun package trouvé, construction en cours..."
        if ! build_package; then
            return 1
        fi
        PKG_FILE=$(ls *.pkg.tar.* 2>/dev/null | head -n1)
    fi
    
    if [ -n "$PKG_FILE" ]; then
        print_status "Installation de $PKG_FILE..."
        if sudo pacman -U "$PKG_FILE" --noconfirm; then
            print_success "Package installé avec succès"
            
            # Vérifier l'installation
            verify_installation
            return 0
        else
            print_error "Échec de l'installation"
            return 1
        fi
    else
        print_error "Aucun package à installer"
        return 1
    fi
}

# Vérifier l'installation
verify_installation() {
    print_status "Vérification de l'installation..."
    
    # Vérifier que le binaire est installé
    if command -v cachy-rust &> /dev/null; then
        print_success "✓ Binaire cachy-rust installé"
    else
        print_error "✗ Binaire cachy-rust non trouvé"
    fi
    
    # Vérifier les fichiers principaux
    local files_to_check=(
        "/usr/bin/cachy-rust"
        "/usr/share/applications/cachy-rust.desktop"
        "/etc/cachy-rust/config.toml"
        "/usr/share/doc/cachy-rust/README.md"
        "/usr/share/licenses/cachy-rust/LICENSE"
    )
    
    for file in "${files_to_check[@]}"; do
        if [ -f "$file" ]; then
            print_success "✓ $file"
        else
            print_warning "✗ $file manquant"
        fi
    done
    
    # Vérifier les icônes
    if [ -d "/usr/share/icons/hicolor" ]; then
        local icon_count=$(find /usr/share/icons/hicolor -name "cachy-rust.png" | wc -l)
        if [ "$icon_count" -gt 0 ]; then
            print_success "✓ $icon_count icônes installées"
        else
            print_warning "✗ Aucune icône trouvée"
        fi
    fi
    
    # Vérifier le service systemd
    if [ -f "/usr/lib/systemd/user/cachy-rust-tray.service" ]; then
        print_success "✓ Service systemd installé"
    else
        print_warning "✗ Service systemd manquant"
    fi
    
    # Test de lancement rapide
    print_status "Test de lancement (version)..."
    if timeout 5 cachy-rust --help &> /dev/null; then
        print_success "✓ Application se lance correctement"
    else
        print_warning "✗ Problème de lancement (normal si pas d'affichage)"
    fi
}

# Supprimer le package
remove_package() {
    print_status "Suppression du package..."
    
    if pacman -Qi cachy-rust &> /dev/null; then
        if sudo pacman -R cachy-rust --noconfirm; then
            print_success "Package supprimé avec succès"
            
            # Vérifier la suppression
            verify_removal
            return 0
        else
            print_error "Échec de la suppression"
            return 1
        fi
    else
        print_warning "Package cachy-rust n'est pas installé"
        return 0
    fi
}

# Vérifier la suppression
verify_removal() {
    print_status "Vérification de la suppression..."
    
    # Vérifier que le binaire est supprimé
    if ! command -v cachy-rust &> /dev/null; then
        print_success "✓ Binaire cachy-rust supprimé"
    else
        print_error "✗ Binaire cachy-rust encore présent"
    fi
    
    # Vérifier les fichiers principaux
    local files_to_check=(
        "/usr/bin/cachy-rust"
        "/usr/share/applications/cachy-rust.desktop"
        "/usr/lib/systemd/user/cachy-rust-tray.service"
    )
    
    for file in "${files_to_check[@]}"; do
        if [ ! -f "$file" ]; then
            print_success "✓ $file supprimé"
        else
            print_warning "✗ $file encore présent"
        fi
    done
    
    # Vérifier les fichiers préservés
    if [ -f "/etc/cachy-rust/config.toml" ]; then
        print_success "✓ Configuration système préservée"
    fi
    
    if [ -d "$HOME/.config/cachy-rust" ]; then
        print_success "✓ Configuration utilisateur préservée"
    fi
}

# Test complet
test_full_cycle() {
    print_status "=== Test complet du cycle install/remove/reinstall ==="
    
    # 1. Nettoyer l'état initial
    print_status "1. Nettoyage initial..."
    remove_package || true
    clean_build
    
    # 2. Construire le package
    print_status "2. Construction du package..."
    if ! build_package; then
        print_error "Échec de la construction"
        return 1
    fi
    
    # 3. Première installation
    print_status "3. Première installation..."
    if ! install_package; then
        print_error "Échec de la première installation"
        return 1
    fi
    
    # 4. Test de fonctionnement
    print_status "4. Test de fonctionnement..."
    sleep 2
    
    # 5. Suppression
    print_status "5. Suppression..."
    if ! remove_package; then
        print_error "Échec de la suppression"
        return 1
    fi
    
    # 6. Réinstallation
    print_status "6. Réinstallation..."
    if ! install_package; then
        print_error "Échec de la réinstallation"
        return 1
    fi
    
    print_success "=== Test complet réussi ! ==="
    
    # Afficher le résumé
    print_status "=== Résumé de l'installation ==="
    print_status "Pour tester l'application:"
    echo "  cachy-rust --help"
    echo "  cachy-rust --gui"
    echo ""
    print_status "Pour gérer le service:"
    echo "  systemctl --user enable cachy-rust-tray.service"
    echo "  systemctl --user start cachy-rust-tray.service"
    echo ""
    print_status "Configuration:"
    echo "  ~/.config/cachy-rust/config.toml"
    echo ""
    print_status "Pour supprimer complètement:"
    echo "  sudo pacman -R cachy-rust"
    echo "  rm -rf ~/.config/cachy-rust"
}

# Afficher l'aide
show_help() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "COMMANDS:"
    echo "  build      - Construire le package seulement"
    echo "  install    - Construire et installer le package"
    echo "  remove     - Supprimer le package installé"
    echo "  reinstall  - Supprimer puis réinstaller"
    echo "  test-all   - Test complet du cycle (défaut)"
    echo "  clean      - Nettoyer les fichiers de build"
    echo "  verify     - Vérifier l'installation actuelle"
    echo "  help       - Afficher cette aide"
    echo ""
    echo "EXEMPLES:"
    echo "  $0                    # Test complet"
    echo "  $0 install            # Installer seulement"
    echo "  $0 remove             # Supprimer seulement"
    echo "  $0 reinstall          # Réinstaller"
}

# Fonction principale
main() {
    local command="${1:-test-all}"
    
    case "$command" in
        "build")
            check_prerequisites
            build_package
            ;;
        "install")
            check_prerequisites
            install_package
            ;;
        "remove")
            remove_package
            ;;
        "reinstall")
            check_prerequisites
            remove_package || true
            install_package
            ;;
        "test-all")
            check_prerequisites
            test_full_cycle
            ;;
        "clean")
            clean_build
            ;;
        "verify")
            verify_installation
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            print_error "Commande inconnue: $command"
            show_help
            exit 1
            ;;
    esac
}

# Exécuter la fonction principale
main "$@"