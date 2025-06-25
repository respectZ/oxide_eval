#[derive(Debug)]
pub enum EvaluatorError {
    VariableNotFound(String),
    PropertyNotFound(String, String),
}

impl std::fmt::Display for EvaluatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluatorError::VariableNotFound(var) => write!(f, "Variable not found: {}", var),
            EvaluatorError::PropertyNotFound(obj, prop) => {
                write!(f, "Property '{}' not found in object '{}'", prop, obj)
            }
        }
    }
}
