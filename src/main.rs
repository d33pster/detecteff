use argrust::{Arguments, FetchTypes, ArgumentDescription};
use detecteff::Files;
use rustypath::RPath;
use colorized::*;

fn main() {
    let mut args = Arguments::new(std::env::args().skip(1).collect());
    args.add("--scan", ArgumentDescription::new().short("-s"));
    args.add("--recursive", ArgumentDescription::new().short("-r"));
    args.add("--format", ArgumentDescription::new().short("-f"));
    args.add("--help", ArgumentDescription::new().short("-h"));
    args.add("--version", ArgumentDescription::new().short("-v"));
    args.add("--delete", ArgumentDescription::new().short("-d"));
    args.add("--ignore", ArgumentDescription::new().short("-i"));

    
    args.analyse();

    if args.ifarg_force("--version") {
        println!("detecteff {}", "v0.3.1".color(Colors::RedFg));
        std::process::exit(0);
    }

    if args.ifarg_force("--help") {
        println!("detecteff {}", "help".color(Colors::YellowFg));
        println!("   -\n   {}","[INFO]".color(Colors::BlueFg));
        println!("   | -h, --help : show help text and exit.");
        println!("   | -v, --version : show version and exit.");
        println!("   -\n   {}","[FLAG]".color(Colors::BlueFg));
        println!("   | -r, --recursive : recursive mode. Default -> OFF");
        println!("   | -f, --format : show formatted output. Default -> OFF");
        println!("   -\n   {}","[INPUT]".color(Colors::BlueFg));
        println!("   | -s, --scan <directory> : scan the directory for duplicate files. ({})", "Mandatory".color(Colors::RedFg));
        println!("   | -i, --ignore <directory1>, <directory2>, ... : ignore these directories. ({})", "Optional".color(Colors::RedFg));
        println!("   -\n   {}", "[IRREVERSIBLE FLAG]".color(Colors::RedFg));
        println!("   | -d, --delete : delete any found duplicates. Default -> OFF");
        std::process::exit(0);
    }

    let mut recursive = false;
    let mut formatted = false;
    let mut delete = false;
    let mut ignore: Vec<String> = Vec::new();

    if args.ifarg_force("--delete") {
        delete = true;
    }

    if args.ifarg_force("--recursive") {
        recursive = true;
    }

    if args.ifarg_force("--format") {
        formatted = true;
    }

    if args.ifarg_force("--ignore"){
        ignore = args.fetch("--ignore", FetchTypes::TillNext).get();
    }

    if args.ifarg_force("--scan") {
        let path = RPath::new_from(&args.fetch("--scan", FetchTypes::Fetch(1)).get()[0]).expand();

        let mut dir = Files::new(path, recursive, ignore);

        dir.find_duplicates(true);

        println!("");

        if formatted {
            dir.formatted();
        } else {
            dir.show();
        }

        if delete {
            dir.delete_duplicates(formatted);
        }
    }
}