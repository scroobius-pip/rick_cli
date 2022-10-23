use async_trait::async_trait;

use crate::lib::query_language::{operation::*, operation_list::*};

use super::entities::{CharacterPage, EpisodePage, LocationPage};

#[derive(Clone,PartialEq,Debug)]
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
        let result = match &self.0 {
            RMResponseEnum::Characters(page) => {
                let mut new_page = page.clone();
                for operation in operation_list.iter() {
                    match &operation.0 {
                        OperationEnum::Contains(field_name, field_value) => {
                            let field_name: String = field_name.into();
                            let field_value_check: String = field_value.into();
                            let new_page_result = new_page
                                .results
                                .iter()
                                .filter(|&result| {
                                    let field_value = match field_name.as_str() {
                                        "name" => &result.name,
                                        "status" => &result.status,
                                        "species" => &result.species,
                                        "type" => &result._type,
                                        "gender" => &result.gender,
                                        _ => "",
                                    };
                                    field_value.contains(&field_value_check)
                                })
                                .map(|result| result.clone())
                                .collect::<Vec<_>>();
                            new_page.results = new_page_result;
                        }
                        OperationEnum::Length(_, _) => {}
                        OperationEnum::Index(_) => {}
                        OperationEnum::Sort(_, _) => {}
                        OperationEnum::Pick(_) => {}
                        _ => {} // other operations are only handled before the request is made. e.g the implementation of OperationListEvaluator on MockRequest
                    }
                }
                Ok(RMResponse(RMResponseEnum::Characters(new_page)))
            }
            RMResponseEnum::Episodes(page) => {
                let mut new_page = page.clone();
                for operation in operation_list.0.iter() {
                    match &operation.0 {
                        OperationEnum::Contains(field_name, field_value) => {
                            let field_name: String = field_name.into();
                            let field_value_check: String = field_value.into();
                            let new_page_result = new_page
                                .results
                                .iter()
                                .filter(|&result| {
                                    let field_value = match field_name.as_str() {
                                        "name" => &result.name,
                                        "air_date" => &result.air_date,
                                        "episode" => &result.episode,
                                        _ => "",
                                    };
                                    field_value.contains(&field_value_check)
                                })
                                .map(|result| result.clone())
                                .collect::<Vec<_>>();
                            new_page.results = new_page_result;
                        }
                        OperationEnum::Length(_, _) => {}
                        OperationEnum::Index(_) => {}
                        OperationEnum::Sort(_, _) => {}
                        OperationEnum::Pick(_) => {}
                        _ => {} // other operations are only handled before the request is made. e.g the implementation of OperationListEvaluator on MockRequest
                    }
                }
                Ok(RMResponse(RMResponseEnum::Episodes(new_page)))
            }
            RMResponseEnum::Locations(page) => {
                let mut new_page = page.clone();
                for operation in operation_list.0.iter() {
                    match &operation.0 {
                        OperationEnum::Contains(field_name, field_value) => {
                            let field_name: String = field_name.into();
                            let field_value_check: String = field_value.into();
                            let new_page_result = new_page
                                .results
                                .iter()
                                .filter(|&result| {
                                    let field_value = match field_name.as_str() {
                                        "name" => &result.name,
                                        "type" => &result._type,
                                        "dimension" => &result.dimension,
                                        _ => "",
                                    };
                                    field_value.contains(&field_value_check)
                                })
                                .map(|result| result.clone())
                                .collect::<Vec<_>>();
                            new_page.results = new_page_result;
                        }
                        OperationEnum::Length(_, _) => {}
                        OperationEnum::Index(_) => {}
                        OperationEnum::Sort(_, _) => {}
                        OperationEnum::Pick(_) => {}
                        _ => {} // other operations are only handled before the request is made. e.g the implementation of OperationListEvaluator on MockRequest
                    }
                }
                Ok(RMResponse(RMResponseEnum::Locations(new_page)))
            }
        };
        result
    }
}

// tests
#[cfg(test)]
mod tests {
    use tokio;

    use crate::lib::{
        query_language::{operand::{OperandEnum, Operand}, },
        rm_api::entities::*,
    };

    use super::*;

    #[tokio::test]
    async fn single_contains_operation_characters() {
        let operation_list = OperationList(vec![Operation(OperationEnum::Contains(
            Operand(OperandEnum::String("name".into())),
            Operand(OperandEnum::String("xxxxx".into())),
        ))]);
        let response = RMResponse(RMResponseEnum::Characters(CharacterPage {
            info: Info {
                count: 1,
                pages: 1,
                next: None,
                prev: None,
            },
            results: vec![Character {
                name: "Rick Sanchez".into(),
                ..Default::default()
            }],
        }));

        let evaluated_response = response.evaluate_op(&operation_list).await.unwrap();

        match evaluated_response.0 {
            RMResponseEnum::Characters(page) => {
                assert_eq!(page.results.len(), 0);
            }
            _ => {}
        }

        let operation_list = OperationList(vec![Operation(OperationEnum::Contains(
            Operand(OperandEnum::String("name".into())),
            Operand(OperandEnum::String("Rick".into())),
        ))]);

        let evaluated_response = response.evaluate_op(&operation_list).await.unwrap();
        match evaluated_response.0 {
            RMResponseEnum::Characters(page) => {
                assert_eq!(page.results.len(), 1);
            }
            _ => {}
        }
    }

    #[tokio::test]
    async fn multiple_contains_operation_characters() {
        let operation_list = OperationList(vec![
            Operation(OperationEnum::Contains(
                Operand(OperandEnum::String("name".into())),
                Operand(OperandEnum::String("Rick".into())),
            )),
            Operation(OperationEnum::Contains(
                Operand(OperandEnum::String("status".into())),
                Operand(OperandEnum::String("Alive".into())),
            )),
        ]);

        let response = RMResponse(RMResponseEnum::Characters(CharacterPage {
            info: Info {
                count: 1,
                pages: 1,
                next: None,
                prev: None,
            },
            results: vec![Character {
                name: "Rick Sanchez".into(),
                status: "Alive".into(),
                ..Default::default()
            }],
        }));

        let evaluated_response = response.evaluate_op(&operation_list).await.unwrap();

        match evaluated_response.0 {
            RMResponseEnum::Characters(page) => {
                assert_eq!(page.results.len(), 1);
            }
            _ => {}
        }

        let operation_list = OperationList(vec![
            Operation(OperationEnum::Contains(
                Operand(OperandEnum::String("name".into())),
                Operand(OperandEnum::String("Rick".into())),
            )),
            Operation(OperationEnum::Contains(
                Operand(OperandEnum::String("status".into())),
                Operand(OperandEnum::String("Dead".into())),
            )),
        ]);

        let evaluated_response = response.evaluate_op(&operation_list).await.unwrap();
        match evaluated_response.0 {
            RMResponseEnum::Characters(page) => {
                assert_eq!(page.results.len(), 0);
            }
            _ => {}
        }
    }
}
