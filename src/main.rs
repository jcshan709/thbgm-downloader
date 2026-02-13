mod utility;
mod cli;

use anyhow::*;

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("正在获取游戏列表...");
    utility::fetch_gamelist().await.unwrap();
    eprintln!("正作：{:?}", utility::INTEGERS.get().unwrap());
    eprintln!("小数点作：{:?}", utility::DECIMALS.get().unwrap());
    eprintln!("旧作：{:?}", utility::LEGACIES.get().unwrap());
    Ok(())
}
