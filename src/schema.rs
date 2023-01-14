use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::StatusCode;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};

pub mod query;

// #[derive(SimpleObject)]
// pub struct Player {
//     name: String,
//     id: i32,
//     score: i32
// }

// pub struct Query;

// #[Object(extends)]
// impl Query {
//     #[graphql(entity)]
//     async fn resolve_reference(&self, id: i32) -> Option<Player> {
//         if id == 1 {
//             Some(Player {
//                 name: "Nick".to_string(),
//                 id,
//                 score: 10
//             })
//         } else {
//             None
//         }
//     }
//     async fn get_player(&self) -> Player {
//         Player {
//             name: "Nick".to_string(),
//             id: 1,
//             score: 10
//         }
//     }

//     async fn get_players(&self) -> Vec<Player> {
//         vec![
//             Player {
//               name: "Nick".to_string(),
//               id: 1,
//               score: 10 
//             },
//             Player {
//               name: "Joe".to_string(),
//               id: 2, 
//               score: 11
//             }
//         ]
//     }
// }