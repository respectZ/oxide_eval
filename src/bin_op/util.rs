use serde_json::Number;

pub fn number_string<F>(l: &Number, r: &String, f: F) -> bool
where
    F: Fn(f64, f64) -> bool,
{
    if let Ok(r) = r.parse::<f64>() {
        return f(l.as_f64().unwrap_or_default(), r);
    }
    return false;
}

pub fn string_number<F>(l: &String, r: &Number, f: F) -> bool
where
    F: Fn(f64, f64) -> bool,
{
    if let Ok(l) = l.parse::<f64>() {
        f(l, r.as_f64().unwrap_or_default())
    } else {
        false
    }
}
