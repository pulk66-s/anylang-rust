use std::collections::HashMap;

use anylang_ir::{CommonAstTag, CommonCondIR, CommonIRFunCall, CommonIRFunDef, CommonIRTag, CommonOpIR, CommonOpTag};
use serde_json::{Map, Value};

pub struct Logic {
    #[allow(dead_code)]
    existing_vars: Vec<String>,
    existing_funs: HashMap<String, CommonIRFunDef>,
}

impl Logic {
    pub fn new() -> Self {
        Logic {
            existing_vars: Vec::new(),
            existing_funs: HashMap::new(),
        }
    }

    fn get_next_value(&mut self, obj: &Map<String, Value>) -> Result<Value, String> {
        obj.values()
            .next()
            .cloned()
            .ok_or_else(|| "Object has no values".to_string())
    }

    fn parse_fun_params(&mut self, obj: &Map<String, Value>) -> Result<Vec<String>, String> {
        match obj.get("params") {
            Some(Value::Array(arr)) if arr.iter().all(|e| e.is_string()) => {
                Ok(arr.iter()
                    .filter_map(|e| e.as_str().map(|s| s.to_string()))
                    .collect())
            }
            Some(_) => Err("Expected an array of strings for function parameters".to_string()),
            None => Err("Function parameters not found".to_string()),
        }
    }

    fn parse_fun_body(&mut self, obj: &Map<String, Value>) -> Result<Vec<CommonIRTag>, String> {
        match obj.get("body") {
            Some(Value::Array(arr)) => self.parse_array(arr),
            Some(_) => Err("Expected an array for function body".to_string()),
            None => Err("Function body not found".to_string()),
        }
    }

    fn parse_fun_name(&mut self, obj: &Map<String, Value>) -> Result<String, String> {
        match obj.get("name") {
            Some(Value::String(name)) => Ok(name.clone()),
            Some(_) => Err("Expected a string for function name".to_string()),
            None => Err("Function name not found".to_string()),
        }
    }

    fn fun_exists(&mut self, name: &str) -> bool {
        self.existing_funs.contains_key(name)
    }

    fn parse_fun_def(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        let value = self.get_value_object(&obj)?;
        let params = self.parse_fun_params(&value)?;
        let body = self.parse_fun_body(&value)?;
        let name = self.parse_fun_name(&value)?;

        if self.fun_exists(&name) {
            return Err(format!("Function '{}' already exists", name));
        }
        self.existing_funs.insert(
            name.clone(),
            CommonIRFunDef::new(name.clone(), params, body, None),
        );
        Ok(CommonIRTag::FunDef(
            self.existing_funs.get(&name).unwrap().clone(),
        ))
    }

    fn get_value(&mut self, obj: &Map<String, Value>) -> Result<Value, String> {
        match self.get_next_value(obj) {
            Ok(Value::Object(o)) => o
                .get("value")
                .cloned()
                .ok_or_else(|| "Value not found in object".to_string()),
            Err(e) => Err(e),
            _ => Err("Expected an object value for tag".to_string()),
        }
    }

    fn get_value_object(&mut self, obj: &Map<String, Value>) -> Result<Map<String, Value>, String> {
        match self.get_value(obj) {
            Ok(Value::Object(o)) => Ok(o),
            Err(e) => Err(e),
            _ => Err("Expected an object value".to_string()),
        }
    }

    fn get_tag(&mut self, obj: &Map<String, Value>) -> Result<CommonAstTag, String> {
        match self.get_next_value(obj) {
            Ok(Value::Object(o)) => match o.get("tag") {
                Some(Value::String(tag)) => {
                    CommonAstTag::from_str(tag).map_err(|_| format!("Unknown tag: {}", tag))
                }
                Some(_) => Err("Expected a string value for tag".to_string()),
                None => Err("Tag not found in object".to_string()),
            },
            Err(e) => Err(e),
            _ => Err("Expected an object value for tag".to_string()),
        }
    }

    fn parse_cond(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        let value = self.get_value_object(obj)?;
        let cond = match value.get("cond") {
            Some(Value::Object(o)) => self.parse_object(o),
            Some(_) => Err("Expected an object for cond condition".to_string()),
            None => Err("Cond condition not found".to_string()),
        }?;
        let then_branch = match value.get("then_branch") {
            Some(Value::Array(arr)) => self.parse_array(arr),
            Some(_) => Err("Expected an array for cond then branch".to_string()),
            None => Err("Cond then branch not found".to_string()),
        }?;
        let else_branch = match value.get("else_branch") {
            Some(Value::Array(arr)) => self.parse_array(arr),
            Some(_) => Err("Expected an array for cond else branch".to_string()),
            None => Err("Cond else branch not found".to_string()),
        }?;

        Ok(CommonIRTag::Cond(CommonCondIR::new(
            cond,
            then_branch,
            else_branch,
        )))
    }

    fn parse_binary_op(&mut self, obj: &Map<String, Value>, op_tag: CommonOpTag, op_name: &str) -> Result<CommonIRTag, String> {
        let value = self.get_value_object(obj)?;
        let args = match value.get("args") {
            Some(Value::Array(arr)) if arr.len() == 2 => {
                let left = self.parse(&arr[0])?;
                let right = self.parse(&arr[1])?;
                Ok((left, right))
            }
            Some(_) => Err(format!("Expected an array of two elements for {} operation", op_name)),
            None => Err(format!("Arguments not found for {} operation", op_name)),
        }?;
        let left = args.0.get(0).ok_or("Should not happend".to_string())?;
        let right = args.1.get(0).ok_or("Should not happend".to_string())?;

        Ok(CommonIRTag::Op(CommonOpIR::new(op_tag, left.clone(), right.clone())))
    }

    fn parse_add(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        self.parse_binary_op(obj, CommonOpTag::Add, "add")
    }

    fn parse_sub(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        self.parse_binary_op(obj, CommonOpTag::Sub, "sub")
    }

    fn parse_mul(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        self.parse_binary_op(obj, CommonOpTag::Mul, "mul")
    }

    fn parse_div(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        self.parse_binary_op(obj, CommonOpTag::Div, "div")
    }

    fn parse_eq(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        self.parse_binary_op(obj, CommonOpTag::Eq, "eq")
    }

    fn parse_neq(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        self.parse_binary_op(obj, CommonOpTag::Neq, "neq")
    }

    fn parse_lt(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        self.parse_binary_op(obj, CommonOpTag::Lt, "lt")
    }

    fn parse_gt(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        self.parse_binary_op(obj, CommonOpTag::Gt, "gt")
    }

    fn parse_lte(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        self.parse_binary_op(obj, CommonOpTag::Lte, "lte")
    }

    fn parse_gte(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        self.parse_binary_op(obj, CommonOpTag::Gte, "gte")
    }

    fn parse_number(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        let value = self.get_value(obj)?;
        match value {
            Value::Number(num) => {
                let int_val = num.as_i64()
                    .or_else(|| num.as_f64().map(|f| f as i64))
                    .ok_or_else(|| format!("Number {} cannot be converted to integer", num))?;
                Ok(CommonIRTag::Number(int_val))
            }
            _ => Err("Expected a number value".to_string()),
        }
    }

    fn parse_var(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        let value = self.get_value(obj)?;
        match value {
            Value::String(name) => Ok(CommonIRTag::Var(name.clone())),
            _ => Err("Expected a string value for variable name".to_string()),
        }
    }

    fn parse_fun_call(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        let value = self.get_value_object(obj)?;
        let name = match value.get("name") {
            Some(Value::String(name)) => Ok(name.clone()),
            Some(_) => Err("Expected a string for function name".to_string()),
            None => Err("Function name not found".to_string()),
        }?;
        let args = match value.get("args") {
            Some(Value::Array(arr)) => {
                arr.iter()
                    .map(|arg| self.parse(arg))
                    .collect::<Result<Vec<_>, _>>()
                    .map(|vec| vec.into_iter().flatten().collect())
            }
            Some(_) => Err("Expected an array for function arguments".to_string()),
            None => Ok(Vec::new()),
        }?;
        Ok(CommonIRTag::FunCall(CommonIRFunCall::new(name, args)))
    }

    fn parse_object(&mut self, obj: &Map<String, Value>) -> Result<CommonIRTag, String> {
        match self.get_tag(&obj)? {
            CommonAstTag::FunDef => self.parse_fun_def(obj),
            CommonAstTag::FunCall => self.parse_fun_call(obj),
            CommonAstTag::Cond => self.parse_cond(obj),
            CommonAstTag::Add => self.parse_add(obj),
            CommonAstTag::Sub => self.parse_sub(obj),
            CommonAstTag::Mul => self.parse_mul(obj),
            CommonAstTag::Div => self.parse_div(obj),
            CommonAstTag::Eq => self.parse_eq(obj),
            CommonAstTag::Neq => self.parse_neq(obj),
            CommonAstTag::Le => self.parse_lt(obj),
            CommonAstTag::Ge => self.parse_gt(obj),
            CommonAstTag::Lte => self.parse_lte(obj),
            CommonAstTag::Gte => self.parse_gte(obj),
            CommonAstTag::Number => self.parse_number(obj),
            CommonAstTag::Var => self.parse_var(obj),
        }
    }

    fn parse_array(&mut self, arr: &Vec<Value>) -> Result<Vec<CommonIRTag>, String> {
        arr.into_iter()
            .map(|item| self.parse(item))
            .collect::<Result<Vec<_>, _>>()
            .map(|vec| vec.into_iter().flatten().collect())
    }

    pub fn parse(&mut self, ast: &Value) -> Result<Vec<CommonIRTag>, String> {
        match ast {
            Value::Object(obj) => self.parse_object(obj).map(|tag| vec![tag]),
            Value::Array(arr) => self.parse_array(arr),
            f => Err(format!("Expected an object, got: {:?}", f)),
        }
    }
}
