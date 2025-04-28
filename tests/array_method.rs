#[cfg(test)]
#[test]
fn test_method() {
    use std::collections::HashMap;

    use oxide_eval::{context::ContextEntry, Evaluator};
    use serde_json::{Number, Value};

    let mut context = HashMap::new();
    context.insert(
        "a".to_string(),
        ContextEntry::Variable(Value::String("Hello World!".into())),
    );
    context.insert(
        "mul".to_string(),
        ContextEntry::Function(Box::new(|args| {
            let a = args[0].as_f64().unwrap();
            Value::Number(Number::from_f64(a * 10.0).unwrap())
        })),
    );
    let evaluator = Evaluator::new(context);
    let res1 = evaluator.evaluate("['a','b','c'].join(' ')").unwrap();
    let res2 = evaluator.evaluate("['1','2','3'].join('-')").unwrap();

    assert_eq!(res1, "a b c");
    assert_eq!(res2, "1-2-3");
}
