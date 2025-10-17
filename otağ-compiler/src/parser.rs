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
        Rule::output_statement => Ok(Statement::OutputStatement(parse_output_statement(inner)?)),
        _ => Err("Unknown statement".into()),
    }
}

fn parse_variable_declaration(pair: pest::iterators::Pair<Rule>) -> Result<VariableDeclaration, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let type_str = inner.next().unwrap().as_str();
    
    let var_type = match type_str {
        "tamsayı" => VariableType::Tamsayi,
        "metin" => VariableType::Metin,
        "ondalıklı" => VariableType::Ondalikli,
        "mantıksal" => VariableType::Mantiksal,
        _ => return Err("Unknown type".into()),
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
    let inner = pair.into_inner().next().unwrap();
    
    match inner.as_rule() {
        Rule::identifier => Ok(Expression::VariableRef(inner.as_str().to_string())),
        Rule::literal => Ok(Expression::Literal(parse_literal(inner)?)),
        _ => Err("Complex expressions not yet supported".into()),
    }
}

fn parse_literal(pair: pest::iterators::Pair<Rule>) -> Result<VariableValue, Box<dyn std::error::Error>> {
    let inner = pair.into_inner().next().unwrap();
    
    match inner.as_rule() {
        Rule::string_literal => {
            let s = inner.as_str();
            // Remove quotes
            let content = &s[1..s.len()-1];
            Ok(VariableValue::String(content.to_string()))
        },
        Rule::int_literal => Ok(VariableValue::Int(inner.as_str().parse()?)),
        Rule::float_literal => Ok(VariableValue::Float(inner.as_str().parse()?)),
        Rule::boolean_literal => {
            let val = inner.as_str() == "true";
            Ok(VariableValue::Bool(val))
        },
        _ => Err("Unknown literal".into()),
    }
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
        let input = "print isim";
        let program = parse(input).unwrap();
        assert_eq!(program.statements.len(), 1);
        if let Statement::OutputStatement(out) = &program.statements[0] {
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
        let input = "dogru_mu = true";
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
}
