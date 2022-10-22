pub mod query_language;
pub mod rm_api;

// tests
#[cfg(test)]
mod tests {
    use rocket::tokio;

    use crate::lib::{
        query_language::{operation::*, operation_list::*},
        rm_api::{request::MockRequest, response::RMResponseEnum},
    };

    #[tokio::test]
    async fn simple_root_query() {
        let operation_list = OperationList(vec![Operation(OperationEnum::Root(Root::CHARACTERS))]);
        let mock_response = MockRequest.evaluate_op(&operation_list).await.unwrap();
        matches!(
            mock_response,
            super::rm_api::response::RMResponse(RMResponseEnum::Characters(_))
        );
        let new_mock_response = mock_response.evaluate_op(&operation_list).await.unwrap();
        let d = new_mock_response.0;
    }
}
