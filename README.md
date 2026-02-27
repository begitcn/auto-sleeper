# Auto Sleeper

Tray utility that puts your Windows PC to sleep when it has been idle for a configured amount of time. Built with **Tauri v2**.

English | [简体中文](./README.zh-CN.md)

## ✨ Features

- **Tray resident**: the main window is hidden by default; tray menu is the primary UI.
- **Idle detection**: uses Windows APIs to compute idle seconds.
- **Auto sleep**: triggers sleep when idle time exceeds the threshold.
- **Threshold switch**: 15 / 30 / 60 minutes.
- **Autostart on boot**: powered by `tauri-plugin-autostart` (desktop).

## 🚀 Quickstart (Development)

```bash
pnpm install
pnpm tauri dev
```

Note: `beforeDevCommand` is set to `pnpm dev`, so `pnpm tauri dev` will start the frontend dev server automatically.

## 📦 Build

```bash
pnpm tauri build
```

## 🧰 Tech Stack

- **Tauri**: v2
- **Frontend**: Vite + Vanilla TypeScript
- **Rust**: Tokio interval loop + `windows-sys`
- **Plugins**: `tauri-plugin-autostart`, `tauri-plugin-opener`

## ⚙️ Configuration Notes

- **Hidden window by default**: configured via `visible: false` in `src-tauri/tauri.conf.json`.
- **Capabilities**: if you call `enable/isEnabled/disable` from `@tauri-apps/plugin-autostart`, allow the corresponding permissions in your Tauri v2 capabilities.

## 🤖 AI Disclosure

This project includes code and documentation **generated with AI assistance**. Review critical behavior (sleep, permissions, autostart) before shipping and adapt it to your security/compliance requirements.

## 🤝 Contributing

Issues and PRs are welcome.

- For bugs, include OS version, reproduction steps, and logs/screenshots.
- For PRs, describe your motivation and how you tested.

## 📄 License

MIT License. See `LICENSE`.
