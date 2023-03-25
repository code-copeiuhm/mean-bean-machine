use rocket::fs::FileServer;
use rocket::routes;

#[get("/")]
fn world() -> &'static str {
    //send html side
    "Hello, world!"
}


#[rocket::main]
pub(crate) async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().mount("/", FileServer::from("src/coffee-girl/public"))
        .launch()
        .await?;

    Ok(())
}