use crate::util::env_util;
use encoding::{all, decode, DecoderTrap, EncodingRef};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};
/*
获取程序运行目录
 */
fn get_root_path() -> Result<String, String> {
    let file_name = env_util::get_process_root_path();
    // let file_name = env::var("HOME");
    if let Err(e) = file_name {
        return Err(e.to_string());
    }
    app_log!("{:?}", file_name.clone().unwrap());
    return Ok(file_name.unwrap());
}

#[tauri::command]
pub fn get_text_config(name: &str) -> Result<String, String> {
    let file_name = get_root_path()?;
    // 拼接文件路径和文件名称获取完整文件路径
    let name = format!("{}\\{}", file_name, name);
    match File::open(name) {
        Ok(mut file) => {
            let mut data = vec![];
            let _ = file.read_to_end(&mut data);
            // 将data内容转换为字符串
            let (result, _) = decode(&data, DecoderTrap::Strict, all::UTF_8 as EncodingRef);
            match result {
                Ok(restlt_str) => Ok(restlt_str),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn set_text_config(name: &str, content: &str) -> Result<String, String> {
    let file_name = get_root_path()?;
    // 拼接文件路径和文件名称获取完整文件路径
    let name = format!("{}\\{}", file_name, name);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(name)
        .expect("打开文件出错");
    // 清空文件内容
    if let Err(e) = file.set_len(0) {
        return Err(e.to_string());
    };
    match file.write_all(content.as_bytes()) {
        Ok(_) => Ok("成功".to_string()),
        Err(e) => Err(format!("失败： {:?}", e)),
    }
}
