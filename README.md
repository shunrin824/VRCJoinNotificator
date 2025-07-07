# VRCJoinNotificator

VRChatのログファイルをリアルタイムで監視し、プレイヤーの参加・退出やその他のイベントを検知してVR内およびDiscordに通知するRust製ツールです。

## 主な機能

### ログ監視機能
- **リアルタイムログ解析**: VRChatのログファイルを100ms間隔で監視
- **プレイヤー参加・退出検知**: ユーザーの Join/Leave を自動検出
- **URL再生検知**: 動画やストリーミングコンテンツの再生を検知
- **スクリーンショット検知**: VRChatカメラ機能での撮影を検知
- **ワールド移動検知**: 新しいワールドへの移動を検知
- **招待通知検知**: InviteやRequestInviteメッセージを検知

### 通知システム
- **XSOverlay通知**: VR内でのリアルタイム通知表示
- **Discord Webhook**: テキスト通知と画像自動送信(高画質な画像を連続で撮影する場合、変換処理で負荷が上がる可能性があります。)
- **SDMS連携**: 外部データ管理システムへの自動アップロード

### 画像処理
- **自動圧縮**: Discord容量制限（10MB）対応の自動WebP変換
- **解像度調整**: 設定可能な最大解像度制限
- **品質最適化**: ファイルサイズに応じた10%刻みでの品質調整
- **マルチスレッド動作**: 高速で画像を処理

## システム要件

- **OS**: Windows 10/11 (VRChatログファイルパス依存)
- **VRChat**: 最新版
- **Rust**: 1.70以降 (ビルド時のみ)

### オプション連携
- **XSOverlay**: VR内通知機能
- **Discord**: Webhook通知機能
- **SDMS**: 外部データ管理システム

## インストール

### リリース版の使用（推奨）
1. [Releases](../../releases)から最新版の `VRCJoinNotificator.exe` をダウンロード
2. 任意のフォルダに配置
3. 設定ファイルを作成（後述）
4. 実行

### ソースからビルド
```bash
# リポジトリをクローン
git clone https://github.com/yourusername/VRCJoinNotificator.git
cd VRCJoinNotificator

# Windows向けクロスコンパイル
cargo build --release --target x86_64-pc-windows-gnu

# 通常ビルド
cargo build --release
```
### ワンライナー
```bash
git clone https://github.com/yourusername/VRCJoinNotificator.git && cd VRCJoinNotificator && cargo build --release --target x86_64-pc-windows-gnu
```

## 設定

実行ファイルと同じディレクトリに `config.txt` を作成：

```ini
# デバッグモード（true/false）
debug_mode=false

# 画像処理時の最大スレッド数 (オプション)
max_convertpic_threads=4

# Discord Webhook URL（オプション）
discord_webhook_url=https://discord.com/api/webhooks/YOUR_WEBHOOK_URL

# Discordへ画像アップロード時の最大解像度（ピクセル、長辺基準、0で制限なし）
discord_webhook_image_resolution=3840

# SDMS サーバー設定（オプション）
idms_server_url=https://your-server.example.com/upload
idms_server_auth_username=username
idms_server_auth_password=password
```

### 設定項目詳細

| 項目 | 説明 | 必須 | デフォルト |
|------|------|------|-----------|
| `debug_mode` | デバッグ情報出力 | x | `false` |
| `max_convertpic_threads` | 画像処理時の最大スレッド数 | x | 4 |
| `discord_webhook_url` | Discord Webhook URL | x | なし |
| `discord_webhook_image_resolution` | 画像最大解像度 | x | 制限なし |
| `idms_server_url` | SDMS サーバーURL | x | なし |
| `idms_server_auth_username` | SDMS 認証ユーザー名 | x | なし |
| `idms_server_auth_password` | SDMS 認証パスワード | x | なし |

## 使用方法

### 基本操作
1. **VRChatを起動** - ログファイル生成を確認
2. **VRCJoinNotificatorを実行** - 管理者権限不要
3. **初期化完了まで待機** - "ログの解析を開始します" 表示
4. **VRChatでプレイ** - 自動的にイベント検知・通知

### 通知される情報
- **プレイヤー参加**: `JOIN: [ 11人] PlayerName`
- **プレイヤー退出**: `LEFT: [ 10人] PlayerName`
- **ワールド移動**: `ROOM: WorldName`
- **スクリーンショット**: `CAM : C:\\Users\\...\\screenshot.png`
- **URL再生**: `URL : https://example.com/video.mp4`
- **招待通知**: Discord Webhookでの通知

### XSOverlay通知
VR内で下記の内容の通知をリアルタイムで受け取れます：
- プレイヤーの参加・退出
- URL再生の検知
- 複数ユーザーの同時参加・退出

### Discord通知
設定したWebhookチャンネルに以下が自動投稿されます：
- 招待（Invite）通知
- 参加要求（RequestInvite）通知
- スクリーンショット（ワールド名・参加者リスト付き）

## 高度な設定

### ログファイルの場所
プログラムは以下の場所からVRChatログを自動検索します：
```
%USERPROFILE%\AppData\LocalLow\VRChat\VRChat\
```

### 画像処理設定
- **自動圧縮**: 10MB超過時に自動的にWebP形式に変換・圧縮
- **解像度制限**: `discord_webhook_image_resolution` で最大解像度を制御
- **品質調整**: ファイルサイズに応じて10%ずつ品質を下げて最適化

### デバッグモード
`debug_mode=true` に設定すると詳細なログが出力されます：
```
Debug: 1920
Debug: 画像コピー
Debug: 1920x1080
Debug: /tmp/screenshot.webp
```

## トラブルシューティング

### よくある問題と解決方法

#### 「config.txtが見つかりませんでした」
- **原因**: 設定ファイルが実行ファイルと同じディレクトリにない
- **解決方法**: `config.txt` を実行ファイルと同じ場所に作成

#### 「ログファイルを読み込めません」
- **原因**: VRChatが起動していない、ログファイルがロックされている、またはログファイルが標準の場所から移動されている
- **解決方法**: VRChatを起動してからツールを実行

#### Discord通知が届かない
- **原因**: Webhook URLが正しくない、またはDiscordサーバーの権限不足
- **解決方法**: 
  1. Webhook URLを再確認
  2. Discordチャンネルの権限を確認
  3. `debug_mode=true` でエラー詳細を確認

#### XSOverlay通知が表示されない
- **原因**: XSOverlayが起動していない、またはポート42069が使用中
- **解決方法**:
  1. XSOverlayを起動
  2. 他のアプリケーションがポートを使用していないか確認

#### IDMS送信エラー
- **原因**: サーバーURL、認証情報が正しくない
- **解決方法**:
  1. `idms_server_url` の確認
  2. 認証情報の確認
  3. サーバーのアクセス可能性を確認

### パフォーマンス最適化
- **CPU使用率が高い場合**: VRChatとツールを別々のCPUコアで実行
- **メモリ使用量が多い場合**: VRChat終了時にツールも再起動
- **通知遅延がある場合**: `debug_mode=false` に設定

## 開発者向け情報

### プロジェクト構造
```
src/
├── main.rs           # メインループ・ログ解析エンジン
├── function.rs       # 設定管理・ユーティリティ関数
├── log_read.rs       # ログファイル読み取り・パース
├── xsoverlay.rs      # XSOverlay UDP通信
├── webhook.rs        # Discord Webhook・画像送信
├── image.rs          # WebP変換・画像最適化
└── idms.rs           # SDMS連携・データアップロード
```

### 主要依存関係
```toml
regex = "1.11.1"           # ログパターンマッチング
reqwest = "0.12.9"         # HTTP通信
tokio = "1.42.0"           # 非同期ランタイム
image = "0.25.6"           # 画像処理
webp = "0.3.0"             # WebP変換
sysinfo = "0.33.1"         # プロセス監視
serde = "1.0.215"          # JSON処理
base64 = "0.22.1"          # Basic認証
tempfile = "3.20.0"        # 一時ファイル
```

### ビルド設定
```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```

## ライセンス
このソフトウェアは MIT Licenseのもとで公開されています。

## 謝辞

- VRChatコミュニティ
- XSOverlay開発チーム  
- Rustコミュニティ

## サポート

- **Issues**: [GitHub Issues](../../issues)
- **Discord**: [コミュニティサーバー](#)
- **Email**: [shunrin824-dev@shunrin.com](mailto:shunrin824-dev@shunrin.com)

---

**注意**: このツールはVRChatの利用規約に準拠して使用してください。自動化ツールの使用については、VRChatの最新のガイドラインを確認することをお勧めします。

Copyright © 2024-2025 shunrin824 All Rights Reserved.
