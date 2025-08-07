#!/bin/bash

# Script de gestion du service systemd CachyRust
# Usage: ./manage-service.sh [install|uninstall|status|start|stop|restart]

SERVICE_NAME="cachy-rust"
SERVICE_FILE="$HOME/.config/systemd/user/$SERVICE_NAME.service"
BINARY_PATH="$(pwd)/target/release/cachy-rust"

create_service_file() {
    mkdir -p "$HOME/.config/systemd/user"
    
    cat > "$SERVICE_FILE" << EOF
[Unit]
Description=CachyRust Update Manager
After=network.target
Wants=network.target

[Service]
Type=simple
ExecStart=$BINARY_PATH
Restart=always
RestartSec=10
Environment=DISPLAY=:0
Environment=WAYLAND_DISPLAY=wayland-0

[Install]
WantedBy=default.target
EOF
}

case "$1" in
    install)
        echo "🔧 Installation du service systemd..."
        create_service_file
        systemctl --user daemon-reload
        systemctl --user enable "$SERVICE_NAME.service"
        echo "✅ Service installé et activé pour démarrage automatique"
        echo "   Pour démarrer maintenant: ./manage-service.sh start"
        ;;
    
    uninstall)
        echo "🗑️ Désinstallation du service systemd..."
        systemctl --user stop "$SERVICE_NAME.service" 2>/dev/null
        systemctl --user disable "$SERVICE_NAME.service" 2>/dev/null
        rm -f "$SERVICE_FILE"
        systemctl --user daemon-reload
        echo "✅ Service désinstallé"
        ;;
    
    status)
        if systemctl --user is-enabled "$SERVICE_NAME.service" >/dev/null 2>&1; then
            echo "📊 Statut du service:"
            systemctl --user status "$SERVICE_NAME.service" --no-pager
        else
            echo "❌ Service non installé"
            exit 1
        fi
        ;;
    
    start)
        echo "▶️ Démarrage du service..."
        systemctl --user start "$SERVICE_NAME.service"
        echo "✅ Service démarré"
        ;;
    
    stop)
        echo "⏹️ Arrêt du service..."
        systemctl --user stop "$SERVICE_NAME.service"
        echo "✅ Service arrêté"
        ;;
    
    restart)
        echo "🔄 Redémarrage du service..."
        systemctl --user restart "$SERVICE_NAME.service"
        echo "✅ Service redémarré"
        ;;
    
    *)
        echo "Usage: $0 {install|uninstall|status|start|stop|restart}"
        echo ""
        echo "Commandes disponibles:"
        echo "  install   - Installe et active le service pour démarrage automatique"
        echo "  uninstall - Désinstalle le service"
        echo "  status    - Affiche le statut du service"
        echo "  start     - Démarre le service"
        echo "  stop      - Arrête le service"
        echo "  restart   - Redémarre le service"
        exit 1
        ;;
esac