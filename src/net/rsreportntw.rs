use chrono::{prelude::Local, Datelike, Timelike};
use std::io::Write;
use std::path::Path;
use std::fs::OpenOptions;
use std::process::Command;
use local_ip_address::local_ip;


#[derive(Clone)]
pub struct RSreportntw {
    day: u32,
    month: u32,
    year: i32,
    hour: u32,
    minute: u32,
    second: u32,
    path_file: String,
    addr: String
}

impl RSreportntw {

    pub fn new(path: String, filename: String, ip: String) -> Self {
        RSreportntw { 
            day: Local::now().day(), 
            month: Local::now().month(), 
            year: Local::now().year(),
            hour: Local::now().hour(),
            minute: Local::now().minute(),
            second: Local::now().second(),
            path_file: format!("{}/{}", path, filename),
            addr: ip
        }
    }

    pub fn chknetwork(self) {
        let output = if cfg!(target_os = "windows") {
            let windows_command = format!("ping /n 1 {}", self.addr);           
            Command::new("cmd")
                    .args(["/C", windows_command.as_str()])
                    .output()
                    .expect("Ping command failed")
        } else {
            let unix_command = format!("ping -c 1 {}", self.addr);
            Command::new("sh")
                    .arg("-c")
                    .arg(unix_command)
                    .output()
                    .expect("Ping command failed")
        };

        if output.status.code().unwrap() != 0 {
            self.write_csv()
        }
    }

    fn write_csv(&self) {
        let file_path = Path::new(self.path_file.as_str());
        
        if ! file_path.try_exists().unwrap() {
            let mut file_creation = OpenOptions::new()
                .create(true)
                .write(true)
                .open(file_path)
                .unwrap();
            
            file_creation.write_all("\"fecha\", \"hora\", \"ip_origen\" ,\"ip_destino\"\n".as_bytes()).unwrap();
        }

        let local_addr = match local_ip() {
            Ok(res) => res.to_string(),
            Err(e) => {
                println!("{:?}", e);
                String::from("0.0.0.0")
            }
        };

        let record = String::from(format!("\"{}/{}/{}\",\"{}:{}:{}\",\"{}\",\"{}\"\n", 
                                        self.day, 
                                        self.month, 
                                        self.year, 
                                        self.hour, 
                                        self.minute, 
                                        self.second, 
                                        local_addr, 
                                        self.addr.to_string()
                                    )
                                );
        
        let mut append_to_csv = OpenOptions::new()
            .create(false)
            .append(true)
            .truncate(false)
            .open(file_path)
            .unwrap();
        append_to_csv.write_all(record.as_bytes()).unwrap();
        
    }
}