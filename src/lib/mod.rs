pub mod query_language;
pub mod rm_api;

// tests
#[cfg(test)]
mod tests {
    use rocket::tokio;

    use crate::lib::rm_api::response::RMResponseEnum;

    use super::{
        query_language::{Operation, OperationList, OperationListEvaluator, Root},
        rm_api::request::MockRequest,
    };

    #[tokio::test]
    async fn simple_root_query() {
    
        let operation_list = OperationList(vec![Operation::Root(Root::CHARACTERS)]);
        let mock_response = MockRequest.evaluate_op(&operation_list).await.unwrap();
        matches!(mock_response, super::rm_api::response::RMResponse(RMResponseEnum::Characters(_)));
        let new_mock_response = mock_response.evaluate_op(&operation_list).await.unwrap();
        let d = new_mock_response.0;
        
    }
}
