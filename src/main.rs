#[macro_use]
extern crate rocket;

// Download data from some blockchain related server and display it

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}
#[get("/world")] // <- route attribute
fn world() -> &'static str {
    // <- request handler
    "hello, world!"
}
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![world])
        .launch()
        .await?;

    Ok(())
}