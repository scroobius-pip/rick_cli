use std::error::Error;

use async_trait::async_trait;

use crate::lib::query_language::*;

use self::builder::{CharactersRequest, RequestURLBuilder};

use super::{
    entities::*,
    response::{RMResponse, RMResponseEnum},
    Rickuest,
};

pub mod builder;
struct DirectRequest;
struct ProxyRequest;
pub struct MockRequest;

#[async_trait]
impl OperationListEvaluator for MockRequest {
    async fn evaluate_op(
        &self,
        operation_list: &OperationList,
    ) -> Result<RMResponse, Box<dyn Error>> {
        let result = match operation_list.0[0] {
            Operation::Root(Root::CHARACTERS) => {
                let builder = CharactersRequest::new("https://rickandmortyapi.com");
                for operation in operation_list.0.iter() {
                    match operation {
                        Operation::Name(name) => {
                            builder.name(name.into());
                        }
                        Operation::Page(page) => {
                            // builder.page(page);
                        }
                        _ => {} // other operations are only handled after the request is made. e.g the implementation of OperationListEvaluator on RMResponse
                    };
                }
                let url = builder.build_url();
                MockRequest
                    .get_characters(url)
                    .await
                    .map(|response| RMResponse(RMResponseEnum::Characters(response)))
            }
            _ => Err("".into()),
        };
        result
    }
}

#[async_trait]
impl Rickuest for MockRequest {
    async fn get_characters(self, url: String) -> Result<CharacterPage, Box<dyn Error>> {
        let character_page = CharacterPage {
            info: Info {
                count: 1,
                pages: 1,
                next: None,
                prev: None,
            },
            results: vec![Character {
                id: 1,
                name: "Rick Sanchez".to_string(),
                status: "Alive".to_string(),
                species: "Human".to_string(),
                _type: "Genius".to_string(),
                ..Default::default()
            }],
        };
        Ok(character_page)
    }

    async fn get_episodes(self, url: String) -> Result<EpisodePage, Box<dyn Error>> {
        Ok(Default::default())
    }

    async fn get_locations(self, url: String) -> Result<LocationPage, Box<dyn Error>> {
        Ok(Default::default())
    }
}

// tests
#[cfg(test)]
mod tests {}
