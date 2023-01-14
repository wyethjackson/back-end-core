use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::StatusCode;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};

#[derive(SimpleObject)]
pub struct Player {
    pub name: String,
    pub id: i32,
    pub score: i32,
    pub words: Vec<Word>
}

#[derive(SimpleObject)]
pub struct Game {
    pub name: String,
    pub id: i32,
    pub winner: Player,
    pub players: Vec<Player>,
    pub round_score: i32,
    pub rounds: Vec<Round>
}

#[derive(SimpleObject)]
pub struct Round {
    pub roundNum: i32,
    pub winner: Player,
    pub judge: Player,
    pub sentences: Vec<Sentence>
}

#[derive(SimpleObject)]
pub struct Word {
    pub text: String,
    pub id: i32,
}

#[derive(SimpleObject)]
pub struct Sentence {
    pub player: Player,
    pub words: Vec<Word>
}