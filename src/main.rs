use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::StatusCode;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};

// #[derive(SimpleObject)]
// struct Dog {
//     color: String,
//     name: String,
//     age: i32,
//     id: i32,
//     owner: User,
// }

// struct User {
//     id: i32,
// }

// #[Object(extends)]
// impl User {
//     #[graphql(external)]
//     async fn id(&self) -> &i32 {
//         &self.id
//     }

//     async fn dogs(&self) -> Vec<Dog> {
//         vec![
//             Dog {
//                 name: "Bongo".to_string(),
//                 color: "Tri Color".to_string(),
//                 age: 9,
//                 id: 1,
//                 owner: User { id: 3 },
//             },
//             Dog {
//                 name: "Oakley".to_string(),
//                 color: "Tri Color".to_string(),
//                 age: 3,
//                 id: 2,
//                 owner: User { id: 1 },
//             },
//         ]
//     }
// }

// struct Query;

// #[Object(extends)]
// impl Query {
//     #[graphql(entity)]
//     async fn resolve_user(&self, id: i32) -> User {
//         User { id }
//     }

//     async fn get_dog(&self) -> Dog {
//         Dog {
//             color: "Tri Color".to_string(),
//             name: "Oakley".to_string(),
//             age: 3,
//             id: 1,
//             owner: User { id: 1 },
//         }
//     }
// }

// // Node
// type User {
//   name: String!
//   id: Int!
// }
// // Rust
// #[derive(SimpleObject)]
// struct User {
//     name: String,
//     id: i32,
// }

#[derive(SimpleObject)]
struct User {
    name: String,
    id: i32,
}

struct Query;

#[Object(extends)]
impl Query {
    #[graphql(entity)]
    async fn resolve_reference(&self, id: i32) -> Option<User> {
        if id == 1 {
            Some(User {
                name: "Nick".to_string(),
                id,
            })
        } else {
            None
        }
    }
    async fn get_user(&self) -> User {
        User {
            name: "Nick".to_string(),
            id: 1,
        }
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:5010");

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
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