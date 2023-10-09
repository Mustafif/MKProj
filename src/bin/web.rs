use rocket::{get, launch, routes};
use rocket_dyn_templates::{context, Template};
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};
use mkproj_com::cached_asset::CachedNameFile;

#[get("/")]
async fn index() -> Template {
    let gh_buttons = mkproj_com::github_proj::GHButtons::read().await.unwrap();
    let buttons = gh_buttons.buttons;
    Template::render("index", context! {
        buttons: buttons,
    })
}
/// Opens a asset file as cached with max age of 3600
#[get("/assets/<file..>")]
pub async fn assets(file: PathBuf) -> Option<CachedNameFile> {
    NamedFile::open(Path::new("assets/").join(file))
        .await
        .ok()
        .map(|file| CachedNameFile::max_age(file, 3600))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, assets])
}
