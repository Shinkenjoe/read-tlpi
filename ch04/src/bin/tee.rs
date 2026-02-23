use clap::{ Parser };
use std::io::{ Read, stdin, stdout, Write};
use std::fs::{OpenOptions};

#[derive(Parser)]
#[command(version, about,
          long_about = "read in standard input until eof, write to both stdout and file")]
struct Cli{
    file: String,

    /// append to file
    #[arg(short)]
    append: bool,
}

fn main(){
    let cli = Cli::parse();

    let mut file = OpenOptions::new().write(true).create(true).clone();
    if cli.append {
        file.append(true);
    } else {
        file.truncate(true);
    }
    let mut file = file.open(cli.file).expect("open");
    
    let mut buf : Vec<u8> = Vec::new();
    let num_read = stdin().read_to_end(&mut buf).expect("read");
    if stdout().write(&buf).expect("write") != num_read {
        panic!("different number of bytes written to stdout than read");
    }
    if file.write(&buf).expect("write") != num_read {
        panic!("different number of bytes written to file than read");
    }    

}
