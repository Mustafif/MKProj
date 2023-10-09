use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::stdin;
use tokio::fs::read_to_string;
use toml::from_str;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GithubButton {
    title: String,
    description: String,
    url: String,
    badges: Vec<BadgeParam>,
    badges_urls: Vec<String>,
}

impl GithubButton {
    pub fn new(title: &str, description: &str, url: &str, badges: Vec<BadgeParam>) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            url: url.to_string(),
            badges: badges.clone(),
            badges_urls: badges.iter().map(|b| b.to_string()).collect(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GHButtons{
    pub buttons: Vec<GithubButton>,
}

impl GHButtons{
    pub async fn read() -> Result<Self>{
        let content = read_to_string("github_btns.toml").await.unwrap();
        let buttons: GHButtons = from_str(&content)?;
        Ok(buttons)
    }
    pub async fn append() -> Result<()>{
        let mut buttons = GHButtons::read().await?.buttons;
        let mut input = String::new();
        println!("Title:");
        stdin().read_line(&mut input)?;
        let title = input.trim().to_string();
        input.clear();
        println!("Description:");
        stdin().read_line(&mut input)?;
        let description = input.trim().to_string();
        input.clear();
        println!("Org: (MKProj/Moka-Reads)");
        stdin().read_line(&mut input)?;
        let org = input.trim().to_string();
        input.clear();
        let url = format!("https://github.com/{}/{}", org, title.to_lowercase());
        let mut badges = Vec::new();
        println!("Badges: (GHLicense, CratesVersion, CratesDownload, GHTopLang, GHCommitBranch)");
        loop {
            let repo = title.to_lowercase();
            println!("Badge:");
            stdin().read_line(&mut input)?;
            let badge = match input.trim() {
                "GHLicense" => Badge::GHLicense,
                "CratesVersion" => Badge::CratesVersion,
                "CratesDownload" => Badge::CratesDownload,
                "GHTopLang" => Badge::GHTopLang,
                "GHCommitBranch" => {
                    input.clear();
                    println!("Branch:");
                    stdin().read_line(&mut input)?;
                    Badge::GHCommitBranch(input.trim().to_string())
                }
                "All" => {
                    if &org == "MKProj" {
                        badges.push(BadgeParam::new_mkproj(&repo, Badge::GHLicense));
                        badges.push(BadgeParam::new_mkproj(&repo, Badge::CratesVersion));
                        badges.push(BadgeParam::new_mkproj(&repo, Badge::CratesDownload));
                        badges.push(BadgeParam::new_mkproj(&repo, Badge::GHTopLang));
                        badges.push(BadgeParam::new_mkproj(&repo, Badge::GHCommitBranch("main".to_string())));
                    } else {
                        badges.push(BadgeParam::new_moka(&repo, Badge::GHLicense));
                        badges.push(BadgeParam::new_moka(&repo, Badge::CratesVersion));
                        badges.push(BadgeParam::new_moka(&repo, Badge::CratesDownload));
                        badges.push(BadgeParam::new_moka(&repo, Badge::GHTopLang));
                        badges.push(BadgeParam::new_moka(&repo, Badge::GHCommitBranch("main".to_string())));
                    }
                    break;
                }
                "quit" => break,
                _ => continue,
            };
            if &org == "MKProj" {
                badges.push(BadgeParam::new_mkproj(&repo, badge));
            } else {
                badges.push(BadgeParam::new_moka(&repo, badge));
            }
            input.clear();
        }
        buttons.push(GithubButton::new(&title, &description, &url, badges));
        let gh_buttons = Self{buttons};
        let content = toml::to_string(&gh_buttons)?;
        tokio::fs::write("github_btns.toml", content).await?;
        Ok(())
    }
}



#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Badge {
    GHLicense,
    CratesVersion,
    CratesDownload,
    GHTopLang,
    GHCommitBranch(String),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BadgeParam {
    user: String,
    repo: String,
    badge: Badge,
}

impl BadgeParam {
    pub fn new_mkproj(repo: &str, badge: Badge) -> Self {
        Self {
            user: "MKProj".to_string(),
            repo: repo.to_string(),
            badge,
        }
    }
    pub fn new_moka(repo: &str, badge: Badge) -> Self {
        Self {
            user: "Moka-Reads".to_string(),
            repo: repo.to_string(),
            badge,
        }
    }
}

impl Display for BadgeParam {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let url = match &self.badge {
            Badge::GHLicense => format!(
                "https://img.shields.io/github/license/{}/{}",
                self.user, self.repo
            ),
            Badge::CratesVersion => format!("https://img.shields.io/crates/v/{}", self.repo),
            Badge::CratesDownload => format!("https://img.shields.io/crates/d/{}", self.repo),
            Badge::GHTopLang => format!(
                "https://img.shields.io/github/languages/top/{}/{}",
                self.user, self.repo
            ),
            Badge::GHCommitBranch(branch) => format!(
                "https://img.shields.io/github/commit-activity/t/{}/{}/{}?label=Commits%20({})",
                self.user, self.repo, branch, branch
            ),
        };
        f.write_str(&url)
    }
}
