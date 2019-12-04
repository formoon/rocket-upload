Deal MultiPart Form Data in Rust Rocket web frame, easily upload files to server.

All code is in lib.rs. main.rs is a good example to see how it work.

First, add dependency in Cargo.toml:  
```rust
rocket_upload = "0.1.0"
```
Next, use the crate on the head of your code:  
```rust
use rocket_upload::MultipartDatas;
```
Finally in your handler function:  
```rust
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
```

The project references the code of Magic Len's https://crates.io/crates/rocket-multipart-form-data

