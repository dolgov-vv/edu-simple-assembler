use pest::{Parser};
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use super::*;

#[derive(Parser)]
#[grammar = "pest/assembler.pest"]
pub struct AssemblerParser;

impl AssemblerParser {
    pub fn parse_program(prg: &str) -> Result<Program, Error<Rule>> {
        let pairs = AssemblerParser::parse(Rule::program, prg)?;
        let mut prg = Program::default();
        for pair in pairs.into_iter() {
            match pair.as_rule() {
                Rule::command => AssemblerParser::parse_command(&mut prg, pair),
                Rule::label => {
                    let label = pair.into_inner().as_str().to_string();
                    prg.set_label(label, prg.get_command_count());
                },
                Rule::EOI => (),
                _ => unreachable!()
            };
        }
        Ok(prg)
    }

    fn parse_command(prg: &mut Program, pair: Pair<Rule>) {
        use Rule::*;
        let pair = pair.into_inner().next().expect("Invalid command inner structure");
        let cmd = match pair.as_rule() {
            unit_command => AssemblerParser::parse_unit_command(&pair),
            unary_reg_command => AssemblerParser::parse_unary_reg_command(prg, &pair),
            unary_regnum_command => AssemblerParser::parse_unary_regnum_command(prg, &pair),
            unary_regstr_command => AssemblerParser::parse_unary_regstr_command(prg, &pair),
            unary_ip_command => AssemblerParser::parse_unary_ip_command(prg, &pair),
            binary_reg_regnum_command => AssemblerParser::parse_binary_reg_regnum_command(prg, &pair),
            binary_reg_ip_command => AssemblerParser::parse_binary_reg_ip_command(prg, &pair),
            _ => unreachable!()
        };
        prg.append_command(cmd);
    }

    fn read_command_parts<'a>(pair: &'a Pair<Rule>) -> (Rule, &'a str, Option<&'a str>) {
        let mut parts: Pairs<Rule> = pair.clone().into_inner();
        let rule = parts.next().unwrap().as_rule();
        let lhs = parts.next().unwrap().as_str();
        let rhs = parts.next().and_then(|x| Some(x.as_str()));
        (rule, lhs, rhs)
    }

    fn parse_unit_command(pair: &Pair<Rule>) -> Command {
        let mut parts: Pairs<Rule> = pair.clone().into_inner();
        let rule = parts.next().unwrap().as_rule();
        match rule {
            Rule::ret_verb => Command::Ret,
            _ => unreachable!()
        }
    }

    fn parse_binary_reg_regnum_command(prg: &mut Program, pair: &Pair<Rule>) -> Command {
        if let (rule, lhs, Some(rhs)) = AssemblerParser::read_command_parts(pair) {
            let lhs = prg.get_or_create_reg(lhs);

            if let Ok(num) = rhs.parse::<i32>() {
                Command::new_regnum(rule, lhs, num)
            } else {
                let reg = prg.get_or_create_reg(rhs);
                Command::new_regreg(rule, lhs, reg)
            }
        } else {
            panic!("Invalid command inner structure.");
        }
    }

    fn parse_binary_reg_ip_command(prg: &mut Program, pair: &Pair<Rule>) -> Command {
        if let (rule, lhs, Some(rhs)) = AssemblerParser::read_command_parts(pair) {
            let lhs = prg.get_or_create_reg(lhs);
            Command::new_regip(rule, lhs, rhs.to_string())
        } else {
            panic!("Invalid command inner structure.");
        }
    }

    fn parse_unary_reg_command(prg: &mut Program, pair: &Pair<Rule>) -> Command {
        let (rule, lhs, _) = AssemblerParser::read_command_parts(pair);
        let lhs = prg.get_or_create_reg(lhs);
        Command::new_reg(rule, lhs)
    }

    fn parse_unary_regstr_command(prg: &mut Program, pair: &Pair<Rule>) -> Command {
        let mut parts: Pairs<Rule> = pair.clone().into_inner();
        let rule = parts.next().unwrap().as_rule();
        let param = parts.next().unwrap();
        match param.as_rule() {
            Rule::ident => Command::new_reg(rule, prg.get_or_create_reg(param.as_str())),
            Rule::string => Command::new_str(rule, param.into_inner().next().unwrap().as_str()),
            _ => unreachable!()
        }
    }

    fn parse_unary_regnum_command(prg: &mut Program, pair: &Pair<Rule>) -> Command {
        let (rule, lhs, _) = AssemblerParser::read_command_parts(pair);
        if let Ok(num) = lhs.parse::<i32>() {
            Command::new_num(rule, num)
        } else {
            let reg = prg.get_or_create_reg(lhs);
            Command::new_reg(rule, reg)
        }
    }

    fn parse_unary_ip_command(_prg: &mut Program, pair: &Pair<Rule>) -> Command {
        let (rule, lhs, _) = AssemblerParser::read_command_parts(pair);
        Command::new_ip(rule, lhs.to_string())
    }
}
