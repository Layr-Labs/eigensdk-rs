use alloy::primitives::{keccak256, B256};
use alloy::sol;
use alloy::sol_types::private::Bytes;
use alloy::sol_types::SolValue;
use field_info::FieldTypeInfo;
use field_info_derive::FieldTypeInfo;

/// Task abstraction
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Task<Input>
where
    Input: Clone + SolValue + FieldTypeInfo,
{
    #[allow(missing_docs)]
    pub input: Input,
    #[allow(missing_docs)]
    pub task_created_block: u32,
    #[allow(missing_docs)]
    pub quorum_numbers: Bytes,
    #[allow(missing_docs)]
    pub quorum_threshold_percentage: u8,
}

//const SIGNATURE: &'static str = "NewTaskCreated(uint32,(uint256,uint32,bytes,uint32))";
//const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
//    22u8, 149u8, 184u8, 208u8, 110u8, 200u8, 0u8, 180u8, 97u8, 94u8, 116u8, 92u8, 251u8, 91u8,
//    208u8, 12u8, 31u8, 40u8, 117u8, 97u8, 93u8, 66u8, 146u8, 92u8, 59u8, 90u8, 250u8, 84u8, 59u8,
//    178u8, 76u8, 72u8,
//]);

impl<Input: Clone + SolValue + FieldTypeInfo> Task<Input> {
    pub fn encode(&self) -> Vec<u8> {
        self.input.abi_encode()
    }

    fn encode_task_created_block(&self) -> Vec<u8> {
        self.task_created_block.abi_encode()
    }

    fn encode_quorum_numbers(&self) -> Vec<u8> {
        self.quorum_numbers.abi_encode()
    }

    pub fn get_name(&self) -> &'static str {
        self.input.sol_name()
    }

    pub fn get_inputs_fields(&self) -> &'static [&'static str] {
        Input::field_types()
    }

    fn get_signature(&self) -> B256 {
        let name = self.get_name();
        let signature = format!("NewTaskCreated(uint32,({},uint32,bytes,uint32))))", name);
        keccak256(signature.as_bytes())
    }
}

sol! {
    #[derive(Debug, FieldTypeInfo)]
    struct Foo {
        uint32 bar;
        address[] baz;
    }
}

mod tests {
    use alloy::{
        primitives::U256,
        sol,
        sol_types::sol_data::{Bool, FixedArray},
    };

    use super::*;

    #[test]
    fn test_get_name() {
        let t: Task<Foo> = Task {
            input: Foo {
                bar: 1,
                baz: vec![],
            },
            task_created_block: 1,
            quorum_numbers: Bytes::from_static(&[1, 2, 3]),
            quorum_threshold_percentage: 1,
        };

        let ret = t.get_inputs_fields();
        println!("Ret: {:?}", ret);
    }

    /*
    #[test]
    fn test_get_name_solidity_struct() {
        sol! {
            struct MyStruct {
                uint256 a;
            }
        }

        let task = Task::<MyStruct> {
            input: MyStruct { a: U256::from(1) },
            task_created_block: 1,
            quorum_numbers: Bytes::from_static(&[1, 2, 3]),
            quorum_threshold_percentage: 1,
        };

        dbg!(task.get_inputs_fields());

        assert_eq!(task.input.sol_name(), "asd");
    }
    */
}
