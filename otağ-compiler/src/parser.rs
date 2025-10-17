use pest::Parser;
use pest_derive::Parser;
use crate::ast::*;
use crate::types::*;

#[derive(Parser)]
#[grammar = "src/grammar.pest"]
pub struct OtagParser;

pub fn parse(input: &str) -> Result<Program, Box<dyn std::error::Error>> {
    let pairs = OtagParser::parse(Rule::program, input)?;
    
    let mut statements = Vec::new();
    
    for pair in pairs {
        if pair.as_rule() == Rule::statement {
            statements.push(parse_statement(pair)?);
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
            let val = inner.as_str() == "doğru";
            Ok(VariableValue::Bool(val))
        },
        _ => Err("Unknown literal".into()),
    }
}
