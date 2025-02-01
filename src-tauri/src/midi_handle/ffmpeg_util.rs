use crate::{util::env_util, windows_interface::{self, hidden_proecss}};



//更具视频路径，调用ffmpeg命令，获取视频时长    
pub fn get_video_duration(video_path: &str) -> Result<u64, String> {
    //ffmpeg -i input.mp4 2>&1 | grep Duration
    let param_str = vec![
        "-i",
        video_path
    ];
    let cmd_str = env_util::build_root_command_arg("plugin\\ffmpeg\\bin\\ffmpeg.exe", Box::new(param_str))?;
    
    let output = hidden_proecss::cmd_exec_no_window(&cmd_str);

    if let Err(stderr) = output {
        let lines: Vec<_> = stderr.lines().collect();

        for line in lines {
            if line.contains("Duration:") {
                app_log!("line : {}", line);
                let play_time = line.split(',').find_map(|s| {
                    if s.contains("Duration:") {
                        return s.trim().split(' ').nth(1);
                    }
                    None
                });
                if let Some(play_time) = play_time {
                    // 解析播放时间并返回总毫秒数
                    let total_milliseconds = time_to_milliseconds(play_time)?;
                    return Ok(total_milliseconds);
                }
                return Err("Failed to get video duration".to_string());
            }
        }
    }
    Err("Failed to exec cmd".to_string())
}

pub fn time_to_milliseconds(play_time: &str) -> Result<u64, String> {
    // 按冒号和小数点分割字符串
    let play_time: Vec<&str> = play_time.split(&[':', '.'][..]).collect();

    // 检查分割后的部分是否符合预期
    if play_time.len() != 4 {
        return Err("Invalid time format. Expected format: HH:MM:SS.ss".to_string());
    }

    // 解析小时、分钟、秒和百分之一秒
    let hours: u64 = play_time[0].parse().map_err(|_| "Failed to parse hours")?;
    let minutes: u64 = play_time[1].parse().map_err(|_| "Failed to parse minutes")?;
    let seconds: u64 = play_time[2].parse().map_err(|_| "Failed to parse seconds")?;
    let centiseconds: u64 = play_time[3].parse().map_err(|_| "Failed to parse centiseconds")?;

    // 计算总毫秒数
    Ok((hours * 3600 + minutes * 60 + seconds) * 1000 + centiseconds * 10)
}