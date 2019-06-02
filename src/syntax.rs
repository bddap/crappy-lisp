use crate::lambda_calculus::{Call, Expr, Lambda, Var};
use serde_json::Value;

#[derive(Clone, PartialEq, Debug)]
enum Sexpr {
    String(String),
    Sexprs(Vec<Sexpr>),
}

impl Sexpr {
    fn from_json(json: &Value) -> Result<Sexpr, String> {
        let ret = match json {
            Value::Array(vals) => Sexpr::Sexprs(
                vals.iter()
                    .map(Sexpr::from_json)
                    .collect::<Result<Vec<Sexpr>, _>>()?,
            ),
            Value::String(st) => Sexpr::String(st.to_string()),
            _ => Err("got a value that was not an array or a string")?,
        };
        Ok(ret)
    }

    /// uncurries function applications and lambdas
    fn desugar(&self) -> Result<Expr, String> {
        match self {
            Sexpr::String(name) => Ok(Expr::Var(Var {
                name: name.to_string(),
            })),
            Sexpr::Sexprs(sexprs) => Sexpr::desugar_list(sexprs),
        }
    }

    fn desugar_list(ls: &[Sexpr]) -> Result<Expr, String> {
        if ls.is_empty() {
            return Err("empty sexpr not allowed".into());
        }
        if ls[0] == Sexpr::String("=>".to_string()) {
            return Err("=> symbol cannot start an expression".into());
        }
        if ls.len() == 1 {
            ls[0].desugar()
        } else if ls[1] == Sexpr::String("=>".to_string()) {
            let arg_name = match &ls[0] {
                Sexpr::String(st) => st,
                Sexpr::Sexprs(_) => {
                    return Err("functions can only take identifiers as arguments".into())
                }
            };
            Ok(Sexpr::desugar_lambda(&arg_name, &ls[2..])?.into())
        } else {
            Ok(Sexpr::desugar_call(ls)?.into())
        }
    }

    fn desugar_lambda(arg_name: &str, rest: &[Sexpr]) -> Result<Lambda, String> {
        let body = match rest {
            [a] => a.desugar()?,
            rest => Sexpr::desugar_list(rest)?,
        };
        Ok(Lambda {
            arg_name: arg_name.to_string(),
            body,
        })
    }

    fn desugar_call(rest: &[Sexpr]) -> Result<Call, String> {
        assert!(rest.len() > 1);
        assert!(!rest[1].is_arrow());
        let ret = if rest.len() == 2 {
            Call {
                func: rest[0].desugar()?,
                arg: rest[1].desugar()?,
            }
        } else if rest[2].is_arrow() {
            // a lambda is comming, we don't consume it
            Call {
                func: rest[0].desugar()?,
                arg: Sexpr::desugar_list(&rest[1..])?.into(),
            }
        } else {
            Call {
                func: Sexpr::desugar_call(&rest[..2])?.into(),
                arg: Sexpr::desugar_list(&rest[2..])?.into(),
            }
        };
        Ok(ret)
    }

    fn is_arrow(&self) -> bool {
        self == &Sexpr::String("=>".to_string())
    }
}

// fn split_on_arrow(ls: &[Sexpr]) -> Result<(&[Sexpr], &Sexpr, &[Sexpr]), String> {
//     let (pre, post) = ls
//         .split_first(is_arrow)
//         .expect("sexpr does not contain any arrows");
//     if pre.len() == 0 {
//         return Err("arrow cannot appear at start of expression".into());
//     }
//     if post.len() == 0 {
//         return Err("arrow cannot appear at end of expression".into());
//     }
//     Ok((pre[..pre.len() - 1], pre[pre.len() - 1], post))
// }

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse() {
        Sexpr::from_json(&json!([])).unwrap();
        Sexpr::from_json(&json!(["a", "b", [], ["c", ["d"]]])).unwrap();
    }

    /// parse json then desugar into an Expr
    fn desu(jsonv: &Value) -> Result<Expr, String> {
        Sexpr::from_json(&jsonv).unwrap().desugar()
    }

    #[test]
    fn desugar() {
        desu(&json!([])).unwrap_err();
        desu(&json!(["a", "b", [], ["c", ["d"]]])).unwrap_err();
        assert_eq!(
            desu(&json!(["a", "=>", "b", "=>", ["a", "b"]])).unwrap(),
            Lambda {
                arg_name: "a".to_string(),
                body: Lambda {
                    arg_name: "b".to_string(),
                    body: Call {
                        func: Var {
                            name: "a".to_string()
                        }
                        .into(),
                        arg: Var {
                            name: "b".to_string()
                        }
                        .into()
                    }
                    .into()
                }
                .into()
            }
            .into()
        );
        assert_eq!(
            desu(&json!(["a", "=>", "b", "=>", "a", "b"])).unwrap(),
            desu(&json!(["a", "=>", ["b", "=>", ["a", "b"]]])).unwrap(),
        );

        // Ideally, this would be true, but I haven't gotten arround to making it so
        // assert_eq!(
        //     desu(&json!(["a", "b", "c", "d", "=>", "d", "d"])).unwrap(),
        //     desu(&json!([[["a", "b"], "c"], ["d", "=>", ["d", "d"]]])).unwrap()
        // );

        // This is current behavior
        assert_eq!(
            desu(&json!(["a", "b", "c", "d", "=>", "d", "d"])).unwrap(),
            desu(&json!([["a", "b"], ["c", ["d", "=>", ["d", "d"]]]])).unwrap()
        );
    }
}
