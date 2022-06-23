use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/")]
pub fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}

#[get("/<file..>", rank = 2)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
