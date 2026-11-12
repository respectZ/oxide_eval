use oxide_eval::Evaluator;
use std::collections::HashMap;

#[test]
fn test_unsigned_right_shift() {
    let context = HashMap::new();
    let evaluator = Evaluator::new(context);
    let res1 = evaluator.evaluate("-5 >>> 1").unwrap();
    let res2 = evaluator.evaluate("\"-5\" >>> 1").unwrap();
    let res3 = evaluator.evaluate("[[\"-5\"]] >>> 1").unwrap();

    let result = 2147483645;
    assert_eq!(res1, result);
    assert_eq!(res2, result);
    assert_eq!(res3, result);
}

#[test]
fn bitwise_operators_wrap_to_32_bits() {
    let evaluator = Evaluator::new(Default::default());
    for (expression, expected) in [
        ("4294967296 | 0", 0_i64),
        ("2147483648 | 0", -2147483648),
        ("~4294967296", -1),
        ("-1 >>> 0", 4294967295),
        ("1 << 4294967296", 1),
        ("1 << -1", -2147483648),
    ] {
        assert_eq!(evaluator.evaluate(expression).unwrap(), expected, "{expression}");
    }
}
