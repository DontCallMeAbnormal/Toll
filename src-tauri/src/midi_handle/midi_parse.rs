use ffmpeg_next::codec::encoder::Encoder;
use ffmpeg_next::format::output::Output;
use ffmpeg_next::frame::Video;
use ffmpeg_next::media::Type;
use ffmpeg_next::util::format::Pixel;
use ffmpeg_next::util::math::rescale;
use ffmpeg_next::Rational;
use image::{ImageBuffer, Rgba};
use midly::num::u28;
use midly::{MidiMessage, Smf, TrackEvent, TrackEventKind}; // 添加 TrackEventKind 引入
use serde_json::json;
use std::fs::File;
use std::io::Write;

#[tauri::command]
pub fn parse_midi(file: Vec<u8>) -> Result<String, String> {
    let smf = Smf::parse(&file).map_err(|e| format!("Failed to parse MIDI file: {}", e))?;

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
                        notes.push(Note {
                            key: key.as_int(),
                            start_time: u28::as_int(delta_time),
                            duration: 0,
                        });
                    }
                    MidiMessage::NoteOff { key, .. } => {
                        if let Some(note) = notes
                            .iter_mut()
                            .find(|n| n.key == key.as_int() && n.duration == 0)
                        {
                            note.duration = u28::as_int(delta_time) - note.start_time;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    let json_notes = notes
        .iter()
        .map(|note| {
            json!({
                "key": note.key,
                "start_time": note.start_time,
                "duration": note.duration
            })
        })
        .collect::<Vec<_>>();

    Ok(json!(json_notes).to_string())
}

#[tauri::command]
pub fn generate_animation(
    musicRhythm: Vec<serde_json::Value>,
    images: Vec<serde_json::Value>,
) -> Result<String, String> {
    // 创建一个动画数据结构
    let mut animation_data = Vec::new();

    // 假设我们根据 musicRhythm 和 images 生成动画帧
    for note in musicRhythm.iter() {
        let key = note["key"].as_u64().unwrap_or(0) as u8;
        let start_time = note["start_time"].as_u64().unwrap_or(0) as u32;
        let duration = note["duration"].as_u64().unwrap_or(0) as u32;

        for image in images.iter() {
            let condition = image["condition"].as_str().unwrap_or("");
            let value = image["value"].as_str().unwrap_or("");

            // 根据条件判断是否显示图片
            let should_display = match condition {
                "greater_than" => start_time > value.parse::<u32>().unwrap_or(0),
                "equal_to" => start_time == value.parse::<u32>().unwrap_or(0),
                "less_than" => start_time < value.parse::<u32>().unwrap_or(0),
                "range" => {
                    let range: Vec<u32> = value
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();
                    if range.len() == 2 {
                        start_time >= range[0] && start_time <= range[1]
                    } else {
                        false
                    }
                }
                "any_value" => true,
                _ => false,
            };

            if should_display {
                animation_data.push(json!({
                    "key": key,
                    "start_time": start_time,
                    "duration": duration,
                    "image": image
                }));
            }
        }
    }

    // 将动画数据序列化为 JSON 字符串
    let json_data = json!(animation_data).to_string();

    // 保存动画数据到文件
    let mut file =
        File::create("animation_data.json").map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(json_data.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    // 生成视频
    generate_video(&animation_data)?;

    Ok(json_data)
}

fn generate_video(animation_data: &Vec<serde_json::Value>) -> Result<(), String> {
    ffmpeg_next::init().map_err(|e| format!("Failed to initialize ffmpeg: {}", e))?;

    let mut output = Output::with_args(&["output.mp4"])
        .map_err(|e| format!("Failed to create output: {}", e))?;
    let mut video_stream = output.add_stream(
        Encoder::find_by_name("libx264").map_err(|e| format!("Failed to find encoder: {}", e))?,
    );
    video_stream.set_format(Pixel::RGBA);
    video_stream.set_width(1920);
    video_stream.set_height(1080);
    video_stream.set_frame_rate(Rational(60, 1));
    video_stream.set_time_base(Rational(1, 60));
    video_stream
        .open()
        .map_err(|e| format!("Failed to open video stream: {}", e))?;

    let mut frame = Video::empty();
    frame.set_format(Pixel::RGBA);
    frame.set_width(1920);
    frame.set_height(1080);

    let mut pts = 0;

    for note in animation_data.iter() {
        let start_time = note["start_time"].as_u64().unwrap_or(0) as i64;
        let duration = note["duration"].as_u64().unwrap_or(0) as i64;
        let image_url = note["image"]["url"].as_str().unwrap_or("");

        // Load image
        let image = image::open(image_url).map_err(|e| format!("Failed to open image: {}", e))?;
        let rgba_image = image.to_rgba8();

        // Convert image to frame
        let mut frame_buffer = vec![0; (1920 * 1080 * 4) as usize];
        for y in 0..1080 {
            for x in 0..1920 {
                let pixel = rgba_image.get_pixel(x, y);
                let index = (y * 1920 + x) * 4;
                frame_buffer[index] = pixel[0];
                frame_buffer[index + 1] = pixel[1];
                frame_buffer[index + 2] = pixel[2];
                frame_buffer[index + 3] = pixel[3];
            }
        }

        frame.set_data(frame_buffer);
        frame.set_pts(pts);

        // Write frames for the duration of the note
        let end_time = start_time + duration;
        while pts < end_time {
            video_stream
                .encode(&frame)
                .map_err(|e| format!("Failed to encode frame: {}", e))?;
            pts += 1;
        }
    }

    video_stream
        .finalize()
        .map_err(|e| format!("Failed to finalize video stream: {}", e))?;

    Ok(())
}

#[derive(Debug)]
struct Note {
    key: u8,
    start_time: u32,
    duration: u32,
}
