# oxide_eval

## Description

oxide_eval is a simple Javascript Evaluator built on top of [oxc](!https://github.com/oxc-project/oxc)

## Example

```rust
use std::collections::HashMap;
use oxide_eval::{context::ContextEntry, Evaluator};
use serde_json::{Number, Value};

let mut context = HashMap::new();
// Map an variable
context.insert(
    "a".to_string(),
    ContextEntry::Variable(Value::Number(24.into())),
);
// Map a function
context.insert(
    "mul".to_string(),
    ContextEntry::Function(Box::new(|args| {
        let a = args[0].as_f64().unwrap();
        Value::Number(Number::from_f64(a * 10.0).unwrap())
    })),
);
let evaluator = Evaluator::new(context);
let res = evaluator.evaluate("a + mul(2)").unwrap();
assert_eq!(res, 44);
```
