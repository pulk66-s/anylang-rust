use crate::ast::TaggedAst;

pub struct LogicOptimizer {
    // Configuration for optimization passes
    pub constant_folding: bool,
    pub dead_code_elimination: bool,
}

impl LogicOptimizer {
    pub fn new() -> Self {
        Self {
            constant_folding: true,
            dead_code_elimination: true,
        }
    }

    pub fn optimize(&self, ast_json: &str) -> Result<String, String> {
        // Parse the input JSON
        let mut ast: TaggedAst = serde_json::from_str(ast_json)
            .map_err(|e| format!("Failed to parse AST JSON: {}", e))?;

        // Apply optimization passes
        if self.constant_folding {
            ast = self.fold_constants(ast);
        }

        if self.dead_code_elimination {
            ast = self.eliminate_dead_code(ast);
        }

        // Serialize back to JSON
        serde_json::to_string_pretty(&ast)
            .map_err(|e| format!("Failed to serialize optimized AST: {}", e))
    }

    /// Constant folding: Evaluate constant expressions at compile time
    fn fold_constants(&self, ast: TaggedAst) -> TaggedAst {
        match ast.tag.as_str() {
            "binary_op" => self.fold_binary_op(ast),
            "unary_op" => self.fold_unary_op(ast),
            _ => {
                // Recursively process children
                if let Some(children) = ast.children {
                    let optimized_children: Vec<TaggedAst> = children
                        .into_iter()
                        .map(|child| self.fold_constants(child))
                        .collect();
                    TaggedAst {
                        children: Some(optimized_children),
                        ..ast
                    }
                } else {
                    ast
                }
            }
        }
    }

    fn fold_binary_op(&self, ast: TaggedAst) -> TaggedAst {
        // Get operator before moving children
        let operator = match ast.get_attr_string("operator") {
            Some(op) => op,
            None => return ast,
        };

        // First, optimize children
        let children = match &ast.children {
            Some(children) if children.len() == 2 => {
                vec![
                    self.fold_constants(children[0].clone()),
                    self.fold_constants(children[1].clone()),
                ]
            }
            _ => return ast,
        };

        let left = &children[0];
        let right = &children[1];

        // Check if both operands are literals
        if left.tag != "literal_number" || right.tag != "literal_number" {
            return TaggedAst {
                children: Some(children),
                ..ast
            };
        }

        let left_val = match left.get_attr_number("value") {
            Some(v) => v,
            None => {
                return TaggedAst {
                    children: Some(children),
                    ..ast
                }
            }
        };

        let right_val = match right.get_attr_number("value") {
            Some(v) => v,
            None => {
                return TaggedAst {
                    children: Some(children),
                    ..ast
                }
            }
        };

        let _operator_check = match ast.get_attr_string("operator") {
            Some(op) => op,
            None => {
                return TaggedAst {
                    children: Some(children),
                    ..ast
                }
            }
        };

        // Perform the operation
        let result = match operator.as_str() {
            "+" => left_val + right_val,
            "-" => left_val - right_val,
            "*" => left_val * right_val,
            "/" => {
                if right_val == 0.0 {
                    return TaggedAst {
                        children: Some(children),
                        ..ast
                    }; // Don't fold division by zero
                }
                left_val / right_val
            }
            _ => {
                return TaggedAst {
                    children: Some(children),
                    ..ast
                }
            } // Unknown operator
        };

        // Return a literal number node
        TaggedAst::literal_number(result)
    }

    fn fold_unary_op(&self, ast: TaggedAst) -> TaggedAst {
        let children = match ast.children {
            Some(ref children) if children.len() == 1 => children,
            _ => return ast,
        };

        let operand = self.fold_constants(children[0].clone());

        if operand.tag != "literal_number" {
            return TaggedAst {
                children: Some(vec![operand]),
                ..ast
            };
        }

        let value = match operand.get_attr_number("value") {
            Some(v) => v,
            None => return ast,
        };

        let operator = match ast.get_attr_string("operator") {
            Some(op) => op,
            None => return ast,
        };

        let result = match operator.as_str() {
            "-" => -value,
            _ => return ast,
        };

        TaggedAst::literal_number(result)
    }

    /// Dead code elimination: Remove unreachable code
    fn eliminate_dead_code(&self, ast: TaggedAst) -> TaggedAst {
        match ast.tag.as_str() {
            "block" => self.eliminate_dead_in_block(ast),
            _ => {
                // Recursively process children
                if let Some(children) = ast.children {
                    let optimized_children: Vec<TaggedAst> = children
                        .into_iter()
                        .map(|child| self.eliminate_dead_code(child))
                        .collect();
                    TaggedAst {
                        children: Some(optimized_children),
                        ..ast
                    }
                } else {
                    ast
                }
            }
        }
    }

    fn eliminate_dead_in_block(&self, mut ast: TaggedAst) -> TaggedAst {
        let children = match ast.children.take() {
            Some(c) => c,
            None => return ast,
        };

        let mut result_children = Vec::new();
        let mut found_return = false;

        for child in children {
            if found_return {
                // Skip dead code after return
                continue;
            }

            let optimized_child = self.eliminate_dead_code(child);
            
            if optimized_child.tag == "return" {
                found_return = true;
            }

            result_children.push(optimized_child);
        }

        TaggedAst {
            children: Some(result_children),
            ..ast
        }
    }
}

impl Default for LogicOptimizer {
    fn default() -> Self {
        Self::new()
    }
}
