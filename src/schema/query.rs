use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::StatusCode;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};
mod ransom_notes;
pub struct Query;

#[Object(extends)]
impl Query {
    #[graphql(entity)]
    async fn resolve_reference(&self, id: i32) -> Option<ransom_notes::Player> {

        if id == 1 {
            Some(ransom_notes::Player {
                name: "Nick".to_string(),
                id,
                score: 10,
                words: vec![ransom_notes::Word {
                  text: String::from("How"),
                  id: 1
                }, ransom_notes::Word {
                  text: String::from("What"),
                  id: 1
                }]
            })
        } else {
            None
        }
    }
    async fn player(&self) -> ransom_notes::Player {
        ransom_notes::Player {
                name: "Nick".to_string(),
                id: 1,
                score: 10,
                words: vec![ransom_notes::Word {
                  text: String::from("How"),
                  id: 1
                }, ransom_notes::Word {
                  text: String::from("What"),
                  id: 1
                }]
            }
    }

    async fn players(&self) -> Vec<ransom_notes::Player> {
        vec![
            ransom_notes::Player {
                name: "Nick".to_string(),
                id: 1,
                score: 10,
                words: vec![ransom_notes::Word {
                  text: String::from("How"),
                  id: 1
                }, ransom_notes::Word {
                  text: String::from("What"),
                  id: 1
                }]
            },
            ransom_notes::Player {
                name: "Joe".to_string(),
                id: 2,
                score: 11,
                words: vec![ransom_notes::Word {
                  text: String::from("Some"),
                  id: 1
                }, ransom_notes::Word {
                  text: String::from("Thing"),
                  id: 1
                }]
            }
        ]
    }

    async fn game(&self) -> ransom_notes::Game {
        ransom_notes::Game {
          name: String::from("My Game"),
          id: 1,
          winner: ransom_notes::Player {
                name: "Nick".to_string(),
                id: 1,
                score: 10,
                words: vec![ransom_notes::Word {
                  text: String::from("How"),
                  id: 1
                }, ransom_notes::Word {
                  text: String::from("What"),
                  id: 1
                }]
            },
          players: vec![
            ransom_notes::Player {
                name: "Nick".to_string(),
                id: 1,
                score: 10,
                words: vec![ransom_notes::Word {
                  text: String::from("How"),
                  id: 1
                }, ransom_notes::Word {
                  text: String::from("What"),
                  id: 1
                }]
            },
            ransom_notes::Player {
                name: "Joe".to_string(),
                id: 2,
                score: 11,
                words: vec![ransom_notes::Word {
                  text: String::from("Some"),
                  id: 1
                }, ransom_notes::Word {
                  text: String::from("Thing"),
                  id: 1
                }]
            }
        ],
          round_score: 10,
          rounds: vec![
            ransom_notes::Round {
              roundNum: 1,
              winner: ransom_notes::Player {
                name: "Nick".to_string(),
                id: 1,
                score: 10,
                words: vec![ransom_notes::Word {
                  text: String::from("How"),
                  id: 1
                }, ransom_notes::Word {
                  text: String::from("What"),
                  id: 1
                }]
              },
              judge: ransom_notes::Player {
                name: "Joe".to_string(),
                id: 2,
                score: 11,
                words: vec![ransom_notes::Word {
                  text: String::from("Some"),
                  id: 1
                }, ransom_notes::Word {
                  text: String::from("Thing"),
                  id: 1
                }]
              },
              sentences: vec![ransom_notes::Sentence {
                player: ransom_notes::Player {
                    name: "Nick".to_string(),
                    id: 1,
                    score: 10,
                    words: vec![ransom_notes::Word {
                      text: String::from("How"),
                      id: 1
                    }, ransom_notes::Word {
                      text: String::from("What"),
                      id: 1
                    }]
                },
                words: vec![ransom_notes::Word {
                  text: String::from("How"),
                  id: 1
                }, ransom_notes::Word {
                  text: String::from("What"),
                  id: 1
                }]
              }]
            }
          ]
        }
    }
}