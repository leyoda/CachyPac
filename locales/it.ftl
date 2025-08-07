# File di traduzione italiano per CachyPac
# Formato Fluent - https://projectfluent.org/

## Interfaccia principale
app-title = CachyPac - Gestore Pacchetti Intelligente
app-subtitle = Automazione aggiornamenti Pacman con intelligenza artificiale

## Menu principale
menu-updates = Aggiornamenti
menu-packages = Pacchetti
menu-history = Cronologia
menu-settings = Impostazioni
menu-about = Informazioni

## Scheda Aggiornamenti
updates-title = Aggiornamenti disponibili
updates-check = Controlla aggiornamenti
updates-checking = Controllo in corso...
updates-no-updates = Nessun aggiornamento disponibile
updates-count = { $count ->
    [0] Nessun aggiornamento
    [1] 1 aggiornamento disponibile
    *[other] { $count } aggiornamenti disponibili
}
updates-install-all = Installa tutti gli aggiornamenti
updates-install-selected = Installa selezione
updates-package-name = Nome pacchetto
updates-current-version = Versione attuale
updates-new-version = Nuova versione
updates-size = Dimensione
updates-description = Descrizione

## Scheda Pacchetti
packages-title = Gestore pacchetti
packages-search = Cerca pacchetti...
packages-search-placeholder = Digita il nome di un pacchetto
packages-installed = Installati
packages-available = Disponibili
packages-install = Installa
packages-remove = Rimuovi
packages-upgrade = Aggiorna
packages-info = Informazioni
packages-dependencies = Dipendenze
packages-files = File

## Scheda Cronologia
history-title = Cronologia operazioni
history-date = Data
history-operation = Operazione
history-packages = Pacchetti
history-status = Stato
history-details = Dettagli
history-clear = Cancella cronologia
history-export = Esporta
history-filter = Filtra per tipo
history-filter-all = Tutte le operazioni
history-filter-install = Installazioni
history-filter-remove = Rimozioni
history-filter-update = Aggiornamenti

## Scheda Impostazioni
settings-title = Impostazioni
settings-general = Generale
settings-updates = Aggiornamenti
settings-notifications = Notifiche
settings-advanced = Avanzate
settings-language = Lingua
settings-theme = Tema
settings-auto-update = Aggiornamento automatico
settings-check-interval = Intervallo controllo
settings-telegram-bot = Bot Telegram
settings-telegram-token = Token bot
settings-telegram-chat-id = ID chat
settings-backup = Backup
settings-restore = Ripristina
settings-reset = Reimposta
settings-save = Salva
settings-cancel = Annulla

## Notifiche
notification-update-available = Aggiornamenti disponibili
notification-update-complete = Aggiornamenti completati
notification-install-complete = Installazione completata
notification-remove-complete = Rimozione completata
notification-error = Errore
notification-success = Successo
notification-warning = Avviso
notification-info = Informazione

## Messaggi di errore
error-network = Errore di rete: impossibile connettersi ai repository
error-permission = Permessi insufficienti: eseguire come amministratore
error-package-not-found = Pacchetto non trovato: { $package }
error-dependency = Errore dipendenza: { $dependency } richiesto
error-disk-space = Spazio su disco insufficiente
error-download = Errore download: { $url }
error-install = Errore installazione: { $package }
error-remove = Errore rimozione: { $package }
error-config = Errore configurazione: { $file }
error-database = Errore database: { $message }

## Messaggi di successo
success-install = { $package } installato con successo
success-remove = { $package } rimosso con successo
success-update = { $count ->
    [1] 1 pacchetto aggiornato
    *[other] { $count } pacchetti aggiornati
}
success-config-saved = Configurazione salvata
success-backup-created = Backup creato: { $file }
success-restore-complete = Ripristino completato

## Dialoghi di conferma
confirm-install = Vuoi installare { $package }?
confirm-remove = Vuoi rimuovere { $package }?
confirm-update-all = Vuoi installare tutti gli aggiornamenti?
confirm-clear-history = Vuoi cancellare tutta la cronologia?
confirm-reset-settings = Vuoi reimpostare tutte le impostazioni?

## Pulsanti
button-yes = Sì
button-no = No
button-ok = OK
button-cancel = Annulla
button-apply = Applica
button-close = Chiudi
button-refresh = Aggiorna
button-back = Indietro
button-next = Avanti
button-finish = Fine

## Stati
status-idle = Inattivo
status-checking = Controllo...
status-downloading = Download...
status-installing = Installazione...
status-removing = Rimozione...
status-updating = Aggiornamento...
status-complete = Completato
status-failed = Fallito
status-cancelled = Annullato

## Unità
unit-bytes = byte
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-seconds = secondi
unit-minutes = minuti
unit-hours = ore

## Intelligenza artificiale
ai-analyzing = Analisi intelligente in corso...
ai-recommendation = Raccomandazione IA
ai-safe-to-update = Questi aggiornamenti sono sicuri da installare
ai-caution-required = Attenzione richiesta per questi aggiornamenti
ai-critical-update = Aggiornamento critico rilevato
ai-dependency-conflict = Conflitto dipendenze rilevato
ai-performance-impact = Impatto prestazioni previsto
ai-security-update = Importante aggiornamento sicurezza

## Pianificatore
scheduler-title = Pianificatore attività
scheduler-enable = Abilita pianificazione
scheduler-daily = Giornaliero
scheduler-weekly = Settimanale
scheduler-monthly = Mensile
scheduler-custom = Personalizzato
scheduler-time = Ora esecuzione
scheduler-next-run = Prossima esecuzione

## Telegram
telegram-title = Integrazione Telegram
telegram-enable = Abilita notifiche Telegram
telegram-test = Testa connessione
telegram-connected = Connesso a Telegram
telegram-disconnected = Disconnesso da Telegram
telegram-message-sent = Messaggio inviato
telegram-message-failed = Invio fallito

## Informazioni
about-title = Informazioni su CachyPac
about-version = Versione { $version }
about-description = Gestore pacchetti intelligente per Arch Linux con automazione IA
about-author = Sviluppato dal team CachyPac
about-license = Licenza Apache 2.0
about-website = Sito web
about-github = Codice sorgente
about-support = Supporto

## Scorciatoie tastiera
shortcut-refresh = Ctrl+R
shortcut-search = Ctrl+F
shortcut-settings = Ctrl+,
shortcut-quit = Ctrl+Q
shortcut-help = F1

## Tooltip
tooltip-refresh = Aggiorna lista pacchetti
tooltip-search = Cerca nei pacchetti
tooltip-install = Installa pacchetto selezionato
tooltip-remove = Rimuovi pacchetto selezionato
tooltip-info = Mostra informazioni pacchetto
tooltip-settings = Apri impostazioni