// custom debug impls for lambda_calculus types

use crate::lambda_calculus::*;
use core::fmt::Debug;
use core::fmt::Error;
use core::fmt::Formatter;

// pub struct Lambda {
//     pub arg_name: String,
//     pub body: Expr,
// }

// pub struct Call {
//     pub func: Expr,
//     pub arg: Expr,
// }

// pub struct Var {
//     pub name: String,
// }

// pub enum Expr {
//     Lambda(Box<Lambda>),
//     Var(Var),
//     Call(Box<Call>),
// }

impl Debug for Lambda {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "[{} => {:?}]", self.arg_name, self.body)
    }
}

impl Debug for Call {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "[{:?} {:?}]", self.func, self.arg)
    }
}

impl Debug for Var {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(fmt, "{}", self.name)
    }
}
impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Expr::Call(c) => c.fmt(fmt),
            Expr::Lambda(l) => l.fmt(fmt),
            Expr::Var(v) => v.fmt(fmt),
        }
    }
}
