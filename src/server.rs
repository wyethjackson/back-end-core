use warp::Filter;
use warp::fs;
use std::path::Path;

#[tokio::main]
async fn server() {
    // Define a route that matches a `GET` request to the `/hello` path and returns a "Hello, World!" message
    let hello = warp::path("hello")
        .and(warp::get())
        .map(|| "Hello, World!");
    let route = warp::path("hello")
    .and(warp::get())
    .map(|| "Hello, World!")
    .or(warp::path("goodbye")
        .and(warp::get())
        .map(|| "Goodbye, World!"));


        // Define a route that serves static files from the "public" directory
    let public = warp::path("public")
        .and(warp::fs::dir(Path::new("public")));

    // Define a catch-all route that returns the index.html file for any other path
    let index = warp::path::end().and(warp::get()).map(|| fs::file("public/index.html"));

    // Combine the routes and start the server
    warp::serve(public.or(index)).run(([127, 0, 0, 1], 8080)).await;
}

