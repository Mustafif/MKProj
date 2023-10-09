/// This file is based on Rust's website caching asset
/// Can be found [here](https://github.com/rust-lang/www.rust-lang.org/blob/master/src/caching.rs)
use rocket::fs::NamedFile;
use rocket::http::{hyper, Header};
use rocket::response::Responder;

#[derive(Responder)]
pub struct CachedNameFile {
    file: NamedFile,
    header: Header<'static>,
}

impl CachedNameFile {
    pub fn max_age(file: NamedFile, max_age: u32) -> Self {
        Self {
            file,
            header: Header::new(
                hyper::header::CACHE_CONTROL.as_str(),
                format!("max-age={max_age}"),
            ),
        }
    }
}
