#!/bin/bash

# CachyRust System Tray Manager
# Ce script gère l'indicateur système pour CachyRust

CACHY_RUST_BIN="${CACHY_RUST_BIN:-/usr/local/bin/cachy-rust}"

# Si le binaire n'existe pas, essayer le binaire de développement
if [ ! -x "$CACHY_RUST_BIN" ]; then
    if [ -x "./target/debug/cachy-rust" ]; then
        CACHY_RUST_BIN="./target/debug/cachy-rust"
    elif [ -x "./target/release/cachy-rust" ]; then
        CACHY_RUST_BIN="./target/release/cachy-rust"
    fi
fi
CONFIG_DIR="$HOME/.config/cachy-rust"
PID_FILE="$CONFIG_DIR/tray.pid"

# Créer le répertoire de configuration s'il n'existe pas
mkdir -p "$CONFIG_DIR"

# Fonction pour vérifier si le processus est en cours d'exécution
is_running() {
    if [ -f "$PID_FILE" ]; then
        local pid=$(cat "$PID_FILE")
        if ps -p "$pid" > /dev/null 2>&1; then
            return 0
        else
            rm -f "$PID_FILE"
            return 1
        fi
    fi
    return 1
}

# Fonction pour démarrer l'indicateur système
start_tray() {
    if is_running; then
        echo "CachyRust tray is already running (PID: $(cat $PID_FILE))"
        return 0
    fi
    
    echo "Starting CachyRust system tray..."
    
    # Vérifier que l'environnement graphique est disponible
    if [ -z "$DISPLAY" ] && [ -z "$WAYLAND_DISPLAY" ]; then
        echo "Error: No graphical environment detected"
        return 1
    fi
    
    # Démarrer en arrière-plan avec mode tray
    nohup "$CACHY_RUST_BIN" --tray > "$CONFIG_DIR/tray.log" 2>&1 &
    local pid=$!
    
    # Sauvegarder le PID
    echo "$pid" > "$PID_FILE"
    
    # Vérifier que le processus a démarré correctement
    sleep 2
    if ps -p "$pid" > /dev/null 2>&1; then
        echo "CachyRust tray started successfully (PID: $pid)"
        return 0
    else
        echo "Failed to start CachyRust tray"
        rm -f "$PID_FILE"
        return 1
    fi
}

# Fonction pour arrêter l'indicateur système
stop_tray() {
    if ! is_running; then
        echo "CachyRust tray is not running"
        return 0
    fi
    
    local pid=$(cat "$PID_FILE")
    echo "Stopping CachyRust tray (PID: $pid)..."
    
    kill "$pid"
    
    # Attendre que le processus se termine
    local count=0
    while ps -p "$pid" > /dev/null 2>&1 && [ $count -lt 10 ]; do
        sleep 1
        count=$((count + 1))
    done
    
    if ps -p "$pid" > /dev/null 2>&1; then
        echo "Force killing CachyRust tray..."
        kill -9 "$pid"
    fi
    
    rm -f "$PID_FILE"
    echo "CachyRust tray stopped"
}

# Fonction pour redémarrer l'indicateur système
restart_tray() {
    stop_tray
    sleep 1
    start_tray
}

# Fonction pour afficher le statut
status_tray() {
    if is_running; then
        local pid=$(cat "$PID_FILE")
        echo "CachyRust tray is running (PID: $pid)"
        return 0
    else
        echo "CachyRust tray is not running"
        return 1
    fi
}

# Fonction d'aide
show_help() {
    echo "Usage: $0 {start|stop|restart|status|help}"
    echo ""
    echo "Commands:"
    echo "  start    - Start the CachyRust system tray"
    echo "  stop     - Stop the CachyRust system tray"
    echo "  restart  - Restart the CachyRust system tray"
    echo "  status   - Show the status of the system tray"
    echo "  help     - Show this help message"
    echo ""
    echo "The system tray provides:"
    echo "  - Quick access to CachyRust from the system tray"
    echo "  - Real-time update notifications"
    echo "  - System status monitoring"
    echo "  - Easy configuration access"
}

# Traitement des arguments
case "${1:-}" in
    start)
        start_tray
        ;;
    stop)
        stop_tray
        ;;
    restart)
        restart_tray
        ;;
    status)
        status_tray
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo "Error: Unknown command '${1:-}'"
        echo ""
        show_help
        exit 1
        ;;
esac