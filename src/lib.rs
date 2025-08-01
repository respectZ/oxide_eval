mod bin_op;
pub mod context;
mod error;
#[cfg(feature = "math")]
mod math;
#[cfg(any(feature = "string", feature = "array"))]
mod method;
#[cfg(feature = "semver-support")]
mod semver_wrapper;
mod unary;
mod util;

use anyhow::{anyhow, bail, Result};
use bin_op::{
    addition, bitwise_operation, compare, division, equality, exponential, multiplication,
    remainder, subtraction, unsigned_right_shift,
};
use context::ContextEntry;
use oxc::{
    allocator::Allocator,
    ast::ast::{
        ArrayExpression, BinaryExpression, BinaryOperator, CallExpression, ChainElement,
        ChainExpression, ConditionalExpression, Expression, LogicalExpression, LogicalOperator,
        ObjectExpression, ObjectPropertyKind, Statement, StaticMemberExpression, UnaryExpression,
        UnaryOperator,
    },
    parser::Parser,
    span::SourceType,
};
use serde_json::{to_string, Map, Value};
use std::collections::HashMap;
use unary::{unary_bitwise_not, unary_negation, unary_plus};
use util::number_from_f64;

use crate::error::EvaluatorError;

pub struct Evaluator {
    context: HashMap<String, ContextEntry>,
}

impl Evaluator {
    pub fn new(context: HashMap<String, ContextEntry>) -> Self {
        Self { context }
    }

    pub fn evaluate(&self, expression: &str) -> Result<Value> {
        let allocator = Allocator::default();
        let parser = Parser::new(&allocator, expression, SourceType::cjs());
        let parsed = parser.parse();
        if parsed.errors.len() > 0 {
            let errors = parsed
                .errors
                .iter()
                .map(|d| d.message.to_string())
                .collect::<String>();
            bail!("Parsing error: {:?}", errors)
        }
        let program = parsed.program;
        let stmt = &program.body.first();
        match stmt {
            Some(Statement::ExpressionStatement(expr)) => self.evaluate_expr(&expr.expression),
            Some(stmt) => bail!("Unsupported statement: {:?}", stmt),
            None => {
                let directive = &program.directives.first();
                if let Some(directive) = directive {
                    return Ok(Value::String(directive.directive.into_string()));
                }
                bail!("No statements found")
            }
        }
    }

    fn evaluate_expr(&self, expr: &Expression) -> Result<Value> {
        match expr {
            Expression::BooleanLiteral(expr) => Ok(Value::Bool(expr.value)),
            Expression::NullLiteral(_) => Ok(Value::Null),
            Expression::NumericLiteral(expr) => Ok(Value::Number(number_from_f64(expr.value)?)),
            // Expression::BigIntLiteral()
            // Expression::RegExpLiteral()
            Expression::StringLiteral(expr) => Ok(Value::String(expr.value.into_string())),
            // Expression::TemplateLiteral(expr) => Ok(Value::String(expr.quasis)),
            Expression::Identifier(expr) => self.evaluate_by_name(&expr.name),
            // Expression::MetaProperty()
            // Expression::Super()
            Expression::ArrayExpression(expr) => self.evaluate_array(expr),
            // Expression::ArrowFunctionExpression()
            // Expression::AssignmentExpression()
            // Expression::AwaitExpression()
            Expression::BinaryExpression(expr) => self.evaluate_binary(expr),
            Expression::CallExpression(expr) => self.evaluate_call(expr),
            Expression::ChainExpression(expr) => self.evaluate_chain(expr),
            // Expression::ClassExpression()
            Expression::ConditionalExpression(expr) => self.evaluate_conditional(expr),
            // Expression::FunctionExpression()
            // Expression::ImportExpression()
            Expression::LogicalExpression(expr) => self.evaluate_logical(expr),
            // Expression::NewExpression()
            Expression::ObjectExpression(expr) => self.evaluate_object(expr),
            Expression::ParenthesizedExpression(expr) => self.evaluate_expr(&expr.expression),
            // Expression::SequenceExpression()
            Expression::StaticMemberExpression(expr) => self.evaluate_static_member(expr),
            // Expression::TaggedTemplateExpression()
            // Expression::ThisExpression()
            Expression::UnaryExpression(expr) => self.evaluate_unary(expr),
            // Expression::UpdateExpression()
            // Expression::YieldExpression()
            // Expression::PrivateInExpression()
            _ => bail!("Unsupported expression: {:?}", expr),
        }
    }

    fn evaluate_array(&self, expr: &ArrayExpression) -> Result<Value> {
        let result = expr
            .elements
            .iter()
            .map(|f| self.evaluate_expr(f.to_expression()).unwrap())
            .collect::<Vec<Value>>();
        Ok(Value::Array(result))
    }
    fn evaluate_binary(&self, expr: &BinaryExpression) -> Result<Value> {
        let left = self.evaluate_expr(&expr.left)?;
        let right = self.evaluate_expr(&expr.right)?;

        #[cfg(feature = "semver-support")]
        {
            use semver_wrapper::SemverWrapper;
            use serde::Deserialize;
            if let (Ok(left), Ok(right)) = (
                SemverWrapper::deserialize(&left),
                SemverWrapper::deserialize(&right),
            ) {
                let cmp = left.version.cmp_precedence(&right.version);
                let result = match expr.operator {
                    BinaryOperator::Equality | BinaryOperator::StrictEquality => cmp.is_eq(),
                    BinaryOperator::Inequality | BinaryOperator::StrictInequality => !cmp.is_eq(),
                    BinaryOperator::LessThan => cmp.is_lt(),
                    BinaryOperator::LessEqualThan => cmp.is_le(),
                    BinaryOperator::GreaterThan => cmp.is_gt(),
                    BinaryOperator::GreaterEqualThan => cmp.is_gt(),
                    _ => bail!(
                        "Unsupported binary operator for semver: {:?}",
                        expr.operator
                    ),
                };
                return Ok(Value::Bool(result));
            }
        }

        match expr.operator {
            BinaryOperator::Equality => Ok(Value::Bool(equality(&left, &right, false))),
            BinaryOperator::Inequality => Ok(Value::Bool(!equality(&left, &right, false))),
            BinaryOperator::StrictEquality => Ok(Value::Bool(equality(&left, &right, true))),
            BinaryOperator::StrictInequality => Ok(Value::Bool(!equality(&left, &right, true))),
            BinaryOperator::LessThan => Ok(Value::Bool(compare(&left, &right, |l, r| l < r))),
            BinaryOperator::LessEqualThan => Ok(Value::Bool(compare(&left, &right, |l, r| l <= r))),
            BinaryOperator::GreaterThan => Ok(Value::Bool(compare(&left, &right, |l, r| l > r))),
            BinaryOperator::GreaterEqualThan => {
                Ok(Value::Bool(compare(&left, &right, |l, r| l >= r)))
            }
            BinaryOperator::Addition => addition(left, right),
            BinaryOperator::Subtraction => subtraction(left, right),
            BinaryOperator::Multiplication => multiplication(left, right),
            BinaryOperator::Division => division(left, right),
            BinaryOperator::Remainder => remainder(left, right),
            BinaryOperator::Exponential => exponential(left, right),
            BinaryOperator::ShiftLeft => bitwise_operation(left, right, |l, r| l << (r & 0x1F)),
            BinaryOperator::ShiftRight => bitwise_operation(left, right, |l, r| l >> (r & 0x1F)),
            BinaryOperator::ShiftRightZeroFill => unsigned_right_shift(left, right),
            BinaryOperator::BitwiseOR => bitwise_operation(left, right, |l, r| l | r),
            BinaryOperator::BitwiseXOR => bitwise_operation(left, right, |l, r| l ^ r),
            BinaryOperator::BitwiseAnd => bitwise_operation(left, right, |l, r| l & r),
            _ => bail!("Unsupported binary operator: {:?}", expr.operator),
        }
    }
    fn evaluate_call(&self, expr: &CallExpression) -> Result<Value> {
        let args = expr
            .arguments
            .iter()
            .map(|f| self.evaluate_expr(f.to_expression()).unwrap())
            .collect::<Vec<Value>>();
        match &expr.callee {
            Expression::Identifier(expr) => {
                let callee_name = expr.name.to_string();

                #[cfg(feature = "semver-support")]
                {
                    use semver_wrapper::SemverWrapper;
                    if callee_name == "semver" {
                        return Ok(serde_json::json!(SemverWrapper::from_values(args)?));
                    }
                }

                match self.context.get(&callee_name) {
                    Some(ContextEntry::Function(f)) => Ok(f(args)),
                    _ => {
                        #[cfg(feature = "string")]
                        {
                            if let Some(Value::String(callee)) = args.get(0) {
                                return match Evaluator::evaluate_str_method(
                                    callee,
                                    &callee_name,
                                    args[1..].to_vec(),
                                ) {
                                    Ok(value) => Ok(value),
                                    _ => {
                                        #[cfg(feature = "math")]
                                        {
                                            if let Ok(result) = Evaluator::evaluate_math_function(
                                                &callee_name,
                                                args,
                                            ) {
                                                return Ok(result);
                                            }
                                        }
                                        bail!("{:?} not found in function context", callee_name)
                                    }
                                };
                            }
                        }
                        #[cfg(feature = "array")]
                        {
                            if let Some(Value::Array(callee)) = args.get(0) {
                                return match Evaluator::evaluate_array_method(
                                    callee,
                                    &callee_name,
                                    args[1..].to_vec(),
                                ) {
                                    Ok(value) => Ok(value),
                                    _ => bail!("{:?} not found in function context", callee_name),
                                };
                            }
                        }
                        #[cfg(feature = "math")]
                        {
                            if let Some(Value::Number(_)) = args.get(0) {
                                return Evaluator::evaluate_math_function(&callee_name, args);
                            }
                        }
                        bail!("{:?} not found in function context", callee_name)
                    }
                }
            }
            _ => {
                let callee = self.evaluate_expr(&expr.callee)?;

                if let Value::String(callee) = &callee {
                    #[cfg(feature = "string")]
                    {
                        let callee_name = expr.callee_name().unwrap_or_default();
                        return Evaluator::evaluate_str_method(callee, callee_name, args);
                    }
                    #[cfg(not(feature = "string"))]
                    bail!("'string' feature is not enabled. callee: {:?}", callee)
                } else if let Value::Array(callee) = &callee {
                    #[cfg(feature = "array")]
                    {
                        let callee_name = expr.callee_name().unwrap_or_default();
                        return Evaluator::evaluate_array_method(callee, callee_name, args);
                    }
                    #[cfg(not(feature = "array"))]
                    bail!("'array' feature is not enabled. callee: {:?}", callee)
                }

                bail!("Unsupported method for {:?}", callee);
            }
        }
    }
    fn evaluate_chain(&self, expr: &ChainExpression) -> Result<Value> {
        let ex = &expr.expression;
        match ex {
            ChainElement::CallExpression(expr) => self.evaluate_call(expr),
            ChainElement::StaticMemberExpression(expr) => self.evaluate_static_member(expr),
            _ => bail!("Unsupported ChainExpression: {:?}", ex),
        }
    }
    fn evaluate_conditional(&self, expr: &ConditionalExpression) -> Result<Value> {
        let test = self.evaluate_value(&self.evaluate_expr(&expr.test)?);
        let expr = match test {
            true => &expr.consequent,
            false => &expr.alternate,
        };
        self.evaluate_expr(expr)
    }
    fn evaluate_logical(&self, expr: &LogicalExpression) -> Result<Value> {
        let operator = expr.operator;
        let left = &self.evaluate_expr(&expr.left)?;
        let right = &self.evaluate_expr(&expr.right)?;
        match operator {
            LogicalOperator::And => Ok(Value::Bool(
                self.evaluate_value(left) && self.evaluate_value(right),
            )),
            LogicalOperator::Coalesce => match left {
                Value::Null => Ok(right.clone()),
                _ => Ok(left.clone()),
            },
            LogicalOperator::Or => match self.evaluate_value(left) {
                true => Ok(left.clone()),
                false => Ok(right.clone()),
            },
        }
    }
    fn evaluate_object(&self, expr: &ObjectExpression) -> Result<Value> {
        let properties = &expr.properties;
        let mut map = Map::new();
        for property in properties {
            match property {
                ObjectPropertyKind::ObjectProperty(object_property) => {
                    let key = match object_property.key.as_expression() {
                        Some(v) => v,
                        _ => bail!("Object key is not an expression"),
                    };
                    let key = self.evaluate_expr(key)?;
                    let key = to_string(&key)?;
                    let value = self.evaluate_expr(&object_property.value)?;
                    map.insert(key, value);
                }
                _ => bail!("Unsupported ObjectPropertyKind: {:?}", property),
            }
        }
        Ok(Value::Object(map))
    }
    fn evaluate_static_member(&self, expr: &StaticMemberExpression) -> Result<Value> {
        let obj = if expr.optional {
            match self.evaluate_expr(&expr.object) {
                Ok(value) => value,
                Err(e) => {
                    if e.downcast_ref::<EvaluatorError>().map_or(false, |err| {
                        matches!(err, EvaluatorError::VariableNotFound(_))
                    }) {
                        return Ok(Value::Null);
                    }
                    return Err(e);
                }
            }
        } else {
            self.evaluate_expr(&expr.object)?
        };
        let property = expr.property.name.to_string();
        match &obj {
            Value::Object(map) => {
                let value = map.get(&property);
                if let Some(value) = value {
                    return Ok(value.clone());
                }
                if expr.optional {
                    return Ok(Value::Null);
                }
                Err(anyhow!(EvaluatorError::PropertyNotFound(
                    obj.to_string(),
                    property.clone(),
                )))
            }
            _ => Ok(obj),
        }
    }
    fn evaluate_unary(&self, expr: &UnaryExpression) -> Result<Value> {
        let operator = expr.operator;
        let value = self.evaluate_expr(&expr.argument)?;
        match operator {
            UnaryOperator::UnaryPlus => unary_plus(value),
            UnaryOperator::UnaryNegation => unary_negation(value),
            UnaryOperator::BitwiseNot => unary_bitwise_not(value),
            UnaryOperator::LogicalNot => Ok(Value::Bool(!self.evaluate_value(&value))),
            _ => {
                bail!("Unsupported UnaryOperator {:?}", operator)
            }
        }
    }

    fn evaluate_by_name(&self, name: &str) -> Result<Value> {
        match self.context.get(name) {
            Some(ContextEntry::Variable(value)) => Ok(value.clone()),
            _ => Err(anyhow!(EvaluatorError::VariableNotFound(name.to_string()))),
        }
    }
    fn evaluate_value(&self, value: &Value) -> bool {
        match value {
            Value::Array(arr) => !arr.is_empty(),
            Value::Bool(bool) => *bool,
            Value::Null => false,
            Value::Number(number) => number.as_f64().is_some_and(|value| value != 0.0),
            Value::Object(_) => true,
            Value::String(str) => !str.is_empty(),
        }
    }
    #[cfg(feature = "string")]
    fn evaluate_str_method(callee: &str, callee_name: &str, args: Vec<Value>) -> Result<Value> {
        use method::string::StringMethod;

        let str_method = StringMethod::new(args);
        match callee_name {
            "replace" => str_method.replace(callee),
            "contains" => str_method.contains(callee),
            "split" => str_method.split(callee),
            "indexOf" => str_method.index_of(callee),
            "lastIndexOf" => str_method.last_index_of(callee),
            "toUpperCase" => str_method.to_upper_case(callee),
            "toLowerCase" => str_method.to_lower_case(callee),
            "substring" => str_method.substring(callee),
            "startsWith" => str_method.starts_with(callee),
            "endsWith" => str_method.ends_with(callee),
            "regexReplace" => str_method.regex_replace(callee),
            "length" => str_method.length(callee),
            "trim" => str_method.trim(callee),
            _ => {
                bail!("Unknown string method: {}", callee_name);
            }
        }
    }
    #[cfg(feature = "array")]
    fn evaluate_array_method(
        callee: &Vec<Value>,
        callee_name: &str,
        args: Vec<Value>,
    ) -> Result<Value> {
        use method::array::ArrayMethod;

        let array_method = ArrayMethod::new(args);
        match callee_name {
            "join" => array_method.join(callee),
            _ => bail!("Unknown array method: {}", callee_name),
        }
    }
    #[cfg(feature = "math")]
    fn evaluate_math_function(callee_name: &str, mut args: Vec<Value>) -> Result<Value> {
        let value = args.remove(0).take();
        match args.len() {
            1 => match callee_name {
                "floor" => math::unary_function(value, f64::floor),
                "ceil" => math::unary_function(value, f64::ceil),
                "round" => math::unary_function(value, f64::round),
                "sin" => math::unary_function(value, f64::sin),
                "cos" => math::unary_function(value, f64::cos),
                "tan" => math::unary_function(value, f64::tan),
                "asin" => math::unary_function(value, f64::asin),
                "acos" => math::unary_function(value, f64::acos),
                "atan" => math::unary_function(value, f64::atan),
                "sqrt" => math::unary_function(value, f64::sqrt),
                "abs" => math::unary_function(value, f64::abs),
                "clamp" => math::unary_function(value, |x| x.clamp(0.0, 1.0)),
                "bitwiseNot" => unary_bitwise_not(value),
                _ => bail!("{:?} not found in function context", callee_name),
            },
            2 => {
                let second = args.remove(0).take();
                match callee_name {
                    "atan2" => math::binary_function(value, second, f64::atan2),
                    "min" => math::binary_function(value, second, f64::min),
                    "max" => math::binary_function(value, second, f64::max),
                    "mod" => remainder(value, second),
                    "pow" => math::binary_function(value, second, f64::powf),
                    "bitwiseAnd" => bitwise_operation(value, second, |l, r| l & r),
                    "bitwiseOr" => bitwise_operation(value, second, |l, r| l | r),
                    "bitwiseLeft" => bitwise_operation(value, second, |l, r| l << (r & 0x1F)),
                    "bitwiseRight" => bitwise_operation(value, second, |l, r| l >> (r & 0x1F)),
                    _ => bail!("{:?} not found in function context", callee_name),
                }
            }
            _ => bail!("{:?} not found in function context", callee_name),
        }
    }
}
