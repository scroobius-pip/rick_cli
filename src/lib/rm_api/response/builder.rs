use async_trait::async_trait;

use crate::lib::{rm_api::entities::*, query_language::{OperationListEvaluator, OperationList}};

use super::RMResponse;

#[async_trait]
impl OperationListEvaluator for CharacterPage {
    async fn evaluate_op(
        &self,
        operation_list: &OperationList,
    ) -> Result<RMResponse, Box<dyn std::error::Error>> {
        // let character_page = self.0.clone();

        Ok(RMResponse::Characters(self.clone()))
        
    }
}


