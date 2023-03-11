
use crate::Rule;
use super::*;

#[derive(Debug, Clone)]
pub enum Command {
    MovRegReg(RegIndex, RegIndex),
    MovRegNum(RegIndex, i32),
    AddRegReg(RegIndex, RegIndex),
    AddRegNum(RegIndex, i32),
    SubRegReg(RegIndex, RegIndex),
    SubRegNum(RegIndex, i32),
    MulRegReg(RegIndex, RegIndex),
    MulRegNum(RegIndex, i32),
    DivRegReg(RegIndex, RegIndex),
    DivRegNum(RegIndex, i32),
    Inc(RegIndex),
    Dec(RegIndex),
    PrintReg(RegIndex),
    PrintStr(String),
    PrintlnReg(RegIndex),
    PrintlnStr(String),
    ReadReg(RegIndex),
    PushReg(RegIndex),
    PushNum(i32),
    PopReg(RegIndex),
    CmpRegReg(RegIndex, RegIndex),
    CmpRegNum(RegIndex, i32),
    Jmp(Label),
    Je(Label),
    Jne(Label),
    Jg(Label),
    Jge(Label),
    Jl(Label),
    Jle(Label),
    Loop(RegIndex, Label),
    Call(Label),
    Ret,
}

impl Command {
    pub fn new_reg(verb: Rule, reg: RegIndex) -> Command {
        use Rule::*;
        let fabrica = match verb {
            inc_verb => Command::Inc,
            dec_verb => Command::Dec,
            push_verb => Command::PushReg,
            pop_verb => Command::PopReg,
            print_verb => Command::PrintReg,
            println_verb => Command::PrintlnReg,
            read_verb => Command::ReadReg,
            _ => unreachable!()
        };
        fabrica(reg)
    }

    pub fn new_num(verb: Rule, num: i32) -> Command {
        use Rule::*;
        let fabrica = match verb {
            push_verb => Command::PushNum,
            _ => unreachable!()
        };
        fabrica(num)
    }

    pub fn new_str(verb: Rule, text: &str) -> Command {
        use Rule::*;
        let fabrica = match verb {
            print_verb => Command::PrintStr,
            println_verb => Command::PrintlnStr,
            _ => unreachable!()
        };
        fabrica(text.to_string())
    }

    pub fn new_ip(verb: Rule, lbl: Label) -> Command {
        use Rule::*;
        let fabrica = match verb {
            jmp_verb => Command::Jmp,
            je_verb => Command::Je,
            jne_verb => Command::Jne,
            jl_verb => Command::Jl,
            jle_verb => Command::Jle,
            jg_verb => Command::Jg,
            jge_verb => Command::Jge,
            call_verb => Command::Call,
            _ => unreachable!()
        };
        fabrica(lbl)
    }

    pub fn new_regnum(verb: Rule, lreg: RegIndex, num: i32) -> Command {
        use Rule::*;
        let fabrica = match verb {
            mov_verb => Command::MovRegNum,
            add_verb => Command::AddRegNum,
            sub_verb => Command::SubRegNum,
            mul_verb => Command::MulRegNum,
            div_verb => Command::DivRegNum,
            cmp_verb => Command::CmpRegNum,
            _ => unreachable!()
        };
        fabrica(lreg, num)
    }

    pub fn new_regreg(verb: Rule, lreg: RegIndex, rreg: RegIndex) -> Command {
        use Rule::*;
        let fabrica = match verb {
            mov_verb => Command::MovRegReg,
            add_verb => Command::AddRegReg,
            sub_verb => Command::SubRegReg,
            mul_verb => Command::MulRegReg,
            div_verb => Command::DivRegReg,
            cmp_verb => Command::CmpRegReg,
            _ => unreachable!()
        };
        fabrica(lreg, rreg)
    }

    pub fn new_regip(verb: Rule, lreg: RegIndex, label: Label) -> Command {
        let fabrica = match verb {
            Rule::loop_verb => Command::Loop,
            _ => unreachable!()
        };
        fabrica(lreg, label)
    }
}
