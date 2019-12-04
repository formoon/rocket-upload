#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

// use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket::response::content::Html;
use rocket::http::ContentType;

use rocket_upload::MultipartDatas;

#[post("/upload/<userid>", data = "<data>")]
fn upload(userid: String, content_type: &ContentType, data: MultipartDatas) -> Html<String>
{
  // to get a param from client
  let mut result=format!("UserID:{}<br>",userid);
  // content_type not used here, just for more informations
  result = format!("{}{:?}<br>",result,content_type);
  // aquire all Form field data
  for t in data.texts {
    result = format!("{}FieldName:{} --- FieldValue:{}<br>",result,t.key,t.value);
  }
  // aquire all files upload 
  for f in data.files {
    result = format!("{}FieldName:{} --- FileName:{} --- StoragePath:{}<br>",
      result,f.name,f.filename,f.path);
    f.persist(Path::new("upload"));
  }
  Html(format!("<html><head></head><body>upload coming...<br>{}</body></html>",result))
}

#[get("/")]
pub fn index() -> Option<NamedFile> {
  NamedFile::open("static/index.html").ok()
}

#[get("/static/<file..>", rank = 2)]
pub fn static_files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("static/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
  rocket::ignite()
    .mount("/", routes![index, static_files, upload])
}

fn main() {
  rocket().launch();
}
