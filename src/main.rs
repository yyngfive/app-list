use structopt::StructOpt;
use std::io::{self,Write};
use std::path::PathBuf;
use std::fs::{self,File};
use std::vec;
use chrono::{Utc, Datelike};
use winreg::enums::*;
use winreg::RegKey;
use fancy_regex::Regex;

#[derive(StructOpt)]
struct Opt{
    // 接收自定义软件安装路径目录（必选，因为我的个人习惯）
    #[structopt(short,long,parse(from_os_str))]
    input:PathBuf,
    #[structopt(short,long,parse(from_os_str))]
    output:Option<PathBuf>,
}
#[derive(Debug)]
struct AppInfo{
    name: String,
    install_path:PathBuf,
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
    let mut own_apps = Vec::new();
    for entry in fs::read_dir(opt.input)?{
        let entry = entry?;
        let path = entry.path();
        if path.is_dir(){
            let mut dir = path.to_str().unwrap().to_string();
            dir.push('\n');
            file.write_all(dir.as_bytes())?;
            let replaced_path = replace_slash(path.to_str().unwrap());
            
            own_apps.push(replaced_path);
        }
    }
    let now = Utc::now();
    println!("{},{},{}",now.year_ce().1,now.month(),now.day());
    let apps = from_reg()?;
    let mut sub_apps = vec![];
    let mut count = 0;
    for item in apps{
        //println!("{:#?}",item);
        //println!("{}\t{}",remove_quotations(&item.name),item.install_path.to_str().unwrap());
        if item.install_path.starts_with("D:\\APP\\"){
            println!("{}",item.install_path.to_str().unwrap());
            sub_apps.push(short_path(item.install_path.to_str().unwrap()));
        }
        
        /*for path in &own_apps{
            if   && !item.install_path.starts_with(path){
                println!("{}",item.name);
            }
        }*/
    }
    
    //println!("{:?}",sub_apps);
    //println!("own_app:{:?}",own_apps);
    println!("sub_app:{}",sub_apps.len());
    println!("{:?}",sub_apps);
    
    for item in own_apps{
        //println!("{}",item);
        if item == String::from(r"D:\APP\AppFlowy") {
            println!("Yes!");
        }
        if !sub_apps.contains(&item){
            println!("{item}");
            count += 1;
        }
    }
    println!("find:{count}");

    Ok(())
}

fn remove_quotations(path:&str)->String{
    let re = Regex::new("\"").unwrap();
    re.replace_all(path,"").to_string()
}

fn replace_slash(path:&str)->String{
    let re = Regex::new(r"\\\\").unwrap();
    re.replace_all(path, r"\").to_string()
}

fn short_path(path:&str)->String{
    let re = Regex::new(r"D:\\APP\\(?:(?!\\).)*").unwrap();
    //println!("{path}");
    match re.find(path) {
        Ok(Some(mat)) => {path[mat.start()..mat.end()].to_string()},
        Err(_) => panic!("Oh!"),
        Ok(None) => String::from(""),
    }
    
}
/* 
fn interset<T: std::cmp::PartialEq>(vec1:Vec<T>,vec2:Vec<T>)->Vec<&'static T>{
    let res = vec1.iter().filter(|&u| !vec2.contains(u)).collect::<Vec<_>>();
    res
}*/

fn from_reg()->io::Result<Vec<AppInfo>>{
    let mut apps = Vec::new();
    let app64 = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")?;
    let app32 = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall")?;
    let app_cu = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")?;
    for app in app64.enum_keys().map(|x| x.unwrap()) {
        let app_info = app64.open_subkey(app)?;
        let mut result = AppInfo{name:String::from(""),install_path:PathBuf::from("\\")};
        for (name,value) in app_info.enum_values().map(|x| x.unwrap()){
            if name == "DisplayName"{
                result.name = value.to_string();
            }
            if name == "InstallLocation"{
                let install_location = remove_quotations(&value.to_string());
                result.install_path = PathBuf::from(replace_slash(&install_location));
            }

        }
        if result.name.as_str() != ""{
            apps.push(result);
        }
        
    }
    for app in app32.enum_keys().map(|x| x.unwrap()) {
        let app_info = app32.open_subkey(app)?;
        let mut result = AppInfo{name:String::from(""),install_path:PathBuf::from("\\")};
        for (name,value) in app_info.enum_values().map(|x| x.unwrap()){
            if name == "DisplayName"{
                result.name = value.to_string();
            }
            if name == "InstallLocation"{
                let install_location = remove_quotations(&value.to_string());
                result.install_path = PathBuf::from(replace_slash(&install_location));
            }

        }
        if result.name.as_str() != ""{
            apps.push(result);
        }
    }
    for app in app_cu.enum_keys().map(|x| x.unwrap()) {
        let app_info = app_cu.open_subkey(app)?;
        let mut result = AppInfo{name:String::from(""),install_path:PathBuf::from("\\")};
        for (name,value) in app_info.enum_values().map(|x| x.unwrap()){
            if name == "DisplayName"{
                result.name = value.to_string();
            }
            if name == "InstallLocation"{
                let install_location = remove_quotations(&value.to_string());
                result.install_path = PathBuf::from(replace_slash(&install_location));
            }

        }
        if result.name.as_str() != ""{
            apps.push(result);
        }
    }
    Ok(apps)
}

fn from_input(){

}
