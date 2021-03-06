//!
//! ## Features
//!
//! Supported binary operators:
//!
//! | Operator | Description |
//! |----------|-------------|
//! | + | Sum |
//! | - | Difference |
//! | * | Product |
//! | / | Division |
//! | % | Modulo |
//! | < | Lower than |
//! | > | Greater than |
//! | <= | Lower than or equal |
//! | >= | Greater than or equal |
//! | == | Equal |
//! | != | Not equal |
//! | && | Logical and |
//! | || | Logical or |
//! 
//!Supported binary operators: `!` `!=` `""` `''` `()` `[]` `,` `>` `<` `>=` `<=` `==`
//!`+` unary/binary `-` `*` `/` `%` `&&` `||` `n..m`.
//!
//!Supported unary operators: ``
//!
//!Built-in functions: `min()` `max()` `len()` `is_empty()` `array()` `converge()`.
//!See the `builtin` module for a detailed description of each.
//!
//!Where can eval be used?
//!-----------------------
//!
//!* Template engine
//!* Scripting language
//!* ...
//!
//!Usage
//!-----
//!
//!Add dependency to Cargo.toml
//!
//!```toml
//![dependencies]
//!evalexpr = "0.4"
//!```
//!
//!In your `main.rs` or `lib.rs`:
//!
//!```rust
//!extern crate evalexpr as eval;
//!```
//!
//!Examples
//!--------
//!
//!You can do mathematical calculations with supported operators:
//!
//!```rust
//!use eval::{eval, to_value};
//!
//!assert_eq!(eval("1 + 2 + 3"), Ok(to_value(6)));
//!assert_eq!(eval("2 * 2 + 3"), Ok(to_value(7)));
//!assert_eq!(eval("2 / 2 + 3"), Ok(to_value(4.0)));
//!assert_eq!(eval("2 / 2 + 3 / 3"), Ok(to_value(2.0)));
//!```
//!
//!You can eval with context:
//!
//!```rust
//!use eval::{Expr, to_value};
//!
//!assert_eq!(Expr::new("foo == bar")
//!               .value("foo", true)
//!               .value("bar", true)
//!               .exec(),
//!           Ok(to_value(true)));
//!```
//!
//!You can access data like javascript by using `.` and `[]`. `[]` supports expression.
//!
//!```rust
//!use eval::{Expr, to_value};
//!use std::collections::HashMap;
//!
//!let mut object = HashMap::new();
//!object.insert("foos", vec!["Hello", "world", "!"]);
//!
//!assert_eq!(Expr::new("object.foos[1-1] == 'Hello'")
//!               .value("object", object)
//!               .exec(),
//!           Ok(to_value(true)));
//!```
//!
//!You can eval with function:
//!
//!```rust
//!use eval::{Expr, to_value};
//!
//!assert_eq!(Expr::new("say_hello()")
//!               .function("say_hello", |_| Ok(to_value("Hello world!")))
//!               .exec(),
//!           Ok(to_value("Hello world!")));
//!```
//!
//!You can create an array with `array()`:
//!
//!```rust
//!use eval::{eval, to_value};
//!
//!assert_eq!(eval("array(1, 2, 3, 4, 5)"), Ok(to_value(vec![1, 2, 3, 4, 5])));
//!```
//!
//!You can create an integer array with `n..m`:
//!
//!```rust
//!use eval::{eval, to_value};
//!
//!assert_eq!(eval("0..5"), Ok(to_value(vec![0, 1, 2, 3, 4])));
//!```
//!
//!License
//!-------
//!
//!evalexpr is primarily distributed under the terms of the MIT license.
//!See [LICENSE](LICENSE) for details. 
//!

mod configuration;
mod error;
mod function;
mod operator;
mod token;
mod tree;
mod value;

// Exports

pub use configuration::{Configuration, EmptyConfiguration, HashMapConfiguration};
pub use error::Error;
pub use function::Function;
pub use tree::Node;
pub use value::Value;

pub fn eval(string: &str) -> Result<Value, Error> {
    tree::tokens_to_operator_tree(token::tokenize(string)?)?.eval(&EmptyConfiguration)
}

pub fn eval_with_configuration(
    string: &str,
    configuration: &Configuration,
) -> Result<Value, Error> {
    tree::tokens_to_operator_tree(token::tokenize(string)?)?.eval(configuration)
}

pub fn build_operator_tree(string: &str) -> Result<Node, Error> {
    tree::tokens_to_operator_tree(token::tokenize(string)?)
}

#[cfg(test)]
mod test {
    use crate::{eval, value::Value};
    use configuration::HashMapConfiguration;
    use error::Error;
    use eval_with_configuration;
    use Function;

    #[test]
    fn test_unary_examples() {
        assert_eq!(eval("3"), Ok(Value::Int(3)));
        assert_eq!(eval("3.3"), Ok(Value::Float(3.3)));
        assert_eq!(eval("true"), Ok(Value::Boolean(true)));
        assert_eq!(eval("false"), Ok(Value::Boolean(false)));
        assert_eq!(
            eval("blub"),
            Err(Error::VariableIdentifierNotFound("blub".to_string()))
        );
        assert_eq!(eval("-3"), Ok(Value::Int(-3)));
        assert_eq!(eval("-3.6"), Ok(Value::Float(-3.6)));
        assert_eq!(eval("----3"), Ok(Value::Int(3)));
    }

    #[test]
    fn test_binary_examples() {
        assert_eq!(eval("1+3"), Ok(Value::Int(4)));
        assert_eq!(eval("3+1"), Ok(Value::Int(4)));
        assert_eq!(eval("3-5"), Ok(Value::Int(-2)));
        assert_eq!(eval("5-3"), Ok(Value::Int(2)));
        assert_eq!(eval("5 / 4"), Ok(Value::Int(1)));
        assert_eq!(eval("5 *3"), Ok(Value::Int(15)));
        assert_eq!(eval("1.0+3"), Ok(Value::Float(4.0)));
        assert_eq!(eval("3.0+1"), Ok(Value::Float(4.0)));
        assert_eq!(eval("3-5.0"), Ok(Value::Float(-2.0)));
        assert_eq!(eval("5-3.0"), Ok(Value::Float(2.0)));
        assert_eq!(eval("5 / 4.0"), Ok(Value::Float(1.25)));
        assert_eq!(eval("5.0 *3"), Ok(Value::Float(15.0)));
        assert_eq!(eval("5.0 *-3"), Ok(Value::Float(-15.0)));
        assert_eq!(eval("5.0 *- 3"), Ok(Value::Float(-15.0)));
        assert_eq!(eval("5.0 * -3"), Ok(Value::Float(-15.0)));
        assert_eq!(eval("5.0 * - 3"), Ok(Value::Float(-15.0)));
        assert_eq!(eval("-5.0 *-3"), Ok(Value::Float(15.0)));
        assert_eq!(eval("3+-1"), Ok(Value::Int(2)));
        assert_eq!(eval("-3-5"), Ok(Value::Int(-8)));
        assert_eq!(eval("-5--3"), Ok(Value::Int(-2)));
    }

    #[test]
    fn test_arithmetic_precedence_examples() {
        assert_eq!(eval("1+3-2"), Ok(Value::Int(2)));
        assert_eq!(eval("3+1*5"), Ok(Value::Int(8)));
        assert_eq!(eval("2*3-5"), Ok(Value::Int(1)));
        assert_eq!(eval("5-3/3"), Ok(Value::Int(4)));
        assert_eq!(eval("5 / 4*2"), Ok(Value::Int(2)));
        assert_eq!(eval("1-5 *3/15"), Ok(Value::Int(0)));
        assert_eq!(eval("15/7/2.0"), Ok(Value::Float(1.0)));
        assert_eq!(eval("15.0/7/2"), Ok(Value::Float(15.0 / 7.0 / 2.0)));
        assert_eq!(eval("15.0/-7/2"), Ok(Value::Float(15.0 / -7.0 / 2.0)));
        assert_eq!(eval("-15.0/7/2"), Ok(Value::Float(-15.0 / 7.0 / 2.0)));
        assert_eq!(eval("-15.0/7/-2"), Ok(Value::Float(-15.0 / 7.0 / -2.0)));
    }

    #[test]
    fn test_braced_examples() {
        assert_eq!(eval("(1)"), Ok(Value::Int(1)));
        assert_eq!(eval("( 1.0 )"), Ok(Value::Float(1.0)));
        assert_eq!(eval("( true)"), Ok(Value::Boolean(true)));
        assert_eq!(eval("( -1 )"), Ok(Value::Int(-1)));
        assert_eq!(eval("-(1)"), Ok(Value::Int(-1)));
        assert_eq!(eval("-(1 + 3) * 7"), Ok(Value::Int(-28)));
        assert_eq!(eval("(1 * 1) - 3"), Ok(Value::Int(-2)));
        assert_eq!(eval("4 / (2 * 2)"), Ok(Value::Int(1)));
        assert_eq!(eval("7/(7/(7/(7/(7/(7)))))"), Ok(Value::Int(1)));
    }

    #[test]
    fn test_mod_examples() {
        assert_eq!(eval("1 % 4"), Ok(Value::Int(1)));
        assert_eq!(eval("6 % 4"), Ok(Value::Int(2)));
        assert_eq!(eval("1 % 4 + 2"), Ok(Value::Int(3)));
    }

    #[test]
    fn test_boolean_examples() {
        assert_eq!(eval("true && false"), Ok(Value::Boolean(false)));
        assert_eq!(
            eval("true && false || true && true"),
            Ok(Value::Boolean(true))
        );
        assert_eq!(eval("5 > 4 && 1 <= 1"), Ok(Value::Boolean(true)));
        assert_eq!(eval("5.0 <= 4.9 || !(4 > 3.5)"), Ok(Value::Boolean(false)));
    }

    #[test]
    fn test_with_configuration() {
        let mut configuration = HashMapConfiguration::new();
        configuration.insert_variable("tr".to_string(), Value::Boolean(true));
        configuration.insert_variable("fa".to_string(), Value::Boolean(false));
        configuration.insert_variable("five".to_string(), Value::Int(5));
        configuration.insert_variable("six".to_string(), Value::Int(6));
        configuration.insert_variable("half".to_string(), Value::Float(0.5));
        configuration.insert_variable("zero".to_string(), Value::Int(0));

        assert_eq!(
            eval_with_configuration("tr", &configuration),
            Ok(Value::Boolean(true))
        );
        assert_eq!(
            eval_with_configuration("fa", &configuration),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            eval_with_configuration("tr && false", &configuration),
            Ok(Value::Boolean(false))
        );
        assert_eq!(
            eval_with_configuration("five + six", &configuration),
            Ok(Value::Int(11))
        );
        assert_eq!(
            eval_with_configuration("five * half", &configuration),
            Ok(Value::Float(2.5))
        );
        assert_eq!(
            eval_with_configuration("five < six && true", &configuration),
            Ok(Value::Boolean(true))
        );
    }

    #[test]
    fn test_functions() {
        let mut configuration = HashMapConfiguration::new();
        configuration.insert_function(
            "sub2".to_string(),
            Function::new(
                1,
                Box::new(|arguments| {
                    if let Value::Int(int) = arguments[0] {
                        Ok(Value::Int(int - 2))
                    } else {
                        Err(Error::expected_number(arguments[0].clone()))
                    }
                }),
            ),
        );
        configuration.insert_variable("five".to_string(), Value::Int(5));

        assert_eq!(
            eval_with_configuration("sub2 5", &configuration),
            Ok(Value::Int(3))
        );
        assert_eq!(
            eval_with_configuration("sub2(5)", &configuration),
            Ok(Value::Int(3))
        );
        assert_eq!(
            eval_with_configuration("sub2 five", &configuration),
            Ok(Value::Int(3))
        );
        assert_eq!(
            eval_with_configuration("sub2(five)", &configuration),
            Ok(Value::Int(3))
        );
        assert_eq!(
            eval_with_configuration("sub2(3) + five", &configuration),
            Ok(Value::Int(6))
        );
    }

    #[test]
    fn test_errors() {
        assert_eq!(
            eval("-true"),
            Err(Error::expected_number(Value::Boolean(true)))
        );
        assert_eq!(
            eval("1-true"),
            Err(Error::expected_number(Value::Boolean(true)))
        );
        assert_eq!(eval("true-"), Err(Error::wrong_argument_amount(1, 2)));
        assert_eq!(eval("!(()true)"), Err(Error::AppendedToLeafNode));
    }
}
