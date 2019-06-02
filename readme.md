# Lambda calculus interpreter

This project is the result of a timed challenge. It's not really useful for anything, but it was super fun and educational.

The challenge in a nutshell: create a lambda calculus-based language, interpreter, and type inference algorithm.

Combinators like Y-Combinator should be supported.

The language had to function enough to write the following programs:
- Compute sqrt using bisection
- get the nth fibbonacci number
- write an infinitely recursive program

The time allowed was one day but I only had ~8 hours of free time available to take the challenge.

## Implementation journal

I started by sketching out the required test programs. You can see my initial guesses in the examples directory.
I didn't have time to plug them into the interpreter, so they have not been tested.

Next I had to learn the lambda calculus. This part took the most time. During this period, I switched back and forth
between reading and coding. After several failed starts, I eventually arrived as a somewhat
sane design for the interpreter. (See `src/lambda_calculus.rs` for the implementation)
Side note: I don't yet know how it is useful, but Church encoding is beautiful.

Lambda expressions can be expressed as a Rust enum with three variants. This enum is the central part of the interpreter.

```rust
enum Expr {
    Lambda(Lambda), // example: (λ f . (f f))
    Var(Var),       // example: f
    Call(Call),     // example: (f f)
}
```

A method `eval` is implemented for `Expr`. The `eval` function performs beta-reduction. Alpha conversion is trivial
in this case; the `replace` function on `Lambda` checks its single named argument against "find" value. If the
names match, the lambda's find function does not recurse into the lambda's body. Check out the function definition
for a clearer explination.

After assembling the bones of the interpreter, I needed to test the interpreter. In order to test the interpreter,
I needed a way to input expressions, which brings me the last stage of todays development, the parser.

I macgyvered serde_json into a quick and dirty lexer (See `src/syntax.rs`).

In this case, parsing happens in three stages.

1. bytes are parsed as json
2. json lists and strings are converted to an intermediate type called `Sexpr`.
   (Anthing that is not a string or list results in a parse error.)
3. `Sexpr` is desugared into `Expr`

`Sexpr` is a sugared version of `Expr`, it's possible for lambdas and function calls in `Sexpr`
to take and pass mulitiple arguments. This isn't allowed in basic lambda expressions.
`Sexpr` implements a `desugar` method makes currying easy.

With desugaring implemented, expressions look like this:

```
[f => a => a f] // (λ f . (λ a . a f))
[a b c d]       // (((a b) c) d)
```

Though the lexer is a json parser so expressionss are currently uglier than the ideal:

```
["f", "=>", "a", "=>", "a", "f"] // (λ f . (λ a . a f))
["a", "b", "c", "d"]             // (((a b) c) d)
```

Don't look at that example too long. You'll hurt your eyes.

## Summary of progress

- Write a simple lambda calculus-based language interpreter
  - Kinda done, but not tested
- Type inference algorithm
  - Not done, would take another day or two. I'd need to learn how type inference works.
- Support Y-Combinator
  - The types in `src/lambda_calculus.rs` allow it, but some tweaks would be neccesary to make
    programs run to completions
- Runnable test programs
  - Test programs were never loaded into the interpreter.

Very fun challenge, glad I had an excuse to try it.
