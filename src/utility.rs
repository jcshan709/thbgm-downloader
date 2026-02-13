use std::sync::OnceLock;
use surf;
use scraper::{Html, Selector};

pub const THBWIKI: &str = "https://touhou.review";
pub static INTEGERS: OnceLock<Vec<u32>> = OnceLock::new();
pub static DECIMALS: OnceLock<Vec<f32>> = OnceLock::new();
pub static LEGACIES: OnceLock<Vec<u32>> = OnceLock::new();

pub async fn fetch_gamelist() -> surf::Result<()> {
    if INTEGERS.get().is_some() || DECIMALS.get().is_some() {
        return Ok(());
    }

    // 请求 thbwiki 上的`官方游戏`页面
    let client = surf::client()
        .with(surf::middleware::Redirect::new(64));
    let req = surf::get(format!("{}/index.php?title=官方游戏&action=edit&viewsource=1", THBWIKI));
    let mut resp = client.send(req).await?;
    let body = resp.body_string().await?;
    let doc = Html::parse_document(&body);
    let selector = Selector::parse("#wpTextbox1").unwrap();
    let element = doc.select(&selector).next().unwrap();

    let mut integers = Vec::new();
    let mut decimals = Vec::new();

    // 解析游戏列表 跳过1-5作以特殊处理
    element.inner_html().lines().filter_map(|line| {
        let line = line.trim();
        line.strip_prefix("|序号 = TH")
    }).skip(5).flat_map(str::parse::<f32>).for_each(|x| {
        if x == x as u32 as f32 {
            integers.push(x as u32);
        } else {
            decimals.push(x);
        }
    });

    // 存储
    LEGACIES.set((1..=5).collect()).unwrap();
    INTEGERS.set(integers).unwrap();
    DECIMALS.set(decimals).unwrap();
    Ok(())
}

// 游戏编号
pub struct Game(u8);

impl Game {
    pub fn new(game_id: u8) -> Self {
        
        Self(game_id)
    }
}