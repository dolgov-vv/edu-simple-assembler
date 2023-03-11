use std::io;
use std::io::Write;
use super::*;
use text_io::read;

#[derive(Debug)]
pub struct ExecutingContext<'a> {
    program: &'a Program,
    ip: IP,
    stack: Vec<i32>,
    reg_values: Vec<i32>,
    flags: Flags,
}

impl ExecutingContext<'_> {
    pub fn new(prg: &Program) -> ExecutingContext {
        ExecutingContext {
            program: prg,
            ip: 0,
            stack: Vec::with_capacity(256),
            reg_values: vec![0; prg.get_reg_count()],
            flags: Flags { is_sign: false, is_zero: false },
        }
    }

    #[inline]
    pub fn get_reg_value(&self, reg: &RegIndex) -> i32 {
        self.reg_values[*reg]
    }

    #[inline]
    pub fn set_reg_value(&mut self, reg: &RegIndex, value: i32) {
        self.reg_values[*reg] = value;
    }

    pub fn execute(&mut self) {
        let end_of_program = self.program.get_command_count();
        self.ip = 0;
        while self.ip < end_of_program {
            self.execute_step();
        }
    }

    pub fn execute_step(&mut self) {
        use Command::*;
        if let Some(command) = self.program.get_command(self.ip) {
            self.ip = match command {
                MovRegReg(lhs, rhs) => self.execute_regreg_command(lhs, rhs, |_, b| b),
                AddRegReg(lhs, rhs) => self.execute_regreg_command(lhs, rhs, |a, b| a+b),
                SubRegReg(lhs, rhs) => self.execute_regreg_command(lhs, rhs, |a, b| a-b),
                MulRegReg(lhs, rhs) => self.execute_regreg_command(lhs, rhs, |a, b| a*b),
                DivRegReg(lhs, rhs) => self.execute_regreg_command(lhs, rhs, |a, b| a/b),

                MovRegNum(lhs, num) => self.execute_regnum_command(lhs, num, |_, x| x),
                AddRegNum(lhs, num) => self.execute_regnum_command(lhs, num, |a, x| a+x),
                SubRegNum(lhs, num) => self.execute_regnum_command(lhs, num, |a, x| a-x),
                MulRegNum(lhs, num) => self.execute_regnum_command(lhs, num, |a, x| a*x),
                DivRegNum(lhs, num) => self.execute_regnum_command(lhs, num, |a, x| a/x),

                CmpRegReg(lreg, rreg) => {
                    let lval = self.get_reg_value(lreg);
                    let rval = self.get_reg_value(rreg);
                    self.update_flags(lval - rval);
                    self.ip + 1
                },
                CmpRegNum(lreg, rnum) => {
                    let lval = self.get_reg_value(lreg);
                    self.update_flags(lval - *rnum);
                    self.ip + 1
                },

                Inc(reg) => self.execute_unary_reg(reg, true, |x| x+1),
                Dec(reg) => self.execute_unary_reg(reg, true, |x| x-1),
                PopReg(reg) => {
                    let poped_value = self.stack.pop().unwrap();
                    self.execute_unary_reg(reg, false, |_| poped_value)
                },
                ReadReg(reg) => self.execute_unary_reg(reg, false, |_| read!()),

                PushReg(reg) => {
                    let lval = self.get_reg_value(reg);
                    self.stack.push(lval);
                    self.ip + 1
                },
                PushNum(num) => {
                    self.stack.push(*num);
                    self.ip + 1
                },

                PrintReg(reg) => {
                    let lval = self.get_reg_value(reg);
                    print!("{}", lval);
                    io::stdout().flush().unwrap();
                    self.ip + 1
                },
                PrintStr(text) => {
                    print!("{}", (*text).as_str());
                    io::stdout().flush().unwrap();
                    self.ip + 1
                },
                PrintlnReg(reg) => {
                    let lval = self.get_reg_value(reg);
                    println!("{}", lval);
                    self.ip + 1
                },
                PrintlnStr(text) => {
                    println!("{}", (*text).as_str());
                    self.ip + 1
                },

                Jmp(label) => self.program.get_label_ip(label).unwrap(),
                Je(label) => self.jump_if(self.flags.is_zero, label),
                Jne(label) => self.jump_if(!self.flags.is_zero, label),
                Jg(label) => self.jump_if(!self.flags.is_zero && !self.flags.is_sign, label),
                Jge(label) => self.jump_if(self.flags.is_zero || !self.flags.is_sign, label),
                Jl(label) => self.jump_if(!self.flags.is_zero && self.flags.is_sign, label),
                Jle(label) => self.jump_if(self.flags.is_zero || self.flags.is_sign, label),
                Loop(lreg, label) => {
                    let counter = self.get_reg_value(lreg) - 1;
                    self.set_reg_value(lreg, counter);
                    self.jump_if(counter > 0, label)
                },
                Call(label) => {
                    self.stack.push((self.ip + 1) as i32);
                    self.program.get_label_ip(label).unwrap()
                },
                Ret => self.stack.pop().unwrap() as IP,
            }
        }
    }

    fn execute_regreg_command<F: Fn(i32, i32) -> i32>(
        &mut self, lreg: &RegIndex, rreg: &RegIndex, op: F) -> IP
    {
        let lval = self.get_reg_value(lreg);
        let rval = self.get_reg_value(rreg);
        let res = op(lval, rval);
        self.set_reg_value(lreg, res);
        self.update_flags(res);
        self.ip + 1
    }

    fn execute_regnum_command<F: Fn(i32, i32) -> i32>(
        &mut self, lreg: &RegIndex, rnum: &i32, op: F) -> IP
    {
        let lval = self.get_reg_value(lreg);
        let res = op(lval, *rnum);
        self.set_reg_value(lreg, res);
        self.update_flags(res);
        self.ip + 1
    }

    fn execute_unary_reg<F: Fn(i32) -> i32>(&mut self, reg: &RegIndex, update_flags: bool, op: F) -> IP {
        let val = op(self.get_reg_value(reg));
        self.set_reg_value(reg, val);
        if update_flags { self.update_flags(val) }
        self.ip + 1
    }

    #[inline]
    fn jump_if(&self, cond: bool, label: &Label) -> IP {
        if cond {
            self.program.get_label_ip(label).unwrap()
        } else {
            self.ip + 1
        }
    }

    #[inline]
    fn update_flags(&mut self, value: i32) {
        self.flags.is_zero = value == 0;
        self.flags.is_sign = value < 0;
    }
}
