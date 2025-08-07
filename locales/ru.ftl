# Файл русского перевода для CachyPac
# Формат Fluent - https://projectfluent.org/

## Основной интерфейс
app-title = CachyPac - Умный менеджер пакетов
app-subtitle = Автоматизация обновлений Pacman с искусственным интеллектом

## Главное меню
menu-updates = Обновления
menu-packages = Пакеты
menu-history = История
menu-settings = Настройки
menu-about = О программе

## Вкладка Обновления
updates-title = Доступные обновления
updates-check = Проверить обновления
updates-checking = Проверка...
updates-no-updates = Нет доступных обновлений
updates-count = { $count ->
    [0] Нет обновлений
    [1] 1 доступное обновление
    [2] { $count } доступных обновления
    [3] { $count } доступных обновления
    [4] { $count } доступных обновления
    *[other] { $count } доступных обновлений
}
updates-install-all = Установить все обновления
updates-install-selected = Установить выбранные
updates-package-name = Имя пакета
updates-current-version = Текущая версия
updates-new-version = Новая версия
updates-size = Размер
updates-description = Описание

## Вкладка Пакеты
packages-title = Менеджер пакетов
packages-search = Поиск пакетов...
packages-search-placeholder = Введите имя пакета
packages-installed = Установленные
packages-available = Доступные
packages-install = Установить
packages-remove = Удалить
packages-upgrade = Обновить
packages-info = Информация
packages-dependencies = Зависимости
packages-files = Файлы

## Вкладка История
history-title = История операций
history-date = Дата
history-operation = Операция
history-packages = Пакеты
history-status = Статус
history-details = Подробности
history-clear = Очистить историю
history-export = Экспорт
history-filter = Фильтр по типу
history-filter-all = Все операции
history-filter-install = Установки
history-filter-remove = Удаления
history-filter-update = Обновления

## Вкладка Настройки
settings-title = Настройки
settings-general = Общие
settings-updates = Обновления
settings-notifications = Уведомления
settings-advanced = Дополнительно
settings-language = Язык
settings-theme = Тема
settings-auto-update = Автообновление
settings-check-interval = Интервал проверки
settings-telegram-bot = Telegram бот
settings-telegram-token = Токен бота
settings-telegram-chat-id = ID чата
settings-backup = Резервная копия
settings-restore = Восстановить
settings-reset = Сбросить
settings-save = Сохранить
settings-cancel = Отмена

## Уведомления
notification-update-available = Доступны обновления
notification-update-complete = Обновления завершены
notification-install-complete = Установка завершена
notification-remove-complete = Удаление завершено
notification-error = Ошибка
notification-success = Успех
notification-warning = Предупреждение
notification-info = Информация

## Сообщения об ошибках
error-network = Ошибка сети: невозможно подключиться к репозиториям
error-permission = Недостаточно прав: запустите от имени администратора
error-package-not-found = Пакет не найден: { $package }
error-dependency = Ошибка зависимости: требуется { $dependency }
error-disk-space = Недостаточно места на диске
error-download = Ошибка загрузки: { $url }
error-install = Ошибка установки: { $package }
error-remove = Ошибка удаления: { $package }
error-config = Ошибка конфигурации: { $file }
error-database = Ошибка базы данных: { $message }

## Сообщения об успехе
success-install = { $package } успешно установлен
success-remove = { $package } успешно удален
success-update = { $count ->
    [1] 1 пакет обновлен
    [2] { $count } пакета обновлено
    [3] { $count } пакета обновлено
    [4] { $count } пакета обновлено
    *[other] { $count } пакетов обновлено
}
success-config-saved = Конфигурация сохранена
success-backup-created = Резервная копия создана: { $file }
success-restore-complete = Восстановление завершено

## Диалоги подтверждения
confirm-install = Хотите установить { $package }?
confirm-remove = Хотите удалить { $package }?
confirm-update-all = Хотите установить все обновления?
confirm-clear-history = Хотите очистить всю историю?
confirm-reset-settings = Хотите сбросить все настройки?

## Кнопки
button-yes = Да
button-no = Нет
button-ok = ОК
button-cancel = Отмена
button-apply = Применить
button-close = Закрыть
button-refresh = Обновить
button-back = Назад
button-next = Далее
button-finish = Готово

## Состояния
status-idle = Ожидание
status-checking = Проверка...
status-downloading = Загрузка...
status-installing = Установка...
status-removing = Удаление...
status-updating = Обновление...
status-complete = Завершено
status-failed = Неудача
status-cancelled = Отменено

## Единицы измерения
unit-bytes = байт
unit-kb = КБ
unit-mb = МБ
unit-gb = ГБ
unit-seconds = секунд
unit-minutes = минут
unit-hours = часов

## Искусственный интеллект
ai-analyzing = Выполняется интеллектуальный анализ...
ai-recommendation = Рекомендация ИИ
ai-safe-to-update = Эти обновления безопасны для установки
ai-caution-required = Требуется осторожность с этими обновлениями
ai-critical-update = Обнаружено критическое обновление
ai-dependency-conflict = Обнаружен конфликт зависимостей
ai-performance-impact = Ожидается влияние на производительность
ai-security-update = Важное обновление безопасности

## Планировщик
scheduler-title = Планировщик задач
scheduler-enable = Включить планирование
scheduler-daily = Ежедневно
scheduler-weekly = Еженедельно
scheduler-monthly = Ежемесячно
scheduler-custom = Настраиваемый
scheduler-time = Время выполнения
scheduler-next-run = Следующий запуск

## Telegram
telegram-title = Интеграция с Telegram
telegram-enable = Включить уведомления Telegram
telegram-test = Проверить соединение
telegram-connected = Подключено к Telegram
telegram-disconnected = Отключено от Telegram
telegram-message-sent = Сообщение отправлено
telegram-message-failed = Отправка не удалась

## О программе
about-title = О CachyPac
about-version = Версия { $version }
about-description = Умный менеджер пакетов для Arch Linux с автоматизацией ИИ
about-author = Разработано командой CachyPac
about-license = Лицензия Apache 2.0
about-website = Веб-сайт
about-github = Исходный код
about-support = Поддержка

## Горячие клавиши
shortcut-refresh = Ctrl+R
shortcut-search = Ctrl+F
shortcut-settings = Ctrl+,
shortcut-quit = Ctrl+Q
shortcut-help = F1

## Подсказки
tooltip-refresh = Обновить список пакетов
tooltip-search = Поиск в пакетах
tooltip-install = Установить выбранный пакет
tooltip-remove = Удалить выбранный пакет
tooltip-info = Показать информацию о пакете
tooltip-settings = Открыть настройки