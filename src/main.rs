use structopt::StructOpt;
use std::io::{self,Write};
use std::path::PathBuf;
use std::fs::{self,File};
use chrono::{Utc, Datelike};
use winreg::enums::*;
use winreg::RegKey;

#[derive(StructOpt)]
struct Opt{
    #[structopt(short,long,parse(from_os_str))]
    input:PathBuf,
    #[structopt(short,long,parse(from_os_str))]
    output:Option<PathBuf>,
}

fn main()->io::Result<()>{
    let opt = Opt::from_args();
    
    //https://github.com/automerge/automerge-rs/blob/18a3f617043fd53bd05fdea96ff5d079a8654509/rust/automerge-cli/src/main.rs
    let mut file:Box<dyn std::io::Write>;
    
    if let Some(filename) = opt.output{
        file = Box::new(File::create(filename)?);
    }else{
        file = Box::new(io::stdout());
    }
    //rust cookbook
    for entry in fs::read_dir(opt.input)?{
        let entry = entry?;
        let path = entry.path();
        if path.is_dir(){
            let mut dir = path.to_str().unwrap().to_string();
            dir.push('\n');
            //file.write_all(dir.as_bytes())?;
        }
    }
    let now = Utc::now();
    println!("{},{},{}",now.year_ce().1,now.month(),now.day());
    let current_user = 
        "HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
    let system = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")?;
    let mut count = 0;
    let mut count_display = 0;
    for app in system.enum_keys().map(|x| x.unwrap()) {
        //println!("{}", app);
        count += 1;
        let app_info = system.open_subkey(app)?;
        for (name,value) in app_info.enum_values().map(|x| x.unwrap()){
            if name == "DisplayName"{
                println!("{}",value.to_string());
                count_display += 1;
            }
        }
    }
    println!("count:{},display:{}",count,count_display);
    Ok(())
}

fn from_reg()->io::Result<Vec<String>>{
    let apps = Vec::new();
    let app64 = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")?;
    let app32 = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall")?;
    let app_cu = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")?;
    for app in app64.enum_keys().map(|x| x.unwrap()) {
        let app_info = app64.open_subkey(app)?;
        for (name,value) in app_info.enum_values().map(|x| x.unwrap()){
            if name == "DisplayName"{
                println!("{}",value.to_string());
                apps.push(value.to_string());
            }
        }
    }
    Ok(apps)
}

fn from_input(){

}
