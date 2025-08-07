# Deutsche Übersetzungsdatei für CachyPac
# Fluent-Format - https://projectfluent.org/

## Hauptoberfläche
app-title = CachyPac - Intelligenter Paketmanager
app-subtitle = Pacman-Update-Automatisierung mit künstlicher Intelligenz

## Hauptmenü
menu-updates = Updates
menu-packages = Pakete
menu-history = Verlauf
menu-settings = Einstellungen
menu-about = Über

## Updates-Tab
updates-title = Verfügbare Updates
updates-check = Nach Updates suchen
updates-checking = Überprüfung läuft...
updates-no-updates = Keine Updates verfügbar
updates-count = { $count ->
    [0] Keine Updates
    [1] 1 Update verfügbar
    *[other] { $count } Updates verfügbar
}
updates-install-all = Alle Updates installieren
updates-install-selected = Auswahl installieren
updates-package-name = Paketname
updates-current-version = Aktuelle Version
updates-new-version = Neue Version
updates-size = Größe
updates-description = Beschreibung

## Pakete-Tab
packages-title = Paketmanager
packages-search = Pakete suchen...
packages-search-placeholder = Paketnamen eingeben
packages-installed = Installiert
packages-available = Verfügbar
packages-install = Installieren
packages-remove = Entfernen
packages-upgrade = Aktualisieren
packages-info = Informationen
packages-dependencies = Abhängigkeiten
packages-files = Dateien

## Verlauf-Tab
history-title = Operationsverlauf
history-date = Datum
history-operation = Operation
history-packages = Pakete
history-status = Status
history-details = Details
history-clear = Verlauf löschen
history-export = Exportieren
history-filter = Nach Typ filtern
history-filter-all = Alle Operationen
history-filter-install = Installationen
history-filter-remove = Entfernungen
history-filter-update = Updates

## Einstellungen-Tab
settings-title = Einstellungen
settings-general = Allgemein
settings-updates = Updates
settings-notifications = Benachrichtigungen
settings-advanced = Erweitert
settings-language = Sprache
settings-theme = Design
settings-auto-update = Automatisches Update
settings-check-interval = Überprüfungsintervall
settings-telegram-bot = Telegram-Bot
settings-telegram-token = Bot-Token
settings-telegram-chat-id = Chat-ID
settings-backup = Sicherung
settings-restore = Wiederherstellen
settings-reset = Zurücksetzen
settings-save = Speichern
settings-cancel = Abbrechen

## Benachrichtigungen
notification-update-available = Updates verfügbar
notification-update-complete = Updates abgeschlossen
notification-install-complete = Installation abgeschlossen
notification-remove-complete = Entfernung abgeschlossen
notification-error = Fehler
notification-success = Erfolg
notification-warning = Warnung
notification-info = Information

## Fehlermeldungen
error-network = Netzwerkfehler: Verbindung zu Repositories nicht möglich
error-permission = Unzureichende Berechtigungen: Als Administrator ausführen
error-package-not-found = Paket nicht gefunden: { $package }
error-dependency = Abhängigkeitsfehler: { $dependency } erforderlich
error-disk-space = Unzureichender Speicherplatz
error-download = Download-Fehler: { $url }
error-install = Installationsfehler: { $package }
error-remove = Entfernungsfehler: { $package }
error-config = Konfigurationsfehler: { $file }
error-database = Datenbankfehler: { $message }

## Erfolgsmeldungen
success-install = { $package } erfolgreich installiert
success-remove = { $package } erfolgreich entfernt
success-update = { $count ->
    [1] 1 Paket aktualisiert
    *[other] { $count } Pakete aktualisiert
}
success-config-saved = Konfiguration gespeichert
success-backup-created = Sicherung erstellt: { $file }
success-restore-complete = Wiederherstellung abgeschlossen

## Bestätigungsdialoge
confirm-install = Möchten Sie { $package } installieren?
confirm-remove = Möchten Sie { $package } entfernen?
confirm-update-all = Möchten Sie alle Updates installieren?
confirm-clear-history = Möchten Sie den gesamten Verlauf löschen?
confirm-reset-settings = Möchten Sie alle Einstellungen zurücksetzen?

## Schaltflächen
button-yes = Ja
button-no = Nein
button-ok = OK
button-cancel = Abbrechen
button-apply = Anwenden
button-close = Schließen
button-refresh = Aktualisieren
button-back = Zurück
button-next = Weiter
button-finish = Fertig

## Zustände
status-idle = Bereit
status-checking = Überprüfung...
status-downloading = Herunterladen...
status-installing = Installieren...
status-removing = Entfernen...
status-updating = Aktualisieren...
status-complete = Abgeschlossen
status-failed = Fehlgeschlagen
status-cancelled = Abgebrochen

## Einheiten
unit-bytes = Bytes
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-seconds = Sekunden
unit-minutes = Minuten
unit-hours = Stunden

## Künstliche Intelligenz
ai-analyzing = Intelligente Analyse läuft...
ai-recommendation = KI-Empfehlung
ai-safe-to-update = Diese Updates sind sicher zu installieren
ai-caution-required = Vorsicht bei diesen Updates erforderlich
ai-critical-update = Kritisches Update erkannt
ai-dependency-conflict = Abhängigkeitskonflikt erkannt
ai-performance-impact = Leistungsauswirkung erwartet
ai-security-update = Wichtiges Sicherheitsupdate

## Planer
scheduler-title = Aufgabenplaner
scheduler-enable = Planung aktivieren
scheduler-daily = Täglich
scheduler-weekly = Wöchentlich
scheduler-monthly = Monatlich
scheduler-custom = Benutzerdefiniert
scheduler-time = Ausführungszeit
scheduler-next-run = Nächste Ausführung

## Telegram
telegram-title = Telegram-Integration
telegram-enable = Telegram-Benachrichtigungen aktivieren
telegram-test = Verbindung testen
telegram-connected = Mit Telegram verbunden
telegram-disconnected = Von Telegram getrennt
telegram-message-sent = Nachricht gesendet
telegram-message-failed = Senden fehlgeschlagen

## Über
about-title = Über CachyPac
about-version = Version { $version }
about-description = Intelligenter Paketmanager für Arch Linux mit KI-Automatisierung
about-author = Entwickelt vom CachyPac-Team
about-license = Apache 2.0 Lizenz
about-website = Website
about-github = Quellcode
about-support = Support

## Tastenkürzel
shortcut-refresh = Strg+R
shortcut-search = Strg+F
shortcut-settings = Strg+,
shortcut-quit = Strg+Q
shortcut-help = F1

## Tooltips
tooltip-refresh = Paketliste aktualisieren
tooltip-search = In Paketen suchen
tooltip-install = Ausgewähltes Paket installieren
tooltip-remove = Ausgewähltes Paket entfernen
tooltip-info = Paketinformationen anzeigen
tooltip-settings = Einstellungen öffnen