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
    height_quality: &'a str,
    url: &'a str,
    extension: &'a str,
    download_path: &'a str,
    command: Command,
}

impl<'a> Media<'a> {
    pub fn new(
        media_type: MediaType,
        height_quality: &'a str,
        url: &'a str,
        extension: &'a str,
        download_path: &'a str,
    ) -> Self {
        let mut new_media = Media {
            media_type,
            height_quality,
            url,
            extension,
            download_path,
            command: Command::new("yt-dlp"),
        };
        new_media.build_download_command();

        new_media
    }

    fn build_download_command(&mut self) {
        println!("Building download command...");
        let format = match self.media_type {
            MediaType::VIDEO => {
                let _ = format!(
                    "bv[ext={}][height<={}]+ba/bv+ba",
                    self.extension, self.height_quality
                );
                "bv[ext=mp4][height<=720]+ba/bv+ba"
            }
            MediaType::AUDIO => "ba[ext=m4a]/ba",
        };

        let config = vec![
            "-P",
            self.download_path,
            "-o",
            OUTPUT_NAME_FORMAT,
            "--progress-template",
            PROGRESS_FORMAT,
            "-f",
            format,
            self.url,
        ];

        print!("Built command: ");
        for token in &config {
            print!("{} ", token);
        }
        println!();

        self.command.args(config);
    }

    pub fn download(&mut self) -> Result<(), Error> {
        let mut child = self.command.stdout(Stdio::piped()).spawn()?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "Could not read standard output.")
            .unwrap();

        let reader = BufReader::new(stdout);
        let mut current_progress_string = String::new();

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
}
