use std::process::Command;

mod parser;
use crate::parser::{BiOSToken, parse_bios_string};
mod renderer;
use crate::renderer::Frame;






fn main() {
    

    let input_dir: &str = "input";
    let input_file: &str = &format!("{}/{}", input_dir, "intro.txt");

    let output_dir: &str = "output";
    let output_file: &str = &format!("{}/{}", output_dir, "output.mp4");
    std::fs::create_dir_all(output_dir).expect("failed to create frames dir");

    
    let font_path: &str = "assets/font.ttf";
    let frames_dir: &str = &format!("{}/{}", output_dir, "frames");
    std::fs::create_dir_all(frames_dir).expect("failed to create frames dir");

    let width: u32 = 1440 / 2;
    let height: u32 = 1080 / 2;
    

    let text: String = std::fs::read_to_string(input_file).expect("Input file error");
    let tokens: Vec<BiOSToken> = parse_bios_string(&text.trim());

    let mut frame: Frame = Frame::new(width, height, font_path, 32.0, tokens, frames_dir);
    frame.padding_x = 16;
    frame.padding_y = 16;




    frame.render_tokens();




    let status: std::process::ExitStatus = Command::new("ffmpeg")
        .args(&[
            "-y",
            "-framerate", "60",
            "-i", &format!("{}/frame_%05d.bmp", frames_dir),
            "-vf", "setpts=4*PTS",
            "-c:v", "libx264",
            "-pix_fmt", "yuv420p",
            output_file,
        ])
        .status()
        .expect("failed to execute ffmpeg");

    if status.success() {
        println!("Video created successfully: {}", output_file);
    } else {
        eprintln!("ffmpeg failed");
    }
    std::fs::remove_dir_all(frames_dir).unwrap();



}
