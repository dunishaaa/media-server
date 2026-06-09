use std::fs;

const CONFIG_PATH: &str = "./config.txt";
pub fn parse_config() -> Vec<String>{
    let mut folders: Vec<String> = vec![];
    let contents = fs::read_to_string(CONFIG_PATH).expect(format!("Unable to read config file at {}", CONFIG_PATH).as_str());
    folders = contents.split('\n').map(|x| x.to_string()).collect();
    //folders.sort();
    folders
}