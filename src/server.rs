// #[tokio::main]
// async
// fn main() {
    // database::ransom_notes_dao::person();
    // let schema = Schema::build(schema::query::Query, EmptyMutation, EmptySubscription).finish();
    //
    // println!("Playground: http://localhost:5010");
    //
    // let graphql_post = async_graphql_warp::graphql(schema).and_then(
    //     |(schema, request): (
    //         Schema<schema::query::Query, EmptyMutation, EmptySubscription>,
    //         async_graphql::Request,
    //     )| async move {
    //         Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
    //     },
    // );
    //
    // let graphql_playground = warp::path::end().and(warp::get()).map(|| {
    //     HttpResponse::builder()
    //         .header("content-type", "text/html")
    //         .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    // });
    //
    // let routes = graphql_playground
    //     .or(graphql_post)
    //     .recover(|err: Rejection| async move {
    //         if let Some(GraphQLBadRequest(err)) = err.find() {
    //             return Ok::<_, Infallible>(warp::reply::with_status(
    //                 err.to_string(),
    //                 StatusCode::BAD_REQUEST,
    //             ));
    //         }
    //
    //         Ok(warp::reply::with_status(
    //             "INTERNAL_SERVER_ERROR".to_string(),
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //         ))
    //     });
    //
    // warp::serve(routes).run(([0, 0, 0, 0], 5010)).await;
// }

// use warp::Filter;
//
// #[tokio::main]
// async fn main() {
//     let hello = warp::path("hello")
//         .map(|| "Hello, world!");
//
//     warp::serve(hello)
//         .run(([127, 0, 0, 1], 3030))
//         .await;
// }

