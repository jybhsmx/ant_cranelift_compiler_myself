use ant_ast::expr::{FloatValue, IntValue};
use ant_typed_ast::typed_expr::TypedExpression;
use bigdecimal::ToPrimitive;

use crate::traits::{LiteralExprToConst, ToLeBytes};

impl ToLeBytes for IntValue {
    fn to_le_bytes(&self) -> Vec<u8> {
        match self {
            IntValue::I64(val) => val.to_le_bytes().to_vec(),
            IntValue::I32(val) => val.to_le_bytes().to_vec(),
            IntValue::I16(val) => val.to_le_bytes().to_vec(),
            IntValue::I8(val) => vec![*val as u8], // 单字节
            IntValue::ISize(val) => val.to_le_bytes().to_vec(),
            IntValue::U64(val) => val.to_le_bytes().to_vec(),
            IntValue::U32(val) => val.to_le_bytes().to_vec(),
            IntValue::U16(val) => val.to_le_bytes().to_vec(),
            IntValue::U8(val) => vec![*val], // 单字节
            IntValue::USize(val) => val.to_le_bytes().to_vec(),
        }
    }
}


#[derive(Clone, Debug)]
pub enum ConstVal {
    Int(IntValue),
    Str(String),
    Bool(bool),
    F32(f32),
    F64(f64),
}

impl ToLeBytes for ConstVal {
    fn to_le_bytes(&self) -> Vec<u8> {
        match self {
            ConstVal::Int(value) => value.to_le_bytes(),
            ConstVal::F32(value) => value.to_le_bytes().to_vec(),
            ConstVal::F64(value) => value.to_le_bytes().to_vec(),
            ConstVal::Str(value) => value.as_bytes().to_vec(), // 转换为字节数组
            ConstVal::Bool(value) => vec![if *value { 1 } else { 0 }], // 0 或 1
        }
    }
}

impl LiteralExprToConst for TypedExpression {
    type ConstType = Option<ConstVal>;

    fn to_const(&self) -> Self::ConstType {
        if !self.is_literal() {
            return None;
        }

        match self {
            Self::Int { value, .. } => Some(ConstVal::Int(*value)),
            Self::StrLiteral { value, .. } => Some(ConstVal::Str(value.to_string())),
            Self::Bool { value, .. } => Some(ConstVal::Bool(*value)),
            Self::Float { value, .. } => Some(match value {
                FloatValue::F32(it) => ConstVal::F32(it.to_f32().unwrap()),
                FloatValue::F64(it) => ConstVal::F64(it.to_f64().unwrap()),
            }),
            _ => None
        }
    }
}