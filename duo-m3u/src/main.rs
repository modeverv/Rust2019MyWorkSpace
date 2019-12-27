//! # duo-m3u
//! duoのm3uを`~/Downloads`に書き出し、オープンします。
//! ### usage
//! ```bash
//! $ duo-m3u [Disc01|Disc02|Disc03|Disc04|Disc05|review] [skip num] [take num]
//! ```

use glob::glob;
use std::env;
use std::process::Command;
use std::process;
extern crate my_lib;

/// make m3u file and open.
fn main() {
    // args
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    if args.len() != 3 {
        println!("usage: \nduo-m3u [Disc01|Disc02|Disc03|Disc04|Disc05|review] [skip num] [take num]");
        process::exit(1);
    }
    let skip: usize = args[2].parse().unwrap();
    let take: usize = args[3].parse().unwrap();
    let disc = &args[1];

    // pattern
    let pattern = if disc == "review" {
        String::from("/Volumes/MySD/iTunes/iTunes Media/Music/DUO3.0/DUO3.0CD復習用/*.m4a")
    } else {
        format!("{}{}{}","/Volumes/MySD/iTunes/iTunes Media/Music/DUO3.0/DUO3.0CD基礎用", disc, "/*.m4a")
    };
    let output_file = "/Users/seijiro/Downloads/duo.m3u";
    
    // make m3u file
    make_m3u(&output_file, &pattern, take, skip);
    
    // and open
    let _  =  Command::new("open")
    .arg(output_file)
    .output()
    .expect("failed to execute process");
}

/// make m3u file
fn make_m3u(output_file: &str,pattern: &str, take: usize, skip: usize) {
    //my_lib::typename(pattern);
    //println!("{}", pattern);
    let mut m3u_str = String::from("#EXTM3U\n");
    for entry in glob(pattern).expect("Failed").skip(skip).take(take) {
    //for entry in glob(pattern).expect("Failed") {
        match entry {
            Ok(path) => {
                //println!("{:?}", path.display());
                m3u_str = push_m3u_element(&m3u_str, path.to_str().unwrap());
            },
            Err(e) => println!("{:?}", e),
        }
    }
    //println!("{}",m3u_str);
    let _ = my_lib::write_file(output_file,&m3u_str);
}

/// #EXTINF:1450,just do it.mp4
/// /Volumes/smb/sdd1/video/youtube/just do it.mp4
fn push_m3u_element(m3u: &str,elem: &str) -> String {
    let mut m3u = String::from(m3u);
    m3u.push_str("#EXTINF:-1,a.m4a\n");
    m3u.push_str(&format!("{}\n",elem));
    m3u
}

