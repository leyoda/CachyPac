#!/bin/bash
# Script post-installation pour CachyPac
# Ce script configure l'environnement après l'installation du package AUR

set -e

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Fonction d'affichage avec couleurs
print_info() {
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

# Fonction pour créer l'utilisateur système
create_system_user() {
    print_info "Création de l'utilisateur système cachypac..."
    
    if ! id "cachypac" &>/dev/null; then
        useradd --system --home-dir /var/lib/cachypac --shell /usr/bin/nologin cachypac
        print_success "Utilisateur système 'cachypac' créé"
    else
        print_info "L'utilisateur système 'cachypac' existe déjà"
    fi
}

# Fonction pour configurer les permissions
setup_permissions() {
    print_info "Configuration des permissions..."
    
    # Répertoires de données
    chown -R cachypac:cachypac /var/lib/cachypac
    chmod 755 /var/lib/cachypac
    
    # Répertoires de logs
    chown -R cachypac:cachypac /var/log/cachypac
    chmod 755 /var/log/cachypac
    
    # Configuration
    chown -R root:cachypac /etc/cachypac
    chmod 750 /etc/cachypac
    chmod 640 /etc/cachypac/config.toml
    
    print_success "Permissions configurées"
}

# Fonction pour configurer sudo
setup_sudo() {
    print_info "Configuration des privilèges sudo pour pacman..."
    
    local sudoers_file="/etc/sudoers.d/cachypac"
    
    if [ ! -f "$sudoers_file" ]; then
        cat > "$sudoers_file" << 'EOF'
# Permissions pour CachyPac
# Permet à l'utilisateur cachypac d'exécuter pacman sans mot de passe
cachypac ALL=(root) NOPASSWD: /usr/bin/pacman
cachypac ALL=(root) NOPASSWD: /usr/bin/pacman-key
EOF
        chmod 440 "$sudoers_file"
        print_success "Privilèges sudo configurés"
    else
        print_info "Les privilèges sudo sont déjà configurés"
    fi
}

# Fonction pour créer la configuration par défaut
create_default_config() {
    print_info "Création de la configuration par défaut..."
    
    if [ ! -f "/etc/cachypac/config.toml" ]; then
        cp /etc/cachypac/config.toml.example /etc/cachypac/config.toml
        
        # Adapter les chemins pour l'installation système
        sed -i 's|~/.local/share/cachypac|/var/lib/cachypac|g' /etc/cachypac/config.toml
        
        print_success "Configuration par défaut créée"
    else
        print_info "La configuration existe déjà"
    fi
}

# Fonction pour configurer systemd
setup_systemd() {
    print_info "Configuration du service systemd..."
    
    # Recharger les services systemd
    systemctl daemon-reload
    
    print_success "Service systemd configuré"
    print_info "Pour activer le service au démarrage : sudo systemctl enable cachypac"
    print_info "Pour démarrer le service : sudo systemctl start cachypac"
}

# Fonction pour afficher les informations post-installation
show_post_install_info() {
    echo
    print_success "Installation de CachyPac terminée !"
    echo
    print_info "Prochaines étapes :"
    echo "  1. Configurer CachyPac : sudo nano /etc/cachypac/config.toml"
    echo "  2. Activer le service : sudo systemctl enable cachypac"
    echo "  3. Démarrer le service : sudo systemctl start cachypac"
    echo "  4. Vérifier le statut : sudo systemctl status cachypac"
    echo
    print_info "Pour l'interface graphique :"
    echo "  - Lancer : cachypac"
    echo "  - Ou depuis le menu applications"
    echo
    print_info "Documentation :"
    echo "  - Guide rapide : /usr/share/doc/cachypac/quick_start.md"
    echo "  - Guide complet : /usr/share/doc/cachypac/user_guide.md"
    echo "  - GitHub : https://github.com/leyoda/CachyPac"
    echo
    print_warning "Important : Configurez les notifications Telegram si souhaité"
    print_warning "Testez d'abord en mode manuel avant d'activer les mises à jour automatiques"
}

# Fonction principale
main() {
    print_info "Démarrage de la configuration post-installation de CachyPac..."
    
    # Vérifier les privilèges root
    if [[ $EUID -ne 0 ]]; then
        print_error "Ce script doit être exécuté en tant que root"
        exit 1
    fi
    
    # Exécuter les étapes de configuration
    create_system_user
    setup_permissions
    setup_sudo
    create_default_config
    setup_systemd
    
    # Afficher les informations finales
    show_post_install_info
    
    print_success "Configuration post-installation terminée avec succès !"
}

# Exécuter le script principal
main "$@"