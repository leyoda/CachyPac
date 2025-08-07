#!/bin/bash

echo "🔧 Installation du service CachyPac (Development)"
echo "================================================="

# Vérifier que l'exécutable existe
if [ ! -f "target/release/cachypac" ]; then
    echo "❌ Exécutable non trouvé. Compilez d'abord avec: cargo build --release"
    exit 1
fi

echo "✅ Exécutable trouvé: target/release/cachypac"

# Copier le fichier service
echo "📋 Installation du fichier service..."
sudo cp systemd/cachypac-dev.service /etc/systemd/system/cachypac.service

if [ $? -eq 0 ]; then
    echo "✅ Fichier service copié vers /etc/systemd/system/cachypac.service"
else
    echo "❌ Erreur lors de la copie du fichier service"
    exit 1
fi

# Recharger systemd
echo "🔄 Rechargement de systemd..."
sudo systemctl daemon-reload

if [ $? -eq 0 ]; then
    echo "✅ Systemd rechargé"
else
    echo "❌ Erreur lors du rechargement de systemd"
    exit 1
fi

# Activer le service
echo "🚀 Activation du service..."
sudo systemctl enable cachypac

if [ $? -eq 0 ]; then
    echo "✅ Service activé au démarrage"
else
    echo "❌ Erreur lors de l'activation du service"
    exit 1
fi

echo ""
echo "🎉 Installation terminée avec succès !"
echo ""
echo "📋 Commandes utiles :"
echo "  • Démarrer le service:    sudo systemctl start cachypac"
echo "  • Arrêter le service:     sudo systemctl stop cachypac"
echo "  • Statut du service:      sudo systemctl status cachypac"
echo "  • Logs du service:        sudo journalctl -u cachypac -f"
echo "  • Désactiver le service:  sudo systemctl disable cachypac"
echo ""
echo "⚠️  Pour démarrer le service maintenant:"
echo "   sudo systemctl start cachypac"