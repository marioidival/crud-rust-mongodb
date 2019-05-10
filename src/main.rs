extern crate bson;
extern crate pretty_env_logger;
extern crate serde;
extern crate warp;
#[macro_use]
extern crate serde_derive;
extern crate mongodb;

// TODO: Add bson::oid::ObjectId

use mongodb::{db::ThreadedDatabase, Client, ThreadedClient};
use std::env;
use warp::{http::StatusCode, Filter};

#[derive(Debug, Deserialize, Serialize)]
struct Contact {
    #[serde(rename = "_id")]
    id: u64,
    name: String,
    phone: String,
    email: String,
}

fn list_contact(mdb: Client) -> Result<impl warp::Reply, warp::Rejection> {
    let coll = mdb.db("crudmongodb").collection("contacts");
    let cursor = coll.find(None, None).unwrap();

    for result in cursor {
        if let Ok(item) = result {
            println!("item: {:?}", item);
        }
    }

    if true == false {
        return Err(warp::reject::bad_request());
    }
    Ok(StatusCode::OK)
}

fn create_contact(_create: Contact, _mdb: Client) -> Result<impl warp::Reply, warp::Rejection> {
    // logic to save a new contact
    if false {
        return Err(warp::reject::bad_request());
    }
    Ok(StatusCode::OK)
}

fn update_contact(
    _id: u64,
    _updated: Contact,
    _mdb: Client,
) -> Result<impl warp::Reply, warp::Rejection> {
    // logic to update contact
    if false {
        return Err(warp::reject::bad_request());
    }
    Ok(StatusCode::OK)
}

fn delete_contact(_id: u64, _mdb: Client) -> Result<impl warp::Reply, warp::Rejection> {
    if false {
        return Err(warp::reject::bad_request());
    }
    Ok(StatusCode::OK)
}

fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "contacts=info");
    }
    pretty_env_logger::init();
    //
    let connect = Client::connect("localhost", 27017).unwrap();
    // useful to pass "states"
    let db = warp::any().map(move || connect.clone());

    // Create path to /contacts
    let contacts = warp::path("contacts");
    // Create a `Filter` to matches the end of a route.
    let contacts_index = contacts.and(warp::path::index());

    // Create a `Filter` to get contacts ids
    // useful -> /contacts/{HERE}
    let contacts_id = contacts
        .and(warp::path::param::<u64>())
        .and(warp::path::index());

    // Create an endpoint to GET /contacts
    let list = warp::get2()
        // add filter to matches end of route
        .and(contacts_index)
        // add filter with state, or database
        .and(db.clone())
        // pass values of filters to a "view/controller/handler".
        // here is a future... :thinking_face:
        .and_then(list_contact);

    // Create an endpoint to POST /contacts
    let create = warp::post2()
        // add filter to matches end of route
        .and(contacts_index)
        // add filter to transform body into json
        .and(warp::body::json())
        // add filter with state, or database
        .and(db.clone())
        // pass values of filters to a "view/controller/handler".
        .and_then(create_contact);

    // Create an endpoint to PUT /contacts/{id}
    let update = warp::put2()
        // add filter to extract contact id
        .and(contacts_id)
        // add filter to transform body into json
        .and(warp::body::json())
        // add filter with state, or database
        .and(db.clone())
        // pass values of filters to a "view/controller/handler".
        .and_then(update_contact);

    // Create an endpoint to DELETE /contacts/{id}
    let delete = warp::delete2()
        // add filter to extract contact id
        .and(contacts_id)
        // add filter with state, or database
        .and(db.clone())
        // pass values of filters to a "view/controller/handler".
        .and_then(delete_contact);

    let api = list.or(create).or(update).or(delete);
    let routes = api.with(warp::log("contacts"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030));
}
