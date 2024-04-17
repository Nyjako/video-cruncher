use std::process::ExitCode;
use std::path::Path;
use native_dialog::FileDialog;
use subprocess::{Exec, NullFile};

const VIDEO_FORMATS: [&str; 4] = ["mp4", "m4v", "mov", "mkv"];
const SUBTITLE_FILE_EXTENSIONS: [&str; 4] = ["srt", "ass", "ssa", "sub"];

fn main() -> ExitCode {
    
    if !check_for_ffmpeg() {
        eprintln!("Could not find ffmpeg!");
        return ExitCode::FAILURE;
    }

    let fd = FileDialog::new()
        .set_location("~")
        .add_filter("Video files", &VIDEO_FORMATS)
        .show_open_multiple_file()
        .map_err(|err|{
            eprint!("Failed while opening files:\n{}", err);
        }).expect("Failed while opening files");
            
    for f in fd {
        println!("{}", f.to_str().unwrap());
        let command = generate_command_for_file( String::from(f.to_str().unwrap()) );
        if let Some(command) = command {
            let status = command.join().expect("Failed to run command");
            if !status.success() {
                eprintln!("Command failed with exit code: {:?}", status);
            }
        }
    }

    ExitCode::SUCCESS
}

fn check_for_ffmpeg() -> bool {
    let result = Exec::shell("ffmpeg -version")
        .stdout(NullFile)
        .join()
        .expect("Failed to execute ffmpeg");

    result.success()
}

fn generate_command_for_file(video_file: String) -> Option<Exec>
{
    let file = Path::new(video_file.as_str());
    let file_name = file.file_stem().expect("Could not extract filename from filepath!");
    let path = file.parent().expect("Failed to extract path from filepath!")
        .to_str().expect("Failed to convert path to str!");

    if !file.exists() {
        eprintln!("File \"{}\" does not exist!", video_file);
        ()
    }

    let extension = file.extension().expect(format!("Extension of \"{}\" could not be found!", video_file).as_str());

    if !VIDEO_FORMATS.contains( &extension.to_str().expect("Failed to convert osstr to str") ) {
        eprintln!("Could not find extension of \"{}\"!", video_file);
        ()
    }

    let mut command: Vec<String> = Vec::new();

    command.push("-hide_banner".to_string());
    command.push(format!("-i \"{}\"", video_file));
    command.push("-crf 24.0".to_string());

    let mut output_extension: &str;
    let mut has_sub: bool = false;

    if extension == "mkv" {
        output_extension = "mkv";
        command.push(format!("-vf \"subtitles='{}'\"", video_file));
    }
    else {
        output_extension = "mp4";

        // check if subtitle file exist
        for i in SUBTITLE_FILE_EXTENSIONS {
            let current_check = format!("{}/{}.{}", path, file_name.to_str().unwrap(), i);
            if Path::new(current_check.as_str()).exists() {
                command.push(format!("-vf \"subtitles='{}'\"", current_check));
                command.push("-map 0".to_string());
                command.push("-map 1".to_string());
                has_sub = true;
                output_extension = "mkv";
                break;
            }
        }
    }

    command.push("-vcodec libx265".to_string());
    command.push("-c:a aac".to_string());
    command.push("-b:a 128k".to_string());
    if has_sub {
        command.push("-scodec copy".to_string());
        command.push("-metadata:s:s:0 language=pl".to_string());
        command.push("-disposition:s:0 default".to_string());
    }
    else if output_extension == "mkv" {
        command.push("-c:s copy".to_string());
        command.push("-map_metadata 0".to_string());
        command.push("-map_chapters 0".to_string());
        command.push("-map 0".to_string());
    }
    command.push("-preset slow".to_string());
    command.push("-nostdin".to_string());
    command.push("-y".to_string());

    let output_file = format!(
        "\"{}/{} {}.{}\"",
        path,
        file_name.to_str().expect("Failed to convert file_name to str!"),
        "(h265)",
        output_extension
    );

    command.push(format!("{}", output_file));

    // Join the arguments into a single string
    let full_command =  format!("ffmpeg {}", command.join(" "));
    println!("FFMPEG command:\n$ {}", full_command);

    let output_command = Exec::shell(full_command);

    Some(output_command)
}