use async_trait::async_trait;

use crate::lib::query_language::{operation::*, operation_list::*};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};

use super::entities::{CharacterPage, EpisodePage, LocationPage};

#[derive(Clone, PartialEq, Debug)]
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
                        OperationEnum::Sort(sort_direction, field_name) => {
                            let field_name: String = field_name.into();
                            let sort_direction: String = sort_direction.into();
                            match sort_direction.as_str() {
                                "ASC" => {
                                    new_page.results.sort_by(|a, b| match field_name.as_str() {
                                        "name" => a.name.cmp(&b.name),
                                        "type" => a._type.cmp(&b._type),
                                        "dimension" => a.dimension.cmp(&b.dimension),
                                        "residents" => a.residents.cmp(&b.residents),
                                        "id" => a.id.cmp(&b.id),
                                        "created" => {
                                            DateTime::parse_from_rfc3339(&a.created).unwrap().cmp(
                                                &DateTime::parse_from_rfc3339(&b.created).unwrap(),
                                            )
                                        }
                                        _ => a.name.cmp(&b.name),
                                    })
                                }
                                "DSC" => {
                                    new_page.results.sort_by(|a, b| match field_name.as_str() {
                                        "name" => b.name.cmp(&a.name),
                                        "type" => b._type.cmp(&a._type),
                                        "dimension" => b.dimension.cmp(&a.dimension),
                                        "residents" => b.residents.cmp(&a.residents),
                                        "id" => b.id.cmp(&a.id),
                                        "created" => {
                                            DateTime::parse_from_rfc3339(&b.created).unwrap().cmp(
                                                &DateTime::parse_from_rfc3339(&a.created).unwrap(),
                                            )
                                        }
                                        _ => b.name.cmp(&a.name),
                                    });
                                }
                                _ => unreachable!("sort direction should be ASC or DSC"),
                            };
                        }
                        OperationEnum::Length(_, _) => {}
                        OperationEnum::Index(_) => {}
                        OperationEnum::Limit(count) => {
                            let count: u32 = count.into();
                            new_page.results = new_page.results.iter().take(count as usize).cloned().collect();
                        }

                        // OperationEnum::Pick(_) => {}
                        _ => {} // other operations are only handled before the request is made. e.g the implementation of OperationListEvaluator on MockRequest
                    }
                }
                Ok(RMResponse(RMResponseEnum::Locations(new_page)))
            }

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
                        OperationEnum::Sort(sort_direction, field_name) => {
                            let field_name: String = field_name.into();
                            let sort_direction: String = sort_direction.into();
                            match sort_direction.as_str() {
                                "ASC" => {
                                    new_page.results.sort_by(|a, b| match field_name.as_str() {
                                        "name" => a.name.cmp(&b.name),
                                        "status" => a.status.cmp(&b.status),
                                        "species" => a.species.cmp(&b.species),
                                        "type" => a._type.cmp(&b._type),
                                        _ => a.name.cmp(&b.name),
                                    })
                                }
                                "DSC" => {
                                    new_page.results.sort_by(|a, b| match field_name.as_str() {
                                        "name" => b.name.cmp(&a.name),
                                        "status" => b.status.cmp(&a.status),
                                        "species" => b.species.cmp(&a.species),
                                        "type" => b._type.cmp(&a._type),
                                        _ => b.name.cmp(&a.name),
                                    });
                                }
                                _ => {},
                            }
                        }
                        OperationEnum::Limit(count) => {
                            let count: u32 = count.into();
                            new_page.results = new_page.results.iter().take(count as usize).cloned().collect();
                        }
                        _ => {}
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
                        OperationEnum::Sort(sort_direction, field_name) => {
                            let field_name: String = field_name.into();
                            let sort_direction: String = sort_direction.into();
                            match sort_direction.as_str() {
                                "ASC" => {
                                    new_page.results.sort_by(|a, b| match field_name.as_str() {
                                        "name" => a.name.cmp(&b.name),
                                        "id" => a.id.cmp(&b.id),
                                        "air_date" => std::cmp::Ordering::Equal,
                                        "episode" => a.episode.cmp(&b.episode),
                                        "characters" => a.characters.cmp(&b.characters),
                                        _ => std::cmp::Ordering::Equal,
                                    })
                                }
                                "DSC" => {
                                    new_page.results.sort_by(|a, b| match field_name.as_str() {
                                        "name" => b.name.cmp(&a.name),
                                        "id" => b.id.cmp(&a.id),
                                        "air_date" => std::cmp::Ordering::Equal,
                                        "episode" => b.episode.cmp(&a.episode),
                                        "characters" => b.characters.cmp(&a.characters),
                                        _ => std::cmp::Ordering::Equal,
                                    });
                                }
                                _ => {},
                            }
                        }
                        OperationEnum::Limit(count) => {
                            let count: u32 = count.into();
                            new_page.results = new_page.results.iter().take(count as usize).cloned().collect();
                        }
                        // OperationEnum::Pick(_) => {}
                        _ => {} // other operations are only handled before the request is made. e.g the implementation of OperationListEvaluator on MockRequest
                    }
                }
                Ok(RMResponse(RMResponseEnum::Episodes(new_page)))
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
        query_language::operand::{Operand, OperandEnum},
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

    #[tokio::test]
    async fn sort_operation_characters() {
        let operation_list = OperationList(vec![Operation(OperationEnum::Sort(
            Operand(OperandEnum::String("name".into())),
            Operand(OperandEnum::String("ASC".into())),
        ))]);

        let response = RMResponse(RMResponseEnum::Characters(CharacterPage {
            info: Info {
                count: 2,
                pages: 1,
                next: None,
                prev: None,
            },
            results: vec![
                Character {
                    name: "Rick Sanchez".into(),
                    ..Default::default()
                },
                Character {
                    name: "Morty Smith".into(),
                    ..Default::default()
                },
            ],
        }));

        let evaluated_response = response.evaluate_op(&operation_list).await.unwrap();

        match evaluated_response.0 {
            RMResponseEnum::Characters(page) => {
                assert_eq!(page.results[0].name, "Morty Smith");
                assert_eq!(page.results[1].name, "Rick Sanchez");
            }
            _ => {}
        }

        let operation_list = OperationList(vec![Operation(OperationEnum::Sort(
            Operand(OperandEnum::String("name".into())),
            Operand(OperandEnum::String("DSC".into())),
        ))]);

        let evaluated_response = response.evaluate_op(&operation_list).await.unwrap();
        match evaluated_response.0 {
            RMResponseEnum::Characters(page) => {
                assert_eq!(page.results[0].name, "Rick Sanchez");
                assert_eq!(page.results[1].name, "Morty Smith");
            }
            _ => {}
        }
    }

    #[tokio::test]
    async fn sort_operation_locations() {
        let operation_list = OperationList(vec![Operation(OperationEnum::Sort(
            Operand(OperandEnum::String("ASC".into())),
            Operand(OperandEnum::String("name".into())),
        ))]);

        let response = RMResponse(RMResponseEnum::Locations(LocationPage {
            info: Info {
                count: 2,
                pages: 1,
                next: None,
                prev: None,
            },
            results: vec![
                Location {
                    name: "Earth (C-137)".into(),
                    created: "2017-11-10T12:42:04.162Z".into(),
                    ..Default::default()
                },
                Location {
                    name: "Earth (Replacement Dimension)".into(),
                    created: "2018-01-10T18:20:41.703Z".into(),
                    ..Default::default()
                },
            ],
        }));

        let evaluated_response = response.evaluate_op(&operation_list).await.unwrap();

        match evaluated_response.0 {
            RMResponseEnum::Locations(page) => {
                assert_eq!(page.results[0].name, "Earth (C-137)");
                assert_eq!(page.results[1].name, "Earth (Replacement Dimension)");
            }
            _ => {}
        }

        let operation_list = OperationList(vec![Operation(OperationEnum::Sort(
            Operand(OperandEnum::String("DSC".into())),
            Operand(OperandEnum::String("name".into())),
        ))]);

        let evaluated_response = response.evaluate_op(&operation_list).await.unwrap();
        match evaluated_response.0 {
            RMResponseEnum::Locations(page) => {
                assert_eq!(page.results[0].name, "Earth (Replacement Dimension)");
                assert_eq!(page.results[1].name, "Earth (C-137)");
            }
            _ => {}
        }

        let operation_list = OperationList(vec![Operation(OperationEnum::Sort(
            Operand(OperandEnum::String("ASC".into())),
            Operand(OperandEnum::String("created".into())),
        ))]);

        let evaluated_response = response.evaluate_op(&operation_list).await.unwrap();
        match evaluated_response.0 {
            RMResponseEnum::Locations(page) => {
                assert_eq!(page.results[0].created, "2017-11-10T12:42:04.162Z");
                assert_eq!(page.results[1].created, "2018-01-10T18:20:41.703Z");
            }

            _ => {}
        }

        let operation_list = OperationList(vec![Operation(OperationEnum::Sort(
            Operand(OperandEnum::String("DSC".into())),
            Operand(OperandEnum::String("created".into())),
        ))]);

        let evaluated_response = response.evaluate_op(&operation_list).await.unwrap();
        match evaluated_response.0 {
            RMResponseEnum::Locations(page) => {
                assert_eq!(page.results[0].created, "2018-01-10T18:20:41.703Z");
                assert_eq!(page.results[1].created, "2017-11-10T12:42:04.162Z");
            }
            _ => {}
        }
    }

    #[tokio::test]

    async fn limit_operation_characters() {
        let operation_list = OperationList(vec![Operation(OperationEnum::Limit(
            Operand(OperandEnum::Number(1.0)),
        ))]);

        let response = RMResponse(RMResponseEnum::Characters(CharacterPage {
            info: Info {
                count: 2,
                pages: 1,
                next: None,
                prev: None,
            },
            results: vec![
                Character {
                    name: "Rick Sanchez".into(),
                    ..Default::default()
                },
                Character {
                    name: "Morty Smith".into(),
                    ..Default::default()
                },
            ],
        }));

        let evaluated_response = response.evaluate_op(&operation_list).await.unwrap();

        match evaluated_response.0 {
            RMResponseEnum::Characters(page) => {
                assert_eq!(page.results.len(), 1);
            }
            _ => {}
        }
    }
}
