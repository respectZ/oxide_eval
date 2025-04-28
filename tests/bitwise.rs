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
