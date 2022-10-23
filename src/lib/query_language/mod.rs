pub mod operand;
pub mod operation;
pub mod operation_list;

// tests
#[cfg(test)]
mod parsing_tests {
    use super::{*, operation_list::*,operand::*,operation::*};

    #[test]
    fn test_roo() {
        let operation_list_string = "CHARACTERS";
        let operation_list = OperationList(vec![Operation(OperationEnum::Root(Root::CHARACTERS))]);
        assert_eq!(operation_list_string, operation_list.to_string())
    }

    #[test]
    fn converting_to_string() {
        let operation_list_string = "CHARACTERS::NAME(rick)";
        let operation_list = OperationList(vec![
            Operation(OperationEnum::Root(Root::CHARACTERS)),
            Operation(OperationEnum::Name(Operand(OperandEnum::String(
                "rick".to_string(),
            )))),
        ]);

        assert_eq!(operation_list_string, operation_list.to_string());

        let operation_list_string = "CHARACTERS::NAME(rick)::PAGE(1)";
        let operation_list = OperationList(vec![
            Operation(OperationEnum::Root(Root::CHARACTERS)),
            Operation(OperationEnum::Name(Operand(OperandEnum::String(
                "rick".to_string(),
            )))),
            Operation(OperationEnum::Page(Operand(OperandEnum::Number(1.0)))),
        ]);

        assert_eq!(operation_list_string, operation_list.to_string());

        let operation_list_string = "CHARACTERS::NAME(rick)::PAGE(1)::CONTAINS(status, alive)";
        let operation_list = OperationList(vec![
            Operation(OperationEnum::Root(Root::CHARACTERS)),
            Operation(OperationEnum::Name(Operand(OperandEnum::String(
                "rick".to_string(),
            )))),
            Operation(OperationEnum::Page(Operand(OperandEnum::Number(1.0)))),
            Operation(OperationEnum::Contains(
                Operand(OperandEnum::String("status".to_string())),
                Operand(OperandEnum::String("alive".to_string())),
            )),
        ]);

        assert_eq!(operation_list_string, operation_list.to_string());
    }

    #[test]
    fn parsing_from_string() {
        let operation_list_string = "CHARACTERS::NAME(rick)";
        let parsed_operation_list = OperationList::parse_str(operation_list_string).unwrap();

        let expected_operation_list = OperationList(vec![
            Operation(OperationEnum::Root(Root::CHARACTERS)),
            Operation(OperationEnum::Name(Operand(OperandEnum::String(
                "rick".to_string(),
            )))),
        ]);

        assert_eq!(parsed_operation_list, expected_operation_list);
     
    }

    #[test]
    fn complex_chained(){
        let operation_list_string = "CHARACTERS::NAME(rick)::PAGE(1)::CONTAINS(status, alive)";
        let parsed_operation_list = OperationList::parse_str(operation_list_string).unwrap();

        let expected_operation_list = OperationList(vec![
            Operation(OperationEnum::Root(Root::CHARACTERS)),
            Operation(OperationEnum::Name(Operand(OperandEnum::String(
                "rick".to_string(),
            )))),
            Operation(OperationEnum::Page(Operand(OperandEnum::Number(1.0)))),
            Operation(OperationEnum::Contains(
                Operand(OperandEnum::String("status".to_string())),
                Operand(OperandEnum::String("alive".to_string())),
            )),
        ]);

        assert_eq!(parsed_operation_list, expected_operation_list);
    }
}
