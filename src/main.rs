use structopt::StructOpt;
use std::io::Write;
use std::path::PathBuf;
use std::{fs,io,fs::File};

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
    for entry in fs::read_dir(opt.input)?{
        let entry = entry?;
        let path = entry.path();
        if path.is_dir(){
            let mut dir = path.to_str().unwrap().to_string();
            dir.push('\n');
            file.write_all(dir.as_bytes())?;
        }
    }
    Ok(())
}
