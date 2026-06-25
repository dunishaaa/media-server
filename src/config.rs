use std::{fs::{self, File}, io::Write, path::Path};
use local_ip_address::local_ip;

const CONFIG_PATH: &str = "./config.txt";
pub fn parse_config() -> Vec<String>{
    let mut folders: Vec<String> = vec![];
    let contents = fs::read_to_string(CONFIG_PATH).expect(format!("Unable to read config file at {}", CONFIG_PATH).as_str());
    folders = contents.split('\n').map(|x| x.to_string()).collect();
    //folders.sort();
    folders
}

pub fn write_ip() -> std::io::Result<()>{
    let local_ip = local_ip().unwrap();
    let path = Path::new("frontend/.env");
    let env_file = File::create(path);
    match env_file {
        Ok(mut file) => {
            println!("File frontend/.env created...");

            println!("Writing frontend/.env file...");
            file.write_all(
                format!("VITE_IP={:}\nVITE_PORT=3000", local_ip.to_string()).as_bytes()
            )?;
            println!("Succesfully generated frontend/.env file");
        },
        Err(e) => println!("Error: {:}", e)
    }


    Ok(())
}
