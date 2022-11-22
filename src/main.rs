extern crate ferris_says;
use ferris_says::say;
use std::io::{stdout, BufWriter };
use std::io;

fn main(){
    let stdout = stdout();
    let mut out = String::new();
    
    println!("What do you want Ferris to say?");
    io::stdin().read_line(&mut out);

    let width = 30;
    let mut writer = BufWriter::new(stdout.lock());
    say(out.as_bytes(), width,&mut writer).unwrap();


}
