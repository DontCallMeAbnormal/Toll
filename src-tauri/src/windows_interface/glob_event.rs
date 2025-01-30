use lazy_static::lazy_static;
use rdev::{grab, Event, EventType};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// 定义一个结构体来包装闭式函数
pub struct EventCallback(Box<dyn Fn(Event) + Send + Sync + 'static>);

impl EventCallback {
    pub fn new(callback: impl Fn(Event) + Send + Sync + 'static) -> Self {
        EventCallback(Box::new(callback))
    }
}

lazy_static! {
    static ref ENVENT_MAP: Arc<Mutex<HashMap<String, EventCallback>>> = {
        let map = HashMap::new();
        Arc::new(Mutex::new(map))
    };
}

/// 将事件类型转换为描述性字符串
///
/// 本函数通过匹配不同的事件类型，返回一个描述该事件的字符串
/// 这对于调试和日志记录等场景非常有用
///
/// 参数:
/// - `event_type`: 输入的事件类型
///
/// 返回值:
/// - 一个描述事件类型的字符串
fn event_type_to_string(event_type: EventType) -> String {
    match event_type {
        EventType::KeyPress(key) => format!("KeyPress: {:?}", key),
        EventType::KeyRelease(key) => format!("KeyRelease: {:?}", key),
        EventType::MouseMove { x, y } => format!("MouseMove: ({}, {})", x, y),
        EventType::ButtonPress(button) => format!("ButtonPress: {:?}", button),
        EventType::ButtonRelease(button) => format!("ButtonRelease: {:?}", button),
        EventType::Wheel { delta_x, delta_y } => format!("Wheel: ({}, {})", delta_x, delta_y),
    }
}

pub fn string_to_keyevent_type(event_type: &str) -> Option<EventType> {
    if event_type.starts_with("Key") {
        let keyname: Vec<&str> = event_type.split("_").collect();
        // 将keyname转换为枚举类型rdev::Key
        let key: rdev::Key = match *(keyname.get(1).unwrap()) {
            "Alt" => rdev::Key::Alt,
            "AltGr" => rdev::Key::AltGr,
            "Backspace" => rdev::Key::Backspace,
            "CapsLock" => rdev::Key::CapsLock,
            "ControlLeft" => rdev::Key::ControlLeft,
            "ControlRight" => rdev::Key::ControlRight,
            "Delete" => rdev::Key::Delete,
            "DownArrow" => rdev::Key::DownArrow,
            "End" => rdev::Key::End,
            "Escape" => rdev::Key::Escape,
            "F1" => rdev::Key::F1,
            "F10" => rdev::Key::F10,
            "F11" => rdev::Key::F11,
            "F12" => rdev::Key::F12,
            "F2" => rdev::Key::F2,
            "F3" => rdev::Key::F3,
            "F4" => rdev::Key::F4,
            "F5" => rdev::Key::F5,
            "F6" => rdev::Key::F6,
            "F7" => rdev::Key::F7,
            "F8" => rdev::Key::F8,
            "F9" => rdev::Key::F9,
            "Home" => rdev::Key::Home,
            "LeftArrow" => rdev::Key::LeftArrow,
            "MetaLeft" => rdev::Key::MetaLeft,
            "MetaRight" => rdev::Key::MetaRight,
            "PageDown" => rdev::Key::PageDown,
            "PageUp" => rdev::Key::PageUp,
            "Return" => rdev::Key::Return,
            "RightArrow" => rdev::Key::RightArrow,
            "ShiftLeft" => rdev::Key::ShiftLeft,
            "ShiftRight" => rdev::Key::ShiftRight,
            "Space" => rdev::Key::Space,
            "Tab" => rdev::Key::Tab,
            "UpArrow" => rdev::Key::UpArrow,
            "PrintScreen" => rdev::Key::PrintScreen,
            "ScrollLock" => rdev::Key::ScrollLock,
            "Pause" => rdev::Key::Pause,
            "NumLock" => rdev::Key::NumLock,
            "BackQuote" => rdev::Key::BackQuote,
            "Num1" => rdev::Key::Num1,
            "Num2" => rdev::Key::Num2,
            "Num3" => rdev::Key::Num3,
            "Num4" => rdev::Key::Num4,
            "Num5" => rdev::Key::Num5,
            "Num6" => rdev::Key::Num6,
            "Num7" => rdev::Key::Num7,
            "Num8" => rdev::Key::Num8,
            "Num9" => rdev::Key::Num9,
            "Num0" => rdev::Key::Num0,
            "Minus" => rdev::Key::Minus,
            "Equal" => rdev::Key::Equal,
            "KeyQ" => rdev::Key::KeyQ,
            "KeyW" => rdev::Key::KeyW,
            "KeyE" => rdev::Key::KeyE,
            "KeyR" => rdev::Key::KeyR,
            "KeyT" => rdev::Key::KeyT,
            "KeyY" => rdev::Key::KeyY,
            "KeyU" => rdev::Key::KeyU,
            "KeyI" => rdev::Key::KeyI,
            "KeyO" => rdev::Key::KeyO,
            "KeyP" => rdev::Key::KeyP,
            "LeftBracket" => rdev::Key::LeftBracket,
            "RightBracket" => rdev::Key::RightBracket,
            "KeyA" => rdev::Key::KeyA,
            "KeyS" => rdev::Key::KeyS,
            "KeyD" => rdev::Key::KeyD,
            "KeyF" => rdev::Key::KeyF,
            "KeyG" => rdev::Key::KeyG,
            "KeyH" => rdev::Key::KeyH,
            "KeyJ" => rdev::Key::KeyJ,
            "KeyK" => rdev::Key::KeyK,
            "KeyL" => rdev::Key::KeyL,
            "SemiColon" => rdev::Key::SemiColon,
            "Quote" => rdev::Key::Quote,
            "BackSlash" => rdev::Key::BackSlash,
            "IntlBackslash" => rdev::Key::IntlBackslash,
            "KeyZ" => rdev::Key::KeyZ,
            "KeyX" => rdev::Key::KeyX,
            "KeyC" => rdev::Key::KeyC,
            "KeyV" => rdev::Key::KeyV,
            "KeyB" => rdev::Key::KeyB,
            "KeyN" => rdev::Key::KeyN,
            "KeyM" => rdev::Key::KeyM,
            "Comma" => rdev::Key::Comma,
            "Dot" => rdev::Key::Dot,
            "Slash" => rdev::Key::Slash,
            "Insert" => rdev::Key::Insert,
            "KpReturn" => rdev::Key::KpReturn,
            "KpMinus" => rdev::Key::KpMinus,
            "KpPlus" => rdev::Key::KpPlus,
            "KpMultiply" => rdev::Key::KpMultiply,
            "KpDivide" => rdev::Key::KpDivide,
            "Kp0" => rdev::Key::Kp0,
            "Kp1" => rdev::Key::Kp1,
            "Kp2" => rdev::Key::Kp2,
            "Kp3" => rdev::Key::Kp3,
            "Kp4" => rdev::Key::Kp4,
            "Kp5" => rdev::Key::Kp5,
            "Kp6" => rdev::Key::Kp6,
            "Kp7" => rdev::Key::Kp7,
            "Kp8" => rdev::Key::Kp8,
            "Kp9" => rdev::Key::Kp9,
            "KpDelete" => rdev::Key::KpDelete,
            "Function" => rdev::Key::Function,
            _ => panic!("Invalid key name"),
        };
        if event_type.starts_with("KeyPress") {
            return Some(EventType::KeyPress(key));
        }
        if event_type.starts_with("KeyRelease") {
            return Some(EventType::KeyRelease(key));
        }
    }
    None
}
/// 注册一个事件监听器。
///
/// 此函数用于向全局事件映射中添加一个事件监听器。监听器通过指定的事件类型与一个回调函数关联，
/// 当该事件类型发生时，回调函数将被调用。为了确保线程安全性，回调函数需要实现 `Send`, `Sync`
/// 和 `'static` trait。
///
/// # 参数
/// * `event_type` - 事件的类型，指定了监听器将响应的事件。
/// * `callback` - 一个闭包，当事件发生时将被调用。该闭包必须是 `Send`, `Sync` 和 `'static` 的，
///   以确保其可以在多线程环境中安全地被调用。
pub fn add_event_listener(event_type: EventType, callback: EventCallback) {
    let mut map = ENVENT_MAP.lock().unwrap();
    let event_type = event_type_to_string(event_type);
    map.insert(event_type, callback);
}

/// 移除所有事件监听器
///
/// 此函数会锁定事件映射表 `ENVENT_MAP`，移除其中所有事件与监听器的绑定关系。
/// 它通过获取 `ENVENT_MAP` 的可变锁并清空其内容来实现这一目标。
#[warn(dead_code)]
pub fn remove_all_event_listener() {
    let mut map = ENVENT_MAP.lock().unwrap();
    map.clear();
}

/// 移除指定类型的事件监听器
///
/// 本函数通过事件类型字符串从全局事件映射中移除对应的事件监听器列表
///
/// 参数:
/// - `event_type`: 事件类型，用于标识要移除监听器的事件类型
pub fn remove_event_listener(event_type: EventType) {
    let mut map = ENVENT_MAP.lock().unwrap();
    let event_type = event_type_to_string(event_type);
    map.remove(&event_type);
}

// 启动过滤程序
pub fn start_linstener() {
    // This will block.
    if let Err(error) = grab(callback) {
        app_log!("监听 Error: {:?}", error);
    }
}

fn callback(event: Event) -> Option<Event> {
    // 获取特定类型的事件处理回调
    let binding = ENVENT_MAP.lock().unwrap();
    let s = binding.get(&event_type_to_string(event.event_type));
    if let Some(callback) = s {
        callback.0(event.clone());
        return Some(event);
    }
    Some(event)
}
