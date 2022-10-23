use std::error::Error;

use self::{
    query_language::operation_list::{OperationList, OperationListEvaluator},
    rm_api::response::RMResponseEnum,
};

pub mod query_language;
pub mod rm_api;


pub async fn query_api<T: OperationListEvaluator>(
    request: T,
    input: &str,
) -> Result<RMResponseEnum, Box<dyn Error>> {
    let operation_list = OperationList::parse_str(input)?;
    let response = request.evaluate_op(&operation_list).await?;
    let evaluated_response = response.evaluate_op(&operation_list).await?.0;
    Ok(evaluated_response)
}

// tests
#[cfg(test)]
mod tests {
  use tokio;

    use crate::lib::{
        query_language::operation_list::*,
        rm_api::{request::mock_request::MockRequest, response::RMResponseEnum},
    };

    #[tokio::test]
    async fn simple_root_query() {
        let query = "CHARACTERS::CONTAINS(name, xxxxxx)";
        let operation_list = OperationList::parse_str(query).unwrap();
        let mock_response = MockRequest.evaluate_op(&operation_list).await.unwrap();
        let evaluated_response = mock_response.evaluate_op(&operation_list).await.unwrap().0;

        match evaluated_response {
            RMResponseEnum::Characters(page) => {
                assert_eq!(page.results.len(), 0)
            }
            RMResponseEnum::Episodes(_) => panic!(),
            RMResponseEnum::Locations(_) => panic!(),
        }

        let query = "CHARACTERS::CONTAINS(name, Ri)";
        let operation_list = OperationList::parse_str(query).unwrap();
        let mock_response = MockRequest.evaluate_op(&operation_list).await.unwrap();
        let evaluated_response = mock_response.evaluate_op(&operation_list).await.unwrap().0;

        match evaluated_response {
            RMResponseEnum::Characters(page) => {
                assert_eq!(page.results.len(), 1)
            }
            RMResponseEnum::Episodes(_) => panic!(),
            RMResponseEnum::Locations(_) => panic!(),
        }
    }
}
