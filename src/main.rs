#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use rocket::serde::{Serialize, json::Json};
use rocket::response::Redirect;

#[derive(Serialize)]
struct Message {
    message: String
}

 #[get("/")]
fn index()-> Template {
    let mut context:HashMap<String,String> = HashMap::new();
    context.insert("message".to_string(),"you expected a website but it was I dio".to_string());
    Template::render("index",context)
 }

#[get("/message")]
fn message() -> Json<Message> {
    let message = r#"https://i.imgflip.com/5dpbsy.jpg"#.to_string();
    Json(Message { message: message })
}

#[get("/additional_content")]
fn rickroll() -> Redirect {
    Redirect::to(uri!("http://www.youtube.com/watch?v=gvGyS5j9aFY"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/",routes![index,message,rickroll]).attach(Template::fairing())
}
