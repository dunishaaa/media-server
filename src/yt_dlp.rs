use std::{
    io::{BufReader, Error, Read, Write},
    process::{Command, Stdio},
};

struct Video<'a> {
    url: &'a str,
    extension: &'a str,
    specs: Vec<&'a str>,
}
struct Audio<'a> {
    url: &'a str,
    extension: &'a str,
    specs: Vec<&'a str>,
}
trait Downloadable {
    fn download(&self) {
        let command = self.build_download_command();
    }

    fn build_download_command(&self) -> &mut Command;
}

impl Downloadable for Video {
    fn build_download_command(&self) -> &mut Command {
        let config = vec![
            "--progress-template",
            "down-prog: %(progress._percent)s|%(progress.downloaded_bytes)s|%(progress.total_bytes)s",
            self.url
        ];
        Command::new("yt-dlp").args(config)
    }
}
impl Downloadable for Audio {
    fn build_download_command(&self) -> &mut Command {
        let mut config = vec![
            "--progress-template",
            "down-prog: %(progress._percent)s|%(progress.downloaded_bytes)s|%(progress.total_bytes)s",
            "-f",
            "ba[ext=m4a]/ba",
            self.url
        ];
        Command::new("yt-dlp").args(config)
    }
}

//yt-dlp --progress-template "%(progress._percent)s|%(progress.downloaded_bytes)s|%(progress.total_bytes)s"
pub fn test(link: String) -> Result<(), Error> {
    let mut child = Command::new("yt-dlp")
        .arg("--progress-template")
        .arg("down-prog: %(progress._percent)s|%(progress.downloaded_bytes)s|%(progress.total_bytes)s")
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
