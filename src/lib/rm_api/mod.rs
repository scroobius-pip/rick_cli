use std::error::Error;
use async_trait::async_trait;
pub mod entities;
pub mod request;
pub mod response;

use self::entities::{CharacterPage, EpisodePage, LocationPage};

#[async_trait]
pub trait Rickuest {
    async fn get_characters(self, url: String) -> Result<CharacterPage, Box<dyn Error>>;
    async fn get_episodes(self, url: String) -> Result<EpisodePage, Box<dyn Error>>;
    async fn get_locations(self, url: String) -> Result<LocationPage, Box<dyn Error>>;
}

// tests
#[cfg(test)]
mod tests {
    use rocket::tokio;

    use super::request::builder::{CharactersRequest, RequestURLBuilder};
    use super::request::MockRequest;
    use super::Rickuest;
    // use super::*;
    #[tokio::test]
    async fn test_mock_request() {
        let url = CharactersRequest::new("https://rickandmortyapi.com")
            .name("rick")
            .build_url();
        let response = MockRequest.get_characters(url).await;
        assert!(response.is_ok());
    }
}
