# English translation file for CachyPac
# Fluent format - https://projectfluent.org/

## Main interface
app-title = CachyPac - Intelligent Package Manager
app-subtitle = Pacman update automation with artificial intelligence

## Main menu
menu-updates = Updates
menu-packages = Packages
menu-history = History
menu-settings = Settings
menu-about = About

## Updates tab
updates-title = Available updates
updates-check = Check for updates
updates-checking = Checking...
updates-no-updates = No updates available
updates-count = { $count ->
    [0] No updates
    [1] 1 update available
    *[other] { $count } updates available
}
updates-install-all = Install all updates
updates-install-selected = Install selected
updates-package-name = Package name
updates-current-version = Current version
updates-new-version = New version
updates-size = Size
updates-description = Description

## Packages tab
packages-title = Package manager
packages-search = Search packages...
packages-search-placeholder = Type a package name
packages-installed = Installed
packages-available = Available
packages-install = Install
packages-remove = Remove
packages-upgrade = Upgrade
packages-info = Information
packages-dependencies = Dependencies
packages-files = Files

## History tab
history-title = Operation history
history-date = Date
history-operation = Operation
history-packages = Packages
history-status = Status
history-details = Details
history-clear = Clear history
history-export = Export
history-filter = Filter by type
history-filter-all = All operations
history-filter-install = Installations
history-filter-remove = Removals
history-filter-update = Updates

## Settings tab
settings-title = Settings
settings-general = General
settings-updates = Updates
settings-notifications = Notifications
settings-advanced = Advanced
settings-language = Language
settings-theme = Theme
settings-auto-update = Automatic update
settings-check-interval = Check interval
settings-telegram-bot = Telegram bot
settings-telegram-token = Bot token
settings-telegram-chat-id = Chat ID
settings-backup = Backup
settings-restore = Restore
settings-reset = Reset
settings-save = Save
settings-cancel = Cancel

## Notifications
notification-update-available = Updates available
notification-update-complete = Updates completed
notification-install-complete = Installation completed
notification-remove-complete = Removal completed
notification-error = Error
notification-success = Success
notification-warning = Warning
notification-info = Information

## Error messages
error-network = Network error: unable to connect to repositories
error-permission = Insufficient permissions: run as administrator
error-package-not-found = Package not found: { $package }
error-dependency = Dependency error: { $dependency } required
error-disk-space = Insufficient disk space
error-download = Download error: { $url }
error-install = Installation error: { $package }
error-remove = Removal error: { $package }
error-config = Configuration error: { $file }
error-database = Database error: { $message }

## Success messages
success-install = { $package } installed successfully
success-remove = { $package } removed successfully
success-update = { $count ->
    [1] 1 package updated
    *[other] { $count } packages updated
}
success-config-saved = Configuration saved
success-backup-created = Backup created: { $file }
success-restore-complete = Restore completed

## Confirmation dialogs
confirm-install = Do you want to install { $package }?
confirm-remove = Do you want to remove { $package }?
confirm-update-all = Do you want to install all updates?
confirm-clear-history = Do you want to clear all history?
confirm-reset-settings = Do you want to reset all settings?

## Buttons
button-yes = Yes
button-no = No
button-ok = OK
button-cancel = Cancel
button-apply = Apply
button-close = Close
button-refresh = Refresh
button-back = Back
button-next = Next
button-finish = Finish

## States
status-idle = Idle
status-checking = Checking...
status-downloading = Downloading...
status-installing = Installing...
status-removing = Removing...
status-updating = Updating...
status-complete = Complete
status-failed = Failed
status-cancelled = Cancelled

## Units
unit-bytes = bytes
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-seconds = seconds
unit-minutes = minutes
unit-hours = hours

## Artificial intelligence
ai-analyzing = Intelligent analysis in progress...
ai-recommendation = AI Recommendation
ai-safe-to-update = These updates are safe to install
ai-caution-required = Caution required for these updates
ai-critical-update = Critical update detected
ai-dependency-conflict = Dependency conflict detected
ai-performance-impact = Performance impact expected
ai-security-update = Important security update

## Scheduler
scheduler-title = Task scheduler
scheduler-enable = Enable scheduling
scheduler-daily = Daily
scheduler-weekly = Weekly
scheduler-monthly = Monthly
scheduler-custom = Custom
scheduler-time = Execution time
scheduler-next-run = Next run

## Telegram
telegram-title = Telegram integration
telegram-enable = Enable Telegram notifications
telegram-test = Test connection
telegram-connected = Connected to Telegram
telegram-disconnected = Disconnected from Telegram
telegram-message-sent = Message sent
telegram-message-failed = Send failed

## About
about-title = About CachyPac
about-version = Version { $version }
about-description = Intelligent package manager for Arch Linux with AI automation
about-author = Developed by the CachyPac team
about-license = Apache 2.0 License
about-website = Website
about-github = Source code
about-support = Support

## Keyboard shortcuts
shortcut-refresh = Ctrl+R
shortcut-search = Ctrl+F
shortcut-settings = Ctrl+,
shortcut-quit = Ctrl+Q
shortcut-help = F1

## Tooltips
tooltip-refresh = Refresh package list
tooltip-search = Search in packages
tooltip-install = Install selected package
tooltip-remove = Remove selected package
tooltip-info = Show package information
tooltip-settings = Open settings