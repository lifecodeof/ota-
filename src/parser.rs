use pest::Parser;
use pest_derive::Parser;
use crate::ast::*;
use crate::types::*;

#[derive(Parser)]
#[grammar = "src/grammar.pest"]
pub struct OtagParser;

pub fn parse(input: &str) -> Result<Program, Box<dyn std::error::Error>> {
    let mut pairs = OtagParser::parse(Rule::program, input)?;
    
    let program_pair = pairs.next().unwrap();
    let mut statements = Vec::new();
    
    for inner in program_pair.into_inner() {
        if inner.as_rule() == Rule::statement {
            statements.push(parse_statement(inner)?);
        }
    }
    
    Ok(Program { statements })
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement, Box<dyn std::error::Error>> {
    let inner = pair.into_inner().next().unwrap();
    
    match inner.as_rule() {
        Rule::variable_declaration => Ok(Statement::VariableDeclaration(parse_variable_declaration(inner)?)),
        Rule::assignment => Ok(Statement::Assignment(parse_assignment(inner)?)),
        Rule::output_statement => Ok(Statement::Output(parse_output_statement(inner)?)),
        Rule::if_statement => Ok(Statement::IfStatement(parse_if_statement(inner)?)),
        Rule::while_statement => Ok(Statement::WhileLoop(parse_while_statement(inner)?)),
        Rule::for_statement => Ok(Statement::ForLoop(parse_for_statement(inner)?)),
        Rule::break_statement => Ok(Statement::BreakStatement),
        Rule::continue_statement => Ok(Statement::ContinueStatement),
        _ => Err(format!("Unknown statement type: {:?}", inner.as_rule()).into()),
    }
}

fn parse_variable_declaration(pair: pest::iterators::Pair<Rule>) -> Result<VariableDeclaration, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    // "'ı" is matched but not captured
    let type_str = inner.next().unwrap().as_str();
    
    let var_type = match type_str {
        "tamsayı" => VariableType::Tamsayi,
        "metin" => VariableType::Metin,
        "ondalıklı" => VariableType::Ondalikli,
        "mantıksal" => VariableType::Mantiksal,
        _ => return Err(format!("Unknown variable type '{}'. Expected one of: tamsayı, metin, ondalıklı, mantıksal", type_str).into()),
    };
    
    Ok(VariableDeclaration { name, var_type })
}

fn parse_assignment(pair: pest::iterators::Pair<Rule>) -> Result<Assignment, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let expr = parse_expression(inner.next().unwrap())?;
    
    Ok(Assignment { name, expression: expr })
}

fn parse_output_statement(pair: pest::iterators::Pair<Rule>) -> Result<OutputStatement, Box<dyn std::error::Error>> {
    let expr = parse_expression(pair.into_inner().next().unwrap())?;
    
    Ok(OutputStatement { expression: expr })
}

fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Result<Expression, Box<dyn std::error::Error>> {
    let mut terms = Vec::new();
    let mut ops = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::term => {
                let expr = parse_term(inner)?;
                terms.push(expr);
            }
            Rule::operator => {
                let op_str = inner.as_str();
                ops.push(match op_str {
                    "+" => BinaryOperator::Add,
                    ">" => BinaryOperator::GreaterThan,
                    ">=" => BinaryOperator::GreaterThanOrEqual,
                    _ => return Err(format!("Unknown operator: {}", op_str).into()),
                });
            }
            _ => return Err(format!("Unexpected token in expression: {:?}", inner.as_rule()).into()),
        }
    }

    if terms.is_empty() {
        return Err("Expression cannot be empty".into());
    }

    if terms.len() == 1 {
        Ok(terms[0].clone())
    } else {
        // Build left-associated binary operations
        let mut result = terms[0].clone();
        for i in 0..ops.len() {
            let right = terms[i + 1].clone();
            result = Expression::BinaryOp(Box::new(result), ops[i].clone(), Box::new(right));
        }
        Ok(result)
    }
}

fn parse_term(pair: pest::iterators::Pair<Rule>) -> Result<Expression, Box<dyn std::error::Error>> {
    let inner = pair.into_inner().next().unwrap();
    
    match inner.as_rule() {
        Rule::identifier => Ok(Expression::VariableRef(inner.as_str().to_string())),
        Rule::literal => Ok(Expression::Literal(parse_literal(inner)?)),
        _ => Err(format!("Expected identifier or literal, found: {:?}", inner.as_rule()).into()),
    }
}

fn parse_literal(pair: pest::iterators::Pair<Rule>) -> Result<VariableValue, Box<dyn std::error::Error>> {
    let inner = pair.into_inner().next().unwrap();
    let s = inner.as_str();

    match inner.as_rule() {
        Rule::string_literal => {
            // Remove quotes
            let content = &s[1..s.len()-1];
            Ok(VariableValue::String(content.to_string()))
        },
        Rule::int_literal => Ok(VariableValue::Int(s.trim().parse()?)),
        Rule::float_literal => Ok(VariableValue::Float(s.trim().parse()?)),
        Rule::boolean_literal => {
            let val = s.trim() == "doğru";
            Ok(VariableValue::Bool(val))
        },
        _ => Err(format!("Unknown literal type: {:?}", inner.as_rule()).into()),
    }
}

fn parse_if_statement(pair: pest::iterators::Pair<Rule>) -> Result<IfStatement, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    // Skip "eğer"
    let condition_pair = inner.next().unwrap();
    let expr_pair = condition_pair.into_inner().next().unwrap();
    let condition = Condition { expression: Box::new(parse_expression(expr_pair)?) };
    // Skip "ise"
    let then_block = parse_control_block(inner.next().unwrap())?;
    let else_block = if let Some(yoksa_pair) = inner.next() {
        Some(parse_control_block(yoksa_pair)?)
    } else {
        None
    };
    // Skip "son"
    Ok(IfStatement { condition, then_block, else_block })
}

fn parse_while_statement(pair: pest::iterators::Pair<Rule>) -> Result<WhileLoop, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    // Skip "döngü"
    let condition_pair = inner.next().unwrap();
    let condition = Condition { expression: Box::new(parse_expression(condition_pair)?) };
    // Skip "ise"
    let body = parse_control_block(inner.next().unwrap())?;
    // Skip "son"
    Ok(WhileLoop { condition, body })
}

fn parse_for_statement(pair: pest::iterators::Pair<Rule>) -> Result<ForLoop, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    // Skip "için"
    let var_name = inner.next().unwrap().as_str().to_string();
    let loop_variable = LoopVariable { name: var_name, is_auto_generated: false };
    // Skip "in"
    let range_spec = inner.next().unwrap();
    let (range_start, range_end, step) = parse_range_spec(range_spec)?;
    // Skip "ise"
    let body = parse_control_block(inner.next().unwrap())?;
    // Skip "son"
    Ok(ForLoop { loop_variable, range_start, range_end, step, body })
}

fn parse_control_block(pair: pest::iterators::Pair<Rule>) -> Result<ControlBlock, Box<dyn std::error::Error>> {
    let mut statements = Vec::new();
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::statement {
            statements.push(parse_statement(inner)?);
        }
    }
    Ok(ControlBlock { statements })
}

fn parse_range_spec(pair: pest::iterators::Pair<Rule>) -> Result<(Box<Expression>, Box<Expression>, Option<Box<Expression>>), Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let start = Box::new(parse_expression(inner.next().unwrap())?);
    // Skip "dan"
    let end = Box::new(parse_expression(inner.next().unwrap())?);
    let step = if let Some(step_pair) = inner.next() {
        // Skip "adım"
        Some(Box::new(parse_expression(step_pair)?))
    } else {
        None
    };
    Ok((start, end, step))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_assignment() {
        let input = "isim = 123";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
        if let Statement::Assignment(assign) = &program.statements[0] {
            assert_eq!(assign.name, "isim");
            if let Expression::Literal(VariableValue::Int(123)) = &assign.expression {
                // ok
            } else {
                panic!("Wrong expression");
            }
        } else {
            panic!("Not assignment");
        }
    }

    #[test]
    fn test_parse_output() {
        let input = "söyle isim";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
        if let Statement::Output(out) = &program.statements[0] {
            if let Expression::VariableRef(name) = &out.expression {
                assert_eq!(name, "isim");
            } else {
                panic!("Wrong expression");
            }
        } else {
            panic!("Not output");
        }
    }

    #[test]
    fn test_parse_string_assignment() {
        let input = r#"mesaj = "Merhaba""#;
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
        if let Statement::Assignment(assign) = &program.statements[0] {
            assert_eq!(assign.name, "mesaj");
            if let Expression::Literal(VariableValue::String(s)) = &assign.expression {
                assert_eq!(s, "Merhaba");
            } else {
                panic!("Wrong expression");
            }
        } else {
            panic!("Not assignment");
        }
    }

    #[test]
    fn test_parse_float_assignment() {
        let input = "pi = 3.14";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
        if let Statement::Assignment(assign) = &program.statements[0] {
            assert_eq!(assign.name, "pi");
            if let Expression::Literal(VariableValue::Float(f)) = &assign.expression {
                assert!((f - 3.14).abs() < 0.001);
            } else {
                panic!("Wrong expression");
            }
        } else {
            panic!("Not assignment");
        }
    }

    #[test]
    fn test_parse_bool_assignment() {
        let input = "dogru_mu = doğru";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
        if let Statement::Assignment(assign) = &program.statements[0] {
            assert_eq!(assign.name, "dogru_mu");
            if let Expression::Literal(VariableValue::Bool(b)) = &assign.expression {
                assert_eq!(*b, true);
            } else {
                panic!("Wrong expression");
            }
        } else {
            panic!("Not assignment");
        }
    }

    #[test]
    fn test_parse_binary_add() {
        let input = "toplam = 5 + 3";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
        if let Statement::Assignment(assign) = &program.statements[0] {
            assert_eq!(assign.name, "toplam");
            if let Expression::BinaryOp(left, BinaryOperator::Add, right) = &assign.expression {
                if let Expression::Literal(VariableValue::Int(5)) = &**left {
                    // ok
                } else {
                    panic!("Left not 5");
                }
                if let Expression::Literal(VariableValue::Int(3)) = &**right {
                    // ok
                } else {
                    panic!("Right not 3");
                }
            } else {
                panic!("Not binary add");
            }
        } else {
            panic!("Not assignment");
        }
    }

    #[test]
    fn test_parse_variable_declaration() {
        let input = "x'ı tamsayı olarak tanımla";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
        if let Statement::VariableDeclaration(decl) = &program.statements[0] {
            assert_eq!(decl.name, "x");
            assert_eq!(decl.var_type, VariableType::Tamsayi);
        } else {
            panic!("Not variable declaration");
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let input = "eğer x > 5 ise\nsöyle \"Büyük\"\nson";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
        if let Statement::IfStatement(if_stmt) = &program.statements[0] {
            // Check condition
            if let Expression::BinaryOp(left, BinaryOperator::GreaterThan, right) = &*if_stmt.condition.expression {
                if let Expression::VariableRef(var) = &**left {
                    assert_eq!(var, "x");
                } else {
                    panic!("Left not variable x");
                }
                if let Expression::Literal(VariableValue::Int(5)) = &**right {
                    // ok
                } else {
                    panic!("Right not 5");
                }
            } else {
                panic!("Condition not binary op >");
            }
            // Check then block
            assert_eq!(if_stmt.then_block.statements.len(), 1);
            if let Statement::Output(_) = &if_stmt.then_block.statements[0] {
                // ok
            } else {
                panic!("Then block not output");
            }
            // No else block
            assert!(if_stmt.else_block.is_none());
        } else {
            panic!("Not if statement");
        }
    }
}
