use encoding::all;
use encoding::decode;
use encoding::DecoderTrap;
use std::ffi::OsStr;
use std::iter;
use std::os::windows::ffi::OsStrExt;
use std::process::Command;
use std::ptr;
use std::time::Duration;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::FALSE;
use winapi::shared::minwindef::TRUE;
use winapi::shared::minwindef::WORD;
use winapi::um::handleapi::CloseHandle;
use winapi::um::minwinbase::OVERLAPPED;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::minwinbase::STILL_ACTIVE;
use winapi::um::processthreadsapi::{
    CreateProcessW, GetExitCodeProcess, TerminateProcess, LPPROCESS_INFORMATION, LPSTARTUPINFOW,
    PROCESS_INFORMATION, STARTUPINFOW,
};
use winapi::um::winbase::CREATE_NO_WINDOW;
use winapi::um::winbase::CREATE_UNICODE_ENVIRONMENT;
use winapi::um::winnt::{HANDLE, LPCWSTR, LPWSTR};
use winapi::um::winuser::SW_SHOW;
use std::os::windows::process::CommandExt;
#[allow(dead_code)]
pub fn execute_hidden_cmd(cmd_command: &str) -> Result<HANDLE, String> {
    app_log!("{:?}", cmd_command);
    unsafe {
        // 设置STARTUPINFO结构体
        let mut si: STARTUPINFOW = std::mem::zeroed();
        si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
        si.wShowWindow = SW_SHOW as WORD;

        let command_wide: Vec<u16> = OsStr::new(cmd_command)
            .encode_wide()
            .chain(iter::once(0))
            .collect();

        // 进程信息结构体
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();
        //CREATE_NO_WINDOW
        let result = CreateProcessW(
            std::ptr::null_mut() as LPCWSTR,
            command_wide.as_ptr() as LPWSTR, // 从*mut c_char 更改为 *mut wchar_t
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            CREATE_NO_WINDOW | CREATE_UNICODE_ENVIRONMENT,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut si as LPSTARTUPINFOW,
            &mut pi as LPPROCESS_INFORMATION,
        );

        std::thread::sleep(Duration::from_secs(1));

        if result == FALSE {
            app_log!("进程创建失败");
            return Err(String::from("进程创建失败"));
        }

        Ok(pi.hProcess)
    }
}

pub fn close_process(process_id: HANDLE) -> Result<bool, String> {
    // 在适当的时候终止进程
    let result = unsafe { TerminateProcess(process_id, 0) };
    if result == 0 {
        return Err(String::from("错误无法终止进程或进程已被关闭"));
    }
    // 关闭进程句柄
    let close_result = unsafe { CloseHandle(process_id) };
    if close_result == 0 {
        return Err(String::from("错误无法关闭进程句柄或进程已被关闭"));
    }
    Ok(true)
}

// 获取进程运行状态如果是运行中则返回true否则返回false
pub fn get_process_status(process_id: usize) -> bool {
    let mut exit_code: DWORD = 0;
    unsafe {
        if GetExitCodeProcess(process_id as HANDLE, &mut exit_code) != 0 {
            if exit_code == STILL_ACTIVE {
                return true;
            } else {
                return false;
            }
        }
    }
    // 如果无法获取退出码，认为进程不活跃。
    return false;
}

// 创建管道并获取句柄
fn create_pipe() -> (HANDLE, HANDLE) {
    use winapi::shared::ntdef::NULL;
    use winapi::um::namedpipeapi::CreatePipe;

    let mut sa: SECURITY_ATTRIBUTES = unsafe { std::mem::zeroed() };
    sa.nLength = std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32;
    sa.bInheritHandle = TRUE;
    let mut r: HANDLE = ptr::null_mut();
    let mut w: HANDLE = ptr::null_mut();
    unsafe {
        CreatePipe(&mut r, &mut w, &mut sa, 0);
        if r == NULL || w == NULL {
            panic!("Failed to create pipe");
        }
        (r, w)
    }
}

/// 读取管道数据并返回
pub fn read_pipe(ptr: HANDLE) -> String {
    let buffer: Vec<u8> = read_pipe_bytes(ptr);
    std_to_string(&buffer)
}

/// 读取管道数据并返回
pub fn read_pipe_bytes(ptr: HANDLE) -> Vec<u8> {
    let mut buffer = [0u8; 4096];
    let mut read_bytes = 0;
    let mut sa: OVERLAPPED = unsafe { std::mem::zeroed() };
    loop {
        let success = unsafe {
            winapi::um::fileapi::ReadFile(
                ptr,
                buffer.as_mut_ptr() as *mut _,
                buffer.len() as u32,
                &mut read_bytes,
                &mut sa,
            )
        };
        println!("{:?}", read_bytes);
        if !(success == TRUE) || read_bytes == 0 {
            break;
        }
    }
    let buffer: Vec<u8> = buffer
        .iter()
        .filter(|predicate| **predicate != 0)
        .map(|f| *f)
        .collect();
    buffer
}

/// 将标准输出转换为字符串
pub fn std_to_string(stdout: &Vec<u8>) -> String {
    let (result, _) = decode(
        stdout,
        DecoderTrap::Strict,
        all::GBK as encoding::EncodingRef,
    );
    let mut output = String::new();
    output.push_str(&result.unwrap_or(String::new()));
    output.trim_end().into()
}

/// 执行命令，不显示窗口
/// cmd: 命令
/// 返回值：执行结果
/// 示例：
/// cmd_exec_no_window("ipconfig /all")
pub fn cmd_exec_no_window(cmd: &str) -> Result<String, String> {
    let output = Command::new("cmd")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["/C", cmd])
        .output();
    if let Err(err) = output {
        app_log!("Failed to execute command: {}", err);
        return Err(format!("Failed to execute command: {}", err));
    }
    let output = output.unwrap();
    if output.status.success() {
        let chars = std_to_string(&output.stdout);
        app_log!("cmd_exec_no_window:{}", chars);
        return Ok(chars);
    } else {
        let chars = std_to_string(&output.stderr);
        app_log!("Failed cmd_exec_no_window:{}", chars);
        return Err(chars);
    }
}