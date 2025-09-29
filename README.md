# rot

The [book](https://interpreterbook.com/), _Writing an Interpreter in Go_ by Thurston Ball, works through creating an interpreter for the Monkey language.

This repo is an attempt to follow the guidance of the book, to develop a language, using [rust](https://rust-lang.org/).

## Examples



```
// comments begin with two forward slashes

// declarations follow the pattern of `let` or `const`, then an idenfier, a colon, the type, an equals, and the value.

// the format depends on the type.

let x: int64 = 5;

// functions are declared as:
// let <ident>: fn = <return tuple> <- <parameter tuple> { <statements> };
let foo: fn = (z: i64) <- (x: i64, y:i64) { return i64.add(x, y); };
let result = foo(1, 2)

// defining functions
proto unary_math_operation: fn = (res: i64) <- (x: i64);

const square: unary_math_operation = (out: i64) <- (in: i64) {
  out = i64.mul(in, in);
};

let sum_of_squares: fn = (res: i64) <- (op: unary_math_operation, x: i64) {
  res = op(x);
};
```

## Mathmatic operations
```
// let out = 5 + 6;
let out = i64.add(5, 6);
let out = i64.sub(5, 6);
let out = i64.mul(5, 6);
let out = i64.div(5, 6);
let out = i64.mod(5, 6);

```

## Thoughts

1. Types shall not be inferred but may be extrapolated.

1. No null values.

1. Primitive types:
- bool: true / false
- fn: function
- i64: 64 bit signed integer

1. First class functions

