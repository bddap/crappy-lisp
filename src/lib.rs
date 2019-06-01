// enum Expr {
//     Lambda(Vec<String>, Box<Expr>),
//     Call(Box<(Expr, Expr)>),
//     Var(String),
// }

// // This implementation will have no need for alpha conversion
// // the beta reduction step will not overwrite inner lambda arguments

// // an expression with no free variables
// struct Combinator {}

#[derive(Clone)]
struct Lambda {
    arg_name: String,
    body: Expr,
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
}

#[derive(Clone)]
struct Call {
    func: Expr,
    arg: Expr,
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
}

#[derive(Clone)]
struct Var {
    name: String,
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

#[derive(Clone)]
enum Expr {
    Lambda(Box<Lambda>),
    Var(Var),
    Call(Box<Call>),
}

impl Expr {
    fn eval(self) -> Expr {
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
