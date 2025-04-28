use serde_json::Value;

type BoxFunction = Box<dyn Fn(Vec<Value>) -> Value>;
pub enum ContextEntry {
    Variable(Value),
    Function(BoxFunction),
}
