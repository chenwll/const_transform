use crate::swc_core::common::sync::Lrc;
use serde::{Deserialize, Serialize};
use serde_json::{self, Map, Value};
use std::rc::Rc;
use swc_core::{
    self,
    atoms::Atom,
    common::{
        comments::SingleThreadedComments, input::SourceFileInput, FileName, SourceMap, DUMMY_SP,
    },
    ecma::{
        ast::*,
        codegen::to_code_default,
        parser::{lexer::Lexer, Parser, Syntax, TsSyntax},
        visit::{VisitMut, VisitMutWith},
    },
};
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

pub struct ConstReplacer<'a> {
    replaced_value: Value,
    replaced_name: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonValue<'a> {
    replaced_name: &'a str,
    replaced_value: Value,
}
impl<'a> ConstReplacer<'a> {
    pub fn new(config: &'a str) -> Result<ConstReplacer, JsError> {
        let config: JsonValue = serde_json::from_str(config).map_err(|e| JsError::new(&format!("config parse error: {}", e)))?;
        let replaced_name = config.replaced_name;
        let replaced_value = config.replaced_value;
        Ok(ConstReplacer {
            replaced_name: replaced_name,
            replaced_value: replaced_value,
        })
    }
}
impl VisitMut for ConstReplacer<'_> {
    fn visit_mut_var_decl(&mut self, var_decl: &mut VarDecl) {
        // 检查变量声明类型是否为 `const`
        if var_decl.kind == VarDeclKind::Const {
            for declarator in &mut var_decl.decls {
                // 判断变量名是否与要替换的名称相同
                let is_replaced_name = if let Pat::Ident(ref ident) = declarator.name {
                    ident.sym.as_ref() == self.replaced_name
                } else {
                    false
                };

                // 如果变量名匹配且有初始化表达式，则进行检查和可能的替换
                if let Some(init_expr) = &mut declarator.init {
                    // 函数表达式，则递归处理
                    init_expr.visit_mut_with(self);
                    if is_replaced_name {
                        match **init_expr {
                            // 字面量的情况下，替换为新的数字字面量
                            Expr::Lit(Lit::Str(_))
                            | Expr::Lit(Lit::Bool(_))
                            | Expr::Array(_)
                            | Expr::Object(_)
                            | Expr::Lit(Lit::Null(_))
                            | Expr::Lit(Lit::Num(_)) => {
                                let value = self.replaced_value.clone();
                                *init_expr = Box::new(create_ast(value));
                            }
                            // 其他类型表达式，包括箭头函数，不进行替换
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

pub fn create_ast(value: Value) -> Expr {
    match value {
        Value::Number(num) => {
            if let Some(f) = num.as_f64() {
                Expr::Lit(Lit::Num(Number {
                    span: DUMMY_SP,
                    value: f,
                    raw: None,
                }))
            } else {
                // Handle other number types if necessary, such as integers
                unimplemented!()
            }
        }
        Value::String(s) => Expr::Lit(Lit::Str(Str {
            span: DUMMY_SP,
            value: Atom::from(s.clone()),
            raw: None,
        })),
        Value::Bool(s) => Expr::Lit(Lit::Bool(Bool {
            span: DUMMY_SP,
            value: s,
        })),
        Value::Array(s) => {
            let elems: Vec<Option<ExprOrSpread>> = s
                .iter()
                .map(|json_value| {
                    Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(create_ast(json_value.clone())),
                    })
                })
                .collect();

            Expr::Array(ArrayLit {
                span: DUMMY_SP,
                elems: elems,
            })
        }
        Value::Object(s) => {
            let props: Vec<PropOrSpread> = s
                .iter()
                .map(|(key, value)| {
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Str(Str {
                            span: DUMMY_SP,
                            value: key.clone().into(),
                            raw: Some(format!("'{}'", key).into())
                        }),
                        value: Box::new(create_ast(value.clone())),
                    })))
                })
                .collect();
            Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props,
            })
        }
        _ => {
            println!("Unhandled value type:");
            unimplemented!()
        }
    }
}

#[wasm_bindgen]
pub fn const_replace(source: &str, config: &str) -> Result<String, JsError> {
    let cm: Lrc<SourceMap> = Rc::new(SourceMap::default());
    let fm = cm.new_source_file(
        Lrc::new(FileName::Custom("input.js".to_string())),
        source.into(),
    );

    let comments = SingleThreadedComments::default();
    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: false,
            ..Default::default()
        }),
        EsVersion::EsNext,
        SourceFileInput::from(&*fm),
        Some(&comments),
    );

    let mut parser = Parser::new_from(lexer);

    let mut module = parser.parse_module().unwrap();

    let mut replacer = ConstReplacer::new(config)?;

    module.visit_mut_with(&mut replacer);

    let res = to_code_default(Default::default(), Some(&comments), &module);
    Ok(res)
}


#[wasm_bindgen]
pub fn replace_json(source: &str, config: &str) -> Result<String, JsError> {
    let mut json_value: Map<String, Value> = serde_json::from_str(source).unwrap();
     let replacer = ConstReplacer::new(config)?;
     if json_value.contains_key(replacer.replaced_name) {
        json_value[replacer.replaced_name] = replacer.replaced_value;

        let updated_json = serde_json::to_string_pretty(&json_value).unwrap();
        Ok(updated_json)
    } else {
        Err(JsError::new(&format!("Key '{}' does not exist in the JSON", replacer.replaced_name)))
    }
}