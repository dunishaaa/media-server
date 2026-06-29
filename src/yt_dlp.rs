use std::{
    io::{BufReader, Error, Read, Write},
    process::{Command, Stdio},
};

struct Video {
    url: String,
    extension: String,
    filters: Vec<String>,
}
struct Audio {
    url: String,
    extension: String,
}
trait Downloadable {
    fn download(&self);
}

impl Downloadable for Video {
    fn download(&self) {
        let output = Command::new("yt-dlp")
            .arg(&self.url)
            .output()
            .expect("Failed to download video");
    }
}
impl Downloadable for Audio {
    fn download(&self) {}
}
pub fn build_download_command() -> &mut Command {
    let args = vec![
        "--progress-template",
        "down-prog: %(progress._percent)s|%(progress.downloaded_bytes)s|%(progress.total_bytes)s",
        "-f",
    ];
    let mut child = Command::new("yt-dlp").args(args);
    child
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
