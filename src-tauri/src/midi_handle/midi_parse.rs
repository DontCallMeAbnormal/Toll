use midly::num::u28;
use midly::{MidiMessage, Smf, TrackEventKind, Timing, MetaMessage};
use serde_json::json;
use std::collections::HashMap;
use tempfile::Builder;
use crate::midi_handle::{self, ffmpeg_util};
use crate::windows_interface::hidden_proecss::{cmd_exec_no_window, cmd_exec_no_window_args};
use crate::util::env_util;

#[tauri::command]
pub async fn parse_midi(file: Vec<u8>,bpm: f64) -> Result<String, String> {
    let smf = Smf::parse(&file).map_err(|e| format!("Failed to parse MIDI file: {}", e))?;

    let ticks_per_beat = match smf.header.timing {
        Timing::Metrical(ticks_per_beat) => ticks_per_beat.as_int(),
        Timing::Timecode(_, _) => return Err("Unsupported time code timing".to_string()),
    };
    let microseconds_per_beat = get_tempo(&smf, bpm); // 获取 tempo (microseconds per beat)

    // 计算每个滴答的微秒数
    let microseconds_per_tick = microseconds_per_beat as f64 / ticks_per_beat as f64;

    let mut notes = Vec::new();

    for track in smf.tracks {
        let mut delta_time: u28 = u28::new(0);
        for event in track.iter() {
            delta_time += event.delta;
            match event.kind {
                TrackEventKind::Midi {
                    // 使用 midly::TrackEventKind
                    channel: _,
                    message,
                } => match message {
                    MidiMessage::NoteOn { key, vel } if vel > 0 => {
                        // 将 start_time (ticks) 转换为毫秒
                        let start_time_milliseconds = (delta_time.as_int() as f64 * microseconds_per_tick) / 1000.0;

                        notes.push(Note {
                            key: key.as_int(),
                            start_time: u28::as_int(delta_time),
                            start_time_seconds: start_time_milliseconds,
                            duration: 0,
                            duration_seconds: 0.0,
                        });
                    }
                    MidiMessage::NoteOff { key, .. } => {
                        if let Some(note) = notes
                            .iter_mut()
                            .find(|n| n.key == key.as_int() && n.duration == 0)
                        {
                            // 计算 duration (ticks) 转换为毫秒
                            let duration_milliseconds = ((delta_time.as_int() as f64 - note.start_time as f64) * microseconds_per_tick) / 1000.0;

                            note.duration = u28::as_int(delta_time) - note.start_time;
                            note.duration_seconds = duration_milliseconds;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    let mut json_notes = notes
        .iter()
        .map(|note| {
            json!({
                "key": note.key,
                "start_time": note.start_time,
                "duration": note.duration,
                "start_time_seconds": note.start_time_seconds,
                "duration_seconds": note.duration_seconds
            })
        })
        .collect::<Vec<_>>();
    // 把music_rhythm按照start_time排序
    json_notes.sort_by(|a, b| {
        a["start_time_seconds"].as_f64().unwrap_or(0.0)
            .partial_cmp(&b["start_time_seconds"].as_f64().unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(json!(json_notes).to_string())
}

#[tauri::command]
pub async fn generate_animation(
    images: Vec<serde_json::Value>,
    file: Vec<u8>,
    bpm: f64,
    video_play_time: String,
    output_path: String, 
) -> Result<String, String> {
    // 创建一个动画数据结构
    let mut animation_data = Vec::new();
    app_log!("video_play_time is  {:?}",video_play_time);
    app_log!("output_path is  {:?}",output_path);
    // 使用parse_midi函数获取music_rhythm
    let mut music_rhythm:Vec<serde_json::Value> = if let Ok(json_obj) = serde_json::from_str(&parse_midi(file, bpm).await?) {
        json_obj
    }else{
        return Err("Failed to parse music_rhythm".to_string());
    };

    // 对music_rhythm进行排序
    music_rhythm.sort_by(|a, b| {
        a["start_time_seconds"].as_f64().unwrap_or(0.0)
            .partial_cmp(&b["start_time_seconds"].as_f64().unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    // 假设我们根据 musicRhythm 和 images 生成动画帧
    for note in music_rhythm.iter() {
        let key = note["key"].as_u64().unwrap_or(0) as u64;
        let start_time = note["start_time_seconds"].as_f64().unwrap_or(0.0) as f64;
        let duration = note["duration_seconds"].as_f64().unwrap_or(0.0) as f64;

        for image in images.iter() {
            let condition = image["condition"].as_str().unwrap_or("");
            let value = image["value"].as_str().unwrap_or("");

            // 根据条件判断是否显示图片
            let should_display = match condition {
                "greater_than" => key > value.parse::<u64>().unwrap_or(0),
                "equal_to" => key == value.parse::<u64>().unwrap_or(0),
                "less_than" => key < value.parse::<u64>().unwrap_or(0),
                "range" => {
                    let range: Vec<u64> = value
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();
                    if range.len() == 2 {
                        key >= range[0] && key <= range[1]
                    } else {
                        false
                    }
                },
                _ => false,
            };

            if should_display {
                animation_data.push(json!({
                    "key": key,
                    "start_time_seconds": start_time,
                    "duration_seconds": duration,
                    "image": image
                }));
            }
        }
    }
    if let Some(any_value_image) = images.iter().find(|predicate| predicate["condition"] == "any_value") {
        // 检查 any_value_image["image"]["url"] 是否存在，并提供默认值或错误处理
        let any_value_image_url = any_value_image["url"].as_str().ok_or("Image URL not found")?;

        // 将视频播放时间转换为毫秒
        let video_play_time = ffmpeg_util::time_to_milliseconds(&video_play_time)?;

        // 生成视频
        generate_video(&animation_data, any_value_image_url,&output_path,video_play_time)?; // 传递 any_value_image_url
        Ok("".to_string())
    } else {
        Err("any_value_image not found".to_string())
    }
}

fn generate_video(animation_data: &Vec<serde_json::Value>, any_value_image_url: &str, output_path: &str, video_play_time: u64) -> Result<(), String> {
    let root_path = env_util::get_process_root_path()?;
    let temp_dir = format!("{}\\target\\temp", root_path);
    // 创建目标临时目录
    let target_temp_dir = std::path::Path::new(&temp_dir);
    if !target_temp_dir.exists() {
        std::fs::create_dir_all(target_temp_dir).map_err(|e| format!("Failed to create target temp dir: {}", e))?;
    }

    // 使用目标临时目录来创建临时文件
    let temp_dir = Builder::new().prefix("midi_frames").tempdir_in(target_temp_dir).map_err(|e| format!("Failed to create temp dir in target: {}", e))?;

    let mut last_frame_time = 0.0;
    let mut count: i64 = 0;
    let mut image_cache: HashMap<String, std::path::PathBuf> = HashMap::new();

    for (i, note) in animation_data.iter().enumerate() {
        let start_time = note["start_time_seconds"].as_f64().unwrap_or(0.0) as f64;
        let duration = note["duration_seconds"].as_f64().unwrap_or(0.0) as f64;
        let image_url = note["image"]["url"].as_str().unwrap_or("");

        // Check cache
        let frame_file = if let Some(cached_file) = image_cache.get(image_url) {
            let intermediate_frame_file = temp_dir.path().join(format!("frame_{:010}.png", count));
            std::fs::copy(&cached_file, &intermediate_frame_file).map_err(|e| format!("Failed to copy frame: {}", e))?;
            cached_file.clone()
        } else {
            let frame_file = temp_dir.path().join(format!("frame_{:010}.png", count));

            let image_data = image_url.trim_start_matches("data:image/png;base64,");
            let image_data = base64::decode(image_data).map_err(|e| format!("Failed to decode base64 image: {}", e))?;
            let image = image::load_from_memory(&image_data).map_err(|e| format!("Failed to load image from memory: {}", e))?;
            let rgba_image = image.to_rgba8();
            rgba_image.save(&frame_file).map_err(|e| format!("Failed to save image: {}", e))?;
            image_cache.insert(image_url.to_string(), frame_file.clone());
            frame_file
        };
        count += 1;
        // Write frames for the duration of the note
        let end_time = start_time + duration;
        last_frame_time = start_time;
        while last_frame_time < end_time {
            let intermediate_frame_file = temp_dir.path().join(format!("frame_{:010}.png", count));
            count += 1;
            if intermediate_frame_file != frame_file {
                std::fs::copy(&frame_file, &intermediate_frame_file).map_err(|e| format!("Failed to copy frame: {}", e))?;
            }
            last_frame_time += 1.0 / 60.0 * 1000.0; // 帧率60 则每帧 (1 / 60 * 1000) 毫秒
        }
        last_frame_time = end_time;
        // 如果不是最后一个音符，插入 any_value 的图片直到下一个音符的开始时间
        if i < animation_data.len() - 1 {
            let next_start_time = animation_data[i + 1]["start_time_seconds"].as_f64().unwrap_or(0.0) as f64;
            while last_frame_time < next_start_time {
                let _any_value_image = get_any_value_image(&temp_dir, count, any_value_image_url, &mut image_cache)?; // 传递 any_value_image_url 和缓存
                count += 1;
                last_frame_time += 1.0 / 60.0 * 1000.0; // 帧率60 则每帧 (1 / 60 * 1000) 毫秒
            }
        }
    }
    app_log!("结束数据准备 一共{:?}帧", count);

    // Construct the ffmpeg command
    let binding = temp_dir.path().join("frame_%010d.png");
    let input_pattern = binding.to_str().unwrap_or("");
    let output_file = format!(r"{}\output_temp.mp4", &output_path);

    

    let cmd_str = env_util::build_root_command("plugin\\ffmpeg\\bin\\ffmpeg.exe", "")?;
    app_log!("cmd_str => {:?}", cmd_str);
    let param_str = vec![
        &cmd_str,
        "-framerate",
        "60",
        "-i",
        &input_pattern,
        "-c:v",
        "libx264",
        "-pix_fmt",
        "yuv420p",
        &output_file,
    ];
    app_log!("生成视频 cmd_str => {:?}", param_str);
    match cmd_exec_no_window_args(param_str) {
        Ok(_) => {
            let video_time = midi_handle::ffmpeg_util::get_video_duration(&output_file);
            if let Ok(v_time) = video_time {
                app_log!("视频时长: {:?}, 需要转换的时长： {:?}", v_time, video_play_time);

                let speed_factor = v_time as f64 / video_play_time as f64;

                let mut cmd_str = String::new();
                let mut  setpts_filter = String::new();
                let mut  atempo_filter = String::new();
                if speed_factor > 1.0 {
                    setpts_filter = format!("setpts=PTS/{}", speed_factor);
                    atempo_filter = format!("atempo={}", speed_factor)
                } else {
                    setpts_filter = format!("setpts=PTS*{}", 1.0 / speed_factor);
                    atempo_filter = format!("atempo=1/{}", speed_factor)
                };
                // 时长转换后的视频存储位置
                let output_trans_file = format!(r"{}\output.mp4", &output_path);
                cmd_str = env_util::build_root_command("plugin\\ffmpeg\\bin\\ffmpeg.exe", "")?;
                let param_str = vec![
                    &cmd_str,
                    "-i",
                    &output_file,
                    "-vf",
                    &setpts_filter,
                    "-af",
                    &atempo_filter,
                    &output_trans_file,
                ];
                app_log!("时长转换： cmd_str => {:?}", param_str);
                match cmd_exec_no_window_args(param_str) {
                    Ok(_) => {
                        app_log!("视频转换成功");
                        // 删除文件output_file
                        let _ = std::fs::remove_file(&output_file);
                        return Ok(());
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Err("无法获取视频时长".to_string())
        },
        Err(e) => {
            Err(e)
        }
    }
}

// 修改 get_any_value_image 函数，使其能够从入参获取 any_value 的图片并使用缓存
fn get_any_value_image(
    temp_dir: &tempfile::TempDir,
    frame_time: i64,
    any_value_image_url: &str,
    image_cache: &mut HashMap<String, std::path::PathBuf>,
) -> Result<std::path::PathBuf, String> {
    let any_value_frame_file = temp_dir.path().join(format!("frame_{:010}.png", frame_time));

    if let Some(cached_file) = image_cache.get(any_value_image_url) {
        std::fs::copy(&cached_file, &any_value_frame_file).map_err(|e| format!("Failed to copy frame: {}", e))?;
        return Ok(cached_file.clone());
    }

    let any_value_image_data = any_value_image_url.trim_start_matches("data:image/png;base64,");
    let any_value_image_data = base64::decode(any_value_image_data).map_err(|e| format!("Failed to decode base64 image: {}", e))?;
    let any_value_image = image::load_from_memory(&any_value_image_data).map_err(|e| format!("Failed to load image from memory: {}", e))?;
    let rgba_any_value_image = any_value_image.to_rgba8();
    rgba_any_value_image.save(&any_value_frame_file).map_err(|e| format!("Failed to save image: {}", e))?;
    image_cache.insert(any_value_image_url.to_string(), any_value_frame_file.clone());
    Ok(any_value_frame_file)
}

fn get_tempo(smf: &Smf, bpm: f64) -> f64 {
    let mut microseconds_per_beat = 60_000_000.0 / bpm;

    for track in smf.tracks.iter() {
        for event in track.iter() {
            if let TrackEventKind::Meta(MetaMessage::Tempo(tempo)) = event.kind {
                microseconds_per_beat = tempo.as_int() as f64;
                break;
            }
        }
    }

    microseconds_per_beat
}

#[derive(Debug)]
struct Note {
    key: u8,
    start_time: u32,
    duration: u32,
    start_time_seconds: f64,
    duration_seconds: f64,
}