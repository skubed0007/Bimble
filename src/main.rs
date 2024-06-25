use std::{env::args, fs::File, io::Read, process::exit, thread::sleep, time::Duration};

use clearscreen::clear;
use colored::Colorize;
use regex::Regex;

fn main() {
    clear().unwrap();
    let pf: Vec<String> = args().collect();
    let mut pfci = 0;

    if pf.len() <= 1 {
        println!(
            "{}",
            "ERROR - NEED AT LEAST 1 PROJECT FOLDER TO COMPILE!!".red()
        );
        return;
    }

    let mut fns: Vec<String> = Vec::new();

    for pf in pf.iter().skip(1) {
        match File::open(pf.clone() + "/main.bb") {
            Ok(mut mf) => {
                println!("{}{:?}", "found main file!! -- ".green(), mf);
                sleep(Duration::from_millis(500));
                let mut wc = String::new();
                match mf.read_to_string(&mut wc) {
                    Ok(_) => {
                        let nlsepcode = wc.split('\n');
                        for i in nlsepcode.clone() {
                            if i.starts_with("ON") {
                                // Adjust the regex to match only the desired pattern
                                let funcdeclarerg = Regex::new(r"ON\s+(\w+)\(\)\{").unwrap();
                                if let Some(cap) = funcdeclarerg.captures(i) {
                                    if let Some(funcnm) = cap.get(1) {
                                        fns.push(funcnm.as_str().to_string());
                                    } else {
                                        println!(
                                            "ERROR - Could not capture function name in line: {}",
                                            i
                                        );
                                    }
                                } else {
                                    println!(
                                        "{}{}",
                                        "Function Declare using wrong syntax: ".red(),
                                        i.red()
                                    );
                                    println!("{}", "CANCELLING BUILD".blink().blue());
                                    exit(0);
                                }
                            } else if i.trim() == "}" {
                                continue;
                            }
                        }
                    }
                    Err(err) => {
                        println!(
                            "{}{}{}{}",
                            "Error Opening main file in the project: ",
                            pf.clone(),
                            " : ERR - ",
                            err.to_string()
                        );
                    }
                }
            }
            Err(err) => {
                if pfci != 0 {
                    println!(
                        "{}{}",
                        "Error opening file 'main.bb' in project folder provided! \nerr - ",
                        err.to_string()
                    );
                    exit(-1);
                } else {
                    pfci += 1;
                }
            }
        }
    }

    // Printing the captured function names
    for func in fns {
        println!("Captured function name: {}", func);
    }
}
