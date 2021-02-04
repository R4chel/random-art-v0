#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

struct Position {
    x : u32,
    y : u32

}
struct Point {
    position : Position
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}




fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
