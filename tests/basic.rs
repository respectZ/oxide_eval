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

#[test]
fn test_directive() {
    use oxide_eval::Evaluator;

    let evaluator = Evaluator::new(std::collections::HashMap::new());
    let res = evaluator.evaluate("'directive'").unwrap();
    assert_eq!(res, "directive");
}

#[test]
fn invalid_array_elements_and_arguments_return_errors() {
    let evaluator = oxide_eval::Evaluator::new(Default::default());
    for expression in [
        "[missing]",
        "call(missing)",
        "[,]",
        "[...[1, 2]]",
        "call(...[1, 2])",
    ] {
        assert!(evaluator.evaluate(expression).is_err(), "{expression}");
    }
}

#[test]
fn logical_operators_short_circuit_and_return_operands() {
    let evaluator = oxide_eval::Evaluator::new(Default::default());
    for (expression, expected) in [
        ("false && missing", serde_json::json!(false)),
        ("true || missing", serde_json::json!(true)),
        ("1 ?? missing", serde_json::json!(1)),
        ("2 && 3", serde_json::json!(3)),
        ("0 && 3", serde_json::json!(0)),
        ("0 || 3", serde_json::json!(3)),
        ("null ?? 3", serde_json::json!(3)),
        ("[] ? 1 : 2", serde_json::json!(1)),
        ("![]", serde_json::json!(false)),
    ] {
        assert_eq!(evaluator.evaluate(expression).unwrap(), expected, "{expression}");
    }
    assert!(evaluator.evaluate("true && missing").is_err());
}

#[test]
fn relational_operators_compare_numbers_numerically() {
    let evaluator = oxide_eval::Evaluator::new(Default::default());
    for expression in [
        "2 < 10",
        "10 > 2",
        "'2' < 10",
        "true < 2",
        "null <= 0",
        "10 >= 10",
        "'2' > '10'",
        "'😀' < '\\uE000'",
    ] {
        assert_eq!(evaluator.evaluate(expression).unwrap(), true, "{expression}");
    }
    assert_eq!(evaluator.evaluate("2 > 10").unwrap(), false);
}

#[test]
fn object_keys_are_property_names() {
    let evaluator = oxide_eval::Evaluator::new(Default::default());
    for expression in ["({foo: 1}).foo", "({'foo': 1}).foo", "({['f' + 'oo']: 1}).foo"] {
        assert_eq!(evaluator.evaluate(expression).unwrap(), 1, "{expression}");
    }
    assert_eq!(evaluator.evaluate("({[1 + 1]: 3})").unwrap(), serde_json::json!({"2": 3}));
}

#[test]
fn member_access_reads_properties_instead_of_returning_receivers() {
    let evaluator = oxide_eval::Evaluator::new(Default::default());
    assert_eq!(evaluator.evaluate("'é😀'.length").unwrap(), 3);
    assert_eq!(evaluator.evaluate("[1, 2].length").unwrap(), 2);
    assert_eq!(evaluator.evaluate("null?.foo").unwrap(), serde_json::Value::Null);
    for expression in ["(42).foo", "null.foo", "'abc'.unknown"] {
        assert!(evaluator.evaluate(expression).is_err(), "{expression}");
    }
}

#[test]
fn large_numbers_are_not_clamped_to_i64() {
    let evaluator = oxide_eval::Evaluator::new(Default::default());
    for (expression, expected) in [
        ("9223372036854775808", 9223372036854775808.0),
        ("1e30", 1e30),
        ("-1e30", -1e30),
        ("1e20 * 1e10", 1e30),
    ] {
        assert_eq!(evaluator.evaluate(expression).unwrap(), serde_json::json!(expected), "{expression}");
    }
}

#[test]
fn loose_equality_coerces_primitives_but_strict_equality_does_not() {
    let evaluator = oxide_eval::Evaluator::new(Default::default());
    for (expression, expected) in [
        ("true == 1", true),
        ("'' == 0", true),
        ("0 == '  '", true),
        ("[] == false", true),
        ("false == [0]", true),
        ("'garbage' == 0", false),
        ("null == 0", false),
        ("true === 1", false),
        ("[] === false", false),
    ] {
        assert_eq!(evaluator.evaluate(expression).unwrap(), expected, "{expression}");
    }
}

#[test]
fn invalid_numeric_coercions_do_not_become_zero() {
    let evaluator = oxide_eval::Evaluator::new(Default::default());
    for expression in ["'garbage' * 2", "[1, 2] - 0", "[true] * 2"] {
        assert!(evaluator.evaluate(expression).is_err(), "{expression}");
    }
    assert_eq!(evaluator.evaluate("'garbage' < 1").unwrap(), false);
    assert_eq!(evaluator.evaluate("[null] * 2").unwrap(), 0);
    assert_eq!(evaluator.evaluate("' 3 ' * 2").unwrap(), 6);
}

#[test]
fn zero_dividend_has_zero_remainder() {
    let evaluator = oxide_eval::Evaluator::new(Default::default());
    assert_eq!(evaluator.evaluate("0 % 3").unwrap(), 0);
    assert_eq!(evaluator.evaluate("5 % 2").unwrap(), 1);
    assert!(evaluator.evaluate("0 % 0").is_err());
}

#[test]
fn standalone_strings_decode_escapes() {
    let evaluator = oxide_eval::Evaluator::new(Default::default());
    assert_eq!(
        evaluator.evaluate(r#"'a\n\u0062\\c'"#).unwrap(),
        "a\nb\\c"
    );
}
