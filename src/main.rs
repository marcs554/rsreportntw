mod net;

use net::rsreportntw::RSreportntw;
use gumdrop::Options;
use std::time::Duration;
use std::thread::sleep;


#[derive(Debug, Options)]
struct Opt {
    #[options(free)]
    free: Vec<String>,

    #[options(help = "print help panel")]
    help: bool,

    #[options(help="Put the ip to tests connection")]
    ip_dest: Option<String>,

    #[options(help="Put the path to save the file")]
    path: Option<String>,

    #[options(help="Put the file name + .csv")]
    file: Option<String>,
}

fn main() {
    let parse_args = Opt::parse_args_default_or_exit();
    let str_addr = match parse_args.ip_dest {
        Some(ip) => ip,
        None => String::from("127.0.0.1")
    };
    let path_to_file = parse_args.path.unwrap();
    let file_name = parse_args.file.unwrap();

    loop {
        let rs_reportntw = RSreportntw::new(path_to_file.clone(), file_name.clone(), str_addr.clone());
        
        rs_reportntw.chknetwork();
        
        sleep(Duration::from_secs(1))
    }
}
