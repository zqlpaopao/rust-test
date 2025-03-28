# 一、clap_v3

本来是想用structOpt，但是看文档是这样描述的

由于 clap v3 现已发布，并且 structopt 功能已集成（几乎按原样），因此 structopt 现在处于维护模式：不会添加新功能。

错误将被修复，文档改进将被接受。

## 1. 1 添加依赖

```
[dependencies]
clap = { version = "4.2.7", features = ["derive","cargo"] }
features = "0.10.0"
cargo = "0.70.1"


或者
cargo add clap -- features 	cargo
需要注意：如果不启用 cargo feature ，则会报如下错误。
requires `cargo` feature
```

如果使用`command!`、`arg!` 必须在features 中添加cargo



## 1.2 快速启动

```
use std::env::Args;
/////////////////////////////////////// clap_v3 原来的structOpt //////////////////////////////////
use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};

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

pub fn claps(){
    test()
}
```

1、默认执行情况

```
cargo run
Debug mode is off
```

2、参看帮助文档

```
cargo run --help
Run a binary or example of the local package

Usage: cargo run [OPTIONS] [args]...

Arguments:
  [args]...  Arguments for the binary or example to run

Options:
  -q, --quiet                   Do not print cargo log messages
      --bin [<NAME>]            Name of the bin target to run
      --example [<NAME>]        Name of the example target to run
  -p, --package [<SPEC>]        Package with the target to run
  -j, --jobs <N>                Number of parallel jobs, defaults to # of CPUs.
      --keep-going              Do not abort the build as soon as there is an error (unstable)
  -r, --release                 Build artifacts in release mode, with optimizations
      --profile <PROFILE-NAME>  Build artifacts with the specified profile
  -F, --features <FEATURES>     Space or comma separated list of features to activate
      --all-features            Activate all available features
      --no-default-features     Do not activate the `default` feature
      --target <TRIPLE>         Build for the target triple
      --target-dir <DIRECTORY>  Directory for all generated artifacts
      --manifest-path <PATH>    Path to Cargo.toml
      --message-format <FMT>    Error format
      --unit-graph              Output build graph in JSON (unstable)
      --ignore-rust-version     Ignore `rust-version` specification in packages
      --timings[=<FMTS>]        Timing output formats (unstable) (comma separated): html, json
  -h, --help                    Print help
  -v, --verbose...              Use verbose output (-vv very verbose/build.rs output)
      --color <WHEN>            Coloring: auto, always, never
      --frozen                  Require Cargo.lock and cache are up to date
      --locked                  Require Cargo.lock is up to date
      --offline                 Run without accessing the network
      --config <KEY=VALUE>      Override a configuration value
  -Z <FLAG>                     Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details

Run `cargo help run` for more detailed information.
```

3、使用 -dd 参数

```
cargo run -- -dd test 
Debug mode is on
Not printing testing lists...
```



## 1.3 command 解析器



### 1.3.1 基本使用

```
fn command(){
    let matches = Command::new("MyApp")
        .version("1.0")
        .author("ZHangQL Z <ZQL@gmail.com>")
        .about("this is the test project")
        .args(&[//次数是args，如果单个的的arg
            arg!(--config <FILE> "a required file for the configuration and no short").
                required(true)//必须包含
                .require_equals(true)//要求使用等号赋值
                // .default_value() //设置默认值
            ,
            arg!(-d --debug ... "turns on debugging information and allows multiples"),
            arg!([input] "an optional input file to use")
        ])
        .arg(arg!(--two <VALUE>).required(true))//单个的
        .get_matches();

    println!(
        "config: {:?}",
        matches.get_one::<String>("config").expect("required")
    );
    println!(
        "debug: {:?}",
        matches.get_one::<String>("debug")
    );
    println!(
        "input: {:?}",
        matches.get_one::<String>("input")
    );

}

```



查看help

```
RUST_BACKTRACE=1 cargo run  -- --help

this is the test project

Usage: my_test [OPTIONS] --config=<FILE> --two <VALUE> [input]

Arguments:
  [input]  an optional input file to use

Options:
      --config=<FILE>  a required file for the configuration and no short
  -d, --debug...       turns on debugging information and allows multiples
      --two <VALUE>    
  -h, --help           Print help
  -V, --version        Print version

```



运行

```
RUST_BACKTRACE=1 cargo run  -- --config=./config.yaml --two rrr lllll

config: "./config.yaml"
two: Some("rrr")
input: Some("lllll")
```



### 1.3.2 使用command!构建解析器

你也可以使用 command! 宏 构建解析器，不过要想使用 command! 宏，你需要开启 cargo feature。

```
use clap::{arg, command};

fn main() {
    // requires `cargo` feature, reading name, version, author, and description from `Cargo.toml`
    let matches = command!()
        .arg(arg!(--two <VALUE>).required(true))
        .arg(arg!(--one <VALUE>).required(true))
        .get_matches();

    println!(
        "two: {:?}",
        matches.get_one::<String>("two").expect("required")
    );
    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );
}

```



### 1.3.3 Command::next_line_help

使用 Command::next_line_help 方法 可以修改参数打印行为

```
use clap::{arg, command, ArgAction};

fn main() {
    let matches = command!() // requires `cargo` feature
        .next_line_help(true)
        .arg(arg!(--two <VALUE>).required(true).action(ArgAction::Set))
        .arg(arg!(--one <VALUE>).required(true).action(ArgAction::Set))
        .get_matches();

    println!(
        "two: {:?}",
        matches.get_one::<String>("two").expect("required")
    );
    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );
}

```



```
Usage: my_test [OPTIONS] --config=<FILE> --two <VALUE> [input]

Arguments:
  [input]
          an optional input file to use

Options:
      --config=<FILE>
          a required file for the configuration and no short
  -d, --debug...
          turns on debugging information and allows multiples
      --two <VALUE>
          
  -h, --help
          Print help
  -V, --version
          Print version
```

效果就是：参数的描述和参数是分行的，描述信息在参数下一行。



## 1.4 添加命令行参数（Adding Arguments）

我们可以使用 Command::arg 方法来添加 Arg 对象来添加命令行参数

```
fn adding_arg(){
    let matches = command!()
        .arg(Arg::new("name"))
        .get_matches();
    println!("name: {:?}", matches.get_one::<String>("name"));
}
```

查看help

```
RUST_BACKTRACE=1 cargo run  -- --help
Usage: my_test [name]

Arguments:
  [name]  

Options:
  -h, --help     Print help
  -V, --version  Print version
```

2、使用 name 参数：默认

```bash
cargo run
name: None
```

3、使用 name 参数：blob

注意定义的时候没有是直接使用的 不需要key的

```bash
cargo run bob
name: Some("bob")
```

### 1.4.2 设置参数行为

需要注意：参数默认值是一个 Set 类型

我们可以使用 Command::action 方法来设置 参数行为。如果可以添加多个只，我们可以使用 ArgAction::Append

```rust
use clap::{command, Arg, ArgAction};

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("name").action(ArgAction::Append))
        .get_matches();

    let args = matches
        .get_many::<String>("name")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    println!("names: {:?}", &args);
}

```



## 1.5 参数选项

一个参数行为的标志：

- 顺序无关
- 可选参数
- 意图清晰



```
fn arg_switch(){
    let matches = command!()
        .arg(Arg::new("name")
            .short('n')
            .long("name")
        ).get_matches();
    println!("name: {:?}", matches.get_one::<String>("name"));
}
```

上述代码：我们定义了一个name参数，缩写是n，全拼是name，也就是如下形式

```
-n, --name <name>
```


我们使用方式就有如下几种

```
cargo run -- --name blo
cargo run -- --name=blob
cargo run -- -n blob
cargo run -- -n=blob
cargo run -- -nblob
```





### 1.5.1 开启/关闭标志
我们可以是 ArgAction::SetTrue 开启参数

```
use clap::{command, Arg, ArgAction};

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    println!("verbose: {:?}", matches.get_flag("verbose"));

}
```

### 1.5.2参数调用计数
我们可以使用 ArgAction::Count

```
use clap::{command, Arg, ArgAction};

fn main() {
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
```

默认值是0，多次使用参数就会计数

```
cargo run --  --verbose --verbose
```



### 1.5.3 默认值

```
fn default_value(){
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
```

```
cargo run
port: 2023

cargo run 897
port: 897
```



### 1.5.4 参数校验

默认情况下，参数被认为是 String，并且使用 UTF-8 校验。

**枚举值**

```
fn enum_check(){
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
```



```
cargo run rrr 
error: invalid value 'rrr' for '<MODE>'
  [possible values: fast, slow]

cargo run fast
Hare
```



如果我们开启了 derive feature， 则我们也可以实现 ValueEnum 特征实现相同的功能

```
use clap::{arg, builder::PossibleValue, command, value_parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Mode {
    Fast,
    Slow,
}

// Can also be derived with feature flag `derive`
impl ValueEnum for Mode {
    fn value_variants<'a>() -> &'a [Self] {
        &[Mode::Fast, Mode::Slow]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Mode::Fast => PossibleValue::new("fast").help("Run swiftly"),
            Mode::Slow => PossibleValue::new("slow").help("Crawl slowly but steadily"),
        })
    }

}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }

}

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(<MODE>)
                .help("What mode to run the program in")
                .value_parser(value_parser!(Mode)),
        )
        .get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    match matches
        .get_one::<Mode>("MODE")
        .expect("'MODE' is required and parsing will fail if its missing")
    {
        Mode::Fast => {
            println!("Hare");
        }
        Mode::Slow => {
            println!("Tortoise");
        }
    }

}

```

### 1.5.5 校验值

我们可以使用 Arg::value_parser 验证并解析成我们需要的任何类型。

```

fn validated(){
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

cargo run 0
error: invalid value '0' for '<PORT>': 0 is not in 1..=65535

cargo run 1
PORT = 10
```



### 1.5.6 自定义解析器

我们也可以使用自定义解析器用于改进错误信息提示和额外的验证。

```
use std::ops::RangeInclusive;

use clap::{arg, command};

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(<PORT>)
                .help("Network port to use")
                .value_parser(port_in_range),
        )
        .get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    let port: u16 = *matches
        .get_one::<u16>("PORT")
        .expect("'PORT' is required and parsing will fail if its missing");
    println!("PORT = {port}");
}

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}
```



### 1.5。7 参数关系(Argument Relations)

我们可以声明 Arg 和 ArgGroup。ArgGroup 用于声明参数关系。

```
use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, ArgGroup};

fn main() {
    // Create application like normal
    let matches = command!() // requires `cargo` feature
        // Add the version arguments
        .arg(arg!(--"set-ver" <VER> "set version manually"))
        .arg(arg!(--major         "auto inc major").action(ArgAction::SetTrue))
        .arg(arg!(--minor         "auto inc minor").action(ArgAction::SetTrue))
        .arg(arg!(--patch         "auto inc patch").action(ArgAction::SetTrue))
        // Create a group, make it required, and add the above arguments
        .group(
            ArgGroup::new("vers")
                .required(true)
                .args(["set-ver", "major", "minor", "patch"]),
        )
        // Arguments can also be added to a group individually, these two arguments
        // are part of the "input" group which is not required
        .arg(
            arg!([INPUT_FILE] "some regular input")
                .value_parser(value_parser!(PathBuf))
                .group("input"),
        )
        .arg(
            arg!(--"spec-in" <SPEC_IN> "some special input argument")
                .value_parser(value_parser!(PathBuf))
                .group("input"),
        )
        // Now let's assume we have a -c [config] argument which requires one of
        // (but **not** both) the "input" arguments
        .arg(
            arg!(config: -c <CONFIG>)
                .value_parser(value_parser!(PathBuf))
                .requires("input"),
        )
        .get_matches();

    // Let's assume the old version 1.2.3
    let mut major = 1;
    let mut minor = 2;
    let mut patch = 3;
    
    // See if --set-ver was used to set the version manually
    let version = if let Some(ver) = matches.get_one::<String>("set-ver") {
        ver.to_owned()
    } else {
        // Increment the one requested (in a real program, we'd reset the lower numbers)
        let (maj, min, pat) = (
            matches.get_flag("major"),
            matches.get_flag("minor"),
            matches.get_flag("patch"),
        );
        match (maj, min, pat) {
            (true, _, _) => major += 1,
            (_, true, _) => minor += 1,
            (_, _, true) => patch += 1,
            _ => unreachable!(),
        };
        format!("{major}.{minor}.{patch}")
    };
    
    println!("Version: {version}");
    
    // Check for usage of -c
    if matches.contains_id("config") {
        let input = matches
            .get_one::<PathBuf>("INPUT_FILE")
            .unwrap_or_else(|| matches.get_one::<PathBuf>("spec-in").unwrap())
            .display();
        println!(
            "Doing work using input {} and config {}",
            input,
            matches.get_one::<PathBuf>("config").unwrap().display()
        );
    }

}
```


此时 --set-ver <VER>|--major|--minor|--patch 是一个组的参数。

### 1.5.8 自定义校验(Custom Validation)
我们可以创建自定义校验错误 Command::error 方法可以返回指定错误 Error和自定义错误信息

```


use std::path::PathBuf;

use clap::error::ErrorKind;
use clap::{arg, command, value_parser, ArgAction};

fn main() {
    // Create application like normal
    let mut cmd = command!() // requires `cargo` feature
        // Add the version arguments
        .arg(arg!(--"set-ver" <VER> "set version manually"))
        .arg(arg!(--major         "auto inc major").action(ArgAction::SetTrue))
        .arg(arg!(--minor         "auto inc minor").action(ArgAction::SetTrue))
        .arg(arg!(--patch         "auto inc patch").action(ArgAction::SetTrue))
        // Arguments can also be added to a group individually, these two arguments
        // are part of the "input" group which is not required
        .arg(arg!([INPUT_FILE] "some regular input").value_parser(value_parser!(PathBuf)))
        .arg(
            arg!(--"spec-in" <SPEC_IN> "some special input argument")
                .value_parser(value_parser!(PathBuf)),
        )
        // Now let's assume we have a -c [config] argument which requires one of
        // (but **not** both) the "input" arguments
        .arg(arg!(config: -c <CONFIG>).value_parser(value_parser!(PathBuf)));
    let matches = cmd.get_matches_mut();

    // Let's assume the old version 1.2.3
    let mut major = 1;
    let mut minor = 2;
    let mut patch = 3;
    
    // See if --set-ver was used to set the version manually
    let version = if let Some(ver) = matches.get_one::<String>("set-ver") {
        if matches.get_flag("major") || matches.get_flag("minor") || matches.get_flag("patch") {
            cmd.error(
                ErrorKind::ArgumentConflict,
                "Can't do relative and absolute version change",
            )
            .exit();
        }
        ver.to_string()
    } else {
        // Increment the one requested (in a real program, we'd reset the lower numbers)
        let (maj, min, pat) = (
            matches.get_flag("major"),
            matches.get_flag("minor"),
            matches.get_flag("patch"),
        );
        match (maj, min, pat) {
            (true, false, false) => major += 1,
            (false, true, false) => minor += 1,
            (false, false, true) => patch += 1,
            _ => {
                cmd.error(
                    ErrorKind::ArgumentConflict,
                    "Can only modify one version field",
                )
                .exit();
            }
        };
        format!("{major}.{minor}.{patch}")
    };
    
    println!("Version: {version}");
    
    // Check for usage of -c
    if matches.contains_id("config") {
        let input = matches
            .get_one::<PathBuf>("INPUT_FILE")
            .or_else(|| matches.get_one::<PathBuf>("spec-in"))
            .unwrap_or_else(|| {
                cmd.error(
                    ErrorKind::MissingRequiredArgument,
                    "INPUT_FILE or --spec-in is required when using --config",
                )
                .exit()
            })
            .display();
        println!(
            "Doing work using input {} and config {}",
            input,
            matches.get_one::<PathBuf>("config").unwrap().display()
        );
    }

}
```


## 1.6、子命令(Subcommand)

我们可以使用 Command::subcommand 方法添加子命令。每一个子命令都自己的版本、作者、参数和它的子命令。

```
use clap::{arg, command, Command};

fn main() {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds files to myapp")
                .arg(arg!([NAME])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => println!(
            "'myapp add' was used, name is: {:?}",
            sub_matches.get_one::<String>("NAME")
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

}


```

我们使用 Command::arg_required_else_help 如果参数不存在，优雅的退出。
使用 Command::propagate_version 可以打印命令的版本号

## 1.7、测试
我们可以使用 debug_assert! 宏 或者 使用 Command::debug_assert 方法。

```
use clap::{arg, command, value_parser};

fn main() {
    let matches = cmd().get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    let port: usize = *matches
        .get_one::<usize>("PORT")
        .expect("'PORT' is required and parsing will fail if its missing");
    println!("PORT = {port}");

}

fn cmd() -> clap::Command {
    command!() // requires `cargo` feature
        .arg(
            arg!(<PORT>)
                .help("Network port to use")
                .value_parser(value_parser!(usize)),
        )
}

#[test]
fn verify_cmd() {
    cmd().debug_assert();
}


```





# 二、derive feature方式

个人更喜欢这种方式，代码看起来更简洁

## 2.1 添加依赖

```
cargo add clap --features derive
```

这里是不需要cargo的

## 2.2 快速开始

```

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Clis {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
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
fn quick_test(){
    let cli = Clis::parse();

    if let Some(name) = cli.name.as_ref(){
        println!("value for name {name}")
    }

    if let Some(config_path) = cli.config.as_deref(){
        println!("value for config: {:?}",config_path)
    }

    match cli.debug {
        0 => println!("debug mode is off"),
        1=> println!("debug mode is kind of on"),
        2=>println!("debug mode is on"),
        _=> println!("dont be crazy")
    }


    match &cli.command {
        Some(Commands::Test {list})=>{
            if *list{
                println!("printing testing lists...")
            }else{
                println!("not printing testing lists...")
            }
        }
        None=>{

        }
    }
}
```



查看help

```
RUST_BACKTRACE=1 cargo run -- --help
   Compiling my_test v0.1.0 (/Users/zhangqiuli24/Desktop/rust/my_test)
    Finished dev [unoptimized + debuginfo] target(s) in 5.63s
     Running `target/debug/my_test --help`
Usage: my_test [OPTIONS] [NAME] [COMMAND]

Commands:
  test  does testing things
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [NAME]  Optional name to operate on

Options:
  -c, --config <FILE>  Sets a custom config file
  -d, --debug...       Turn debugging information on
  -h, --help           Print help
  -V, --version        Print version
```



调用

```
RUST_BACKTRACE=1 cargo run -- name  -c ./config.toml -d    test --list  

value for name name
value for config: "./config.toml"
debug mode is kind of on
printing testing lists...
```



## 2.2 配置解析器

我们可以是 Parse 属性开启构建解析器

```
#[derive(Parser)]
#[command(name="MyApp")]
#[command(author="ZHangQL")]
#[command(version="1.0")]
#[command(about="这是测试的一些东西",long_about=None)]
struct Clic{
    #[arg(long)]
    two:String,
    #[arg(long)]
    one:String
}
fn parse(){
    let cli = Clic::parse();
    println!("value for two {:?}",cli.two);
    println!("value for one {:?}",cli.one)
}
```



help

```
RUST_BACKTRACE=1 cargo run -- --help                                         
   Compiling my_test v0.1.0 (/Users/zhangql/Desktop/rust/my_test)
    Finished dev [unoptimized + debuginfo] target(s) in 6.34s
     Running `target/debug/my_test --help`
这是测试的一些东西

Usage: my_test --two <TWO> --one <ONE>

Options:
      --two <TWO>  
      --one <ONE>  
  -h, --help       Print help
  -V, --version    Print version


cargo run -- -V
MyApp 1.0
```



**我们也可使用使用 #[command(author, version, about)] 形式从 Cargo.toml 读取配置消息**

读取的是cargo.toml的`[package]`信息

```
#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(long)]
    two: String,
    #[arg(long)]
    one: String,
}

fn main() {
    let cli = Cli::parse();

    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
}
```



## 2.3 Command::next_line_help 换行

我们可以使用 #[command(next_line_help = true)] 方法替代 Command::next_line_help

```
RUST_BACKTRACE=1 cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.98s
     Running `target/debug/my_test --help`
Usage: my_test --two <TWO> --one <ONE>

Options:
      --two <TWO>
          
      --one <ONE>
          
  -h, --help
          Print help
  -V, --version
          Print version

cargo run -- -V
my_test 0.1.0

```



## 2.4 添加可选参数

```

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct CliW{
    name:Option<String>
}
fn switch(){
    let cli = CliW::parse();
    println!("value for name {:?}",cli.name);
}
```



```
cargo run -- --help
Usage: my_test [NAME]

Arguments:
  [NAME]
          

Options:
  -h, --help
          Print help
  -V, --version
          Print version


RUST_BACKTRACE=1 cargo run -- 
value for name None

RUST_BACKTRACE=1 cargo run -- kkk
value for name Some("kkk")
```



## 2.5 添加多值参数

```

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct CliW{
    name:Option<String>,
    more : Vec<String>
}
fn switch(){
    let cli = CliW::parse();
    println!("value for name {:?}",cli.name);
    println!("value for more {:?}",cli.more);
}
```



```
RUST_BACKTRACE=1 cargo run -- --help

Usage: my_test [NAME] [MORE]...

Arguments:
  [NAME]
          
  [MORE]...
          

Options:
  -h, --help
          Print help
  -V, --version
          Print version

RUST_BACKTRACE=1 cargo run -- kkk more1 more2 m3
value for name Some("kkk")
value for more ["more1", "more2", "m3"]

```



## 2.6 长短名

我们可以使用 #[arg(short = ‘n’)] 和 #[arg(long = “name”)] 属性设置参数的短名称和长名称

```
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name.as_deref());
}
```



## 2.7 开启和关闭

```
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("verbose: {:?}", cli.verbose);
}

```

需要注意我们默认调用的是clap::ArgAction::SetTrue



## 2.8 参数计数

```
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let cli = Cli::parse();

    println!("verbose: {:?}", cli.verbose);
}

```



## 2.9 参数默认值

```
我们使用 #[arg(default_value_t)] 属性来给参数设置默认值
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(default_value_t = 2020)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();

    println!("port: {:?}", cli.port);
}
```



## 2.10 参数枚举

我们使用 #[arg(value_enum)] 设置参数枚举 结合枚举类

```
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[arg(value_enum)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Run swiftly
    Fast,
    /// Crawl slowly but steadily
    ///
    /// This paragraph is ignored because there is no long help text for possible values.
    Slow,
}

fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Fast => {
            println!("Hare");
        }
        Mode::Slow => {
            println!("Tortoise");
        }
    }
}

```



## 2.11 参数校验

```
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Network port to use
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,
}

fn main() {
    let cli = Cli::parse();

    println!("PORT = {}", cli.port);
}
```



## 2.12 自定义解析

```
use std::ops::RangeInclusive;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Network port to use
    #[arg(value_parser = port_in_range)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();

    println!("PORT = {}", cli.port);
}

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

```



## 2.13 参数关系

```
use clap::{Args, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    vers: Vers,

    /// some regular input
    #[arg(group = "input")]
    input_file: Option<String>,

    /// some special input argument
    #[arg(long, group = "input")]
    spec_in: Option<String>,

    #[arg(short, requires = "input")]
    config: Option<String>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Vers {
    /// set version manually
    #[arg(long, value_name = "VER")]
    set_ver: Option<String>,

    /// auto inc major
    #[arg(long)]
    major: bool,

    /// auto inc minor
    #[arg(long)]
    minor: bool,

    /// auto inc patch
    #[arg(long)]
    patch: bool,
}

fn main() {
    let cli = Cli::parse();

    // Let's assume the old version 1.2.3
    let mut major = 1;
    let mut minor = 2;
    let mut patch = 3;

    // See if --set_ver was used to set the version manually
    let vers = &cli.vers;
    let version = if let Some(ver) = vers.set_ver.as_deref() {
        ver.to_string()
    } else {
        // Increment the one requested (in a real program, we'd reset the lower numbers)
        let (maj, min, pat) = (vers.major, vers.minor, vers.patch);
        match (maj, min, pat) {
            (true, _, _) => major += 1,
            (_, true, _) => minor += 1,
            (_, _, true) => patch += 1,
            _ => unreachable!(),
        };
        format!("{major}.{minor}.{patch}")
    };

    println!("Version: {version}");

    // Check for usage of -c
    if let Some(config) = cli.config.as_deref() {
        let input = cli
            .input_file
            .as_deref()
            .unwrap_or_else(|| cli.spec_in.as_deref().unwrap());
        println!("Doing work using input {input} and config {config}");
    }
}

```



## 2.14 自定义校验

```
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// set version manually
    #[arg(long, value_name = "VER")]
    set_ver: Option<String>,

    /// auto inc major
    #[arg(long)]
    major: bool,

    /// auto inc minor
    #[arg(long)]
    minor: bool,

    /// auto inc patch
    #[arg(long)]
    patch: bool,

    /// some regular input
    input_file: Option<String>,

    /// some special input argument
    #[arg(long)]
    spec_in: Option<String>,

    #[arg(short)]
    config: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // Let's assume the old version 1.2.3
    let mut major = 1;
    let mut minor = 2;
    let mut patch = 3;

    // See if --set-ver was used to set the version manually
    let version = if let Some(ver) = cli.set_ver.as_deref() {
        if cli.major || cli.minor || cli.patch {
            let mut cmd = Cli::command();
            cmd.error(
                ErrorKind::ArgumentConflict,
                "Can't do relative and absolute version change",
            )
            .exit();
        }
        ver.to_string()
    } else {
        // Increment the one requested (in a real program, we'd reset the lower numbers)
        let (maj, min, pat) = (cli.major, cli.minor, cli.patch);
        match (maj, min, pat) {
            (true, false, false) => major += 1,
            (false, true, false) => minor += 1,
            (false, false, true) => patch += 1,
            _ => {
                let mut cmd = Cli::command();
                cmd.error(
                    ErrorKind::ArgumentConflict,
                    "Can only modify one version field",
                )
                .exit();
            }
        };
        format!("{major}.{minor}.{patch}")
    };

    println!("Version: {version}");

    // Check for usage of -c
    if let Some(config) = cli.config.as_deref() {
        let input = cli
            .input_file
            .as_deref()
            // 'or' is preferred to 'or_else' here since `Option::as_deref` is 'const'
            .or(cli.spec_in.as_deref())
            .unwrap_or_else(|| {
                let mut cmd = Cli::command();
                cmd.error(
                    ErrorKind::MissingRequiredArgument,
                    "INPUT_FILE or --spec-in is required when using --config",
                )
                .exit()
            });
        println!("Doing work using input {input} and config {config}");
    }
}

```



## 2.15子命令

我们使用 #[command(subcommand)] 属性和#[derive(Subcommand)] 联合起来使用声明子命令。

```
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {name:?}")
        }
    }
}
```



















































