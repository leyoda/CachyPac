# Archivo de traducción español para CachyPac
# Formato Fluent - https://projectfluent.org/

## Interfaz principal
app-title = CachyPac - Gestor de Paquetes Inteligente
app-subtitle = Automatización de actualizaciones Pacman con inteligencia artificial

## Menú principal
menu-updates = Actualizaciones
menu-packages = Paquetes
menu-history = Historial
menu-settings = Configuración
menu-about = Acerca de

## Pestaña Actualizaciones
updates-title = Actualizaciones disponibles
updates-check = Buscar actualizaciones
updates-checking = Verificando...
updates-no-updates = No hay actualizaciones disponibles
updates-count = { $count ->
    [0] Sin actualizaciones
    [1] 1 actualización disponible
    *[other] { $count } actualizaciones disponibles
}
updates-install-all = Instalar todas las actualizaciones
updates-install-selected = Instalar selección
updates-package-name = Nombre del paquete
updates-current-version = Versión actual
updates-new-version = Nueva versión
updates-size = Tamaño
updates-description = Descripción

## Pestaña Paquetes
packages-title = Gestor de paquetes
packages-search = Buscar paquetes...
packages-search-placeholder = Escriba el nombre de un paquete
packages-installed = Instalados
packages-available = Disponibles
packages-install = Instalar
packages-remove = Eliminar
packages-upgrade = Actualizar
packages-info = Información
packages-dependencies = Dependencias
packages-files = Archivos

## Pestaña Historial
history-title = Historial de operaciones
history-date = Fecha
history-operation = Operación
history-packages = Paquetes
history-status = Estado
history-details = Detalles
history-clear = Limpiar historial
history-export = Exportar
history-filter = Filtrar por tipo
history-filter-all = Todas las operaciones
history-filter-install = Instalaciones
history-filter-remove = Eliminaciones
history-filter-update = Actualizaciones

## Pestaña Configuración
settings-title = Configuración
settings-general = General
settings-updates = Actualizaciones
settings-notifications = Notificaciones
settings-advanced = Avanzado
settings-language = Idioma
settings-theme = Tema
settings-auto-update = Actualización automática
settings-check-interval = Intervalo de verificación
settings-telegram-bot = Bot de Telegram
settings-telegram-token = Token del bot
settings-telegram-chat-id = ID del chat
settings-backup = Copia de seguridad
settings-restore = Restaurar
settings-reset = Restablecer
settings-save = Guardar
settings-cancel = Cancelar

## Notificaciones
notification-update-available = Actualizaciones disponibles
notification-update-complete = Actualizaciones completadas
notification-install-complete = Instalación completada
notification-remove-complete = Eliminación completada
notification-error = Error
notification-success = Éxito
notification-warning = Advertencia
notification-info = Información

## Mensajes de error
error-network = Error de red: no se puede conectar a los repositorios
error-permission = Permisos insuficientes: ejecutar como administrador
error-package-not-found = Paquete no encontrado: { $package }
error-dependency = Error de dependencia: { $dependency } requerido
error-disk-space = Espacio en disco insuficiente
error-download = Error de descarga: { $url }
error-install = Error de instalación: { $package }
error-remove = Error de eliminación: { $package }
error-config = Error de configuración: { $file }
error-database = Error de base de datos: { $message }

## Mensajes de éxito
success-install = { $package } instalado correctamente
success-remove = { $package } eliminado correctamente
success-update = { $count ->
    [1] 1 paquete actualizado
    *[other] { $count } paquetes actualizados
}
success-config-saved = Configuración guardada
success-backup-created = Copia de seguridad creada: { $file }
success-restore-complete = Restauración completada

## Diálogos de confirmación
confirm-install = ¿Desea instalar { $package }?
confirm-remove = ¿Desea eliminar { $package }?
confirm-update-all = ¿Desea instalar todas las actualizaciones?
confirm-clear-history = ¿Desea limpiar todo el historial?
confirm-reset-settings = ¿Desea restablecer toda la configuración?

## Botones
button-yes = Sí
button-no = No
button-ok = Aceptar
button-cancel = Cancelar
button-apply = Aplicar
button-close = Cerrar
button-refresh = Actualizar
button-back = Atrás
button-next = Siguiente
button-finish = Finalizar

## Estados
status-idle = Inactivo
status-checking = Verificando...
status-downloading = Descargando...
status-installing = Instalando...
status-removing = Eliminando...
status-updating = Actualizando...
status-complete = Completado
status-failed = Fallido
status-cancelled = Cancelado

## Unidades
unit-bytes = bytes
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-seconds = segundos
unit-minutes = minutos
unit-hours = horas

## Inteligencia artificial
ai-analyzing = Análisis inteligente en progreso...
ai-recommendation = Recomendación IA
ai-safe-to-update = Estas actualizaciones son seguras de instalar
ai-caution-required = Se requiere precaución para estas actualizaciones
ai-critical-update = Actualización crítica detectada
ai-dependency-conflict = Conflicto de dependencias detectado
ai-performance-impact = Impacto en el rendimiento esperado
ai-security-update = Actualización de seguridad importante

## Programador
scheduler-title = Programador de tareas
scheduler-enable = Habilitar programación
scheduler-daily = Diario
scheduler-weekly = Semanal
scheduler-monthly = Mensual
scheduler-custom = Personalizado
scheduler-time = Hora de ejecución
scheduler-next-run = Próxima ejecución

## Telegram
telegram-title = Integración con Telegram
telegram-enable = Habilitar notificaciones de Telegram
telegram-test = Probar conexión
telegram-connected = Conectado a Telegram
telegram-disconnected = Desconectado de Telegram
telegram-message-sent = Mensaje enviado
telegram-message-failed = Envío fallido

## Acerca de
about-title = Acerca de CachyPac
about-version = Versión { $version }
about-description = Gestor de paquetes inteligente para Arch Linux con automatización IA
about-author = Desarrollado por el equipo CachyPac
about-license = Licencia Apache 2.0
about-website = Sitio web
about-github = Código fuente
about-support = Soporte

## Atajos de teclado
shortcut-refresh = Ctrl+R
shortcut-search = Ctrl+F
shortcut-settings = Ctrl+,
shortcut-quit = Ctrl+Q
shortcut-help = F1

## Tooltips
tooltip-refresh = Actualizar lista de paquetes
tooltip-search = Buscar en paquetes
tooltip-install = Instalar paquete seleccionado
tooltip-remove = Eliminar paquete seleccionado
tooltip-info = Mostrar información del paquete
tooltip-settings = Abrir configuración