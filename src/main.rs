use std::{
    env,
    path::{Path, PathBuf},
};

use rocket::{
    fs::NamedFile,
    tokio::{time::sleep, time::Duration},
};

#[macro_use]
extern crate rocket;

#[get("/world")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u32) -> String {
    sleep(Duration::from_secs(seconds.into())).await;

    format!("Waited for {} seconds", seconds)
}

// TODO
// - Handle error
// - Directory browsing (Maybe links too for API surfing)
// - Explicit `/list` and `/view` routes for folder / file interaction

#[get("/browse/<path..>")]
async fn get_path(path: PathBuf) -> Option<NamedFile> {
    let path = format!("{}", path.into_os_string().into_string().unwrap());
    let dir: String = env::current_dir()
        .expect("Unable to get current dir")
        .into_os_string()
        .into_string()
        .unwrap();

    println!("Path: {}", Path::new(&dir).join(&path).to_str()?);

    NamedFile::open(Path::new(&dir).join(path)).await.ok()
}

// #[launch]
// fn rocket() -> _ {
//     println!("Init");
//     rocket::build().mount("/hello", routes![index])
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _r = rocket::build()
        .mount("/hello", routes![index])
        .mount("/", routes![delay, get_path])
        .launch()
        .await?;

    Ok(())
}
