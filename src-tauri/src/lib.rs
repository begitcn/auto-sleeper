use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::TrayIconBuilder,
    Manager, // 必须导入这个核心 Trait
    Wry,     // 显式指定运行时类型，解决 autostart() 识别问题
};
// 导入插件扩展 Trait
use tauri_plugin_autostart::ManagerExt; 
use tauri_plugin_autostart::MacosLauncher;
use windows_sys::Win32::System::Power::SetSuspendState;
use windows_sys::Win32::System::SystemInformation::GetTickCount;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};

static IDLE_THRESHOLD_MINUTES: AtomicU32 = AtomicU32::new(30);

fn get_idle_seconds() -> u32 {
    unsafe {
        let mut lii: LASTINPUTINFO = std::mem::zeroed();
        lii.cbSize = std::mem::size_of::<LASTINPUTINFO>() as u32;
        if GetLastInputInfo(&mut lii) != 0 {
            let now = GetTickCount();
            return (now - lii.dwTime) / 1000;
        }
        0
    }
}

fn trigger_sleep() {
    unsafe {
        // 睡眠(S3)且允许网卡唤醒 (WOL)
        SetSuspendState(0, 0, 0);
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--hidden"])))
        .setup(|app| {
            // 获取 AppHandle 并指定为 Wry 运行时，这能强制让编译器找到 autostart 方法
            let handle = app.handle();

            // 1. 启动时隐藏主窗口
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }

            // 2. 创建时长选择子菜单
            let t15 = CheckMenuItem::with_id(handle, "t15", "15 分钟", true, false, None::<&str>)?;
            let t30 = CheckMenuItem::with_id(handle, "t30", "30 分钟", true, true, None::<&str>)?;
            let t60 = CheckMenuItem::with_id(handle, "t60", "60 分钟", true, false, None::<&str>)?;
            let time_submenu = Submenu::with_items(handle, "设置休眠时长", true, &[&t15, &t30, &t60])?;

            // 3. 获取自启动状态并创建菜单
            // 注意：这里明确调用 ManagerExt 提供的 autostart 方法
            let is_start = handle.autolaunch().is_enabled().unwrap_or(false);
            let autostart_menu = CheckMenuItem::with_id(handle, "toggle_autostart", "开机自启动", true, is_start, None::<&str>)?;

            // 4. 其他基础菜单
            let sleep_now = MenuItem::with_id(handle, "sleep_now", "立即休眠", true, None::<&str>)?;
            let quit = MenuItem::with_id(handle, "quit", "退出程序", true, None::<&str>)?;

            let menu = Menu::with_items(handle, &[
                &time_submenu,
                &autostart_menu,
                &PredefinedMenuItem::separator(handle)?,
                &sleep_now,
                &quit,
            ])?;

            // 5. 构建托盘
            let _tray = TrayIconBuilder::<Wry>::new() // 指定 Wry
                .icon(handle.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(move |handle, event| {
                    match event.id.as_ref() {
                        "quit" => handle.exit(0),
                        "sleep_now" => trigger_sleep(),
                        "toggle_autostart" => {
                            // 动态获取当前对号状态并反转
                            if let Ok(checked) = autostart_menu.is_checked() {
                                if checked {
                                    let _ = handle.autolaunch().enable();
                                } else {
                                    let _ = handle.autolaunch().disable();
                                }
                            }
                        }
                        "t15" | "t30" | "t60" => {
                            let minutes = match event.id.as_ref() {
                                "t15" => 15,
                                "t60" => 60,
                                _ => 30,
                            };
                            IDLE_THRESHOLD_MINUTES.store(minutes, Ordering::Relaxed);
                            // 更新对号互斥状态
                            let id = event.id.as_ref();
                            let _ = t15.set_checked(id == "t15");
                            let _ = t30.set_checked(id == "t30");
                            let _ = t60.set_checked(id == "t60");
                        }
                        _ => {}
                    }
                })
                .build(handle)?;

            // 6. 异步监控逻辑
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    let idle_secs = get_idle_seconds();
                    let threshold_secs = IDLE_THRESHOLD_MINUTES.load(Ordering::Relaxed) * 60;
                    if idle_secs >= threshold_secs {
                        trigger_sleep();
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}