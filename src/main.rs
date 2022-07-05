use std::sync::{Arc, Mutex};

use chrono::Local;
use rocket::{launch, routes, get, put, serde::json::Json, State};
use rocket_dyn_templates::{Template, context};
use serde::{Serialize, Deserialize};

struct Messages {
    messages: Arc<Mutex<Vec<String>>>,
}

#[derive(Serialize, Deserialize)]
struct DiunData<'a> {
    diun_version: &'a str,
    hostname: &'a str,
    status: &'a str,
    provider: &'a str,
    image: &'a str,
    hub_link: &'a str,
    mime_type: &'a str,
    digest: &'a str,
    created: &'a str,
    platform: &'a str,
}

#[get("/")]
fn list_all(s: &State<Messages>) -> Template {
    let mut msgs = s.messages.lock().unwrap().clone();
    msgs.reverse();
    Template::render("index", context! {
        messages: msgs,
    })
}

#[put("/submit", data = "<data>")]
fn submit(s: &State<Messages>, data: Json<DiunData<'_>>) -> Result<(), ()> {
    let mut msgs = s.messages.lock().unwrap();
    msgs.push(format!("[{}] Image {} needs updating on {}", Local::now(), data.image, data.hostname));
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .manage(Messages { messages: Arc::new(Mutex::new(vec![])) })
        .mount("/", routes![list_all, submit])
}
