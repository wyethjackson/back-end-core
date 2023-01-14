use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::StatusCode;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};
mod schema;

#[tokio::main]
async fn main() {
    let schema = Schema::build(schema::query::Query, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:5010");

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<schema::query::Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes = graphql_playground
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes).run(([0, 0, 0, 0], 5010)).await;
}