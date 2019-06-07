#[derive(Clone, PartialEq)]
pub struct Lambda {
    pub arg_name: String,
    pub body: Expr,
}

impl Lambda {
    fn call(mut self, arg: Expr) -> Expr {
        self.body = self.body.eval();
        match self.body {
            Expr::Var(s) => s.replace(self.arg_name, arg),
            Expr::Lambda(l) => l.replace(self.arg_name, arg).into(),
            Expr::Call(c) => c.replace(self.arg_name, arg).into(),
        }
    }

    fn replace(mut self, name: String, with: Expr) -> Lambda {
        // alpha conversion is done here
        if self.arg_name != name {
            self.body = self.body.replace(name, with);
        }
        self
    }

    fn should_continue(&self) -> bool {
        self.body.should_continue()
    }
}

#[derive(Clone, PartialEq)]
pub struct Call {
    pub func: Expr,
    pub arg: Expr,
}

impl Call {
    fn eval(mut self) -> Expr {
        self.func = self.func.eval();
        self.arg = self.arg.eval();
        match self.func {
            Expr::Lambda(l) => l.call(self.arg),
            Expr::Var(_) | Expr::Call(_) => Expr::Call(Box::new(self)),
        }
    }

    fn replace(self, name: String, with: Expr) -> Call {
        Call {
            func: self.func.replace(name.clone(), with.clone()),
            arg: self.arg.replace(name, with),
        }
    }

    fn should_continue(&self) -> bool {
        match self.func {
            Expr::Lambda(_) => true,
            _ => self.func.should_continue() || self.arg.should_continue(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Var {
    pub name: String,
}

impl Var {
    fn replace(self, name: String, with: Expr) -> Expr {
        if self.name == name {
            with
        } else {
            Expr::Var(self)
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Expr {
    Lambda(Box<Lambda>),
    Var(Var),
    Call(Box<Call>),
}

impl Expr {
    /// Perform one interpreter tick, returning the result.
    ///
    /// It's sometimes impossible to tell how many ticks it will take to finish the computation,
    /// or even whether the computaion will finish. You can check whether the expression has
    /// has more work to do by calling should_continue.
    ///
    /// ```
    /// use crappy_lisp::Sexpr;
    /// use serde_json::json;
    ///
    /// let program = Sexpr::from_json(&json!([["a", "=>", "a"], "candy"]))
    ///     .unwrap()
    ///     .desugar()
    ///     .unwrap();
    /// let result = Sexpr::from_json(&json!("candy"))
    ///     .unwrap()
    ///     .desugar()
    ///     .unwrap();
    /// assert_eq!(program.eval(), result);
    /// ```
    pub fn eval(self) -> Expr {
        match self {
            Expr::Lambda(_) => self,
            Expr::Var(_) => self,
            Expr::Call(c) => c.eval(),
        }
    }

    fn replace(self, name: String, with: Expr) -> Expr {
        match self {
            Expr::Lambda(l) => l.replace(name, with).into(),
            Expr::Var(s) => s.replace(name, with),
            Expr::Call(c) => c.replace(name, with).into(),
        }
    }

    /// Check whether the expression contains a callable Call (a Call with a lambda in its function
    /// position)
    pub fn should_continue(&self) -> bool {
        match self {
            Expr::Lambda(l) => l.should_continue(),
            Expr::Var(_) => false,
            Expr::Call(c) => c.should_continue(),
        }
    }
}

impl Into<Expr> for Lambda {
    fn into(self) -> Expr {
        Expr::Lambda(Box::new(self))
    }
}

impl Into<Expr> for Call {
    fn into(self) -> Expr {
        Expr::Call(Box::new(self))
    }
}

impl Into<Expr> for Var {
    fn into(self) -> Expr {
        Expr::Var(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::Sexpr;
    use serde_json::{json, Value};

    /// parse json then desugar into an Expr
    fn desu(jsonv: &Value) -> Result<Expr, String> {
        Sexpr::from_json(&jsonv).unwrap().desugar()
    }

    #[test]
    fn eval() {
        let expr = desu(&json!([["arg", "=>", "arg"], "candy"])).unwrap();
        assert_eq!(expr.eval(), desu(&json!("candy")).unwrap());
    }

    #[test]
    fn omega() {
        // ((λ f . (f f)) (λ f . (f f)))
        let expr = desu(&json!([
            ["arg", "=>", "arg", "arg"],
            ["arg", "=>", "arg", "arg"]
        ]))
        .unwrap();

        assert_eq!(expr.clone().eval(), expr);
    }

    #[test]
    fn should_continue() {
        let omega = desu(&json!([
            ["arg", "=>", "arg", "arg"],
            ["arg", "=>", "arg", "arg"]
        ]))
        .unwrap();
        assert!(omega.should_continue());
        assert!(omega.eval().should_continue());

        let y = desu(&json!([
            "f",
            "=>",
            ["x", "=>", "f", "x", "x"],
            ["x", "=>", "f", "x", "x"]
        ]))
        .unwrap();
        assert!(y.should_continue());
        assert!(y.eval().should_continue());

        let prog = desu(&json!([["a", "=>", "a"], "candy"])).unwrap();
        assert!(prog.should_continue());
        assert!(!prog.eval().should_continue());
    }

    #[test]
    fn y_static() {
        let y = desu(&json!([
            "f",
            "=>",
            ["x", "=>", "f", "x", "x"],
            ["x", "=>", "f", "x", "x"]
        ]))
        .unwrap();
        assert_eq!(y.clone(), y.eval());
    }

    #[test]
    fn ycombinator() {
        let y = json!([
            "f",
            "=>",
            ["x", "=>", "f", "x", "x"],
            ["x", "=>", "f", "x", "x"]
        ]);
        let id = json!(["a", "=>", "a"]);
        assert!(!desu(&id).unwrap().should_continue());
        let recursive_id = json!([y, id]);
        let mut prog = desu(&recursive_id).unwrap();

        for _ in 0..100_000 {
            assert!(prog.should_continue());
            prog = prog.eval();
        }
    }
}
