use anyhow::Context;
use clap::Parser;
use log::{info, warn};
use std::{fs, io, path};
/// 在文件中搜索文本并显示包含该模式的行。
#[derive(Parser)]
struct Cli {
    /// 需要搜索的文本
    pattern: String,
    /// 要读取的文件路径
    path: path::PathBuf,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 日志
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
    // 显示进度条
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..2 {
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    let args = Cli::parse();
    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("cloud not read file `{}`", &args.path.display()))
        .unwrap();
    rusty_tool::find_matches(&content, &args.pattern, &mut io::stdout());

    Ok(())
}
