use std::process::Command;
struct Video{
    url: String,
    extension: String,
    filters: Vec<String>
}
//yt-dlp --progress-template "%(progress._percent)s|%(progress.downloaded_bytes)s|%(progress.total_bytes)s"
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


pub fn test(){
    let output = Command::new("yt-dlp")
        .arg("-f")
        .arg("bv[ext=mp4]")
        .arg("https://www.youtube.com/watch?v=u18be_kRmC0")
        .output()
        .expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{:}", stdout);
}

pub fn get_available_formats(){

}

pub fn download_video(){

}

pub fn download_audio(){

}