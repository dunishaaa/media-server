use std::{
    io::{BufReader, Error, Read, Write},
    process::{Command, Stdio},
};
static PROGRESS_FORMAT: &str =
    "my_format|%(progress._percent)s|%(progress.downloaded_bytes)s|%(progress.total_bytes)s";

enum MediaType {
    VIDEO,
    AUDIO,
}

pub struct DownloadableMedia<T: Download> {
    pub media: T,
    pub media_type: MediaType,
}

pub struct Video<'a> {
    pub url: &'a str,
    pub extension: &'a str,
    pub command: Command,
}
pub struct Audio<'a> {
    pub url: &'a str,
    pub extension: &'a str,
    pub command: Command,
}
pub trait Download {
    fn download(&mut self) -> Result<(), Error>;

    fn build_download_command(&mut self);
}

impl<'a> Download for Video<'a> {
    fn download(&mut self) -> Result<(), Error> {
        self.build_download_command();
        let mut child = self.command.stdout(Stdio::piped()).spawn()?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "Coult not read standard output.")
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
        let config = vec!["--progress-template", PROGRESS_FORMAT, self.url];
        self.command.args(config);
    }
}
impl<'a> Download for Audio<'a> {
    fn download(&mut self) -> Result<(), Error> {
        self.build_download_command();
        let mut child = self.command.stdout(Stdio::piped()).spawn()?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "Coult not read standard output.")
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
        let config = vec![
            "--progress-template",
            PROGRESS_FORMAT,
            "-f",
            "ba[ext=m4a]/ba",
            self.url,
        ];
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
        .ok_or_else(|| "Coult not read standard output.")
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
