use std::{
    io::{BufReader, Error, Read, Write},
    process::{Command, Stdio},
};
static PROGRESS_FORMAT: &str =
    "my_format|%(progress._percent)s|%(progress.downloaded_bytes)s|%(progress.total_bytes)s";
static OUTPUT_NAME_FORMAT: &str = "%(uploader)s-%(title)s.%(ext)s";

pub enum MediaType {
    VIDEO,
    AUDIO,
}

pub struct Media<'a> {
    media_type: MediaType,
    quality: &'a str,
    url: &'a str,
    extension: &'a str,
    download_path: &'a str,
    command: Command,
}

pub trait Download {
    fn download(&mut self) -> Result<(), Error>;
    fn build_download_command(&mut self);
}

impl<'a> Media<'a> {
    pub fn new(
        media_type: MediaType,
        quality: &'a str,
        url: &'a str,
        extension: &'a str,
        download_path: &'a str,
    ) -> Self {
        Media {
            media_type,
            quality,
            url,
            extension,
            download_path,
            command: Command::new("yt-dlp"),
        }
    }
}

impl<'a> Download for Media<'a> {
    fn download(&mut self) -> Result<(), Error> {
        self.build_download_command();

        let mut child = self.command.stdout(Stdio::piped()).spawn()?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "Could not read standard output.")
            .unwrap();

        let reader = BufReader::new(stdout);
        for byte in reader.bytes() {
            if let Ok(b) = byte {
                if b == b'\r' || b == b'\n' {
                    continue;
                }
                print!("{}", b as char);
                std::io::stdout().flush()?;
            }
        }

        Ok(())
    }
    fn build_download_command(&mut self) {
        println!("Building download command...");
        let mut config = vec![
            "-P",
            "/home/dunishaaa/media/prueba/",
            "-o",
            OUTPUT_NAME_FORMAT,
            "--progress-template",
            PROGRESS_FORMAT,
            "-f",
        ];
        let format = match self.media_type {
            MediaType::VIDEO => "bv[ext=mp4][height<=720]+ba/bv+ba",
            MediaType::AUDIO => "ba[ext=m4a]/ba",
        };

        config.push(format);
        config.push(self.url);
        print!("Built command: ");
        for token in &config {
            print!("{} ", token);
        }
        println!();

        self.command.args(config);
    }
}

//yt-dlp --progress-template "%(progress._percent)s|%(progress.downloaded_bytes)s|%(progress.total_bytes)s"
pub fn test(link: String) -> Result<(), Error> {
    let mut child = Command::new("yt-dlp")
        .arg("--progress-template")
        .arg(PROGRESS_FORMAT)
        .arg("-f")
        .arg("bv[ext=mp4]")
        .arg(link)
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "Could not read standard output.")
        .unwrap();
    let mut current_progress_string = String::new();

    let reader = BufReader::new(stdout);
    for byte in reader.bytes() {
        if let Ok(b) = byte {
            if b == b'\n' {
                current_progress_string.clear();
            } else if b == b'\r' {
                print!("{}", current_progress_string);
            }
            current_progress_string.push(b as char);
            std::io::stdout().flush()?;
        }
    }
    Ok(())
}
