use super::operand::{Operand, OperandEnum};
use regex_lexer_lalrpop::LexerBuilder;
use std::{error::Error, ops::Deref};

#[derive(Debug, PartialEq, Clone)]
pub enum Root {
    CHARACTERS,
    // EPISODES(Operand),
    EPISODES,
    LOCATIONS,
}

#[derive(Debug, PartialEq)]
pub enum OperationEnum {
    Root(Root), //Every operation starts with a root
    Name(Operand),
    Page(Operand),
    Dimension(Operand),
    /// name of field, value field should contain
    Contains(Operand, Operand),
    Length(Operand, Operand),
    Index(Operand),
    Sort(Operand, Operand),
    Pick(Operand),
}

#[derive(Debug, PartialEq)]
pub struct Operation(pub OperationEnum);

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
            OperationEnum::Dimension(operand) => format!("DIMENSION({})", String::from(operand)),
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
            OperationEnum::Dimension(operand) => format!("DIMENSION({})", String::from(operand)),
            OperationEnum::Pick(operand) => format!("PICK({})", String::from(operand)),
        }
    }
}

impl Operation {
    fn remove_symbols(string: &str, operation_name: &str) -> String {
        string
            .replace("(", "")
            .replace(")", "")
            .replace(operation_name, "")
            .to_string()
    }

    pub fn parse_str(operation_string: &str) -> Result<Self, Box<dyn Error>> {
        let lexer = LexerBuilder::new()
            .token(r"NAME\((.*?)\)", |_, value, _| {
                Some(Operation(OperationEnum::Name(Operand(
                    OperandEnum::String(Operation::remove_symbols(value, "NAME")),
                ))))
            })
            .token(r"PAGE\((.*?)\)", |_, value, _| {
                Some(Operation(OperationEnum::Page(Operand(
                    OperandEnum::Number(
                        Operation::remove_symbols(value, "PAGE")
                            .parse()
                            .unwrap_or(0.0),
                    ),
                ))))
            })
            .token("CHARACTERS", |_, _, _| {
                Some(Operation(OperationEnum::Root(Root::CHARACTERS)))
            })
            .token("EPISODES", |_, _, _| {
                Some(Operation(OperationEnum::Root(Root::EPISODES)))
            })
            .token("LOCATIONS", |_, _, _| {
                Some(Operation(OperationEnum::Root(Root::LOCATIONS)))
            })
            .token(r"CONTAINS\((.*?), (.*?)\)", |_, value, _| {
                let value = Operation::remove_symbols(value, "CONTAINS");
                let mut split = value.split(", ");
                let first = split.next().unwrap();
                let second = split.next().unwrap();

                Some(Operation(OperationEnum::Contains(
                    Operand(OperandEnum::String(first.to_string())),
                    Operand(OperandEnum::String(second.to_string())),
                )))
            })
            .token(r"LENGTH\((.*?), (.*?)\)", |_, value, _| {
                let value = Operation::remove_symbols(value, "LENGTH");
                let mut split = value.split(", ");
                let first = split.next().unwrap();
                let second = split.next().unwrap();

                Some(Operation(OperationEnum::Length(
                    Operand(OperandEnum::String(first.to_string())),
                    Operand(OperandEnum::String(second.to_string())),
                )))
            })
            .token(r"DIMENSION\((.*?)\)", |_, value, _| {
                Some(Operation(OperationEnum::Dimension(Operand(
                    OperandEnum::String(Operation::remove_symbols(value, "DIMENSION")),
                ))))
            })
            .build()?;

        let mut tokens = lexer.tokens(operation_string);
        match tokens.next() {
            Some(Ok(operation)) => Ok(operation),
            _ => Err("Operation not found".into()),
        }
    }
}

impl Deref for Operation {
    type Target = OperationEnum;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_str_name() {
        let parsed_operation = Operation::parse_str("NAME(Rick)").unwrap().0;
        let expected_operation =
            OperationEnum::Name(Operand(OperandEnum::String("Rick".to_string())));

        assert_eq!(parsed_operation, expected_operation);
    }
    #[test]
    fn test_parse_str_page() {
        let parsed_operation = Operation::parse_str("PAGE(1)").unwrap().0;
        let expected_operation = OperationEnum::Page(Operand(OperandEnum::Number(1.0)));

        assert_eq!(parsed_operation, expected_operation);
    }
    #[test]
    fn test_parse_str_characters() {
        let parsed_operation = Operation::parse_str("CHARACTERS").unwrap().0;
        let expected_operation = OperationEnum::Root(Root::CHARACTERS);

        assert_eq!(parsed_operation, expected_operation);
    }
    #[test]
    fn test_parse_str_episodes() {
        let parsed_operation = Operation::parse_str("EPISODES").unwrap().0;
        let expected_operation = OperationEnum::Root(Root::EPISODES);

        assert_eq!(parsed_operation, expected_operation);
    }
    #[test]
    fn test_parse_str_locations() {
        let parsed_operation = Operation::parse_str("LOCATIONS").unwrap().0;
        let expected_operation = OperationEnum::Root(Root::LOCATIONS);

        assert_eq!(parsed_operation, expected_operation);
    }
    #[test]
    fn test_parse_str_contains() {
        let parsed_operation = Operation::parse_str("CONTAINS(name, Morty)").unwrap().0;
        let expected_operation = OperationEnum::Contains(
            Operand(OperandEnum::String("name".to_string())),
            Operand(OperandEnum::String("Morty".to_string())),
        );

        assert_eq!(parsed_operation, expected_operation);
    }

    #[test]
    fn test_parse_str_length() {
        let parsed_operation = Operation::parse_str("LENGTH(name, 3)").unwrap().0;
        let expected_operation = OperationEnum::Length(
            Operand(OperandEnum::String("name".to_string())),
            Operand(OperandEnum::Number(3.0)),
        );

        assert_eq!(parsed_operation, expected_operation);
    }

    #[test]
    fn test_parse_str_dimension() {
        let parsed_operation = Operation::parse_str("DIMENSION(C-137)").unwrap().0;
        let expected_operation =
            OperationEnum::Dimension(Operand(OperandEnum::String("C-137".to_string())));

        assert_eq!(parsed_operation, expected_operation);
    }
}
