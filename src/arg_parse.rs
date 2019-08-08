/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   arg_parse.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/06 17:30:20 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/08 16:30:58 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

enum Arg {
    Flag(fn(&mut Param)),
    Invalid(String),
}

pub struct Param {
    bin_path: String,
    verbose: bool,
    bench: bool,
    run: bool,
}

impl Param {
    pub fn new(argc: usize, argv: Vec<String>) -> Option<Self> {
        let mut param = Param {
            bin_path: argv[0].clone(),
            verbose: false,
            bench: false,
            run: true,
        };
        let mut i: usize = 1;
        while i < argc {
            match parse_arg(&argv[i]) {
                Arg::Flag(fun) => fun(&mut param),
                Arg::Invalid(inv_arg) => {
                    eprintln!(
                        "[err-arg] - invalid argument detected: '{}'",
                        inv_arg
                    );
                    usage(&mut param);
                    return None;
                }
            }
            i += 1;
        }
        return Some(param);
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn bench(&self) -> bool {
        self.bench
    }

    pub fn run(&self) -> bool {
        self.run
    }
}

fn parse_arg(arg: &String) -> Arg {
    match *arg {
        ref str if str == "-h" || str == "--help" => Arg::Flag(usage),
        ref str if str == "-v" || str == "--verbose" => Arg::Flag(set_verbose),
        ref str if str == "-b" || str == "--benchmark" => Arg::Flag(set_bench),
        _ => Arg::Invalid(arg.clone()),
    }
}

fn set_verbose(param: &mut Param) {
    param.verbose = true;
}

fn set_bench(param: &mut Param) {
    param.bench = true;
}

fn usage(param: &mut Param) {
    println!(
        "usage: {} {}\n    {}\n    {}\n    {}",
        param.bin_path,
        "[-h | --help] [-v | --verbose] [-b | --bench]",
        "-h, --help: Show this message.",
        "-v, --verbose: Show additionnal informations per stage of execution.",
        "-b, --benchmark: Show duration of each stage of execution."
    );
    param.run = false;
}
