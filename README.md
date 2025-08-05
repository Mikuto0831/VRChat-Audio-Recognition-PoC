# VRChat Audio Recognition PoC

VRChat向け音声認識ツールの技術検証（Proof of Concept）プロジェクトです。

## プロジェクト概要

このPoCの目的は、Rustを使用してWindowsのシステム音声をリアルタイムでキャプチャし、その音声データをPythonスクリプトに渡して話者分離と文字起こしを行うシステムを構築することです。

## アーキテクチャ

```
┌─────────────────────┐    音声データ    ┌──────────────────────────┐
│   Rust Audio        │ ──────────────► │  Python AI Services      │
│   Capture            │                 │                          │
│                     │                 │  ┌─────────────────────┐ │
│ ・cpalライブラリ     │                 │  │ pyannote.audio      │ │
│ ・システム音声       │                 │  │ (話者分離)           │ │
│   キャプチャ         │                 │  └─────────────────────┘ │
│ ・リアルタイム処理   │                 │                          │
│                     │                 │  ┌─────────────────────┐ │
└─────────────────────┘                 │  │ faster-whisper      │ │
                                        │  │ (音声認識)           │ │
                                        │  └─────────────────────┘ │
                                        └──────────────────────────┘
```

## ディレクトリ構成

```
.
├── .mise.toml                   # miseツール管理設定
├── .gitignore                   # Git除外設定
├── README.md                    # このファイル
├── rust_audio_capture/          # Rust音声キャプチャプログラム
│   ├── Cargo.toml              # Rust依存関係設定
│   └── src/
│       └── main.rs             # メインプログラム
└── python_ai_services/         # Python AI処理サービス
    ├── .venv/                  # Python仮想環境
    └── main.py                 # メインプログラム
```

## 環境要件

- **Windows 10/11** (音声キャプチャのため)
- **mise** (ツール管理)

### 管理されるツール

- **Python 3.11**
- **Rust (最新安定版)**
- **uv (最新版)** - Python依存関係管理

## セットアップ

### 1. miseのインストール

```powershell
winget install jdx.mise
```

### 2. 必要なツールのインストール

```powershell
mise install
```

### 3. 環境の有効化

```powershell
mise env -s pwsh | Out-String | Invoke-Expression
```

## 使用技術

### Rust側 (音声キャプチャ)

- **cpal**: クロスプラットフォーム音声ライブラリ
- **Windows Audio API**: システム音声キャプチャ

### Python側 (AI処理)

- **pyannote.audio**: 話者分離・話者認識
- **faster-whisper**: 高速音声認識
- **torch/torchaudio**: PyTorch深層学習フレームワーク

## 開発フロー

このプロジェクトでは、GitHubフローを採用しています：

1. `main`ブランチから新しいフィーチャーブランチを作成
2. フィーチャーブランチで開発作業を実施
3. Pull Requestを作成してmainブランチにマージ

### 現在の作業ブランチ

- `poc/setup-environment`: 初期環境構築

## ステータス

- [x] プロジェクト初期化とGit設定
- [x] miseによるツール管理環境構築
- [x] Rust音声キャプチャプロジェクトの基本構造作成
- [x] Python AI処理サービスの基本構造作成
- [ ] Rust音声キャプチャ機能の実装
- [ ] Python話者分離機能の実装
- [ ] Python音声認識機能の実装
- [ ] プロセス間通信（IPC）の実装
- [ ] リアルタイム処理の最適化

## ライセンス

このプロジェクトはPoC（技術検証）用途です。

## 開発チーム

VRChat Audio Recognition PoC Team
