use encoding::{all, decode, DecoderTrap};

use crate::{util::env_util, windows_interface::hidden_proecss::execute_hidden_cmd};
use std::process::Command;
#[cfg(test)]
mod tests {
    use super::{get_keypoint, run_adb_command_by_adbpath};

    ///
    /// adb shell input text "android"
    /// adb shell input keyevent <key_code>
    /// adb shell input swipe x1 y1 x2 y2 [duration(ms)]
    /// adb shell input tap x y
    ///
    #[test]
    fn test() {
        key_event("z");
        key_event("x");
        key_event("c");
        key_event("v");
        key_event("b");
        key_event("n");
        key_event("m");
    }

    fn key_event(key: &str) {
        let key_porint = get_keypoint(key);
        match key_porint {
            Ok(point) => {
                let _ = run_adb_command_by_adbpath(
                    "D:\\madeinme\\rustHello\\toll\\src-tauri\\target\\debug\\plugin\\scrcpy\\adb.exe",
                    vec![
                        "shell",
                        "input",
                        "tap",
                        &point.0.to_string(),
                        &point.1.to_string(),
                    ],
                );
            }
            Err(e) => {
                let _ = run_adb_command_by_adbpath(
                    "D:\\madeinme\\rustHello\\toll\\src-tauri\\target\\debug\\plugin\\scrcpy\\adb.exe",
                    vec![
                        "shell",
                        "input",
                        "text",
                        key
                    ],
                );
            }
        }
    }
}

/// 执行adb命令
/// ```
/// run_adb_command(vec!["push","D:\\音阶图.png","/sdcard/Download/WeiXin"])?;
/// ```
pub fn run_adb_command(args: Vec<&str>) -> Result<String, String> {
    // 获取当前执行程序的根目录
    let mut current_exe_path: String = env_util::get_process_root_path()?;
    current_exe_path.push_str("\\plugin\\scrcpy\\adb.exe");
    run_adb_command_by_adbpath(&current_exe_path, args)
}

/// 执行adb命令
/// ```
/// run_adb_command_by_adbpath("adb.exe",vec!["push","D:\\音阶图.png","/sdcard/Download/WeiXin"])?;
/// ```
pub fn run_adb_command_by_adbpath(adb_path: &str, args: Vec<&str>) -> Result<String, String> {
    let mut cmd = Command::new(adb_path);
    for arg in args {
        cmd.arg(arg);
    }
    let output = cmd.output();
    app_log!("adb命令：{:?}", cmd);
    if let Err(e) = output {
        app_log!("adb命令：执行失败： {:?}", e);
        return Err(e.to_string());
    }
    let output = output.expect("获取命令输出失败").stdout; //// 获取命令的标准输出
    let (result, _) = decode(
        &output,
        DecoderTrap::Strict,
        all::UTF_8 as encoding::EncodingRef,
    );
    let output_str = result?;
    if output_str.contains("adb: error:") {
        app_log!("adb命令执行成功但存在错误信息: {:?}", output_str);
        return Err(output_str);
    }
    Ok(output_str)
}

pub fn hidden_run_adb_command(args: Vec<&str>) -> Result<String, String> {
    // 获取当前执行程序的根目录
    let mut current_exe_path: String = env_util::get_process_root_path()?;
    current_exe_path.push_str("\\plugin\\scrcpy\\adb.exe");
    for arg in args {
        current_exe_path.push_str(" ");
        current_exe_path.push_str(arg);
    }
    match execute_hidden_cmd(&current_exe_path) {
        Ok(_) => Ok(String::new()),
        Err(e) => Err(e.to_string()),
    }
}

///
/// 模拟点击屏幕
/// ### 参数
/// <p>x: x轴坐标</p>
/// <p>y: y轴坐标</p>
///
/// ```
/// let r = tap(10,300)
/// ```
pub fn tap(x: i32, y: i32) -> Result<String, String> {
    hidden_run_adb_command(vec![
        "shell",
        "input",
        "tap",
        &x.to_string(),
        &y.to_string(),
    ])
}

pub fn get_keypoint(key: &str) -> Result<(i32, i32), String> {
    if "Q" == key.to_uppercase().to_uppercase() {
        return Ok((70, 1743));
    }
    if "W" == key.to_uppercase() {
        return Ok((170, 1743));
    }
    if "E" == key.to_uppercase() {
        return Ok((270, 1743));
    }
    if "R" == key.to_uppercase() {
        return Ok((370, 1743));
    }
    if "T" == key.to_uppercase() {
        return Ok((470, 1743));
    }
    if "Y" == key.to_uppercase() {
        return Ok((570, 1743));
    }
    if "U" == key.to_uppercase() {
        return Ok((670, 1743));
    }
    if "I" == key.to_uppercase() {
        return Ok((770, 1743));
    }
    if "O" == key.to_uppercase() {
        return Ok((870, 1743));
    }
    if "P" == key.to_uppercase() {
        return Ok((970, 1743));
    }
    if "A" == key.to_uppercase() {
        return Ok((120, 1885));
    }
    if "S" == key.to_uppercase() {
        return Ok((220, 1885));
    }
    if "D" == key.to_uppercase() {
        return Ok((320, 1885));
    }
    if "F" == key.to_uppercase() {
        return Ok((420, 1885));
    }
    if "G" == key.to_uppercase() {
        return Ok((520, 1885));
    }
    if "H" == key.to_uppercase() {
        return Ok((620, 1885));
    }
    if "J" == key.to_uppercase() {
        return Ok((720, 1885));
    }
    if "K" == key.to_uppercase() {
        return Ok((820, 1885));
    }
    if "L" == key.to_uppercase() {
        return Ok((920, 1885));
    }
    if "Z" == key.to_uppercase() {
        return Ok((220, 2030));
    }
    if "X" == key.to_uppercase() {
        return Ok((320, 2030));
    }
    if "C" == key.to_uppercase() {
        return Ok((420, 2030));
    }
    if "V" == key.to_uppercase() {
        return Ok((520, 2030));
    }
    if "B" == key.to_uppercase() {
        return Ok((620, 2030));
    }
    if "N" == key.to_uppercase() {
        return Ok((720, 2030));
    }
    if "M" == key.to_uppercase() {
        return Ok((820, 2030));
    }
    if "BACK" == key.to_uppercase() {
        return Ok((920, 2030));
    }
    if "KG" == key.to_uppercase() {
        return Ok((540, 2156));
    }
    Err(key.to_string())
}
