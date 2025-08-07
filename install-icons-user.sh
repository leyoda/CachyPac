#!/bin/bash

# Script d'installation des icônes CachyRust (version utilisateur)
# Ce script installe les icônes dans le répertoire utilisateur

echo "🚀 Installation des icônes CachyRust (version utilisateur)..."

# Créer les répertoires d'icônes utilisateur
USER_ICON_DIR="$HOME/.local/share/icons/hicolor"
mkdir -p "$USER_ICON_DIR"/{16x16,32x32,48x48,64x64,128x128}/apps

# Copier les nouveaux icônes modernes
echo "📁 Copie des icônes modernes..."
cp resources/cachy-rust-modern-16x16.png "$USER_ICON_DIR/16x16/apps/cachy-rust.png"
cp resources/cachy-rust-modern-32x32.png "$USER_ICON_DIR/32x32/apps/cachy-rust.png"
cp resources/cachy-rust-modern-48x48.png "$USER_ICON_DIR/48x48/apps/cachy-rust.png"
cp resources/cachy-rust-modern-64x64.png "$USER_ICON_DIR/64x64/apps/cachy-rust.png"
cp resources/cachy-rust-modern-128x128.png "$USER_ICON_DIR/128x128/apps/cachy-rust.png"

# Installer le fichier .desktop dans le répertoire utilisateur
USER_APPS_DIR="$HOME/.local/share/applications"
mkdir -p "$USER_APPS_DIR"

echo "📋 Installation du fichier .desktop..."
cp cachy-rust.desktop "$USER_APPS_DIR/"

# Mettre à jour le cache des icônes utilisateur
echo "🔄 Mise à jour du cache des icônes..."
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t "$USER_ICON_DIR" 2>/dev/null || true
fi

# Mettre à jour la base de données des applications
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$USER_APPS_DIR" 2>/dev/null || true
fi

# Notification KDE si disponible
if command -v kbuildsycoca5 >/dev/null 2>&1; then
    echo "🔄 Mise à jour du cache KDE..."
    kbuildsycoca5 2>/dev/null || true
fi

echo "✅ Installation terminée !"
echo "📍 Icônes installés dans: $USER_ICON_DIR"
echo "📍 Application installée dans: $USER_APPS_DIR"
echo ""
echo "🎉 CachyRust est maintenant disponible dans le menu des applications !"
echo "💡 Vous pouvez également lancer l'application avec: ./resources/cachy-rust-launcher.sh"