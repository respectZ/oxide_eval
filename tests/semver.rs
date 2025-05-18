#[cfg(test)]
#[test]
fn test_semver() {
    use std::collections::HashMap;

    use oxide_eval::{context::ContextEntry, Evaluator};
    use serde_json::{Number, Value};

    let mut context = HashMap::new();
    context.insert(
        "a".to_string(),
        ContextEntry::Variable(Value::String("Hello World!".into())),
    );
    context.insert(
        "b".to_string(),
        ContextEntry::Variable(Value::Number(Number::from_f64(-12.3).unwrap())),
    );
    context.insert(
        "mul".to_string(),
        ContextEntry::Function(Box::new(|args| {
            let a = args[0].as_f64().unwrap();
            Value::Number(Number::from_f64(a * 10.0).unwrap())
        })),
    );
    let evaluator = Evaluator::new(context);
    assert_eq!(
        evaluator
            .evaluate("semver('1.0.0') > semver('0.0.2')")
            .unwrap(),
        true
    );
    assert_eq!(
        evaluator
            .evaluate("semver('0.0.2') > semver('0.0.2')")
            .unwrap(),
        false
    );
    assert_eq!(
        evaluator
            .evaluate("semver(1,0,0) > semver('0.0.2')")
            .unwrap(),
        true
    );
    assert_eq!(
        evaluator
            .evaluate("semver(1,0,0) === semver('1.0.0')")
            .unwrap(),
        true
    );
    assert_eq!(
        evaluator
            .evaluate("semver([1,'0',[1]]) >= semver('1.0.0')")
            .unwrap(),
        true
    );
}
