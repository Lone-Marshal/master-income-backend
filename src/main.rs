extern crate iron;
#[macro_use]
extern crate mime;

use std::{io, result};

use iron::mime::mime;
use iron::prelude::*;
use iron::status;
use postgres::{Client, NoTls};
use router::Router;

fn create_db_and_add_one_entity() -> Result<postgres::Client, Box<postgres::error::Error>> {
    let mut client = Client::connect("host=localhost user=root password=root", NoTls)?;

    client.batch_execute("
    CREATE TABLE person (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BYTEA
    )
")?;

    let name = "Ferris";
    let data = None::<&[u8]>;
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    )?;
    return Ok(client);
}

fn search_for_added_entity(client_connection: &mut postgres::Client) -> Result<(), Box<postgres::error::Error>> {
    for row in client_connection.query("SELECT id, name, data FROM person", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }
    Ok(())
}

fn gc(_request: &mut Request) -> IronResult<Response> {
    let mut client = create_db_and_add_one_entity().expect("create error");
    search_for_added_entity(&mut client);
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"Well done"#);
    Ok(response)
}

fn main() {
    let mut router = Router::new();
    router.get("/sql", gc, "root");

    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}
