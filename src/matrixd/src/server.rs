#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn run() {
    let r = rocket::ignite();
    let r = r.mount("/", routes![index]);
    r.launch();

    println!("..");
}
