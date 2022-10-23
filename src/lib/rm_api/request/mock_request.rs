use super::builder::{CharactersRequest, RequestURLBuilder};
use crate::lib::rm_api::{
    entities::*,
    response::{RMResponse, RMResponseEnum},
    Rickuest,
};
use crate::lib::query_language::{
    operation::{OperationEnum, Root},
    operation_list::{OperationList, OperationListEvaluator},
    *,
};
use async_trait::async_trait;

use std::error::Error;

pub struct MockRequest;

#[async_trait]
impl OperationListEvaluator for MockRequest {
    async fn evaluate_op(
        &self,
        operation_list: &OperationList,
    ) -> Result<RMResponse, Box<dyn Error>> {
        let result = match operation_list[0].0 {
            OperationEnum::Root(Root::CHARACTERS) => {
                let mut builder = CharactersRequest::new("https://rickandmortyapi.com");
                for operation in operation_list.0.iter() {
                    match &operation.0 {
                        OperationEnum::Name(name) => {
                            let name: String = name.into();
                            builder.name(name.as_str());
                        }
                        OperationEnum::Page(page) => {
                            let page_number: u32 = page.into();
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
            _ => Err("Invalid Root".into()),
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