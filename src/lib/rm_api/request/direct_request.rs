use super::builder::{CharactersRequest, EpisodesRequest, LocationsRequest, RequestURLBuilder};
use crate::lib::query_language::{
    operation::{OperationEnum, Root},
    operation_list::{OperationList, OperationListEvaluator},
};
use crate::lib::rm_api::{
    entities::*,
    response::{RMResponse, RMResponseEnum},
    Rickuest,
};
use async_trait::async_trait;
use reqwest;
use std::error::Error;
pub struct DirectRequest;

#[async_trait]
impl Rickuest for DirectRequest {
    async fn get_characters(self, url: String) -> Result<CharacterPage, Box<dyn Error>> {
        let req = reqwest::get(url.as_str()).await?;
        let character_page: CharacterPage = req.json().await?;

        Ok(character_page)
    }

    async fn get_episodes(self, url: String) -> Result<EpisodePage, Box<dyn Error>> {
        let req = reqwest::get(url.as_str()).await?;
        let episode_page: EpisodePage = req.json().await?;

        Ok(episode_page)
    }

    async fn get_locations(self, url: String) -> Result<LocationPage, Box<dyn Error>> {
        let req = reqwest::get(url.as_str()).await?;
        let location_page: LocationPage = req.json().await?;

        Ok(location_page)
    }
}

#[async_trait]
impl OperationListEvaluator for DirectRequest {
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
                            builder.page(page_number);
                        }
                        _ => {} // other operations are only handled after the request is made. e.g the implementation of OperationListEvaluator on RMResponse
                    };
                }
                let url = builder.build_url();
                DirectRequest
                    .get_characters(url)
                    .await
                    .map(|response| RMResponse(RMResponseEnum::Characters(response)))
            }
            OperationEnum::Root(Root::EPISODES) => {
                let mut builder = EpisodesRequest::new("https://rickandmortyapi.com", vec![]);
                for operation in operation_list.0.iter() {
                    match &operation.0 {
                        OperationEnum::Name(name) => {
                            let name: String = name.into();
                            builder.name(name.as_str());
                        }
                        OperationEnum::Page(page) => {
                            let page_number: u32 = page.into();
                            builder.page(page_number);
                        }
                        _ => {} // other operations are only handled after the request is made. e.g the implementation of OperationListEvaluator on RMResponse
                    };
                }
                let url = builder.build_url();
                DirectRequest
                    .get_episodes(url)
                    .await
                    .map(|response| RMResponse(RMResponseEnum::Episodes(response)))
            }
            OperationEnum::Root(Root::LOCATIONS) => {
                let mut builder = LocationsRequest::new("https://rickandmortyapi.com");
                for operation in operation_list.0.iter() {
                    match &operation.0 {
                        OperationEnum::Name(name) => {
                            let name: String = name.into();
                            builder.name(name.as_str());
                        }
                        OperationEnum::Page(page) => {
                            let page_number: u32 = page.into();
                            builder.page(page_number);
                        }
                        OperationEnum::Dimension(dimension) => {
                            let dimension: String = dimension.into();
                            builder.dimension(dimension.as_str());
                        }
                        _ => {} // other operations are only handled after the request is made. e.g the implementation of OperationListEvaluator on RMResponse
                    };
                }
                let url = builder.build_url();
                DirectRequest
                    .get_locations(url)
                    .await
                    .map(|response| RMResponse(RMResponseEnum::Locations(response)))
            }
            _ => Err("Invalid Root".into()),
        };
        result
    }
}
