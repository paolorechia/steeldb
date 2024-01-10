use std::collections::HashMap;
use steeldb::SteelDB;
use steeldb_core::json_result::TableJSON;
use steeldb_core::SteelDBInterface;

use axum::{http::StatusCode, routing::post, Json, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/query", post(handle_query));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_query(// this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    // Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<TableJSON>) {
    // insert your application logic here
    let hello_response = TableJSON {
        table_name: "world!".to_owned(),
        columns: HashMap::new(),
        select_columns: Vec::new(),
    };
    let mut database = SteelDB::new();
    let result = database.execute("select name;".to_owned());

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(hello_response))
}
