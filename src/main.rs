// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Carles Pey <cpey@pm.me>
 */

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::io::{self, BufRead};
use std::process::{Command, Stdio};
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
    let re_h_m_file = Regex::new(r"\.h-#").unwrap();
    let re_h_m_path = Regex::new(r"(.*\.h)-").unwrap();
    let re_h_m_func_regex = format!(r"# +define +([^ ].*)\(.*\).*\\");
    let re_h_m_func = Regex::new(&re_h_m_func_regex).unwrap();
    // header file: multiline #define on the line below
    let re_h_m_c_file = Regex::new(r"\.h-[^#].*\\").unwrap();
    let re_h_m_c_path = Regex::new(r"(.*\.h)-").unwrap();
    // header file: multiline #define on the match line
    let re_h_m_m_file = Regex::new(r"\.h:").unwrap();
    let re_h_m_m_path = Regex::new(r"(.*\.h):").unwrap();
    let re_h_m_m_func_regex = format!(r".*[\( \*]+({})\(.*", func);
    let re_h_m_m_func = Regex::new(&re_h_m_m_func_regex).unwrap();

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
        let s = "";
        let p = match re_h_m_c_path.captures(line) {
            Some(x) => x.get(1).map_or("", |m| m.as_str()),
            None => return None,
        };
        Some((s, p, true))
    } else if *multi && re_h_m_m_file.is_match(line) {
        let s = match re_h_m_m_func.captures(line) {
            Some(x) => x.get(1).map_or("", |m| m.as_str()),
            None => return None,
        };
        let p = match re_h_m_m_path.captures(line) {
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

/// Returns a vector with all the callers for the given function name
fn get_callers(name: &str, _search_path: &str) -> Result<Vec<(String, String, String)>> {
    let mut multi = false;
    let mut m_caller = "".to_string();
    let mut funcs = Vec::new();
    let _name = format!("[\\* \\(]+{}\\(", name);

    let mut grep_c = Command::new("git")
        .arg("-C")
        .arg(&_search_path)
        .arg("grep")
        .arg("-WE")
        .arg(&_name)
        .arg("*.c")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute git");

    let mut grep_h = Command::new("git")
        .arg("-C")
        .arg(&_search_path)
        .arg("grep")
        .arg("-WE")
        .arg(&_name)
        .arg("*.h")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute git");

    // Process data from .c files
    let reader = io::BufReader::new(grep_c.stdout.take().expect("Failed to capture stdout"));
    for line in reader.lines() {
        let __name = format!("{}", name);
        match line {
            Ok(l) => match get_function_name(&name, &l, &multi) {
                Some((n, p, _)) => funcs.push((n.to_string(), p.to_string(), __name)),
                None => continue,
            },
            Err(_) => continue,
        };
    }

    // Process data from .h files
    let reader = io::BufReader::new(grep_h.stdout.take().expect("Failed to capture stdout"));
    for line in reader.lines() {
        let __name = format!("{}", name);
        match line {
            Ok(l) => match get_function_name(&name, &l, &multi) {
                Some((n, p, m)) => {
                    if multi == true && n == name && m_caller != "" {
                        let m_c = &m_caller;
                        funcs.push((m_c.to_string(), p.to_string(), __name));
                    } else if multi == false && m == false {
                        funcs.push((n.to_string(), p.to_string(), __name));
                    }
                    if m == true && n != "" {
                        m_caller = n.to_string();
                    } else if n != "" {
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

fn get_callers_thread(
    name: &str,
    path: &str,
    callee: &str,
    max_cnt: u32,
    mut cnt: u32,
    search_path: &str,
) -> Result<thread::JoinHandle<Option<Vec<(String, String, String)>>>> {
    let _name = name.to_string();
    let _path = path.to_string();
    let _callee = callee.to_string();
    let _max_cnt = max_cnt;
    let _cnt = cnt;
    let _search_path = search_path.to_string();
    let thread = thread::spawn(move || -> Option<Vec<(String, String, String)>> {
        get_callers_recursive(&_name, &_path, &_callee, _max_cnt, _cnt, &_search_path)
    });
    Ok(thread)
}

fn print_info(calls: Vec<(String, String, String)>) {
    for (name, path, callee) in calls {
        println!(
            " + Function: {}, calling: {}\n\tpath: {}",
            name, callee, path
        );
    }
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
    let mut threads = Vec::new();

    for n in &args.search_functions {
        println!("++ Call chain for {}", n);
        let th = get_callers_thread(n, "", "", args.iterations, 0, &args.search_path);
        threads.extend(th);
    }
    for t in threads {
        match t.join() {
            Ok(l) => match l {
                Some(v) => print_info(v),
                None => continue,
            },
            Err(_) => continue,
        }
    }
}
