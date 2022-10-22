use async_trait::async_trait;

use super::rm_api::response::RMResponse;
pub enum Root {
    CHARACTERS,
    EPISODES,
    LOCATIONS,
}
pub enum Operation {
    Root(Root), //Every operation starts with a root
    Name(Operand),
    Page(Operand),
     /// name of field, value field should contain
    Contains(Operand, Operand),
    Length(Operand, Operand),
    Index(Operand),
    Sort(SortOperator, Operand),
    Pick(Operand),
}

pub enum SortOperator {
    ASC,
    DSC
}

pub struct OperationList(pub Vec<Operation>);

#[async_trait]
pub trait OperationListEvaluator {
    async fn evaluate_op(
        &self,
        operation_list: &OperationList,
    ) -> Result<RMResponse, Box<dyn std::error::Error>>;
}

impl OperationList {
    async fn evaluate(&self) {}
    fn to_string(&self) -> String {
        let string = String::new();

        string
    }
}

pub enum OperandEnum {
    Number(f32),
    String(String),
    // List(Vec<T>),
}

pub struct Operand(pub OperandEnum);

impl From<Operand> for f32 {
    fn from(operand: Operand) -> Self {
        match operand.0 {
            OperandEnum::Number(n) => n,
            OperandEnum::String(s) => s.parse::<f32>().unwrap_or_default(),
        }
    }
}

impl From<&Operand> for f32 {
    fn from(operand: &Operand) -> Self {
        match &operand.0 {
            OperandEnum::Number(n) => *n,
            OperandEnum::String(s) => s.parse::<f32>().unwrap_or_default(),
        }
    }
}



impl From<&Operand> for String {
    fn from(operand: &Operand) -> Self {
        match &operand.0 {
            OperandEnum::Number(n) => n.to_string(),
            OperandEnum::String(s) => s.clone(),
        }
    }
}


impl From<Operand> for String {
    fn from(operand: Operand) -> Self {
        match operand.0 {
            OperandEnum::Number(n) => n.to_string(),
            OperandEnum::String(s) => s,
        }
    }
}


impl OperationList {}

// tests
#[cfg(test)]
mod parsing_tests {
    use super::*;

    #[test]
    fn test_roo() {
        let operation_list_string = "CHARACTERS";
        let operation_list = OperationList(vec![Operation::Root(Root::CHARACTERS)]);
        assert_eq!(operation_list_string, operation_list.to_string())
    }

    #[test]
    fn test_name() {
        let operation_list_string = "CHARACTERS::NAME(rick)";
        let operation_list = OperationList(vec![
            Operation::Root(Root::CHARACTERS),
            Operation::Name(Operand(OperandEnum::String("rick".to_string()))),
        ]);
        assert_eq!(operation_list_string, operation_list.to_string())
    }
}
