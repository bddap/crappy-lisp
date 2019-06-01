use serde_json::json as lisp; // I find this funny

fn main() {
    // Computing sqrt(x) using bisection
    let _sqrt = lisp!([
        {
            "sqrt": ["fn", ["L", "H", "Q", "depth"], [
                "if",
                ["<=", "depth", 0],
                "L",
                [
                    {"halfway": ["/", ["+", "L", "H"], 2]},
                    "if",
                    ["<", ["exp2", "halfway"], "Q"],
                    ["sqrt", "L", "halfway", ["-", "depth", 1]],
                    ["sqrt", "halfway", "H", ["-", "depth", 1]]
                ]
            ]],
            "exp2": ["fn", ["a"], ["*", "a", "a"]]
        },
        ["fn", ["Q"], ["sqrt", 0, "Q", 32]]
    ]);

    // Write `fib` (fibonacci) function
    let _fib = lisp!([
        {
            "fib": ["fn", ["n"], [
                "if",
                ["==", "n", 1],
                0,
                [
                    "if",
                    ["==", "n", 2],
                    1,
                    ["+", ["fib", ["-", "n", 1]], ["fib", ["-", "n", 2]]]
                ]
            ]],
        },
        ["fn", ["n"], ["fib", "n"]]
    ]);

    // Write a program that infinitely recurses upon itself
    let _recurse = lisp!([
        {"rec": ["fn", [], ["rec"]]},
        ["fn", [], ["rec"]]
    ]);

    // This silly thing is called the Omega program, it does not terminate
    // ((λ f . (f f)) (λ f . (f f)))
}

// needed builtins:
// if
// *
// /
// +
// -
// ==
// fn
