use std::{io::{BufRead, BufReader, Error, ErrorKind}, process::{Command, Stdio}};

struct Video{
    url: String,
    extension: String,
    filters: Vec<String>
}
struct Audio{
    url: String,
    extension: String,
}
trait Downloadable{
    fn download(&self);
}

impl Downloadable for Video {
    fn download(&self){
        let output = Command::new("yt-dlp")
            .arg(&self.url)
            .output()
            .expect("Failed to download video");
    }
}
impl Downloadable for Audio {
    fn download(&self){

    }
}
//yt-dlp --progress-template "%(progress._percent)s|%(progress.downloaded_bytes)s|%(progress.total_bytes)s"
pub fn test() -> Result<(), Error>{
    let stdout= Command::new("yt-dlp")
        .arg("--progress-template")
        .arg("down-prog: %(progress._percent)s|%(progress.downloaded_bytes)s|%(progress.total_bytes)s")
        .arg("-f")
        .arg("bv[ext=mp4]")
        .arg("https://www.youtube.com/watch?v=u18be_kRmC0")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not read standard output."))?; 

    let reader = BufReader::new(stdout);
    reader
        .lines()
        .filter_map(|line| line.ok())
       // .filter(|line| line.find("down-prog: ").is_some())
        .for_each(|line| println!("{:}", line));
    Ok(())
}