use chrono::Local;
use fancy_regex::Regex;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::vec;
use structopt::StructOpt;
use winreg::enums::*;
use winreg::RegKey;

#[derive(StructOpt)]
struct Opt {
    // 接收自定义软件安装路径目录（必选，因为我的个人习惯）
    #[structopt(short, long, parse(from_os_str))]
    input: Option<PathBuf>,
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}
#[derive(Debug)]
struct AppInfo {
    name: String,
    install_path: PathBuf,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    //https://github.com/automerge/automerge-rs/blob/18a3f617043fd53bd05fdea96ff5d079a8654509/rust/automerge-cli/src/main.rs
    let mut file: Box<dyn std::io::Write>;

    if let Some(filename) = opt.output {
        file = Box::new(File::create(filename)?);
    } else {
        file = Box::new(io::stdout());
    }

    //rust cookbook
    //读取输入的目录

    let mut own_apps = Vec::new();
    let have_input:bool;
    let own_path:&str;
    match opt.input {
        Some(input) => {
            own_path = input.to_str();
            for entry in fs::read_dir(input)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let replaced_path = replace_slash(path.to_str().unwrap());
                    own_apps.push(replaced_path);
                }
            }
            have_input = true;
        },
        None => have_input = false,
    }

    //获取全部APP
    let apps = from_reg()?;

    let mut sub_apps = vec![];
    //println!("{:?}",apps);
    let mut count = 1;

    file.write_all(b"Index, Name, Path\n")?;

    //剔除相同的APP
    for item in apps {
        let display_res = format!(
            "{}, {}, {}\n",
            count,
            item.name,
            item.install_path.to_str().unwrap()
        );
        file.write_all(display_res.as_bytes())?;
        if item.install_path.starts_with(own_path) {
            //println!("{}",item.install_path.to_str().unwrap());
            sub_apps.push(short_path(item.install_path.to_str().unwrap()));
        }
        count += 1;
    }

    if have_input == true {
        for item in own_apps {
            if !sub_apps.contains(&item) {
                //println!("{}",&item[7..item.len()]);
                let display_res =
                    format!("{}, {}, {}\n", count, item[7..item.len()].to_string(), item);
                file.write_all(display_res.as_bytes())?;
                count += 1;
            }
        }
    }

    //获取当前时间
    let now = Local::now();
    let time_now = format!("at {:#?}\n", now);
    file.write_all(time_now.as_bytes())?;

    Ok(())
}

fn remove_quotations(path: &str) -> String {
    let re = Regex::new(r#"\\"|""#).unwrap();
    re.replace_all(path, "").to_string()
}

fn replace_slash(path: &str) -> String {
    let re = Regex::new(r"\\\\").unwrap();
    re.replace_all(path, r"\").to_string()
}

fn short_path(path: &str) -> String {
    //TODO: 更加通用的处理方式
    if path.starts_with("D:\\APP\\Steam\\steamapps") {
        //println!("{}",path);
        return path.to_string();
    }
    let re = Regex::new(r"D:\\APP\\(?:(?!\\).)*").unwrap();
    //println!("{path}");
    match re.find(path) {
        Ok(Some(mat)) => path[mat.start()..mat.end()].to_string(),
        Err(_) => panic!("Oh!"),
        Ok(None) => String::from(""),
    }
}

fn from_reg() -> io::Result<Vec<AppInfo>> {
    let mut apps = Vec::new();
    let app64 = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")?;
    let app32 = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall")?;
    let app_cu = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")?;
    for app in app64.enum_keys().map(|x| x.unwrap()) {
        let app_info = app64.open_subkey(app)?;
        let mut result = AppInfo {
            name: String::from(""),
            install_path: PathBuf::from("\\"),
        };
        match app_info.get_value("DisplayName") {
            Ok(value) => result.name = value,
            Err(_) => continue,
        };
        match app_info.get_value("InstallLocation") {
            Ok(value) => {
                let path: String = value;
                let install_location = remove_quotations(&path);
                result.install_path = PathBuf::from(replace_slash(&install_location));
                apps.push(result);
                continue;
            }
            Err(_) => {}
        };
        match app_info.get_value("UninstallString") {
            Ok(value) => {
                let path: String = value;
                let install_location = remove_quotations(&path);
                result.install_path = PathBuf::from(replace_slash(&install_location));
            }
            Err(_) => {}
        };
        apps.push(result);
    }
    for app in app32.enum_keys().map(|x| x.unwrap()) {
        let app_info = app32.open_subkey(app)?;
        let mut result = AppInfo {
            name: String::from(""),
            install_path: PathBuf::from("\\"),
        };
        match app_info.get_value("DisplayName") {
            Ok(value) => result.name = value,
            Err(_) => continue,
        };
        match app_info.get_value("InstallLocation") {
            Ok(value) => {
                let path: String = value;
                let install_location = remove_quotations(&path);
                result.install_path = PathBuf::from(replace_slash(&install_location));
                apps.push(result);
                continue;
            }
            Err(_) => {}
        };
        match app_info.get_value("UninstallString") {
            Ok(value) => {
                let path: String = value;
                let install_location = remove_quotations(&path);
                result.install_path = PathBuf::from(replace_slash(&install_location));
            }
            Err(_) => {}
        };
        apps.push(result);
    }
    for app in app_cu.enum_keys().map(|x| x.unwrap()) {
        let app_info = app_cu.open_subkey(app)?;
        let mut result = AppInfo {
            name: String::from(""),
            install_path: PathBuf::from("\\"),
        };
        match app_info.get_value("DisplayName") {
            Ok(value) => result.name = value,
            Err(_) => continue,
        };
        match app_info.get_value("InstallLocation") {
            Ok(value) => {
                if value == "" {
                    println!("cu{}", result.name);
                }
                let path: String = value;
                let install_location = remove_quotations(&path);
                result.install_path = PathBuf::from(replace_slash(&install_location));
                apps.push(result);
                continue;
            }
            Err(_) => {}
        };
        match app_info.get_value("UninstallString") {
            Ok(value) => {
                let path: String = value;
                let install_location = remove_quotations(&path);
                result.install_path = PathBuf::from(replace_slash(&install_location));
            }
            Err(_) => {}
        };
        apps.push(result);
    }
    Ok(apps)
}

fn from_input() {}
