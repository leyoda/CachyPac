# Fichier de traduction français pour CachyPac
# Format Fluent - https://projectfluent.org/

## Interface principale
app-title = CachyPac - Gestionnaire de Paquets Intelligent
app-subtitle = Automatisation des mises à jour Pacman avec intelligence artificielle

## Menu principal
menu-updates = Mises à jour
menu-packages = Paquets
menu-history = Historique
menu-settings = Paramètres
menu-about = À propos

## Onglet Mises à jour
updates-title = Mises à jour disponibles
updates-check = Vérifier les mises à jour
updates-checking = Vérification en cours...
updates-no-updates = Aucune mise à jour disponible
updates-count = { $count ->
    [0] Aucune mise à jour
    [1] 1 mise à jour disponible
    *[other] { $count } mises à jour disponibles
}
updates-install-all = Installer toutes les mises à jour
updates-install-selected = Installer la sélection
updates-package-name = Nom du paquet
updates-current-version = Version actuelle
updates-new-version = Nouvelle version
updates-size = Taille
updates-description = Description

## Onglet Paquets
packages-title = Gestionnaire de paquets
packages-search = Rechercher des paquets...
packages-search-placeholder = Tapez le nom d'un paquet
packages-installed = Installés
packages-available = Disponibles
packages-install = Installer
packages-remove = Supprimer
packages-upgrade = Mettre à jour
packages-info = Informations
packages-dependencies = Dépendances
packages-files = Fichiers

## Onglet Historique
history-title = Historique des opérations
history-date = Date
history-operation = Opération
history-packages = Paquets
history-status = Statut
history-details = Détails
history-clear = Effacer l'historique
history-export = Exporter
history-filter = Filtrer par type
history-filter-all = Toutes les opérations
history-filter-install = Installations
history-filter-remove = Suppressions
history-filter-update = Mises à jour

## Onglet Paramètres
settings-title = Paramètres
settings-general = Général
settings-updates = Mises à jour
settings-notifications = Notifications
settings-advanced = Avancé
settings-language = Langue
settings-theme = Thème
settings-auto-update = Mise à jour automatique
settings-check-interval = Intervalle de vérification
settings-telegram-bot = Bot Telegram
settings-telegram-token = Token du bot
settings-telegram-chat-id = ID du chat
settings-backup = Sauvegarde
settings-restore = Restauration
settings-reset = Réinitialiser
settings-save = Enregistrer
settings-cancel = Annuler

## Notifications
notification-update-available = Mises à jour disponibles
notification-update-complete = Mises à jour terminées
notification-install-complete = Installation terminée
notification-remove-complete = Suppression terminée
notification-error = Erreur
notification-success = Succès
notification-warning = Avertissement
notification-info = Information

## Messages d'erreur
error-network = Erreur réseau : impossible de se connecter aux dépôts
error-permission = Permissions insuffisantes : exécutez en tant qu'administrateur
error-package-not-found = Paquet non trouvé : { $package }
error-dependency = Erreur de dépendance : { $dependency } requis
error-disk-space = Espace disque insuffisant
error-download = Erreur de téléchargement : { $url }
error-install = Erreur d'installation : { $package }
error-remove = Erreur de suppression : { $package }
error-config = Erreur de configuration : { $file }
error-database = Erreur de base de données : { $message }

## Messages de succès
success-install = { $package } installé avec succès
success-remove = { $package } supprimé avec succès
success-update = { $count ->
    [1] 1 paquet mis à jour
    *[other] { $count } paquets mis à jour
}
success-config-saved = Configuration sauvegardée
success-backup-created = Sauvegarde créée : { $file }
success-restore-complete = Restauration terminée

## Dialogues de confirmation
confirm-install = Voulez-vous installer { $package } ?
confirm-remove = Voulez-vous supprimer { $package } ?
confirm-update-all = Voulez-vous installer toutes les mises à jour ?
confirm-clear-history = Voulez-vous effacer tout l'historique ?
confirm-reset-settings = Voulez-vous réinitialiser tous les paramètres ?

## Boutons
button-yes = Oui
button-no = Non
button-ok = OK
button-cancel = Annuler
button-apply = Appliquer
button-close = Fermer
button-refresh = Actualiser
button-back = Retour
button-next = Suivant
button-finish = Terminer

## États
status-idle = En attente
status-checking = Vérification...
status-downloading = Téléchargement...
status-installing = Installation...
status-removing = Suppression...
status-updating = Mise à jour...
status-complete = Terminé
status-failed = Échec
status-cancelled = Annulé

## Unités
unit-bytes = octets
unit-kb = Ko
unit-mb = Mo
unit-gb = Go
unit-seconds = secondes
unit-minutes = minutes
unit-hours = heures

## Intelligence artificielle
ai-analyzing = Analyse intelligente en cours...
ai-recommendation = Recommandation IA
ai-safe-to-update = Ces mises à jour sont sûres à installer
ai-caution-required = Attention requise pour ces mises à jour
ai-critical-update = Mise à jour critique détectée
ai-dependency-conflict = Conflit de dépendances détecté
ai-performance-impact = Impact sur les performances prévu
ai-security-update = Mise à jour de sécurité importante

## Planificateur
scheduler-title = Planificateur de tâches
scheduler-enable = Activer la planification
scheduler-daily = Quotidien
scheduler-weekly = Hebdomadaire
scheduler-monthly = Mensuel
scheduler-custom = Personnalisé
scheduler-time = Heure d'exécution
scheduler-next-run = Prochaine exécution

## Telegram
telegram-title = Intégration Telegram
telegram-enable = Activer les notifications Telegram
telegram-test = Tester la connexion
telegram-connected = Connecté à Telegram
telegram-disconnected = Déconnecté de Telegram
telegram-message-sent = Message envoyé
telegram-message-failed = Échec de l'envoi

## À propos
about-title = À propos de CachyPac
about-version = Version { $version }
about-description = Gestionnaire de paquets intelligent pour Arch Linux avec automatisation IA
about-author = Développé par l'équipe CachyPac
about-license = Licence Apache 2.0
about-website = Site web
about-github = Code source
about-support = Support

## Raccourcis clavier
shortcut-refresh = Ctrl+R
shortcut-search = Ctrl+F
shortcut-settings = Ctrl+,
shortcut-quit = Ctrl+Q
shortcut-help = F1

## Tooltips
tooltip-refresh = Actualiser la liste des paquets
tooltip-search = Rechercher dans les paquets
tooltip-install = Installer le paquet sélectionné
tooltip-remove = Supprimer le paquet sélectionné
tooltip-info = Afficher les informations du paquet
tooltip-settings = Ouvrir les paramètres