# CachyPac 日本語翻訳ファイル
# Fluent フォーマット - https://projectfluent.org/

## メインインターフェース
app-title = CachyPac - インテリジェントパッケージマネージャー
app-subtitle = 人工知能による Pacman アップデート自動化

## メインメニュー
menu-updates = アップデート
menu-packages = パッケージ
menu-history = 履歴
menu-settings = 設定
menu-about = について

## アップデートタブ
updates-title = 利用可能なアップデート
updates-check = アップデートを確認
updates-checking = 確認中...
updates-no-updates = 利用可能なアップデートはありません
updates-count = { $count ->
    [0] アップデートなし
    *[other] { $count } 個のアップデートが利用可能
}
updates-install-all = すべてのアップデートをインストール
updates-install-selected = 選択項目をインストール
updates-package-name = パッケージ名
updates-current-version = 現在のバージョン
updates-new-version = 新しいバージョン
updates-size = サイズ
updates-description = 説明

## パッケージタブ
packages-title = パッケージマネージャー
packages-search = パッケージを検索...
packages-search-placeholder = パッケージ名を入力
packages-installed = インストール済み
packages-available = 利用可能
packages-install = インストール
packages-remove = 削除
packages-upgrade = アップグレード
packages-info = 情報
packages-dependencies = 依存関係
packages-files = ファイル

## 履歴タブ
history-title = 操作履歴
history-date = 日付
history-operation = 操作
history-packages = パッケージ
history-status = ステータス
history-details = 詳細
history-clear = 履歴をクリア
history-export = エクスポート
history-filter = タイプでフィルター
history-filter-all = すべての操作
history-filter-install = インストール
history-filter-remove = 削除
history-filter-update = アップデート

## 設定タブ
settings-title = 設定
settings-general = 一般
settings-updates = アップデート
settings-notifications = 通知
settings-advanced = 詳細設定
settings-language = 言語
settings-theme = テーマ
settings-auto-update = 自動アップデート
settings-check-interval = 確認間隔
settings-telegram-bot = Telegram ボット
settings-telegram-token = ボットトークン
settings-telegram-chat-id = チャット ID
settings-backup = バックアップ
settings-restore = 復元
settings-reset = リセット
settings-save = 保存
settings-cancel = キャンセル

## 通知
notification-update-available = アップデートが利用可能
notification-update-complete = アップデート完了
notification-install-complete = インストール完了
notification-remove-complete = 削除完了
notification-error = エラー
notification-success = 成功
notification-warning = 警告
notification-info = 情報

## エラーメッセージ
error-network = ネットワークエラー：リポジトリに接続できません
error-permission = 権限不足：管理者として実行してください
error-package-not-found = パッケージが見つかりません：{ $package }
error-dependency = 依存関係エラー：{ $dependency } が必要です
error-disk-space = ディスク容量不足
error-download = ダウンロードエラー：{ $url }
error-install = インストールエラー：{ $package }
error-remove = 削除エラー：{ $package }
error-config = 設定エラー：{ $file }
error-database = データベースエラー：{ $message }

## 成功メッセージ
success-install = { $package } のインストールが完了しました
success-remove = { $package } の削除が完了しました
success-update = { $count ->
    *[other] { $count } 個のパッケージがアップデートされました
}
success-config-saved = 設定が保存されました
success-backup-created = バックアップが作成されました：{ $file }
success-restore-complete = 復元が完了しました

## 確認ダイアログ
confirm-install = { $package } をインストールしますか？
confirm-remove = { $package } を削除しますか？
confirm-update-all = すべてのアップデートをインストールしますか？
confirm-clear-history = すべての履歴をクリアしますか？
confirm-reset-settings = すべての設定をリセットしますか？

## ボタン
button-yes = はい
button-no = いいえ
button-ok = OK
button-cancel = キャンセル
button-apply = 適用
button-close = 閉じる
button-refresh = 更新
button-back = 戻る
button-next = 次へ
button-finish = 完了

## ステータス
status-idle = 待機中
status-checking = 確認中...
status-downloading = ダウンロード中...
status-installing = インストール中...
status-removing = 削除中...
status-updating = アップデート中...
status-complete = 完了
status-failed = 失敗
status-cancelled = キャンセル済み

## 単位
unit-bytes = バイト
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-seconds = 秒
unit-minutes = 分
unit-hours = 時間

## 人工知能
ai-analyzing = インテリジェント分析実行中...
ai-recommendation = AI 推奨
ai-safe-to-update = これらのアップデートは安全にインストールできます
ai-caution-required = これらのアップデートには注意が必要です
ai-critical-update = 重要なアップデートが検出されました
ai-dependency-conflict = 依存関係の競合が検出されました
ai-performance-impact = パフォーマンスへの影響が予想されます
ai-security-update = 重要なセキュリティアップデート

## スケジューラー
scheduler-title = タスクスケジューラー
scheduler-enable = スケジューリングを有効化
scheduler-daily = 毎日
scheduler-weekly = 毎週
scheduler-monthly = 毎月
scheduler-custom = カスタム
scheduler-time = 実行時刻
scheduler-next-run = 次回実行

## Telegram
telegram-title = Telegram 統合
telegram-enable = Telegram 通知を有効化
telegram-test = 接続をテスト
telegram-connected = Telegram に接続済み
telegram-disconnected = Telegram から切断済み
telegram-message-sent = メッセージ送信済み
telegram-message-failed = 送信失敗

## について
about-title = CachyPac について
about-version = バージョン { $version }
about-description = AI 自動化機能付き Arch Linux インテリジェントパッケージマネージャー
about-author = CachyPac チームによる開発
about-license = Apache 2.0 ライセンス
about-website = ウェブサイト
about-github = ソースコード
about-support = サポート

## キーボードショートカット
shortcut-refresh = Ctrl+R
shortcut-search = Ctrl+F
shortcut-settings = Ctrl+,
shortcut-quit = Ctrl+Q
shortcut-help = F1

## ツールチップ
tooltip-refresh = パッケージリストを更新
tooltip-search = パッケージ内を検索
tooltip-install = 選択したパッケージをインストール
tooltip-remove = 選択したパッケージを削除
tooltip-info = パッケージ情報を表示
tooltip-settings = 設定を開く