# CachyPac 中文翻译文件
# Fluent 格式 - https://projectfluent.org/

## 主界面
app-title = CachyPac - 智能包管理器
app-subtitle = 基于人工智能的 Pacman 更新自动化

## 主菜单
menu-updates = 更新
menu-packages = 软件包
menu-history = 历史记录
menu-settings = 设置
menu-about = 关于

## 更新选项卡
updates-title = 可用更新
updates-check = 检查更新
updates-checking = 正在检查...
updates-no-updates = 没有可用更新
updates-count = { $count ->
    [0] 没有更新
    *[other] { $count } 个可用更新
}
updates-install-all = 安装所有更新
updates-install-selected = 安装选中项
updates-package-name = 包名
updates-current-version = 当前版本
updates-new-version = 新版本
updates-size = 大小
updates-description = 描述

## 软件包选项卡
packages-title = 包管理器
packages-search = 搜索软件包...
packages-search-placeholder = 输入包名
packages-installed = 已安装
packages-available = 可用
packages-install = 安装
packages-remove = 移除
packages-upgrade = 升级
packages-info = 信息
packages-dependencies = 依赖关系
packages-files = 文件

## 历史记录选项卡
history-title = 操作历史
history-date = 日期
history-operation = 操作
history-packages = 软件包
history-status = 状态
history-details = 详情
history-clear = 清除历史
history-export = 导出
history-filter = 按类型筛选
history-filter-all = 所有操作
history-filter-install = 安装
history-filter-remove = 移除
history-filter-update = 更新

## 设置选项卡
settings-title = 设置
settings-general = 常规
settings-updates = 更新
settings-notifications = 通知
settings-advanced = 高级
settings-language = 语言
settings-theme = 主题
settings-auto-update = 自动更新
settings-check-interval = 检查间隔
settings-telegram-bot = Telegram 机器人
settings-telegram-token = 机器人令牌
settings-telegram-chat-id = 聊天 ID
settings-backup = 备份
settings-restore = 恢复
settings-reset = 重置
settings-save = 保存
settings-cancel = 取消

## 通知
notification-update-available = 有可用更新
notification-update-complete = 更新完成
notification-install-complete = 安装完成
notification-remove-complete = 移除完成
notification-error = 错误
notification-success = 成功
notification-warning = 警告
notification-info = 信息

## 错误消息
error-network = 网络错误：无法连接到软件源
error-permission = 权限不足：请以管理员身份运行
error-package-not-found = 未找到软件包：{ $package }
error-dependency = 依赖错误：需要 { $dependency }
error-disk-space = 磁盘空间不足
error-download = 下载错误：{ $url }
error-install = 安装错误：{ $package }
error-remove = 移除错误：{ $package }
error-config = 配置错误：{ $file }
error-database = 数据库错误：{ $message }

## 成功消息
success-install = { $package } 安装成功
success-remove = { $package } 移除成功
success-update = { $count ->
    *[other] { $count } 个软件包已更新
}
success-config-saved = 配置已保存
success-backup-created = 备份已创建：{ $file }
success-restore-complete = 恢复完成

## 确认对话框
confirm-install = 您要安装 { $package } 吗？
confirm-remove = 您要移除 { $package } 吗？
confirm-update-all = 您要安装所有更新吗？
confirm-clear-history = 您要清除所有历史记录吗？
confirm-reset-settings = 您要重置所有设置吗？

## 按钮
button-yes = 是
button-no = 否
button-ok = 确定
button-cancel = 取消
button-apply = 应用
button-close = 关闭
button-refresh = 刷新
button-back = 返回
button-next = 下一步
button-finish = 完成

## 状态
status-idle = 空闲
status-checking = 检查中...
status-downloading = 下载中...
status-installing = 安装中...
status-removing = 移除中...
status-updating = 更新中...
status-complete = 完成
status-failed = 失败
status-cancelled = 已取消

## 单位
unit-bytes = 字节
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-seconds = 秒
unit-minutes = 分钟
unit-hours = 小时

## 人工智能
ai-analyzing = 智能分析进行中...
ai-recommendation = AI 推荐
ai-safe-to-update = 这些更新可以安全安装
ai-caution-required = 这些更新需要谨慎处理
ai-critical-update = 检测到关键更新
ai-dependency-conflict = 检测到依赖冲突
ai-performance-impact = 预期性能影响
ai-security-update = 重要安全更新

## 调度器
scheduler-title = 任务调度器
scheduler-enable = 启用调度
scheduler-daily = 每日
scheduler-weekly = 每周
scheduler-monthly = 每月
scheduler-custom = 自定义
scheduler-time = 执行时间
scheduler-next-run = 下次运行

## Telegram
telegram-title = Telegram 集成
telegram-enable = 启用 Telegram 通知
telegram-test = 测试连接
telegram-connected = 已连接到 Telegram
telegram-disconnected = 已断开 Telegram 连接
telegram-message-sent = 消息已发送
telegram-message-failed = 发送失败

## 关于
about-title = 关于 CachyPac
about-version = 版本 { $version }
about-description = 基于 AI 自动化的 Arch Linux 智能包管理器
about-author = 由 CachyPac 团队开发
about-license = Apache 2.0 许可证
about-website = 网站
about-github = 源代码
about-support = 支持

## 键盘快捷键
shortcut-refresh = Ctrl+R
shortcut-search = Ctrl+F
shortcut-settings = Ctrl+,
shortcut-quit = Ctrl+Q
shortcut-help = F1

## 工具提示
tooltip-refresh = 刷新软件包列表
tooltip-search = 在软件包中搜索
tooltip-install = 安装选中的软件包
tooltip-remove = 移除选中的软件包
tooltip-info = 显示软件包信息
tooltip-settings = 打开设置