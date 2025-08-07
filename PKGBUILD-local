# Maintainer: CachyRust Team <cachy-rust@example.com>
pkgname=cachy-rust
pkgver=0.1.0
pkgrel=1
pkgdesc="Modern Pacman update manager with GUI - Gestionnaire de mises à jour Pacman moderne avec interface graphique"
arch=('x86_64')
url="https://github.com/cachyos/cachy-rust"
license=('MIT')
depends=('pacman' 'sudo' 'gtk3' 'cairo' 'pango' 'gdk-pixbuf2' 'glib2')
makedepends=('rust' 'cargo' 'git' 'imagemagick')
optdepends=(
    'telegram-desktop: For Telegram notifications setup'
    'systemd: For system service integration'
)
provides=('cachy-rust')
conflicts=('cachy-rust-git')
backup=('etc/cachy-rust/config.toml')
# Pour les tests locaux, on utilise le répertoire courant
source=()
sha256sums=()

prepare() {
    # Pour les tests locaux, on copie les sources depuis le répertoire parent
    if [ ! -d "$srcdir/cachy-rust-$pkgver" ]; then
        mkdir -p "$srcdir/cachy-rust-$pkgver"
        cp -r "$startdir"/* "$srcdir/cachy-rust-$pkgver/" 2>/dev/null || true
        # Exclure les fichiers de build
        rm -rf "$srcdir/cachy-rust-$pkgver"/{target,pkg,src,*.pkg.tar.*,*.log} 2>/dev/null || true
    fi
    
    cd "$srcdir/cachy-rust-$pkgver"
    
    # Vérifier que Rust est installé
    if ! command -v cargo &> /dev/null; then
        echo "Error: Rust/Cargo is required to build this package"
        exit 1
    fi
    
    # Mettre à jour le registre Cargo
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu" || cargo fetch --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$srcdir/cachy-rust-$pkgver"
    
    # Variables d'environnement pour la compilation optimisée
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    
    # Compilation en mode release avec optimisations
    cargo build --release --all-features
    
    # Générer les icônes de différentes tailles si ImageMagick est disponible
    if command -v convert &> /dev/null; then
        mkdir -p icons
        if [ -f "resources/cachy-rust.svg" ]; then
            for size in 16 32 48 64 128 256; do
                convert "resources/cachy-rust.svg" -resize ${size}x${size} "icons/cachy-rust-${size}.png"
            done
        fi
    fi
}

check() {
    cd "$srcdir/cachy-rust-$pkgver"
    
    # Exécuter les tests unitaires
    cargo test --release
}

package() {
    cd "$srcdir/cachy-rust-$pkgver"
    
    # Installer l'exécutable principal
    install -Dm755 target/release/cachy-rust "$pkgdir/usr/bin/cachy-rust"
    
    # Installer le script de lancement (supprime les warnings graphiques)
    if [ -f "resources/cachy-rust-launcher.sh" ]; then
        install -Dm755 resources/cachy-rust-launcher.sh "$pkgdir/usr/bin/cachy-rust-launcher"
    fi
    
    # Installer le script de gestion du system tray
    if [ -f "resources/cachy-rust-tray.sh" ]; then
        install -Dm755 resources/cachy-rust-tray.sh "$pkgdir/usr/bin/cachy-rust-tray"
    fi
    
    # Installer les fichiers .desktop
    install -Dm644 resources/cachy-rust.desktop "$pkgdir/usr/share/applications/cachy-rust.desktop"
    
    if [ -f "resources/cachy-rust-autostart.desktop" ]; then
        install -Dm644 resources/cachy-rust-autostart.desktop "$pkgdir/etc/xdg/autostart/cachy-rust-autostart.desktop"
    fi
    
    # Installer les icônes
    if [ -d "icons" ]; then
        for size in 16 32 48 64 128 256; do
            if [ -f "icons/cachy-rust-${size}.png" ]; then
                install -Dm644 "icons/cachy-rust-${size}.png" \
                    "$pkgdir/usr/share/icons/hicolor/${size}x${size}/apps/cachy-rust.png"
            fi
        done
    fi
    
    # Icône principale pour les thèmes qui ne supportent pas hicolor
    if [ -f "resources/cachy-rust.png" ]; then
        install -Dm644 resources/cachy-rust.png "$pkgdir/usr/share/pixmaps/cachy-rust.png"
    elif [ -f "icons/cachy-rust-128.png" ]; then
        install -Dm644 icons/cachy-rust-128.png "$pkgdir/usr/share/pixmaps/cachy-rust.png"
    fi
    
    # Installer les services systemd
    if [ -f "resources/cachy-rust-tray.service" ]; then
        install -Dm644 resources/cachy-rust-tray.service "$pkgdir/usr/lib/systemd/user/cachy-rust-tray.service"
    fi
    
    # Installer la configuration par défaut
    install -Dm644 config.example.toml "$pkgdir/etc/cachy-rust/config.toml"
    
    # Installer la documentation
    install -Dm644 README.md "$pkgdir/usr/share/doc/cachy-rust/README.md"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/cachy-rust/LICENSE"
    
    # Installer la documentation d'internationalisation
    if [ -f "docs/INTERNATIONALIZATION.md" ]; then
        install -Dm644 docs/INTERNATIONALIZATION.md "$pkgdir/usr/share/doc/cachy-rust/INTERNATIONALIZATION.md"
    fi
    
    # Installer les scripts d'installation/désinstallation
    if [ -f "install.sh" ]; then
        install -Dm755 install.sh "$pkgdir/usr/share/cachy-rust/install.sh"
    fi
    
    if [ -f "uninstall.sh" ]; then
        install -Dm755 uninstall.sh "$pkgdir/usr/share/cachy-rust/uninstall.sh"
    fi
    
    # Créer les répertoires de configuration utilisateur
    install -dm755 "$pkgdir/etc/skel/.config/cachy-rust"
    
    # Installer les règles sudo optionnelles
    echo "# Optional: Allow cachy-rust to run pacman without password" > cachy-rust.sudoers
    echo "# Uncomment the following line to enable:" >> cachy-rust.sudoers
    echo "# %wheel ALL=(ALL) NOPASSWD: /usr/bin/pacman" >> cachy-rust.sudoers
    install -Dm644 cachy-rust.sudoers "$pkgdir/usr/share/doc/cachy-rust/cachy-rust.sudoers.example"
}

# Fonctions post-installation
post_install() {
    echo "==> CachyRust has been installed successfully!"
    echo ""
    echo "Getting started:"
    echo "  1. Run 'cachy-rust' to launch the GUI"
    echo "  2. Configure Telegram notifications (optional) in the Configuration tab"
    echo "  3. Set up automatic updates in the Scheduler section"
    echo ""
    echo "System integration:"
    echo "  - Desktop entry: Applications > System > CachyRust"
    echo "  - System tray: Run 'cachy-rust-tray start' or enable autostart"
    echo "  - Service: 'systemctl --user enable cachy-rust-tray.service'"
    echo ""
    echo "Configuration:"
    echo "  - User config: ~/.config/cachy-rust/config.toml"
    echo "  - System config: /etc/cachy-rust/config.toml"
    echo ""
    echo "For sudo-less pacman execution:"
    echo "  sudo cp /usr/share/doc/cachy-rust/cachy-rust.sudoers.example /etc/sudoers.d/cachy-rust"
    echo "  sudo visudo -c  # Verify syntax"
    echo ""
    echo "Documentation: /usr/share/doc/cachy-rust/"
    echo ""
    echo "⚠️  IMPORTANT: This tool modifies your system packages."
    echo "   Test in a safe environment before enabling automatic updates."
    
    # Mettre à jour le cache des icônes
    if command -v gtk-update-icon-cache &> /dev/null; then
        gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor
    fi
    
    # Mettre à jour la base de données des applications
    if command -v update-desktop-database &> /dev/null; then
        update-desktop-database -q /usr/share/applications
    fi
}

post_upgrade() {
    post_install
    echo ""
    echo "==> CachyRust has been upgraded to version $pkgver"
    echo "    Check the changelog for new features and breaking changes."
    echo "    Your configuration files have been preserved."
}

pre_remove() {
    echo "==> Stopping CachyRust services..."
    
    # Arrêter les services utilisateur s'ils sont actifs
    if systemctl --user is-active cachy-rust-tray.service &> /dev/null; then
        systemctl --user stop cachy-rust-tray.service
    fi
    
    if systemctl --user is-enabled cachy-rust-tray.service &> /dev/null; then
        systemctl --user disable cachy-rust-tray.service
    fi
    
    # Arrêter les processus en cours
    pkill -f cachy-rust || true
}

post_remove() {
    echo "==> CachyRust has been removed."
    echo ""
    echo "Configuration files preserved:"
    echo "  - ~/.config/cachy-rust/ (user configurations)"
    echo "  - /etc/cachy-rust/ (system configuration)"
    echo ""
    echo "To completely remove all data:"
    echo "  rm -rf ~/.config/cachy-rust"
    echo "  sudo rm -rf /etc/cachy-rust"
    echo ""
    echo "To remove sudo rules (if installed):"
    echo "  sudo rm -f /etc/sudoers.d/cachy-rust"
    
    # Nettoyer le cache des icônes
    if command -v gtk-update-icon-cache &> /dev/null; then
        gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor
    fi
    
    # Mettre à jour la base de données des applications
    if command -v update-desktop-database &> /dev/null; then
        update-desktop-database -q /usr/share/applications
    fi
}