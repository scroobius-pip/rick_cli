
use std::ops::Deref;
use async_trait::async_trait;
use crate::lib::rm_api::response::RMResponse;

use super::operation::Operation;

pub struct OperationList(pub Vec<Operation>);

#[async_trait]
pub trait OperationListEvaluator {
    async fn evaluate_op(
        &self,
        operation_list: &OperationList,
    ) -> Result<RMResponse, Box<dyn std::error::Error>>;
}

impl OperationList {
    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|op| op.into())
            .collect::<Vec<String>>()
            .join("::")
    }
}

impl From<OperationList> for String {
    fn from(operation_list: OperationList) -> Self {
        operation_list.to_string()
    }
}

impl Deref for OperationList {
    type Target = Vec<Operation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

 