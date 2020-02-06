use std::env;
fn main() {
    let args = env::args().skip(1);
    for arg in args {
        print_filenames(arg);
    }
}

fn print_filenames(ns: String) {
    let ns: Vec<_> = ns.split('-').map(str::parse::<u16>).collect();
    match ns[..] {
        [Ok(n)] => println!("{}", filename(n)),
        [Ok(lft), Ok(rght)] if lft < rght => {
            for i in lft..=rght {
                println!("{}", filename(i));
            }
        }
        _ => panic!("Error while parsing"),
    }
}

fn filename(n: u16) -> String {
    format!("REC{:03}.WAV", n)
}
