use std::ops::Deref;

use super::operand::Operand;


pub struct Operation(pub OperationEnum);

pub enum Root {
    CHARACTERS,
    EPISODES,
    LOCATIONS,
}

pub enum OperationEnum {
    Root(Root), //Every operation starts with a root
    Name(Operand),
    Page(Operand),
    /// name of field, value field should contain
    Contains(Operand, Operand),
    Length(Operand, Operand),
    Index(Operand),
    Sort(Operand, Operand),
    Pick(Operand),
}


impl From<Operation> for String {
    fn from(operation: Operation) -> String {
        match &operation.0 {
            OperationEnum::Root(root) => match root {
                Root::CHARACTERS => "CHARACTERS".to_string(),
                Root::EPISODES => "EPISODES".to_string(),
                Root::LOCATIONS => "LOCATIONS".to_string(),
            },
            OperationEnum::Name(operand) => format!("NAME({})", String::from(operand)),
            OperationEnum::Page(operand) => format!("PAGE({})", String::from(operand)),
            OperationEnum::Contains(operand1, operand2) => {
                format!(
                    "CONTAINS({}, {})",
                    String::from(operand1),
                    String::from(operand2)
                )
            }
            OperationEnum::Length(operand1, operand2) => {
                format!(
                    "LENGTH({}, {})",
                    String::from(operand1),
                    String::from(operand2)
                )
            }
            OperationEnum::Index(operand) => format!("INDEX({})", String::from(operand)),
            OperationEnum::Sort(sort_operator, operand) => {
                format!(
                    "SORT({}, {})",
                    String::from(sort_operator),
                    String::from(operand)
                )
            }
            OperationEnum::Pick(operand) => format!("PICK({})", String::from(operand)),
        }
    }
}

impl From<&Operation> for String {
    fn from(operation: &Operation) -> String {
        match &operation.0 {
            OperationEnum::Root(root) => match root {
                Root::CHARACTERS => "CHARACTERS".to_string(),
                Root::EPISODES => "EPISODES".to_string(),
                Root::LOCATIONS => "LOCATIONS".to_string(),
            },
            OperationEnum::Name(operand) => format!("NAME({})", String::from(operand)),
            OperationEnum::Page(operand) => format!("PAGE({})", String::from(operand)),
            OperationEnum::Contains(operand1, operand2) => {
                format!(
                    "CONTAINS({}, {})",
                    String::from(operand1),
                    String::from(operand2)
                )
            }
            OperationEnum::Length(operand1, operand2) => {
                format!(
                    "LENGTH({}, {})",
                    String::from(operand1),
                    String::from(operand2)
                )
            }
            OperationEnum::Index(operand) => format!("INDEX({})", String::from(operand)),
            OperationEnum::Sort(sort_operator, operand) => {
                format!(
                    "SORT({}, {})",
                    String::from(sort_operator),
                    String::from(operand)
                )
            }
            OperationEnum::Pick(operand) => format!("PICK({})", String::from(operand)),
        }
    }
}

impl Deref for Operation {
    type Target = OperationEnum;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}