use serde_json::Map;

#[test]
fn test_basic() {
    use std::collections::HashMap;

    use oxide_eval::{context::ContextEntry, Evaluator};
    use serde_json::{Number, Value};

    let mut context = HashMap::new();
    context.insert(
        "a".to_string(),
        ContextEntry::Variable(Value::Number(24.into())),
    );
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
}

#[test]
fn test_lt() {
    use std::collections::HashMap;

    use oxide_eval::{context::ContextEntry, Evaluator};
    use serde_json::{Number, Value};

    let mut context = HashMap::new();
    context.insert(
        "a".to_string(),
        ContextEntry::Variable(Value::Number(24.into())),
    );
    context.insert(
        "mul".to_string(),
        ContextEntry::Function(Box::new(|args| {
            let a = args[0].as_f64().unwrap();
            Value::Number(Number::from_f64(a * 10.0).unwrap())
        })),
    );
    let evaluator = Evaluator::new(context);
    // let res1 = evaluator.evaluate("{} < [\"abacv\"]").unwrap();
    let res1 = evaluator.evaluate("[\"t\"] < [\"abacv\"]").unwrap();
    let res2 = evaluator.evaluate("[\"abacv\"] < [\"t\"]").unwrap();
    let res3 = evaluator.evaluate("[\"abacv\"] > {1:{2:3},4:5}").unwrap();

    assert_eq!(res1, false);
    assert_eq!(res2, true);
    assert_eq!(res3, true);
    // assert_eq!(res2, false);
}

#[test]
fn test_conditional() {
    use std::collections::HashMap;

    use oxide_eval::{context::ContextEntry, Evaluator};
    use serde_json::{Number, Value};

    let mut context = HashMap::new();
    context.insert(
        "a".to_string(),
        ContextEntry::Variable(Value::Number(24.into())),
    );
    context.insert(
        "mul".to_string(),
        ContextEntry::Function(Box::new(|args| {
            let a = args[0].as_f64().unwrap();
            Value::Number(Number::from_f64(a * 10.0).unwrap())
        })),
    );
    let evaluator = Evaluator::new(context);
    let res = evaluator.evaluate("a == 24 ? 1 : 4").unwrap();

    assert_eq!(res, 1)
}

#[test]
fn test_parenthesized() {
    use std::collections::HashMap;

    use oxide_eval::{context::ContextEntry, Evaluator};
    use serde_json::{Number, Value};

    let mut context = HashMap::new();
    context.insert(
        "a".to_string(),
        ContextEntry::Variable(Value::Number(24.into())),
    );
    context.insert(
        "mul".to_string(),
        ContextEntry::Function(Box::new(|args| {
            let a = args[0].as_f64().unwrap();
            Value::Number(Number::from_f64(a * 10.0).unwrap())
        })),
    );
    let evaluator = Evaluator::new(context);
    let res = evaluator.evaluate("(2 + 3) * a").unwrap();

    assert_eq!(res, 120);
}

#[test]
fn test_unary() {
    use std::collections::HashMap;

    use oxide_eval::{context::ContextEntry, Evaluator};
    use serde_json::{Number, Value};

    let mut context = HashMap::new();
    context.insert(
        "a".to_string(),
        ContextEntry::Variable(Value::Number(24.into())),
    );
    context.insert(
        "b".to_string(),
        ContextEntry::Variable(Value::String("24".to_string())),
    );
    context.insert(
        "mul".to_string(),
        ContextEntry::Function(Box::new(|args| {
            let a = args[0].as_f64().unwrap();
            Value::Number(Number::from_f64(a * 10.0).unwrap())
        })),
    );
    let evaluator = Evaluator::new(context);
    let res1 = evaluator.evaluate("+[2.4]").unwrap();
    let res2 = evaluator.evaluate("+[[\"2.5\"]]").unwrap();
    let res3 = evaluator.evaluate("+\"2.6\"").unwrap();
    let res4 = evaluator.evaluate("+a").unwrap();
    let res5 = evaluator.evaluate("+b").unwrap();
    let res6 = evaluator.evaluate("+true").unwrap();
    let res7 = evaluator.evaluate("+false").unwrap();

    let res8 = evaluator.evaluate("-[2]").unwrap();
    let res9 = evaluator.evaluate("-[[[[[[4]]]]]]").unwrap();
    let res10 = evaluator.evaluate("-[[[[[[\"4.5\"]]]]]]").unwrap();
    let res11 = evaluator.evaluate("-\"-1.2\"").unwrap();
    let res12 = evaluator.evaluate("-true").unwrap();
    let res13 = evaluator.evaluate("-false").unwrap();
    let res14 = evaluator.evaluate("-null").unwrap();

    let res15 = evaluator.evaluate("~[2]").unwrap();
    let res16 = evaluator.evaluate("~[[[[[[4]]]]]]").unwrap();
    let res17 = evaluator.evaluate("~[[[[[[\"4.5\"]]]]]]").unwrap();
    let res18 = evaluator.evaluate("~\"-1.2\"").unwrap();
    let res19 = evaluator.evaluate("~true").unwrap();
    let res20 = evaluator.evaluate("~false").unwrap();
    let res21 = evaluator.evaluate("~null").unwrap();
    let res22 = evaluator.evaluate("~\"4444.2\"").unwrap();
    let res23 = evaluator.evaluate("~26.5").unwrap();

    let res24 = evaluator.evaluate("!26.5").unwrap();

    assert_eq!(res1, 2.4);
    assert_eq!(res2, 2.5);
    assert_eq!(res3, 2.6);
    assert_eq!(res4, 24);
    assert_eq!(res5, 24);
    assert_eq!(res6, 1);
    assert_eq!(res7, 0);

    assert_eq!(res8, -2);
    assert_eq!(res9, -4);
    assert_eq!(res10, -4.5);
    assert_eq!(res11, 1.2);
    assert_eq!(res12, -1);
    assert_eq!(res13, 0);
    assert_eq!(res14, 0);

    assert_eq!(res15, -3);
    assert_eq!(res16, -5);
    assert_eq!(res17, -5);
    assert_eq!(res18, 0);
    assert_eq!(res19, -2);
    assert_eq!(res20, -1);
    assert_eq!(res21, -1);
    assert_eq!(res22, -4445);
    assert_eq!(res23, -27);

    assert_eq!(res24, false);
}

#[test]
fn test_chain() {
    use std::collections::HashMap;

    use oxide_eval::{context::ContextEntry, Evaluator};
    use serde_json::{Number, Value};

    let mut context = HashMap::new();
    let mut map = Map::new();
    let mut map2 = Map::new();
    map.insert("a".to_string(), Value::Number(32.into()));
    map2.insert("b".to_string(), Value::Object(map));
    context.insert("c".to_string(), ContextEntry::Variable(Value::Object(map2)));
    context.insert(
        "mul".to_string(),
        ContextEntry::Function(Box::new(|args| {
            let a = args[0].as_f64().unwrap();
            Value::Number(Number::from_f64(a * 10.0).unwrap())
        })),
    );
    let evaluator = Evaluator::new(context);
    let res1 = evaluator.evaluate("c.b.a").unwrap();
    let res2 = evaluator.evaluate("c.b?.d").unwrap();
    let res3 = evaluator.evaluate("settings?.ok").unwrap();

    assert_eq!(res1, 32);
    assert_eq!(res2, Value::Null);
    assert_eq!(res3, Value::Null);
}

#[test]
fn test_banana() {
    use std::collections::HashMap;

    use oxide_eval::Evaluator;

    let context = HashMap::new();
    let evaluator = Evaluator::new(context);
    let res = evaluator.evaluate("\"b\"+\"a\"+ +\"a\"+\"a\"").unwrap();

    assert_eq!(res, "baNaNa");
}
