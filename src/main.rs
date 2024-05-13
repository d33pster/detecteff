use argrust::{Arguments, FetchTypes, ArgumentDescription};
use detecteff::Files;
use rustypath::RPath;

fn main() {
    let mut args = Arguments::new(std::env::args().skip(1).collect());
    args.add("--scan", ArgumentDescription::new().short("-s"));
    args.add("--recursive", ArgumentDescription::new().short("-r"));
    args.add("--format", ArgumentDescription::new().short("-f"));
    args.add("--help", ArgumentDescription::new().short("-h"));
    args.add("--version", ArgumentDescription::new().short("-v"));

    
    args.analyse();

    if args.ifarg_force("--version") {
        println!("detecteff v0.1.0");
        std::process::exit(0);
    }

    if args.ifarg_force("--help") {
        println!("detecteff help");
        println!("   -\n   [INFO]");
        println!("   | -h, --help : show help text and exit.");
        println!("   | -v, --version : show version and exit.");
        println!("   -\n   [FLAG]");
        println!("   | -r, --recursive : recursive mode. Default -> OFF");
        println!("   | -f, --format : show formatted output. Default -> OFF");
        println!("   -\n   [INPUT]");
        println!("   | -s, --scan <directory> : scan the directory for duplicate files.");
        std::process::exit(0);
    }

    let mut recursive = false;
    let mut formatted = false;

    if args.ifarg_force("--recursive") {
        recursive = true;
    }

    if args.ifarg_force("--format") {
        formatted = true;
    }

    if args.ifarg_force("--scan") {
        let path = RPath::new_from(&args.fetch("--scan", FetchTypes::Fetch(1)).get()[0]).expand();

        let mut dir = Files::new(path, recursive);

        dir.find_duplicates(true);

        println!("");

        if formatted {
            dir.formatted();
        } else {
            dir.show();
        }
    }
}