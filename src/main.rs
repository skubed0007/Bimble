use clearscreen::clear;
use colored::Colorize;
use regex::Regex;
use s_nor::encrypt;
use std::{
    env::args,
    fs::{remove_dir, remove_file, DirBuilder, File},
    io::{Read, Write},
    path::Path,
    process::exit,
    thread::sleep,
    time::Duration,
};

#[derive(Clone, Debug)]
struct Varr {
    name: String,
    vtype: Vartypes,
    vval: String,
}

#[derive(Debug)]
struct CFG {
    name: String,
    date: String,
    auth: String,
}

#[derive(Debug, Clone)]
enum Vartypes {
    String,
    Fsf,
    I,
}

trait Dis {
    fn dis(v: Varr) {
        println!(
            "{}",
            format!(
                "var_name : {} : var_type : {:?} : var_val : {} :",
                v.name, v.vtype, v.vval
            )
            .green()
        );
    }
}

impl Dis for Varr {}

#[allow(path_statements)]
fn main() {
    let mut vrs: Vec<Varr> = Vec::new();
    let mut undefined_fn_calls: Vec<String> = Vec::new();
    clear().unwrap();
    let mut isinfn = false;
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

    for project_folder in pf.iter().skip(1) {
        match File::open(format!("{}/main.bb", project_folder)) {
            Ok(mut mf) => {
                println!("{}{:?}", "found main file!! -- ".green(), mf);
                sleep(Duration::from_millis(500));
                let mut wc = String::new();
                match mf.read_to_string(&mut wc) {
                    Ok(_) => {
                        let nlsepcode = wc.split('\n');
                        for line in nlsepcode.clone() {
                            if line.starts_with("ON") && !isinfn {
                                println!("{}", "Handling function declaration".blue());

                                let funcdeclarerg = Regex::new(r"ON\s+(\w+)\(\)\{").unwrap();
                                if let Some(cap) = funcdeclarerg.captures(line) {
                                    if let Some(funcnm) = cap.get(1) {
                                        fns.push(funcnm.as_str().to_string());
                                        println!(
                                            "{}{}",
                                            "Function declared: ".cyan(),
                                            funcnm.as_str().cyan()
                                        );
                                    } else {
                                        println!(
                                            "{}{}",
                                            "ERROR - Could not capture function name in line: "
                                                .red(),
                                            line.red()
                                        );
                                    }
                                } else {
                                    println!(
                                        "{}{}",
                                        "Function Declare using wrong syntax: ".red(),
                                        line.red()
                                    );
                                    println!("{}", "CANCELLING BUILD".blink().blue());
                                    exit(0);
                                }
                                isinfn = !line.ends_with("}");
                            } else if line.starts_with("ON") && isinfn {
                                println!(
                                    "{}{}",
                                    "Cannot declare functions inside other functions! - ".red(),
                                    line.red()
                                );
                            } else if line.trim() == "}" {
                                isinfn = false;
                            } else if line.trim().starts_with("may") {
                                println!(
                                    "{}{}",
                                    "Handling 'variables' - ".green(),
                                    line.trim().green()
                                );

                                let vardecltrg =
                                    Regex::new(r#"may\s+(\w+)\s*=\s*(.+)\s*;"#).unwrap();
                                if let Some(cap) = vardecltrg.captures(line.trim()) {
                                    let varnm = cap.get(1).unwrap().as_str().to_string();
                                    let varval = cap.get(2).unwrap().as_str().to_string();
                                    let vartype =
                                        if varval.starts_with('"') && varval.ends_with('"') {
                                            Vartypes::String
                                        } else if varval.parse::<i32>().is_ok() {
                                            Vartypes::I
                                        } else if varval.parse::<f32>().is_ok() {
                                            Vartypes::Fsf
                                        } else {
                                            println!(
                                                "{}{}{}{} :",
                                                "Invalid variable type! : ".red(),
                                                varval.red(),
                                                " : in line : ".red(),
                                                line.trim().red()
                                            );
                                            exit(0);
                                        };
                                    let var = Varr {
                                        name: varnm,
                                        vtype: vartype,
                                        vval: varval,
                                    };
                                    vrs.push(var.clone());
                                    Varr::dis(var);
                                } else {
                                    println!("{}", "Unable to make variable pattern!!".red());
                                    exit(0);
                                }
                            } else if line.trim().starts_with("echonl") {
                                println!(
                                    "{}{}",
                                    "Handling 'echonl' - ".green(),
                                    line.trim().green()
                                );
                                let enlrg = Regex::new(r#"echonl\((.*?)\)\;"#).unwrap();
                                if let Some(cap) = enlrg.captures(line) {
                                    if let Some(text) = cap.get(1) {
                                        let text = text.as_str();
                                        let txt = text.split(',');
                                        for text in txt {
                                            let text = text.trim();
                                            if text.starts_with('"') && text.ends_with('"') {
                                                println!(
                                                    "{}{}",
                                                    "Echoing literal: ".cyan(),
                                                    text.cyan()
                                                );
                                            } else {
                                                let mut found = false;
                                                for var in vrs.iter() {
                                                    if var.name == text {
                                                        found = true;
                                                        println!(
                                                            "{}{}",
                                                            "Echoing variable: ".cyan(),
                                                            text.cyan()
                                                        );
                                                        break;
                                                    }
                                                }
                                                if !found {
                                                    println!(
                                                        "{}{}{}{}",
                                                        "Variable not found in scope: ".red(),
                                                        text.red(),
                                                        " in echonl statement: ".red(),
                                                        line.trim().red()
                                                    );
                                                    exit(0);
                                                }
                                            }
                                        }
                                    } else {
                                        println!(
                                            "ERROR - Could not capture text inside echonl in line: {}",
                                            line
                                        );
                                    }
                                } else {
                                    println!(
                                        "{}{}",
                                        "Invalid 'echonl()' syntax :: ".red(),
                                        line.trim().red()
                                    );
                                    println!("{}", "CANCELLING BUILD".blink().blue());
                                    exit(0);
                                }
                            } else if line.trim() == "out.flush();" {
                                println!("{} {}", "buffer flusher called here : ", line.trim());
                            } else if line.trim().starts_with("echol") {
                                println!(
                                    "{}{}",
                                    "Handling 'echol' - ".green(),
                                    line.trim().green()
                                );

                                let enlrg = Regex::new(r#"echol\((.*?)\)\;"#).unwrap();
                                if let Some(cap) = enlrg.captures(line) {
                                    if let Some(text) = cap.get(1) {
                                        let text = text.as_str();
                                        let txt = text.split(',');
                                        for text in txt {
                                            let text = text.trim();
                                            if text.starts_with('"') && text.ends_with('"') {
                                                println!(
                                                    "{}{}",
                                                    "Echoing literal: ".cyan(),
                                                    text.cyan()
                                                );
                                            } else {
                                                let mut found = false;
                                                for var in vrs.iter() {
                                                    if var.name == text {
                                                        found = true;
                                                        println!(
                                                            "{}{}",
                                                            "Echoing variable: ".cyan(),
                                                            text.cyan()
                                                        );
                                                        break;
                                                    }
                                                }
                                                if !found {
                                                    println!(
                                                        "{}{}{}{}",
                                                        "Variable not found in scope: ".red(),
                                                        text.red(),
                                                        " in echol statement: ".red(),
                                                        line.trim().red()
                                                    );
                                                    exit(0);
                                                }
                                            }
                                        }
                                    } else {
                                        println!(
                                            "ERROR - Could not capture text inside echol in line: {}",
                                            line
                                        );
                                    }
                                } else {
                                    println!(
                                        "{}{}",
                                        "Invalid 'echol()' syntax :: ".red(),
                                        line.trim().red()
                                    );
                                    println!("{}", "CANCELLING BUILD".blink().blue());
                                    exit(0);
                                }
                            } else if line.trim().is_empty() {
                                continue;
                            } else {
                                let mut found_function_call = false;
                                for i in fns.iter() {
                                    if line.trim().starts_with(&(i.clone() + "();")) {
                                        println!(
                                            "{}{}",
                                            "Handling function call: ".green(),
                                            line.trim().green()
                                        );
                                        found_function_call = true;
                                        break;
                                    }
                                }
                                if !found_function_call {
                                    undefined_fn_calls.push(line.trim().to_string());
                                    println!(
                                        "{}{} :",
                                        "Undefined function call found, will recheck later: "
                                            .yellow(),
                                        line.trim().yellow()
                                    );
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!(
                            "{}{}{}{}",
                            "Error Opening main file in the project: ".red(),
                            project_folder,
                            " : ERR - ".red(),
                            err.to_string().red()
                        );
                    }
                }
                let cd = wc;
                let bcd = cd.as_bytes();
                let tmpfol = DirBuilder::new();
                if Path::exists(Path::new("./tmp/vstartups.txt")) {
                    remove_file("./tmp/vstartups.txt").unwrap();
                }
                if Path::exists(Path::new("./tmp")) {
                    remove_dir("./tmp").unwrap();
                }

                match tmpfol.create("./tmp") {
                    Ok(_tmpfol) => {
                        let tempfol = "./tmp";
                        match File::create(tempfol.to_owned() + "/vstartups.txt") {
                            Ok(mut tf) => {
                                match tf.write_all(bcd) {
                                    Ok(_m) => {
                                        let _lcd =
                                            encrypt(&(tempfol.to_owned() + "/vstartups.txt"));
                                        if Path::exists(Path::new(
                                            &(project_folder.to_owned() + "/cfg.bcf"),
                                        )) {
                                            match File::open(project_folder.to_owned() + "/cfg.bcf")
                                            {
                                                Ok(mut cfgf) => {
                                                    let mut cfgs = String::new();
                                                    cfgf.read_to_string(&mut cfgs).unwrap();
                                                    //println!("\n\ncfg : {}",cfgs.trim());
                                                    let cfg = cfgs.split("\n");
                                                    let mut c = CFG {
                                                        name: String::new(),
                                                        date: String::new(),
                                                        auth: String::new(),
                                                    };
                                                    for cfg in cfg {
                                                        if cfg.starts_with("NAME") {
                                                            let i = cfg.split(":");
                                                            for m in i {
                                                                if m != "NAME" {
                                                                    c.name = m.trim().to_string();
                                                                }
                                                            }
                                                        } else if cfg.starts_with("DATE") {
                                                            let i = cfg.split(":");
                                                            for m in i {
                                                                if m != "DATE" {
                                                                    c.date = m.trim().to_string();
                                                                }
                                                            }
                                                        } else if cfg.starts_with("AUTHORS") {
                                                            let i = cfg.split(":");
                                                            for m in i {
                                                                if m != "DATE" {
                                                                    c.auth = m.trim().to_string();
                                                                }
                                                            }
                                                        } else {
                                                            continue;
                                                        }
                                                    }
                                                    println!("\n\ncfg - {:?}\n\n", c);
                                                }
                                                Err(err) => {
                                                    println!("{} {}","unable to open/find config file named 'cfg.cfg' in project folder - ",project_folder);
                                                    println!(
                                                        "{}{}",
                                                        "err - ".red(),
                                                        err.to_string().red()
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        println!(
                                            "{} {}",
                                            "err writting temp data : err - ",
                                            err.to_string()
                                        );
                                    }
                                }
                            }
                            Err(err) => {
                                println!("{} {}", "err making temp file - : ", err.to_string());
                            }
                        }
                    }
                    Err(err) => {
                        println!("{} {}", "err making temp folder - : ", err.to_string())
                    }
                }
            }
            Err(err) => {
                if pfci != 0 {
                    println!(
                        "{}{}",
                        "Error opening file 'main.bb' in project folder provided! \nerr - ".red(),
                        err.to_string().red()
                    );
                    exit(-1);
                } else {
                    pfci += 1;
                }
            }
        }
    }

    for undefined_fn_call in &undefined_fn_calls {
        let mut found = false;
        for func in &fns {
            if undefined_fn_call.starts_with(&(func.clone() + "();")) {
                found = true;
                println!(
                    "{} {} {} :",
                    "function call declared fixing stuff...: ",
                    func.clone(),
                    undefined_fn_call
                );
                break;
            }
        }
        if !found {
            println!(
                "{}{}",
                "ERROR - Undefined function call found: ".red(),
                undefined_fn_call.red()
            );
            exit(1);
        }
    }

    for i in vrs {
        Varr::dis(i);
    }

    println!("{}", "Build successful!".green());
}
