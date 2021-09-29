// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Carles Pey <cpey@pm.me>
 */

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::io::{self, BufRead};
use std::process::{Command, Stdio};
use std::thread;
use structopt::StructOpt;

fn get_function_name<'a>(
    func: &'a str,
    line: &'a str,
    multi: &bool,
) -> Option<(&'a str, &'a str, bool)> {
    // c file
    let re_c_file = Regex::new(r"\.c=").unwrap();
    let re_c_path = Regex::new(r"(.*\.c)=").unwrap();
    let re_c_func = Regex::new(r"\.c=(.*) \*?(.*)\(").unwrap();
    // header file: single line #define
    let re_h_file = Regex::new(r"\.h:").unwrap();
    let re_h_path = Regex::new(r"(.*\.h):").unwrap();
    let re_h_func_regex = format!(r"# +define +([^ ].*)\(.*\).*{}\(.*", func);
    let re_h_func = Regex::new(&re_h_func_regex).unwrap();
    // header file: multiline #define on the first line
    let re_h_m_file = Regex::new(r"\.h-").unwrap();
    let re_h_m_path = Regex::new(r"(.*\.h)-").unwrap();
    let re_h_m_func_regex = format!(r"# +define +([^ ].*)\(.*\).*\\");
    let re_h_m_func = Regex::new(&re_h_m_func_regex).unwrap();
    // header file: multiline #define on the line below
    let re_h_m_c_file = Regex::new(r"\.h:").unwrap();
    let re_h_m_c_path = Regex::new(r"(.*\.h):").unwrap();
    let re_h_m_c_func_regex = format!(r".*[\( \*]+({})\(.*", func);
    let re_h_m_c_func = Regex::new(&re_h_m_c_func_regex).unwrap();

    if re_c_file.is_match(line) {
        let s = match re_c_func.captures(line) {
            Some(x) => x.get(2).map_or("", |m| m.as_str()),
            None => return None,
        };
        let p = match re_c_path.captures(line) {
            Some(x) => x.get(1).map_or("", |m| m.as_str()),
            None => return None,
        };
        Some((s, p, false))
    } else if re_h_m_file.is_match(line) {
        let s = match re_h_m_func.captures(line) {
            Some(x) => x.get(1).map_or("", |m| m.as_str()),
            None => return None,
        };
        let p = match re_h_m_path.captures(line) {
            Some(x) => x.get(1).map_or("", |m| m.as_str()),
            None => return None,
        };
        Some((s, p, true))
    } else if *multi && re_h_m_c_file.is_match(line) {
        let s = match re_h_m_c_func.captures(line) {
            Some(x) => x.get(1).map_or("", |m| m.as_str()),
            None => return None,
        };
        let p = match re_h_m_c_path.captures(line) {
            Some(x) => x.get(1).map_or("", |m| m.as_str()),
            None => return None,
        };
        Some((s, p, false))
    } else if re_h_file.is_match(line) {
        let s = match re_h_func.captures(line) {
            Some(x) => x.get(1).map_or("", |m| m.as_str()),
            None => return None,
        };
        let p = match re_h_path.captures(line) {
            Some(x) => x.get(1).map_or("", |m| m.as_str()),
            None => return None,
        };
        Some((s, p, false))
    } else {
        None
    }
}

fn get_callers(name: &str, search_path: &str) -> Result<Vec<(String, String, String)>> {
    let _name = format!("{}", name);
    let _search_path = format!("{}", search_path);
    let thread = thread::spawn(move || -> Result<Vec<(String, String, String)>> {
        let mut multi = false;
        let mut m_caller = "".to_string();
        let mut funcs = Vec::new();
        let __name = format!("[\\* \\(]+{}\\(", _name);
        // For some reason Command stalls when `find` carries the `-o` flag. Will need to run the
        // search separately for .c and .h files.
        // find . -name "*.c" -o -name "*.h" | xargs git grep -W <name>
        let mut grep_c = Command::new("xargs")
            .arg("git")
            .arg("-C")
            .arg(&_search_path)
            .arg("grep")
            .arg("-WE")
            .arg(&__name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute git");

        let mut grep_h = Command::new("xargs")
            .arg("git")
            .arg("-C")
            .arg(&_search_path)
            .arg("grep")
            .arg("-WE")
            .arg(&__name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute git");

        Command::new("find")
            .arg(&_search_path)
            .arg("-name")
            .arg("*.c")
            .stdout(grep_c.stdin.unwrap())
            .output()
            .expect("Failed to execute find");

        Command::new("find")
            .arg(&_search_path)
            .arg("-name")
            .arg("*.h")
            .stdout(grep_h.stdin.unwrap())
            .output()
            .expect("Failed to execute find");

        let reader = io::BufReader::new(grep_c.stdout.take().expect("Failed to capture stdout"));
        for line in reader.lines() {
            let ___name = format!("{}", _name);
            match line {
                Ok(l) => match get_function_name(&_name, &l, &multi) {
                    Some((n, p, _)) => funcs.push((n.to_string(), p.to_string(), ___name)),
                    None => continue,
                },
                Err(_) => continue,
            };
        }

        let reader = io::BufReader::new(grep_h.stdout.take().expect("Failed to capture stdout"));
        for line in reader.lines() {
            let ___name = format!("{}", _name);
            match line {
                Ok(l) => match get_function_name(&_name, &l, &multi) {
                    Some((n, p, m)) => {
                        if multi == true && n == _name && m_caller != "" {
                            funcs.push((m_caller, p.to_string(), ___name));
                        } else if multi == false && m == false {
                            funcs.push((n.to_string(), p.to_string(), ___name));
                        }
                        if m == true {
                            m_caller = n.to_string();
                        } else {
                            m_caller = "".to_string();
                        }
                        multi = m;
                    }
                    None => {
                        multi = false;
                        m_caller = "".to_string();
                    }
                },
                Err(_) => continue,
            };
        }
        Ok(funcs)
    });

    thread.join().unwrap()
}

fn get_callers_recursive(
    name: &str,
    path: &str,
    callee: &str,
    max_cnt: u32,
    mut cnt: u32,
    search_path: &str,
) -> Option<Vec<(String, String, String)>> {
    if cnt > max_cnt {
        return None;
    }

    if cnt > 0 {
        println!(
            " + Function: {}, calling: {}, iteration: {}\n\tpath: {}",
            name, callee, cnt, path
        );
    }

    if name.to_lowercase().eq("main") {
        return None;
    }

    let mut call = match get_callers(name, search_path) {
        Ok(n) => n,
        Err(_) => return None,
    };

    cnt += 1;
    let mut recurse = Vec::new();
    for (n, p, c) in (&call).into_iter().unique() {
        match get_callers_recursive(n, p, c, max_cnt, cnt, search_path) {
            Some(c) => recurse.extend(c),
            None => continue,
        };
    }
    call.extend(recurse);
    return Some(call);
}

#[derive(StructOpt)]
struct Opt {
    /// Path of the git repo in which to run the search.
    search_path: String,
    /// Number of iterations (defaults to 5).
    #[structopt(short, long)]
    iterations: u32,
    /// List of the function names to get the call tree for.
    search_functions: Vec<String>,
}

fn main() {
    let args = Opt::from_args();

    for n in &args.search_functions {
        println!("++ Call chain for {}", n);
        get_callers_recursive(n, "", "", args.iterations, 0, &args.search_path);
    }
}
