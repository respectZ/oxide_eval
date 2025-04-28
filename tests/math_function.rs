#[cfg(test)]
#[test]
fn test_unary() {
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
    let res1 = evaluator.evaluate("floor('-12.3')").unwrap();
    let res2 = evaluator.evaluate("ceil(b)").unwrap();
    let res3 = evaluator.evaluate("round(b)").unwrap();
    let res4 = evaluator.evaluate("sin(90)").unwrap();
    let res5 = evaluator.evaluate("cos(90)").unwrap();
    let res6 = evaluator.evaluate("tan(90)").unwrap();
    let res7 = evaluator.evaluate("asin(1)").unwrap();
    let res8 = evaluator.evaluate("acos(0)").unwrap();
    let res9 = evaluator.evaluate("atan(1)").unwrap();
    let res10 = evaluator.evaluate("sqrt(64)").unwrap();
    let res11 = evaluator.evaluate("abs(b)").unwrap();
    let res12 = evaluator.evaluate("clamp(b)").unwrap();
    let res13 = evaluator.evaluate("bitwiseNot(b)").unwrap();

    assert_eq!(res1, -13);
    assert_eq!(res2, -12);
    assert_eq!(res3, -12);
    assert_eq!(res4, 0.8939966636005579);
    assert_eq!(res5, -0.4480736161291701);
    assert_eq!(res6, -1.995200412208242);
    assert_eq!(res7, 1.5707963267948966);
    assert_eq!(res8, 1.5707963267948966);
    assert_eq!(res9, 0.7853981633974483);
    assert_eq!(res10, 8);
    assert_eq!(res11, 12.3);
    assert_eq!(res12, 0);
    assert_eq!(res13, 11);
}

#[cfg(test)]
#[test]
fn test_binary() {
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
    let res1 = evaluator.evaluate("atan2(0.1, 0.2)").unwrap();
    let res2 = evaluator.evaluate("min(b, 12)").unwrap();
    let res3 = evaluator.evaluate("max(b, 12)").unwrap();
    let res4 = evaluator.evaluate("mod(5, 2)").unwrap();
    let res5 = evaluator.evaluate("pow(2, 4)").unwrap();
    let res6 = evaluator.evaluate("bitwiseAnd(2, 1)").unwrap();
    let res7 = evaluator.evaluate("bitwiseOr(4, 2)").unwrap();
    let res8 = evaluator.evaluate("bitwiseLeft(4, 1)").unwrap();
    let res9 = evaluator.evaluate("bitwiseRight(4, 1)").unwrap();

    assert_eq!(res1, 0.4636476090008061);
    assert_eq!(res2, -12.3);
    assert_eq!(res3, 12);
    assert_eq!(res4, 1);
    assert_eq!(res5, 16);
    assert_eq!(res6, 0);
    assert_eq!(res7, 6);
    assert_eq!(res8, 8);
    assert_eq!(res9, 2);
}
