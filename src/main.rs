use rocket_dyn_templates::{Template, context};

#[macro_use] extern crate rocket;

mod routes;

#[get("/")]
fn index() -> Template {
    Template::render("index", context!())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .register("/", catchers![routes::api::not_found])
    .attach(Template::fairing())
}

