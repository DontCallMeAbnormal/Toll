use std::{os::windows::process::CommandExt, process::Command};

use super::hidden_proecss::std_to_string;
use serde::{Deserialize, Serialize};
use winapi::um::winbase::CREATE_NO_WINDOW;

#[derive(Debug, Deserialize, Serialize)]
pub struct RouteInfo {
    pub net_taget: String,     // 网络目标
    pub subnet_mask: String,   // 子网掩码
    pub gateway: String,       // 网关
    pub net_interface: String, // 接口地址
}
impl RouteInfo {
    pub fn new(net_taget: &str, mask: &str, gateway: &str, net_interface: &str) -> RouteInfo {
        RouteInfo {
            net_taget: net_taget.to_string(),
            subnet_mask: mask.to_string(),
            gateway: gateway.to_string(),
            net_interface: net_interface.to_string(),
        }
    }
}

#[tauri::command]
pub async fn get_route_info(gateway: &str, ip_addr: &str) -> Result<Vec<RouteInfo>, String> {
    let output = Command::new("route")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["print", "0.0.0.0", "mask", "0.0.0.0", gateway])
        .output()
        .unwrap();

    let mut route_list = Vec::new();
    if output.status.success() {
        let result_str = std_to_string(&output.stdout);
        for item in result_str.lines() {
            if item.contains(gateway) && item.contains(ip_addr) {
                let item: Vec<&str> = item.split(" ").filter(|x| (*x).len() > 0).collect();
                let net_taget = *item.get(0).unwrap_or(&"");
                let mask = *item.get(1).unwrap_or(&"");
                let gateway = *item.get(2).unwrap_or(&"");
                let net_interface = *item.get(3).unwrap_or(&"");
                route_list.push(RouteInfo::new(net_taget, mask, gateway, net_interface));
            }
        }
    }
    Ok(route_list)
}

#[tauri::command]
pub async fn delete_route(route_info: RouteInfo) -> Result<String, String> {
    let output = Command::new("route")
        .creation_flags(CREATE_NO_WINDOW)
        .args([
            "delete",
            &route_info.net_taget,
            "mask",
            &route_info.subnet_mask,
            &route_info.gateway,
        ])
        .output();
    match output {
        Ok(out) => {
            if out.status.success() {
                return Ok(String::from("操作成功"));
            } else {
                return Err(std_to_string(&out.stdout));
            }
        }
        Err(e) => Err(String::from(e.to_string())),
    }
}
#[tauri::command]
pub fn add_route(route_info: RouteInfo) -> Result<String, String> {
    let output = Command::new("route")
        .creation_flags(CREATE_NO_WINDOW)
        .args([
            "add",
            &route_info.net_taget,
            "mask",
            &route_info.subnet_mask,
            &route_info.gateway,
        ])
        .output();
    match output {
        Ok(out) => {
            if out.status.success() {
                return Ok(String::from("操作成功"));
            } else {
                return Err(std_to_string(&out.stdout));
            }
        }
        Err(e) => Err(String::from(e.to_string())),
    }
}
