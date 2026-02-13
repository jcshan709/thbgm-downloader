// 命令行参数解析

use clap::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {

}
