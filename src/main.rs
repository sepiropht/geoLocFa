#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;

#[cfg(test)] mod tests;

use rocket_contrib::serve::StaticFiles;

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount("/", routes![hi])
}


#[get("/hello/<name>")]
fn hi(name: String) -> String {
        name
}

fn main() {
    rocket().launch();
}
