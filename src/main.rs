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

fn get_function_name(line: &str) -> Option<(&str, &str)> {
    let re_file = Regex::new(r"\.c=").unwrap();
    let re_path = Regex::new(r"(.*\.c)=").unwrap();
    let re_fname = Regex::new(r"\.c=(.*) \*?(.*)\(").unwrap();
    if re_file.is_match(line) {
        let s = match re_fname.captures(line) {
            Some(x) => x.get(2).map_or("", |m| m.as_str()),
            None => return None,
        };
        let p = match re_path.captures(line) {
            Some(x) => x.get(1).map_or("", |m| m.as_str()),
            None => return None,
        };
        Some((s, p))
    } else {
        None
    }
}

fn get_callers(name: &str) -> Result<Vec<(String, String, String)>> {
    let _name = format!("{}", name);
    let thread = thread::spawn(move || -> Result<Vec<(String, String, String)>> {
        let mut funcs = Vec::new();
        let __name = format!("{}(", _name);
        //find . -name "*.c" | xargs git grep -W <name>
        let mut grep = Command::new("xargs")
            .arg("git")
            .arg("grep")
            .arg("-W")
            .arg(__name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute git");

        Command::new("find")
            .arg(".")
            .arg("-name")
            .arg("*.c")
            .stdout(grep.stdin.unwrap())
            .output()
            .expect("Failed to execute find");

        let reader = io::BufReader::new(grep.stdout.take().expect("Failed to capture stdout"));
        for line in reader.lines() {
            let ___name = format!("{}", _name);
            match line {
                Ok(l) => match get_function_name(&l) {
                    Some((n, p)) => funcs.push((n.to_string(), p.to_string(), ___name)),
                    None => continue,
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

    if name.eq("main") {
        return None;
    }

    let mut call = match get_callers(name) {
        Ok(n) => n,
        Err(_) => return None,
    };

    cnt += 1;
    let mut recurse = Vec::new();
    for (n, p, c) in (&call).into_iter().unique() {
        match get_callers_recursive(n, p, c, max_cnt, cnt) {
            Some(c) => recurse.extend(c),
            None => continue,
        };
    }
    call.extend(recurse);
    return Some(call);
}

#[derive(StructOpt)]
struct Opt {
    /// Number of iterations.
    #[structopt(short, long)]
    iterations: u32,
    /// Function names to get the call chain for.
    search_functions: Vec<String>,
}

fn main() {
    let args = Opt::from_args();

    for n in &args.search_functions {
        println!("++ Call chain for {}", n);
        get_callers_recursive(n, "", "", args.iterations, 0);
    }
}
