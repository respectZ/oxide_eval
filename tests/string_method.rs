#[cfg(test)]
#[test]
fn test_method() {
    use std::collections::HashMap;

    use oxide_eval::{context::ContextEntry, Evaluator};
    use serde_json::{json, Number, Value};

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
    let res1 = evaluator.evaluate("a.replace('Hello', 'asdfvc')").unwrap();
    let res2 = evaluator.evaluate("a.contains('Hello')").unwrap();
    let res3 = evaluator.evaluate("a.split(' ')").unwrap();
    let res4 = evaluator.evaluate("a.indexOf('o')").unwrap();
    let res5 = evaluator.evaluate("a.lastIndexOf('l')").unwrap();
    let res6 = evaluator.evaluate("a.toUpperCase()").unwrap();
    let res7 = evaluator.evaluate("a.toLowerCase()").unwrap();
    let res8 = evaluator.evaluate("a.substring(3, 5)").unwrap();
    let res9 = evaluator.evaluate("a.startsWith('Hello')").unwrap();
    let res10 = evaluator.evaluate("a.endsWith('!')").unwrap();
    let res11 = evaluator.evaluate("a.regexReplace('[a-z]', 'L')").unwrap();
    let res12 = evaluator.evaluate("a.length()").unwrap();
    let res13 = evaluator.evaluate("(a + '   ').trim()").unwrap();

    assert_eq!(res1, "asdfvc World!");
    assert_eq!(res2, true);
    assert_eq!(res3, json!(["Hello", "World!"]));
    assert_eq!(res4, 4);
    assert_eq!(res5, 9);
    assert_eq!(res6, "HELLO WORLD!");
    assert_eq!(res7, "hello world!");
    assert_eq!(res8, "lo");
    assert_eq!(res9, true);
    assert_eq!(res10, true);
    assert_eq!(res11, "HLLLL WLLLL!");
    assert_eq!(res12, 12);
    assert_eq!(res13, "Hello World!");
}

#[test]
fn test_function() {
    use std::collections::HashMap;

    use oxide_eval::{context::ContextEntry, Evaluator};
    use serde_json::{json, Number, Value};

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
    let res1 = evaluator.evaluate("replace(a, 'Hello', 'asdfvc')").unwrap();
    let res2 = evaluator.evaluate("contains(a, 'Hello')").unwrap();
    let res3 = evaluator.evaluate("split(a, ' ')").unwrap();
    let res4 = evaluator.evaluate("indexOf(a, 'o')").unwrap();
    let res5 = evaluator.evaluate("lastIndexOf(a, 'l')").unwrap();
    let res6 = evaluator.evaluate("toUpperCase(a, )").unwrap();
    let res7 = evaluator.evaluate("toLowerCase(a, )").unwrap();
    let res8 = evaluator.evaluate("substring(a, 3, 5)").unwrap();
    let res9 = evaluator.evaluate("startsWith(a, 'Hello')").unwrap();
    let res10 = evaluator.evaluate("endsWith(a, '!')").unwrap();
    let res11 = evaluator.evaluate("regexReplace(a, '[a-z]', 'L')").unwrap();
    let res12 = evaluator.evaluate("length(a, )").unwrap();
    let res13 = evaluator.evaluate("trim(a + '    ')").unwrap();

    assert_eq!(res1, "asdfvc World!");
    assert_eq!(res2, true);
    assert_eq!(res3, json!(["Hello", "World!"]));
    assert_eq!(res4, 4);
    assert_eq!(res5, 9);
    assert_eq!(res6, "HELLO WORLD!");
    assert_eq!(res7, "hello world!");
    assert_eq!(res8, "lo");
    assert_eq!(res9, true);
    assert_eq!(res10, true);
    assert_eq!(res11, "HLLLL WLLLL!");
    assert_eq!(res12, 12);
    assert_eq!(res13, "Hello World!");
}
