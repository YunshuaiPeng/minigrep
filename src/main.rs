use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        println!("参数解析错误: {}", e);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("运行错误: {}", e);
        process::exit(1);
    }
}
