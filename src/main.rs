extern crate rand;
extern crate getopts;

use std::{env, io, char};
use rand::{thread_rng, Rng};
use getopts::Options;

const KANA_LO: u32 = 0x30A0;
const KANA_HI: u32 = 0x30FF;

fn print_usage(program: &str, opts: Options) {
    let usage = format!("Usage: {} [OPTIONS]", program);
    print!("{}", opts.usage(&usage));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");
    opts.optopt("k", "kana", "append N random katakana characters, up to 255", "N");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let kata_opt = matches.opt_str("k");
    let mut num_kata: u8 = match kata_opt {
        Some(x) => { x.parse().unwrap() }
        None => { 0 }
    };

    let mut input = String::new();
    loop {
        let bytes_read = match io::stdin().read_line(&mut input) {
            Ok(n) => n,
            Err(e) => panic!(e.to_string()),
        };
        if bytes_read == 0 {
            break;  // indicates EOF
        }
    }

    if num_kata > 0 {
        if input.ends_with('\n') {
            input.pop(); // we want to insert the kana before the newline, if it exists
        }
        let mut rng = thread_rng();
        while num_kata > 0 {
            let n: u32 = rng.gen_range(KANA_LO, KANA_HI + 1);
            let c = match char::from_u32(n) {
                Some(x) => x,
                None => '\0', // lol
            };
            input.push(c);
            num_kata -= 1;
        }
    }

    println!("{}", input);
    return;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
