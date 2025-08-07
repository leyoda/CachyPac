# Arquivo de tradução português para CachyPac
# Formato Fluent - https://projectfluent.org/

## Interface principal
app-title = CachyPac - Gestor de Pacotes Inteligente
app-subtitle = Automação de atualizações Pacman com inteligência artificial

## Menu principal
menu-updates = Atualizações
menu-packages = Pacotes
menu-history = Histórico
menu-settings = Configurações
menu-about = Sobre

## Aba Atualizações
updates-title = Atualizações disponíveis
updates-check = Verificar atualizações
updates-checking = Verificando...
updates-no-updates = Nenhuma atualização disponível
updates-count = { $count ->
    [0] Nenhuma atualização
    [1] 1 atualização disponível
    *[other] { $count } atualizações disponíveis
}
updates-install-all = Instalar todas as atualizações
updates-install-selected = Instalar selecionados
updates-package-name = Nome do pacote
updates-current-version = Versão atual
updates-new-version = Nova versão
updates-size = Tamanho
updates-description = Descrição

## Aba Pacotes
packages-title = Gestor de pacotes
packages-search = Procurar pacotes...
packages-search-placeholder = Digite o nome de um pacote
packages-installed = Instalados
packages-available = Disponíveis
packages-install = Instalar
packages-remove = Remover
packages-upgrade = Atualizar
packages-info = Informações
packages-dependencies = Dependências
packages-files = Arquivos

## Aba Histórico
history-title = Histórico de operações
history-date = Data
history-operation = Operação
history-packages = Pacotes
history-status = Estado
history-details = Detalhes
history-clear = Limpar histórico
history-export = Exportar
history-filter = Filtrar por tipo
history-filter-all = Todas as operações
history-filter-install = Instalações
history-filter-remove = Remoções
history-filter-update = Atualizações

## Aba Configurações
settings-title = Configurações
settings-general = Geral
settings-updates = Atualizações
settings-notifications = Notificações
settings-advanced = Avançado
settings-language = Idioma
settings-theme = Tema
settings-auto-update = Atualização automática
settings-check-interval = Intervalo de verificação
settings-telegram-bot = Bot Telegram
settings-telegram-token = Token do bot
settings-telegram-chat-id = ID do chat
settings-backup = Backup
settings-restore = Restaurar
settings-reset = Redefinir
settings-save = Guardar
settings-cancel = Cancelar

## Notificações
notification-update-available = Atualizações disponíveis
notification-update-complete = Atualizações concluídas
notification-install-complete = Instalação concluída
notification-remove-complete = Remoção concluída
notification-error = Erro
notification-success = Sucesso
notification-warning = Aviso
notification-info = Informação

## Mensagens de erro
error-network = Erro de rede: impossível conectar aos repositórios
error-permission = Permissões insuficientes: executar como administrador
error-package-not-found = Pacote não encontrado: { $package }
error-dependency = Erro de dependência: { $dependency } necessário
error-disk-space = Espaço em disco insuficiente
error-download = Erro de download: { $url }
error-install = Erro de instalação: { $package }
error-remove = Erro de remoção: { $package }
error-config = Erro de configuração: { $file }
error-database = Erro de base de dados: { $message }

## Mensagens de sucesso
success-install = { $package } instalado com sucesso
success-remove = { $package } removido com sucesso
success-update = { $count ->
    [1] 1 pacote atualizado
    *[other] { $count } pacotes atualizados
}
success-config-saved = Configuração guardada
success-backup-created = Backup criado: { $file }
success-restore-complete = Restauração concluída

## Diálogos de confirmação
confirm-install = Deseja instalar { $package }?
confirm-remove = Deseja remover { $package }?
confirm-update-all = Deseja instalar todas as atualizações?
confirm-clear-history = Deseja limpar todo o histórico?
confirm-reset-settings = Deseja redefinir todas as configurações?

## Botões
button-yes = Sim
button-no = Não
button-ok = OK
button-cancel = Cancelar
button-apply = Aplicar
button-close = Fechar
button-refresh = Atualizar
button-back = Voltar
button-next = Seguinte
button-finish = Terminar

## Estados
status-idle = Inativo
status-checking = Verificando...
status-downloading = Descarregando...
status-installing = Instalando...
status-removing = Removendo...
status-updating = Atualizando...
status-complete = Concluído
status-failed = Falhado
status-cancelled = Cancelado

## Unidades
unit-bytes = bytes
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-seconds = segundos
unit-minutes = minutos
unit-hours = horas

## Inteligência artificial
ai-analyzing = Análise inteligente em progresso...
ai-recommendation = Recomendação IA
ai-safe-to-update = Estas atualizações são seguras para instalar
ai-caution-required = Cuidado necessário para estas atualizações
ai-critical-update = Atualização crítica detetada
ai-dependency-conflict = Conflito de dependências detetado
ai-performance-impact = Impacto na performance esperado
ai-security-update = Atualização de segurança importante

## Agendador
scheduler-title = Agendador de tarefas
scheduler-enable = Ativar agendamento
scheduler-daily = Diário
scheduler-weekly = Semanal
scheduler-monthly = Mensal
scheduler-custom = Personalizado
scheduler-time = Hora de execução
scheduler-next-run = Próxima execução

## Telegram
telegram-title = Integração Telegram
telegram-enable = Ativar notificações Telegram
telegram-test = Testar ligação
telegram-connected = Ligado ao Telegram
telegram-disconnected = Desligado do Telegram
telegram-message-sent = Mensagem enviada
telegram-message-failed = Envio falhado

## Sobre
about-title = Sobre CachyPac
about-version = Versão { $version }
about-description = Gestor de pacotes inteligente para Arch Linux com automação IA
about-author = Desenvolvido pela equipa CachyPac
about-license = Licença Apache 2.0
about-website = Website
about-github = Código fonte
about-support = Suporte

## Atalhos de teclado
shortcut-refresh = Ctrl+R
shortcut-search = Ctrl+F
shortcut-settings = Ctrl+,
shortcut-quit = Ctrl+Q
shortcut-help = F1

## Tooltips
tooltip-refresh = Atualizar lista de pacotes
tooltip-search = Procurar nos pacotes
tooltip-install = Instalar pacote selecionado
tooltip-remove = Remover pacote selecionado
tooltip-info = Mostrar informações do pacote
tooltip-settings = Abrir configurações