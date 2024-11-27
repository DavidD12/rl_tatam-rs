use clap::Parser;
use rl_model::*;
use rl_tatam::to_tatam;
use rl_tatam::lines_from_file;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// robot language file
    #[arg(short, long)]
    file: String,
    /// composite skill name file
    #[arg(short, long, default_value_t = String::new())]
    composite: String,
    /// verbose level
    #[arg(short, long, default_value_t = 1)]
    verbose: u8,
}
fn main() {
    let args = Args::parse();
    if args.verbose > 0 {
        //
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "info")
        }
        env_logger::init();
    }
    //
    if let Ok(skillset) = load_skillset(&args.file) {
        if args.verbose >= 3 {
            println!("\n--------------------------------------------------\n");
            println!("{}", skillset);
        }
        if args.verbose >= 2 {
            println!("\n--------------------------------------------------\n");
            println!("{}", skillset);
        }

        let tatam = to_tatam(&skillset, &lines_from_file(&args.composite));
        println!("{}", tatam);
    }
}
