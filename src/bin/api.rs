use std::env::args;
use anyhow::Result;
use mkproj_com::github_proj::{GHButtons, GithubButton};

#[tokio::main]
async fn main() -> Result<()>{
    let mut args = args();
    args.next();
    let cmd = args.next().unwrap_or_default();
    match cmd.as_str() {
        "read_gh" => {
            let buttons = GHButtons::read().await?;
            println!("{:#?}", buttons);
        }
        "append_gh" => {
            GHButtons::append().await?;
        }
        "init" => {
            let gh = GHButtons{buttons: vec![GithubButton::new("MKProj", "MKProj", "", vec![])]};
            let content = toml::to_string(&gh)?;
            tokio::fs::write("github_btns.toml", content).await?;
        }
        _ => {
            println!("Commands: read_gh, append_gh");
        }
    }
    Ok(())
}