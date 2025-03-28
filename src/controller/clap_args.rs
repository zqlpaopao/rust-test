#![allow(unused)]

use cargo::util::config::Definition::Cli;
use clap::{arg, command, value_parser, Arg, ArgAction, Command, Parser, Subcommand};
use log::debug;
use std::path::PathBuf;

fn test() {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = matches.get_one::<String>("name") {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches
        .get_one::<u8>("debug")
        .expect("Count's are defaulted")
    {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    if let Some(matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.get_flag("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }

    // Continued program logic goes here...
}

/////////////////////////////////////// Command解析器 //////////////////////////////////
fn command() {
    let matches = Command::new("MyApp")
        .next_line_help(true)
        .version("1.0")
        .author("ZHangQL Z <ZQL@gmail.com>")
        .about("this is the test project")
        .args(&[
            //次数是args，如果单个的的arg
            arg!(--config <FILE> "a required file for the configuration and no short")
                .required(true) //必须包含
                .require_equals(true), //要求使用等号赋值
            // .default_value() //设置默认值
            arg!(-d --debug ... "turns on debugging information and allows multiples"),
            arg!([input] "an optional input file to use"),
        ])
        .arg(arg!(--two <VALUE>).required(true)) //单个的
        .get_matches();

    println!(
        "config: {:?}",
        matches.get_one::<String>("config").expect("required")
    );
    println!("two: {:?}", matches.get_one::<String>("two"));
    println!("input: {:?}", matches.get_one::<String>("input"));
}
/////////////////////////////////////// Command::arg //////////////////////////////////
fn adding_arg() {
    let matches = command!().arg(Arg::new("name")).get_matches();
    println!("name: {:?}", matches.get_one::<String>("name"));
}

/////////////////////////////////////// Command::参数行为 //////////////////////////////////
fn arg_action() {
    let matches = command!()
        .arg(Arg::new("name").action(ArgAction::Append))
        .get_matches();
}

fn arg_switch() {
    let matches = command!()
        .arg(Arg::new("name").short('n').long("name"))
        .get_matches();
    println!("name: {:?}", matches.get_one::<String>("name"));
}

fn count() {
    let matches = command!() // requires `cargo` feature
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count),
        )
        .get_matches();

    println!("verbose: {:?}", matches.get_count("verbose"));
}

///默认值
fn default_value() {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!([PORT])
                .value_parser(value_parser!(u16))
                .default_value("2023"),
        )
        .get_matches();

    println!(
        "port: {:?}",
        matches
            .get_one::<u16>("PORT")
            .expect("default ensures there is always a value")
    );
}

/////////////////////////////////////// 枚举值校验 //////////////////////////////////
fn enum_check() {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(<MODE>)
                .help("What mode to run the program in")
                .value_parser(["fast", "slow"]),
        )
        .get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    match matches
        .get_one::<String>("MODE")
        .expect("'MODE' is required and parsing will fail if its missing")
        .as_str()
    {
        "fast" => {
            println!("Hare");
        }
        "slow" => {
            println!("Tortoise");
        }
        _ => unreachable!(),
    }
}
/////////////////////////////////////// 校验值 //////////////////////////////////
fn validated() {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(<PORT>)
                .help("Network port to use")
                .value_parser(value_parser!(u16).range(1..)),
        )
        .get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    let port: u16 = *matches
        .get_one::<u16>("PORT")
        .expect("'PORT' is required and parsing will fail if its missing");
    println!("PORT = {port}");
}

/////////////////////////////////////// derive feature           //////////////////////////////////

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Clis {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    // value_name  -c, --config <FILE>  Sets a custom config file
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}
fn quick_test() {
    let cli = Clis::parse();

    if let Some(name) = cli.name.as_ref() {
        println!("value for name {name}")
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("value for config: {:?}", config_path)
    }

    match cli.debug {
        0 => println!("debug mode is off"),
        1 => println!("debug mode is kind of on"),
        2 => println!("debug mode is on"),
        _ => println!("dont be crazy"),
    }

    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("printing testing lists...")
            } else {
                println!("not printing testing lists...")
            }
        }
        None => {}
    }
}
///////////////////////////////////////////// 解析器 /////////////////////////////////////////

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Clic {
    #[arg(long)]
    two: String,
    #[arg(long)]
    one: String,
}
fn parse() {
    let cli = Clic::parse();
    println!("value for two {:?}", cli.two);
    println!("value for one {:?}", cli.one)
}

///////////////////////////////////////////// 可选参数和多值参数 /////////////////////////////////////////

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct CliW {
    name: Option<String>,
    more: Vec<String>,
}
fn switch() {
    let cli = CliW::parse();
    println!("value for name {:?}", cli.name);
    println!("value for more {:?}", cli.more);
}
///////////////////////////////////////////// 多值参数 /////////////////////////////////////////

pub fn claps() {
    // test()
    // command()
    // adding_arg()
    // arg_switch()
    // count()
    // default_value()
    // validated()
    // quick_test()
    // parse()
    switch()
}
