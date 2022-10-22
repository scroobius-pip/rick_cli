use async_trait::async_trait;

use crate::lib::query_language::{OperationList, OperationListEvaluator, Operation};

use super::entities::{CharacterPage, EpisodePage, LocationPage};

pub mod builder;

#[derive(Clone)]
pub enum RMResponseEnum {
    Characters(CharacterPage),
    Episodes(EpisodePage),
    Locations(LocationPage),
}

#[derive(Clone)]
pub struct RMResponse(pub RMResponseEnum);

#[async_trait]
impl OperationListEvaluator for RMResponse {
    async fn evaluate_op(
        &self,
        operation_list: &OperationList,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let result = match self.0 {
            RMResponseEnum::Characters(mut page) => {   
                for operation in operation_list.0.iter() {
                    match operation {
                        Operation::Contains(_) => {}
                        Operation::Length(_, _) => {}
                        Operation::Index(_) => {}
                        Operation::Sort(_, _) => {}
                        Operation::Pick(_) => {}
                    
                        _ => {} // other operations are only handled before the request is made. e.g the implementation of OperationListEvaluator on MockRequest
                    }
                }
                Ok(RMResponse(RMResponseEnum::Characters(page)))
            }
            RMResponseEnum::Episodes(mut page) => {
                for operation in operation_list.0.iter() {
                    match operation {
                        Operation::Contains(_) => {}
                        Operation::Length(_, _) => {}
                        Operation::Index(_) => {}
                        Operation::Sort(_, _) => {}
                        Operation::Pick(_) => {}
                        _ => {}
                    }
                }
                Ok(RMResponse(RMResponseEnum::Episodes(page)))
            }
            RMResponseEnum::Locations(mut page) => {
                for operation in operation_list.0.iter() {
                    match operation {
                        Operation::Contains(_) => {}
                        Operation::Length(_, _) => {}
                        Operation::Index(_) => {}
                        Operation::Sort(_, _) => {}
                        Operation::Pick(_) => {}
                        _ => {}
                    }
                }
                Ok(RMResponse(RMResponseEnum::Locations(page)))
            }
        };
        result
    }
}

// tests
#[cfg(test)]
mod tests {
  
    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
    
}