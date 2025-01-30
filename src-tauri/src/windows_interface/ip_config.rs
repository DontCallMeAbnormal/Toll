use std::{os::windows::process::CommandExt, process::Command};

use super::hidden_proecss::std_to_string;
use regex::Regex;
use serde::{Deserialize, Serialize};
use winapi::um::winbase::CREATE_NO_WINDOW;

#[derive(Debug, Deserialize, Serialize)]
pub struct AdapterInfo {
    pub name: String,
    pub ip_address: String,
    pub subnet_mask: String,
    pub gateway: String,
}

impl AdapterInfo {
    pub fn new(name: &str, ipadd: &str, mask: &str, gate: &str) -> AdapterInfo {
        AdapterInfo {
            name: name.to_string(),
            ip_address: ipadd.to_string(),
            subnet_mask: mask.to_string(),
            gateway: gate.to_string(),
        }
    }
}

#[tauri::command]
pub async fn get_all_adapter_info() -> Result<Vec<AdapterInfo>, String> {
    // 注意：这个示例仅适用于Windows，并且依赖于'ipconfig'命令的输出格式
    let output = Command::new("ipconfig")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["/all"])
        .output()
        .expect("获取网络信息失败!!")
        .stdout; // 获取命令的标准输出
                 // 将输出转换为字符串
    let output_str = std_to_string(&output);
    app_log!("output {}", output_str);
    let mut adapter_info_str = String::new();
    // 提取IP地址和子网掩码
    let mut lines = output_str.lines();
    while let Some(line) = lines.next() {
        if line.contains("描述") {
            let miao_su: Vec<String> = line.split(" : ").map(|x| x.replace(". ", "")).collect();
            let empty_str = String::from("");
            let miao_su = miao_su.get(1).unwrap_or(&empty_str);

            if adapter_info_str.len() > 0 && !adapter_info_str.ends_with("$end$") {
                adapter_info_str.push_str("$end$");
            }
            adapter_info_str.push_str(miao_su);
            adapter_info_str.push_str("$|$");
        } else if line.contains("IPv4") {
            adapter_info_str.push_str(&extract_ip_address(line));
            adapter_info_str.push_str("$|$");
        } else if line.contains("子网掩码") {
            adapter_info_str.push_str(&extract_subnet_mask(line));
            adapter_info_str.push_str("$|$");
        } else if line.contains("默认网关") {
            let gateway_str = extract_subnet_subnet(line);
            if gateway_str.len() > 0 {
                adapter_info_str.push_str(&gateway_str);
            } else {
                let next_line = lines.next();
                if next_line.is_some() {
                    let next_line = next_line.unwrap();
                    let re = Regex::new(r"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();
                    // 提取IP地址
                    if let Some(captures) = re.captures(next_line) {
                        adapter_info_str.push_str(&captures.get(0).unwrap().as_str().to_string());
                    }
                }
            }
            adapter_info_str.push_str("$|$");
        }
    }

    let mut adapter_list = Vec::new();

    let split_end: Vec<&str> = adapter_info_str.split("$end$").collect();
    for item in split_end {
        let split_group: Vec<&str> = item.split("$|$").collect();
        let name = *split_group.get(0).unwrap_or(&"");
        let ip_addr = *split_group.get(1).unwrap_or(&"");
        let mask = *split_group.get(2).unwrap_or(&"");
        let gateway = *split_group.get(3).unwrap_or(&"");
        if name.len() > 0 && ip_addr.len() > 0 && mask.len() > 0 {
            adapter_list.push(AdapterInfo::new(name, ip_addr, mask, gateway));
        }
    }

    Ok(adapter_list)
}

#[tauri::command]
pub fn flush_dns() -> Result<String, String> {
    let out = Command::new("ipconfig")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["/flushdns"])
        .output()
        .expect("Failed to execute command");
    if out.status.success() {
        let chars = std_to_string(&out.stdout);
        app_log!("flush_dns:{}", chars);
        return Ok(chars);
    } else {
        // 处理失败的情况
        return Err("Failed to execute command".to_string());
    }
}
fn extract_ip_address(output: &str) -> String {
    // 匹配IP地址的正则表达式
    let re = Regex::new(r"IPv4 地址( )*(\. )*:\s*(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();

    // 提取IP地址
    if let Some(captures) = re.captures(output) {
        return captures.get(3).unwrap().as_str().to_string();
    }
    String::from("")
}

fn extract_subnet_mask(output: &str) -> String {
    // 匹配子网掩码的正则表达式
    let re = Regex::new(r"子网掩码( )*(\. )*:\s*(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();

    // 提取子网掩码
    if let Some(captures) = re.captures(output) {
        return captures.get(3).unwrap().as_str().to_string();
    }
    String::from("")
}

fn extract_subnet_subnet(output: &str) -> String {
    // 匹配默认网关的正则表达式
    let re = Regex::new(r"默认网关( )*(\. )*:\s*(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();

    // 提取默认网关
    if let Some(captures) = re.captures(output) {
        return captures.get(3).unwrap().as_str().to_string();
    }
    String::from("")
}
