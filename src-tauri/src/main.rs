// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// 定义一个宏 `app_log!`，它将日志信息追加到指定的文件中
macro_rules! app_log {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("appLog.log")
            .expect("Failed to open log file");
        // 获取当前时间
        let local = chrono::Local::now();
        // 格式化时间为字符串，精确到分秒
        let formatted = local.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("[{}] {}",formatted,format!($($arg)*));
        writeln!(file, "[{}] {}", formatted, format!($($arg)*))
            .expect("Failed to write to log file");
    }};
}

mod midi_handle;
mod resource_manage;
mod scrcpy;
pub mod util;
mod windows_interface;
use std::sync::{Arc, Mutex};

use rdev::{EventType, Key};
use resource_manage::resource_read::{get_text_config, set_text_config};
use scrcpy::scrcpy_call::{
    close_process_by_id, disable_keybord_input, enable_keyborad_input, get_all_life_process,
    play_scrcpy, push_text_to_adb, upload_file_to_adb, ScrcpyProecss,
};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use windows_interface::{
    glob_event::{add_event_listener, start_linstener, EventCallback},
    ip_config, route,
};
fn start_tauri_app() {
    // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let hide = CustomMenuItem::new("hide".to_string(), "显示/隐藏");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit); // insert the menu items here
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                    }
                }
                "quit" => {
                    let window = app.get_window("main").unwrap();
                    window.close().unwrap();
                    app_log!("==========   程序退出   ==========");
                }
                _ => {}
            },
            SystemTrayEvent::DoubleClick { .. } => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                }
            }
            _ => {}
        })
        .setup(|app| {
            // 开启调试工具
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                // window.open_devtools();
                window.close_devtools();
            }

            // 启动线程监听全局事件
            std::thread::spawn(move || {
                start_linstener();
            });

            // 注册一全局点击alt按键事件
            let app_arc = Arc::new(Mutex::new(app.app_handle()));
            add_event_listener(
                EventType::KeyPress(Key::F4),
                EventCallback::new(move |_| {
                    let _ = app_arc.lock().unwrap().emit_all("press-alt", "");
                }),
            );

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_text_config,
            set_text_config,
            ip_config::get_all_adapter_info,
            ip_config::flush_dns,
            route::get_route_info,
            route::delete_route,
            route::add_route,
            play_scrcpy,
            get_all_life_process,
            close_process_by_id,
            upload_file_to_adb,
            push_text_to_adb,
            enable_keyborad_input,
            disable_keybord_input,
            midi_handle::midi_parse::parse_midi,
            midi_handle::midi_parse::generate_animation,
        ])
        .manage(ScrcpyProecss::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    app_log!("==========   程序启动   ==========");
    start_tauri_app();
}
