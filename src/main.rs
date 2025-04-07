mod datastore;
mod usecase;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use datastore::Datastore;
use rusqlite::Connection;
use std::sync::Mutex;
use usecase::{create_merkle_tree, get_merkle_proof, get_merkle_root, save_tree_to_file};

#[get("/users/root")]
async fn root(data: web::Data<Mutex<Connection>>) -> impl Responder {
    let conn = match data.lock() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error"),
    };

    let root = match get_merkle_root(&conn, "merkle_tree.json") {
        Ok(root) => root,
        Err(e) => {
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    let root_hex = hex::encode(root);
    HttpResponse::Ok().body(root_hex)
}

#[get("/users/{id}/merkle_proof")]
async fn get_user_merkle_proof(
    data: web::Data<Mutex<Connection>>,
    path: web::Path<u32>,
) -> impl Responder {
    let user_id = path.into_inner();

    let conn = match data.lock() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error"),
    };
    let proof = match get_merkle_proof(&conn, user_id, "merkle_tree.json") {
        Ok(proof) => proof,
        Err(e) => {
            return HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    let proof_json = match serde_json::to_string(&proof) {
        Ok(json) => json,
        Err(_) => return HttpResponse::InternalServerError().body("Internal Server Error"),
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .body(proof_json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = Connection::open_in_memory().unwrap();
    let datastore = Datastore::new(conn).unwrap();

    let db = web::Data::new(Mutex::new(datastore.conn));

    {
        let db_clone = db.clone();
        let connection = db_clone.lock().unwrap();
        let mt = create_merkle_tree(&connection).unwrap();
        save_tree_to_file(&mt, "merkle_tree.json").unwrap();
    }

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(root)
            .service(get_user_merkle_proof)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
