# Video Cruncher

Video Cruncher is a Rust program designed to simplify the process of converting video files using FFmpeg. It allows you to select video files through a file chooser dialog and generates FFmpeg commands for each selected file. The program then executes these commands, leveraging FFmpeg to perform the desired conversions.

## How it Works

1. **Run the Executable**: After running the executable, a file chooser dialog will pop up.
2. **Select Video Files**: Choose the video files you want to process.
3. **FFmpeg Commands Generation**: Video Cruncher generates FFmpeg commands for each selected file.
4. **Execution**: The program runs each FFmpeg command to perform the conversions.
5. **Closure**: After all conversions are complete, the program closes.

## Features

- **File Chooser Dialog**: Allows easy selection of video files.
- **FFmpeg Integration**: Utilizes FFmpeg for video conversion.
- **Default Encoder**: Uses H.265 encoder with a CRF (Constant Rate Factor) of 24.
- **Subtitle Handling**: Searches for subtitle files with the same name as the video files. If found, outputs the video with embedded subtitles in MKV format.

## Future Enhancements

- **Graphical User Interface (GUI)**: A GUI will be added in the future for a more user-friendly experience.
- **Customization Options**: Additional options for specifying FFmpeg parameters and encoder settings.
- **Error Handling**: Improved error handling and feedback mechanisms.

## Requirements

- **FFmpeg**: The FFmpeg executable needs to be available. For example, on Linux, it can be located at `/usr/bin/ffmpeg`. On Windows, ensure FFmpeg is added to the global variables.
- **Rust**: Make sure you have Rust installed on your system.

## Usage

1. Clone this repository.
2. Build the project using `cargo build`.
3. Run the executable generated by Cargo.
4. Follow the on-screen instructions to select and process your video files.

## Contributions

Contributions are welcome! If you have any suggestions, bug fixes, or feature enhancements, feel free to open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).