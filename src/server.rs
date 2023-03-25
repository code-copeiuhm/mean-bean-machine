use rocket::fs::FileServer;
use rocket::routes;

#[get("/")]
fn world() -> &'static str {
    //send html side
    "Hello, world!"
}

async fn test() {
    println!("gamer2");
    let m = mean_bean_machine::machines::machine::Machine::new();
    println!("Machine: {:?}", m.client_data());
    let p = m.get_stats().await;
    println!("p: {:?}", p);
}

#[rocket::main]
pub(crate) async fn main() -> Result<(), rocket::Error> {
        println!("gamer1");
        let f = test().await;
        println!("gamer3");

    let _rocket = rocket::build()
        .mount("/", FileServer::from("src/coffee-girl/public"))
        .launch()
        .await?;

    Ok(())
}