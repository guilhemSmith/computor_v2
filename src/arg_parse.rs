/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   arg_parse.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: gsmith <gsmith@student.42.fr>              +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/08/06 17:30:20 by gsmith            #+#    #+#             */
/*   Updated: 2019/08/06 18:24:44 by gsmith           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

enum Arg {
    Flag(fn(&mut Param)),
    Invalid(String),
}

pub struct Param {
    bin_path: String,
    verbose: bool,
	run: bool,
}

impl Param {
    pub fn new(argc: usize, argv: Vec<String>) -> Option<Self> {
        let mut param = Param {
            bin_path: argv[0].clone(),
            verbose: false,
			run: true
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

	pub fn run(&self) -> bool {
		self.run
	}
}

fn parse_arg(arg: &String) -> Arg {
    match *arg {
        ref str if str == "-h" || str == "--help" => Arg::Flag(usage),
        ref str if str == "-v" || str == "--verbose" => Arg::Flag(set_verbose),
        _ => Arg::Invalid(arg.clone()),
    }
}

fn set_verbose(param: &mut Param) {
    param.verbose = true;
}

fn usage(param: &mut Param) {
    println!(
        "usage: {} [-h | --help] [-v | --verbose]\n    {}\n    {}",
        param.bin_path,
        "-h, --help: Show this message.",
        "-v --verbose: Show additionnal informations per stage of execution."
    );
	param.run = false;
}
