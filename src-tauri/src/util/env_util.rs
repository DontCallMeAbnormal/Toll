use std::os::windows::process::CommandExt;
use std::{env, process::Command};
use winapi::um::winbase::CREATE_NO_WINDOW;

use crate::windows_interface;
pub fn get_process_root_path() -> Result<String, String> {
    // 获取当前执行程序的根目录
    let root_dir = env::current_exe().expect("获取程序运行目录失败");
    let root_dir = root_dir.parent();
    match root_dir {
        Some(path_buf) => {
            if let Some(current_exe_path) = path_buf.as_os_str().to_str() {
                return Ok(current_exe_path.to_string());
            }
            return Err("获取程序运行目录失败".to_string());
        }
        None => Err("获取程序运行目录失败".to_string()),
    }
}

/// 创建一个根命令
///
/// 此函数负责构建一个Command实例，该实例代表了将要执行的根命令。根命令通常是指程序启动时
/// 需要运行的主命令，它不依赖于任何其他命令或选项，是命令行程序运行的基础。
///
/// # 参数:
/// -`process_path`: 指定将要执行的程序路径。是相对于程序运行目录的路径
/// -`param`: 程序使用的参数
///
/// # 返回值:
/// 如果命令构建成功，则返回一个Command命令字符串
///
pub fn build_root_command(process_path: &str, param: &str) -> Result<String, String> {
    // 初始化cmd命令字符串
    let mut cmd_command = String::new();

    // 获取当前执行程序的根目录
    let current_exe_path = get_process_root_path()?;
    // 构造scrcpy执行文件的完整路径
    let mut commd_path = String::new();
    commd_path.push_str("\"");
    commd_path.push_str(&current_exe_path);
    commd_path.push_str("\\");
    commd_path.push_str(process_path);
    commd_path.push_str("\"");

    cmd_command.push_str(&commd_path);
    cmd_command.push_str(" ");
    cmd_command.push_str(param);
    Ok(cmd_command)
}
