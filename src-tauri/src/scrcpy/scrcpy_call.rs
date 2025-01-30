use super::adb::{self, run_adb_command};
use crate::{
    util::env_util,
    windows_interface::{
        glob_event::{
            add_event_listener, remove_event_listener, string_to_keyevent_type, EventCallback,
        },
        hidden_proecss::{
            close_process, execute_hidden_cmd, get_process_status, read_pipe, read_pipe_bytes,
            std_to_string,
        },
    },
};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::Write,
    process::Stdio,
    sync::{Arc, Mutex},
    thread, time, u8,
};
use std::{os::windows::process::CommandExt, str::FromStr};
use tauri::{AppHandle, Manager, State};
use winapi::{
    shared::minwindef::DWORD,
    um::{
        handleapi::CloseHandle, minwinbase::STILL_ACTIVE, processthreadsapi::GetExitCodeProcess,
        winbase::CREATE_NO_WINDOW, winnt::HANDLE,
    },
};
pub struct ScrcpyProecss {
    pub process: Arc<Mutex<Vec<ScrcpyInfo>>>,
}
impl ScrcpyProecss {
    pub fn new() -> Self {
        ScrcpyProecss {
            process: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
#[derive(Debug, Copy, Clone, Serialize)]
pub struct ScrcpyInfo {
    pub process_id: u8,
    pub process_handle: usize, //进程指针
    pub stdout_handle: usize,  //标准输出管道指针
}

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct ScrcpyParam {
    pub window_borderless: bool,
    pub window_x: Option<isize>,
    pub window_y: Option<isize>,
    pub window_width: Option<isize>,
    pub window_height: Option<isize>,
    pub push_target: Option<String>,
    pub power_off_on_close: bool,
    pub turn_screen_off: bool,
    pub stay_awake: bool,
    pub always_on_top: bool,
    pub fullscreen: bool,
    pub tcpip: Option<String>,
    pub otg: bool,
    pub hid_mouse: bool,
    pub hid_keyboard: bool,
}

impl ScrcpyParam {
    pub fn build_param_str(&self) -> String {
        let mut param_str = String::new();

        if self.window_borderless {
            param_str.push_str(&ScrcpyParamEnum::get_param(
                ScrcpyParamEnum::WinodwsBorderless,
            ));
        }

        if let Some(x) = self.window_x {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::WinodwsX(x)));
        }

        if let Some(y) = self.window_y {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::WinodwsY(y)));
        }

        if let Some(width) = self.window_width {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::WinodwsWidth(
                width,
            )));
        }

        if let Some(height) = self.window_height {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::WinodwsHeight(
                height,
            )));
        }

        if let Some(path) = &self.push_target {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::PushTarget(
                path.to_string(),
            )));
        }

        if self.power_off_on_close {
            param_str.push_str(&ScrcpyParamEnum::get_param(
                ScrcpyParamEnum::PowerOffOnClose,
            ));
        }

        if self.turn_screen_off {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::TurnScreenOff));
        }

        if self.stay_awake {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::StayAwake));
        }

        if self.always_on_top {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::AlwaysOnTop));
        }

        if self.fullscreen {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::Fullscreen));
        }

        if let Some(ip) = &self.tcpip {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::Tcpip(
                ip.to_string(),
            )));
        }

        if self.otg {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::Otg));
        }

        if self.hid_mouse {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::HidMouse));
        }

        if self.hid_keyboard {
            param_str.push_str(&ScrcpyParamEnum::get_param(ScrcpyParamEnum::Hidkeyboard));
        }

        param_str
    }
}

/**
 * scrcpy的参数
 */
pub enum ScrcpyParamEnum {
    /**禁用窗口边框 */
    WinodwsBorderless,
    /**窗口位置x */
    WinodwsX(isize),
    /**窗口位置y */
    WinodwsY(isize),
    /**窗口宽度 */
    WinodwsWidth(isize),
    /**窗口高度 */
    WinodwsHeight(isize),
    /**文件推送至设备目录 */
    PushTarget(String),
    /**退出设备时息屏 */
    PowerOffOnClose,
    /**在屏幕关闭的状态进行镜像 */
    TurnScreenOff,
    /**使设备不休眠 */
    StayAwake,
    /**保持窗口在最前 */
    AlwaysOnTop,
    /**全屏 */
    Fullscreen,
    /**无线网络调试 */
    Tcpip(String),
    /**otg模式 */
    Otg,
    /**物理模拟鼠标 */
    HidMouse,
    /**物理模拟键盘 */
    Hidkeyboard,
}

impl ScrcpyParamEnum {
    pub fn get_param(scrcpy_param: ScrcpyParamEnum) -> String {
        match scrcpy_param {
            ScrcpyParamEnum::WinodwsBorderless => " --window-borderless ".to_owned(),
            ScrcpyParamEnum::WinodwsX(x) => format!(" --window-x {}", x),
            ScrcpyParamEnum::WinodwsY(y) => format!(" --window-y {}", y),
            ScrcpyParamEnum::WinodwsWidth(width) => format!(" --window-width {} ", width),
            ScrcpyParamEnum::WinodwsHeight(height) => format!(" --window-height {} ", height),
            ScrcpyParamEnum::PushTarget(path) => format!(" --push-target={} ", path),
            ScrcpyParamEnum::PowerOffOnClose => " --power-off-on-close ".to_owned(),
            ScrcpyParamEnum::TurnScreenOff => " --turn-screen-off ".to_owned(),
            ScrcpyParamEnum::StayAwake => " --stay-awake".to_owned(),
            ScrcpyParamEnum::AlwaysOnTop => " --always-on-top".to_owned(),
            ScrcpyParamEnum::Fullscreen => " --fullscreen".to_owned(),
            ScrcpyParamEnum::Tcpip(ip_addr) => {
                // 验证ip地址格式
                let ip_addr = ip_addr.trim();
                if ip_addr.is_empty() || !ip_addr.contains('.') {
                    return String::new();
                }
                return format!(" --tcpip={} ", ip_addr).to_owned();
            }
            ScrcpyParamEnum::Otg => " --otg".to_owned(),
            ScrcpyParamEnum::HidMouse => " --hid-mouse".to_owned(),
            ScrcpyParamEnum::Hidkeyboard => " --hid-keyboard".to_owned(),
        }
    }
}

/**
 * 使用给定的应用句柄来调用scrcpy。
 * # 参数
 * - `app_hanndle` 应用的句柄，用于获取配置和状态管理。
 * # 返回
 * - `Result<u8, String>` 成功时返回一个进程ID（u8类型），失败时返回一个错误信息字符串。
 */
#[tauri::command]
pub async fn play_scrcpy(app_hanndle: AppHandle, scrcpy_param: ScrcpyParam) -> Result<u8, String> {
    app_log!("--------------\n{:?}\n----------", scrcpy_param);
    let param_str = scrcpy_param.build_param_str();
    // 构建执行scrcpy命令
    let cmd_command = env_util::build_root_command("plugin\\scrcpy\\scrcpy.exe", &param_str)?;
    // // 执行隐藏的cmd命令
    let process_handle = execute_hidden_cmd(&cmd_command)?;
    if !get_process_status(process_handle as usize) {
        return Err(format!("{}", "scrcpy启动失败"));
    }
    // 克隆当前应用状态中的Scrcpy进程信息
    let process_arc = Arc::clone(&app_hanndle.state::<ScrcpyProecss>().process);
    // 获取进程列表的锁，并操作进程列表
    let mut process_list = process_arc.lock().unwrap();
    // 将新启动的scrcpy进程信息加入到进程列表中，并返回进程ID
    let process_id = push_state(&mut process_list, process_handle as usize).unwrap();

    return Ok(process_id);
}

/**
 * 异步获取所有活跃的Scrcpy进程信息。
 *
 * # 参数
 * - `state`: 一个包含了Scrcpy进程信息的状态对象的引用。
 *
 * # 返回值
 * - `Result<Vec<ScrcpyInfo>, String>`: 成功时返回一个包含Scrcpy进程信息的向量，失败时返回一个错误信息的字符串。
 */
#[tauri::command]
pub async fn get_all_life_process(
    state: State<'_, ScrcpyProecss>,
) -> Result<Vec<ScrcpyInfo>, String> {
    // 克隆进程列表的Arc，并锁定以进行读写操作。
    let scrcpy_list = Arc::clone(&state.process);
    let mut scrcpy_list = scrcpy_list.lock().unwrap();

    // 不安全操作：直接修改列表，过滤掉已经退出的进程。
    unsafe {
        *scrcpy_list = scrcpy_list
            .iter()
            // 过滤条件：进程仍然活跃（未退出）。
            .filter(|item| {
                let mut exit_code: DWORD = 0;

                // 尝试获取进程的退出码。
                if GetExitCodeProcess(item.process_handle as HANDLE, &mut exit_code) != 0 {
                    // 如果进程仍然活跃，则保留。
                    if exit_code == STILL_ACTIVE {
                        return true;
                    } else {
                        // 已退出的进程不保留。
                        return false;
                    }
                }
                // 如果无法获取退出码，认为进程不活跃。
                let _ = close_process(item.process_handle as HANDLE);
                return false;
            })
            // 将活跃的进程复制到新的列表中。
            .cloned()
            .collect();
    }
    // 返回更新后的活跃进程列表的副本。
    return Ok(scrcpy_list.clone());
}

//根据进程信息中的process_id来终止进程
#[tauri::command]
pub fn close_process_by_id(
    process_id: u8,
    state: State<'_, ScrcpyProecss>,
) -> Result<bool, String> {
    app_log!("关闭进程id:  {}", process_id);
    // 获取进程列表的锁，并操作进程列表
    let mut process_list = state.process.lock().unwrap();
    // 遍历进程列表，找到匹配的进程ID
    for item in process_list.iter_mut() {
        if item.process_id == process_id {
            // 找到匹配的进程，尝试终止进程
            let close_flage = close_process(item.process_handle as HANDLE)?;
            let _ = close_process(item.stdout_handle as HANDLE);
            return Ok(close_flage);
        }
    }
    // 如果无法按预期执行，返回一个通用错误码"500"
    Err("未找到对应需要被关闭的进程".into())
}

#[tauri::command]
pub fn upload_file_to_adb(
    file_base64: &str,
    push_target: &str,
    file_name: &str,
) -> Result<String, String> {
    // 临时文件生成
    let root_path = env_util::get_process_root_path()?;
    // 临时文件路径
    let tmpe_file_name = format!("{}\\{}", root_path, file_name);

    // adb推送到设的目标路径
    let adb_file_path = format!("{}/{}", push_target, file_name);

    // 解码base64获取文件数据
    let file_data = general_purpose::STANDARD
        .decode(file_base64)
        .map_err(|e| e.to_string())?;

    // 创建临时文件
    let temp_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&tmpe_file_name);
    if let Err(e) = temp_file {
        app_log!("临时创建失败： {:?}", e);
        return Err(e.to_string());
    }

    let mut temp_file = temp_file.unwrap();

    // 清空文件内容
    if let Err(e) = temp_file.set_len(0) {
        app_log!("临时文件清除失败： {:?}", e);
        return Err(e.to_string());
    };

    if let Err(e) = temp_file.write_all(&file_data) {
        app_log!("写入临时文件失败： {:?}", e);
        return Err(format!("写入临时文件失败： {:?}", e));
    }
    app_log!("临时文件已创建： {:?}", tmpe_file_name);
    let output_str = run_adb_command(vec!["push", &tmpe_file_name, &adb_file_path])?;
    // 启动线程异步延时删除临时文件
    thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(10));
        match std::fs::remove_file(&tmpe_file_name) {
            Ok(_) => app_log!("删除临时文件成功: {:?}", tmpe_file_name),
            Err(e) => {
                app_log!("删除临时文件失败： {:?}", e);
            }
        }
    });

    if output_str.contains("adb: error:") {
        return Err(format!("推送失败：{:?}", output_str));
    }
    app_log!("推送文件至路径: {:?}", adb_file_path);
    Ok(format!("已推送文件： {:?}", adb_file_path))
}

///推送文本到设备当前的焦点
/// ```
/// push_text_to_adb("你好")
/// ```
#[tauri::command]
pub fn push_text_to_adb(text: &str) -> Result<String, String> {
    Ok(text.to_string())
}

/**
 * 将新的进程状态添加到进程列表中。
 *
 * 此函数遍历当前的进程列表，寻找一个未使用的进程ID，如果找到，则为新进程分配这个ID，并将新进程的信息添加到列表中。
 *
 * # 参数
 * - `process_list` 指向ScrcpyInfo类型Vec的可变引用，代表当前的进程列表。
 * - `process_handle` 新进程的句柄。
 * # 返回值
 * - `Result<u8, u8>`` 成功时返回新分配的进程ID（作为Ok的一部分），如果无法分配新ID（例如，所有ID都已使用），则返回错误码0。
 */
pub fn push_state(process_list: &mut Vec<ScrcpyInfo>, process_handle: usize) -> Result<u8, u8> {
    // 获取当前进程列表中所有进程的ID
    let process_ids: Vec<u8> = process_list.iter().map(|x| x.process_id).collect();

    // 试图找到一个未使用的进程ID
    let mut process_id: u8 = 0;
    for new_id in 1..100 {
        if !process_ids.contains(&new_id) {
            process_id = new_id;
            break;
        }
    }

    // 如果找到了未使用的ID，则添加新进程到列表中
    if process_id != 0 {
        let _scrcpy_info = ScrcpyInfo {
            process_id,
            process_handle,
            stdout_handle: 0,
        };
        // 添加新进程信息到列表
        process_list.push(_scrcpy_info);
        return Ok(process_id);
    }
    // 如果没有未使用的ID，返回错误码0
    Err(0 as u8)
}

fn get_keyborad_defult_tap() -> HashMap<String, (i32, i32)> {
    let mut map = HashMap::new();
    map.insert("KeyPress_KeyQ".to_string(), (70, 1743));
    map.insert("KeyPress_KeyW".to_string(), (170, 1743));
    map.insert("KeyPress_KeyE".to_string(), (270, 1743));
    map.insert("KeyPress_KeyR".to_string(), (370, 1743));
    map.insert("KeyPress_KeyT".to_string(), (470, 1743));
    map.insert("KeyPress_KeyY".to_string(), (570, 1743));
    map.insert("KeyPress_KeyU".to_string(), (670, 1743));
    map.insert("KeyPress_KeyI".to_string(), (770, 1743));
    map.insert("KeyPress_KeyO".to_string(), (870, 1743));
    map.insert("KeyPress_KeyP".to_string(), (970, 1743));
    map.insert("KeyPress_KeyA".to_string(), (120, 1885));
    map.insert("KeyPress_KeyS".to_string(), (220, 1885));
    map.insert("KeyPress_KeyD".to_string(), (320, 1885));
    map.insert("KeyPress_KeyF".to_string(), (420, 1885));
    map.insert("KeyPress_KeyG".to_string(), (520, 1885));
    map.insert("KeyPress_KeyH".to_string(), (620, 1885));
    map.insert("KeyPress_KeyJ".to_string(), (720, 1885));
    map.insert("KeyPress_KeyK".to_string(), (820, 1885));
    map.insert("KeyPress_KeyL".to_string(), (920, 1885));
    map.insert("KeyPress_KeyZ".to_string(), (220, 2030));
    map.insert("KeyPress_KeyX".to_string(), (320, 2030));
    map.insert("KeyPress_KeyC".to_string(), (420, 2030));
    map.insert("KeyPress_KeyV".to_string(), (520, 2030));
    map.insert("KeyPress_KeyB".to_string(), (620, 2030));
    map.insert("KeyPress_KeyN".to_string(), (720, 2030));
    map.insert("KeyPress_KeyM".to_string(), (820, 2030));
    map.insert("KeyPress_Backspace".to_string(), (1020, 2030));
    map.insert("KeyPress_Space".to_string(), (540, 2156));
    map
}

#[tauri::command]
pub fn enable_keyborad_input() -> Result<(), String> {
    app_log!("开始注册映射 ");
    let keyborad_map = get_keyborad_defult_tap();
    for (event_name, point) in keyborad_map.iter() {
        if let Some(event_type) = string_to_keyevent_type(event_name) {
            let x = point.0.clone();
            let y = point.1.clone();
            add_event_listener(
                event_type,
                EventCallback::new(move |event| {
                    app_log!("触发事件：{:?}", event);
                    let _ = adb::tap(x, y);
                }),
            );
        }
    }
    Ok(())
}

#[tauri::command]
pub fn disable_keybord_input() -> Result<(), String> {
    app_log!("清空注册映射 ");
    let keyborad_map = get_keyborad_defult_tap();
    for (event_name, _) in keyborad_map.iter() {
        if let Some(event_type) = string_to_keyevent_type(event_name) {
            remove_event_listener(event_type);
        }
    }
    Ok(())
}
