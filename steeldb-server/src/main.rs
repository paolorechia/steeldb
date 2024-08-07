use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use std::sync::{Arc, Mutex};
use steeldb::SteelDB;
use steeldb_core::json_result::{QueryResultJSON, TableJSON, UserQueryJSON};
use steeldb_core::{ExecutionResult, SteelDBInterface};

fn build_app() -> Router {
    // build our application with a route
    let database = Arc::new(Mutex::new(SteelDB::new()));

    let app = Router::new()
        .route("/query", post(handle_query))
        .with_state(database);
    return app;
}

#[tokio::main]
async fn main() {
    let app = build_app();
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_query(
    State(database): State<Arc<Mutex<SteelDB>>>,
    Json(payload): Json<UserQueryJSON>,
) -> (StatusCode, Json<QueryResultJSON>) {
    // insert your application logic here
    let db_mutex = Arc::clone(&database);
    let result: ExecutionResult;
    {
        let mut db = db_mutex.lock().unwrap();
        result = db.execute(payload.user_query);
    }
    match result {
        ExecutionResult::TableResult(table) => {
            // TODO: avoid cloning data here
            let result = QueryResultJSON {
                table_result: Some(TableJSON {
                    table_name: table.get_table_name().clone(),
                    columns: table.get_columns().clone(),
                    select_columns: table.get_select_columns().clone(),
                }),
                message: "query successful".to_string(),
                status_code: StatusCode::OK.as_u16(),
            };
            return (StatusCode::OK, Json(result));
        }
        ExecutionResult::ParseError(error) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(QueryResultJSON {
                    table_result: None,
                    message: format!("failed to execute query: {error}"),
                    status_code: StatusCode::BAD_REQUEST.as_u16(),
                }),
            );
        }
        ExecutionResult::CommandError(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(QueryResultJSON {
                    table_result: None,
                    message: format!("failed to execute query: {error}"),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                }),
            );
        }
        ExecutionResult::VoidOK => {
            return (
                StatusCode::OK,
                Json(QueryResultJSON {
                    table_result: None,
                    message: format!("Query successful"),
                    status_code: StatusCode::OK.as_u16(),
                }),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::json;
    // use tokio::net::TcpListener;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_app() {
        // Unfortunately these tests require a more elaborate fixture injection.
        // While there are interesting libraries like rstest, it doesn't seem
        // to support tokio out of the box.

        // On the other hand, if we instantiate a new app on each test case,
        // Then we end up in a PANIC triggered by the web framework,
        // likely because it doesn't expect multiple app instances floating around.

        // Test main query OK case
        let app = build_app();
        let mut response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/query")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        json!({
                            "user_query": "select name;"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Test column not found
        response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/query")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        json!({
                            "user_query": "select nope;"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body: &[u8] = &response.into_body().collect().await.unwrap().to_bytes();
        let s = String::from_utf8_lossy(body);
        assert_eq!(true, s.contains("ColumnNotFound(\\\"nope\\\")"));

        // Test malformed query
        response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/query")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        json!({
                            "user_query": "balsdfl"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST.as_u16());
        let body: &[u8] = &response.into_body().collect().await.unwrap().to_bytes();
        let s = String::from_utf8_lossy(body);
        assert_eq!(true, s.contains("UnrecognizedToken"));

        // Test malformed query
        response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/query")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        json!({
                            "hello": "world"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY.as_u16());
    }
}
