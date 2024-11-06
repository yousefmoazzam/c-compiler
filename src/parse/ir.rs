use crate::parse::c;

#[derive(Debug, PartialEq)]
pub enum Value {
    Constant(u8),
}

pub fn parse_value(node: c::Expression) -> Value {
    match node {
        c::Expression::NumericConstant(val) => Value::Constant(val),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_c_constant_to_ir_constant() {
        let value = 2;
        let c_ast_node = c::Expression::NumericConstant(value);
        let expected_ir_ast_node = Value::Constant(value);
        let ir_ast_node = parse_value(c_ast_node);
        assert_eq!(ir_ast_node, expected_ir_ast_node);
    }
}
