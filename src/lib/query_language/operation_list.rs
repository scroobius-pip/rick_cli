use crate::lib::rm_api::response::RMResponse;
use async_trait::async_trait;
use std::{ops::Deref, error::Error};

use super::operation::Operation;

#[derive(Debug, PartialEq,)]
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

    pub fn parse_str(operation_list_string: &str) -> Result<Self, Box<dyn Error>> {
        let mut operation_list = OperationList(vec![]);
        for operation_string in operation_list_string.split("::") {
            let parsed_operation = Operation::parse_str(operation_string)?;
            operation_list.0.push(parsed_operation);
        }
        Ok(operation_list)
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
