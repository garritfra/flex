use crate::lexer::*;
/**
 * Copyright 2020 Garrit Franke
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::parser::parse;

#[test]
fn test_parse_empty_function() {
    let raw = "fn main() {}";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_function_with_return() {
    let raw = "
    fn main() {
        return 1
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_redundant_semicolon() {
    let raw = "
    fn main() {
        return 1;
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_err())
}

#[test]
fn test_parse_no_function_context() {
    let raw = "
    let x = 1
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_err())
}

#[test]
fn test_parse_multiple_functions() {
    let raw = "
    fn foo() {
        let x = 2
        return x
    }

    fn bar() {
        let y = 5
        return y
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_variable_declaration() {
    let raw = "
    fn main() {
        let x = 1
        return x
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_function_with_args() {
    let raw = "
    fn main(foo) {
        return foo
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_function_call() {
    let raw = "
    fn main(foo) {
        foo()
    }

    fn foo() {
        foo(2)
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_return_function_call() {
    let raw = "
    fn main() {
        return fib(2)
    }

    fn fib() {
        return fib(2)
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_function_call_multiple_arguments() {
    let raw = "
    fn main() {
        fib(1, 2, 3)
    }

    fn fib() {
        return 2
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_nexted_function_call() {
    let raw = "
    fn main() {
        fib(fib(2), 2)
    }

    fn fib(n) {
        return 2
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_basic_ops() {
    let raw = "
    fn main() {
        return 2 * 5
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_compound_ops() {
    let raw = "
    fn main() {
        2 * 5 / 3
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_compound_ops_with_function_call() {
    let raw = "
    fn main() {
        return 2 * fib(1) / 3
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_compound_ops_with_strings() {
    let raw = "
    fn main() {
        return 2 * \"Hello\"
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_compound_ops_with_identifier() {
    let raw = "
    fn main(n) {
        return 2 * n
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_compound_ops_with_identifier_first() {
    let raw = "
    fn main(n) {
        return n * 2
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_compound_ops_return() {
    let raw = "
    fn main(n) {
        return 2 * n
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_basic_conditional() {
    let raw = "
    fn main(n) {
        if n {
            return n
        }
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_basic_conditional_with_multiple_statements() {
    let raw = "
    fn main(n) {
        if n {
            let x = 2 * n
            return x
        }
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_conditional_else_if_branch() {
    let raw = "
    fn main(n) {
        if n > 10 {
            let x = 2 * n
            return x
        } else if n <= 10 {
            return n
        }
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_conditional_multiple_else_if_branch_branches() {
    let raw = "
    fn main(n) {
        if n > 10 {
            let x = 2 * n
            return x
        } else if n < 10 {
            return n
        } else if n == 10 {
            return n + 1
        }
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_conditional_else_branch() {
    let raw = "
    fn main(n) {
        if n > 10 {
            let x = 2 * n
            return x
        } else {
            return n
        }
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_conditional_elseif_else_branch() {
    let raw = "
    fn main(n) {
        if n > 10 {
            let x = 2 * n
            return x
        } else if n < 10 {
            return n
        } else if n == 10 {
            return n + 1
        } else {
            return n
        }
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_int_array() {
    let raw = "
    fn main(n) {
        let arr = [1, 2, 3]
        return arr
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_string_array() {
    let raw = "
    fn main(n) {
        return [\"Foo\", \"Bar\", \"Baz\"]
    }
    ";
    let tokens = tokenize(raw);
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}
