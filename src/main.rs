use rocket_dyn_templates::{Template, context};

#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        name: "World",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .attach(Template::fairing())
}

