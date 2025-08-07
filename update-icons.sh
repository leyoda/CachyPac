#!/bin/bash

# Script pour mettre à jour le cache des icônes système
# À exécuter après l'installation pour que l'icône apparaisse dans la barre des tâches KDE

echo "Mise à jour du cache des icônes système..."

# Mettre à jour le cache des icônes hicolor
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    echo "Mise à jour du cache GTK..."
    gtk-update-icon-cache -f -t /usr/share/icons/hicolor/ 2>/dev/null || true
    gtk-update-icon-cache -f -t ~/.local/share/icons/hicolor/ 2>/dev/null || true
fi

# Mettre à jour la base de données des applications
if command -v update-desktop-database >/dev/null 2>&1; then
    echo "Mise à jour de la base de données des applications..."
    update-desktop-database ~/.local/share/applications/ 2>/dev/null || true
    update-desktop-database /usr/share/applications/ 2>/dev/null || true
fi

# Forcer le rafraîchissement de KDE
if command -v kbuildsycoca5 >/dev/null 2>&1; then
    echo "Rafraîchissement du cache KDE..."
    kbuildsycoca5 --noincremental 2>/dev/null || true
fi

# Redémarrer plasmashell pour forcer la prise en compte
if pgrep -x "plasmashell" > /dev/null; then
    echo "Redémarrage de plasmashell pour appliquer les changements..."
    killall plasmashell 2>/dev/null || true
    sleep 2
    plasmashell &
    disown
fi

echo "Cache des icônes mis à jour. L'icône devrait maintenant apparaître dans la barre des tâches."
echo "Si l'icône n'apparaît pas, redémarrez votre session ou votre système."