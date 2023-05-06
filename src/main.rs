use rocket::{fs::FileServer, get, launch, routes};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render("index", context!())
}

#[get("/projects")]
fn projects() -> Template {
    Template::render("projects", context!())
}

#[get("/resume")]
fn resume() -> Template{
    Template::render("resume", context!())
}

#[get("/pages")]
fn pages() -> String{
    "Coming soon!!!".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![projects, index, resume,pages])
        .mount("/static", FileServer::from("static"))
        .mount("/html", FileServer::from("."))
}
