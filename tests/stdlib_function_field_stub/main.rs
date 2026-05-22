use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::rc::{Rc};

pub trait GoTypesBridgeStringArg {
    fn into_go_types_bridge_string(self) -> String;
}

impl GoTypesBridgeStringArg for String {
    fn into_go_types_bridge_string(self) -> String { self }
}

impl<'a> GoTypesBridgeStringArg for &'a str {
    fn into_go_types_bridge_string(self) -> String { self.to_string() }
}

impl<'a> GoTypesBridgeStringArg for &'a String {
    fn into_go_types_bridge_string(self) -> String { self.clone() }
}

impl GoTypesBridgeStringArg for Rc<RefCell<Option<String>>> {
    fn into_go_types_bridge_string(self) -> String {
        self.borrow().as_ref().cloned().unwrap_or_default()
    }
}

pub trait GoTypesBridgeInfoArg {
    fn apply_go_types_bridge_facts(self, type_facts: &[serde_json::Value], exprs_by_pos: &BTreeMap<i32, Vec<ast_Expr>>);
}

impl GoTypesBridgeInfoArg for () {
    fn apply_go_types_bridge_facts(self, _type_facts: &[serde_json::Value], _exprs_by_pos: &BTreeMap<i32, Vec<ast_Expr>>) {}
}

impl GoTypesBridgeInfoArg for Rc<RefCell<Option<types_Info>>> {
    fn apply_go_types_bridge_facts(self, type_facts: &[serde_json::Value], exprs_by_pos: &BTreeMap<i32, Vec<ast_Expr>>) {
        let mut info_guard = self.borrow_mut();
        if let Some(info_value) = info_guard.as_mut() {
            let mut types_guard = info_value.types.borrow_mut();
            if let Some(types_map) = types_guard.as_mut() {
                for fact in type_facts {
                    if fact.get("kind").and_then(|v| v.as_str()) != Some("basic") {
                        continue;
                    }
                    let pos = fact.get("pos").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
                    let Some(exprs) = exprs_by_pos.get(&pos) else { continue; };
                    let name = fact.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
                    let kind = fact.get("basicKind").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
                    let info_bits = fact.get("basicInfo").and_then(|v| v.as_i64()).unwrap_or_default() as i32;
                    for expr in exprs {
                        types_map.insert(expr.clone(), Rc::new(RefCell::new(Some::<types_TypeAndValue>(types_TypeAndValue { r#type: Rc::new(RefCell::new(Some::<types_Type>(__go_types_basic_type(name.clone(), kind, info_bits)))), value: Default::default() }))));
                    }
                }
            }
        }
    }
}

const __GO_TYPES_BRIDGE_HELPER: &str = r#"
package main

import (
	"encoding/json"
	"fmt"
	"go/ast"
	"go/importer"
	"go/parser"
	"go/token"
	"go/types"
	"os"
	"sort"
)

type request struct {
	Path  string `json:"path"`
	Files []file `json:"files"`
}

type file struct {
	Filename string `json:"filename"`
	Source   string `json:"source"`
}

type response struct {
	Package packageFact `json:"package"`
	Errors  []string    `json:"errors"`
	Types   []typeFact  `json:"types"`
}

type packageFact struct {
	Path string `json:"path"`
	Name string `json:"name"`
}

type typeFact struct {
	Pos       int    `json:"pos"`
	Kind      string `json:"kind"`
	Name      string `json:"name"`
	BasicKind int    `json:"basicKind"`
	BasicInfo int    `json:"basicInfo"`
}

func main() {
	var req request
	if err := json.NewDecoder(os.Stdin).Decode(&req); err != nil {
		_ = json.NewEncoder(os.Stdout).Encode(response{Errors: []string{err.Error()}})
		return
	}

	fset := token.NewFileSet()
	files := make([]*ast.File, 0, len(req.Files))
	for _, input := range req.Files {
		file, err := parser.ParseFile(fset, input.Filename, input.Source, parser.ParseComments|parser.SkipObjectResolution)
		if err != nil {
			_ = json.NewEncoder(os.Stdout).Encode(response{Errors: []string{err.Error()}})
			return
		}
		files = append(files, file)
	}

	info := &types.Info{
		Types: make(map[ast.Expr]types.TypeAndValue),
		Defs:  make(map[*ast.Ident]types.Object),
		Uses:  make(map[*ast.Ident]types.Object),
	}
	var errs []string
	config := &types.Config{
		Importer: importer.Default(),
		Error: func(err error) {
			errs = append(errs, err.Error())
		},
	}
	pkg, err := config.Check(req.Path, fset, files, info)
	if err != nil {
		msg := err.Error()
		if len(errs) == 0 || errs[len(errs)-1] != msg {
			errs = append(errs, msg)
		}
	}

	resp := response{Errors: errs}
	if pkg != nil {
		resp.Package = packageFact{Path: pkg.Path(), Name: pkg.Name()}
	}
	for expr, tv := range info.Types {
		if tv.Type == nil || expr == nil {
			continue
		}
		if basic, ok := types.Unalias(tv.Type).Underlying().(*types.Basic); ok {
			resp.Types = append(resp.Types, typeFact{
				Pos:       int(expr.Pos()),
				Kind:      "basic",
				Name:      basic.Name(),
				BasicKind: int(basic.Kind()),
				BasicInfo: int(basic.Info()),
			})
		}
	}
	sort.Slice(resp.Types, func(i, j int) bool {
		if resp.Types[i].Pos != resp.Types[j].Pos {
			return resp.Types[i].Pos < resp.Types[j].Pos
		}
		return resp.Types[i].Name < resp.Types[j].Name
	})
	if err := json.NewEncoder(os.Stdout).Encode(resp); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
"#;

fn __go_types_bridge_error(message: String) -> Box<dyn StdError> {
    Box::new(std::io::Error::new(std::io::ErrorKind::Other, message)) as Box<dyn StdError>
}

fn __go_types_basic_type(name: String, kind: i32, info: i32) -> types_Type {
    types_Type::__go_from(types_Basic {
        __go_kind: types_BasicKind(kind),
        __go_info: types_BasicInfo(info),
        __go_name: name,
    })
}

fn __go_types_config_check<T0: GoTypesBridgeStringArg, T3: GoTypesBridgeInfoArg>(
    path_arg: T0,
    files: Rc<RefCell<Option<Vec<Rc<RefCell<Option<ast_File>>>>>>>,
    info: T3,
) -> Result<types_Package, Box<dyn StdError>> {
    let path = path_arg.into_go_types_bridge_string();
    let file_values = files.borrow().as_ref().cloned().unwrap_or_default();
    let mut request_files = Vec::<serde_json::Value>::new();
    let mut exprs_by_pos = BTreeMap::<i32, Vec<ast_Expr>>::new();
    for file_handle in file_values {
        let file_guard = file_handle.borrow();
        let Some(file) = file_guard.as_ref() else { continue; };
        let filename = file.__go_filename.borrow().as_ref().cloned().unwrap_or_default();
        let source = file.__go_source.borrow().as_ref().cloned().unwrap_or_default();
        if source.is_empty() {
            continue;
        }
        __go_types_collect_file_exprs(file, &mut exprs_by_pos);
        request_files.push(serde_json::json!({
            "filename": filename,
            "source": source,
        }));
    }
    if request_files.is_empty() {
        return Err(__go_types_bridge_error("go/types bridge requires parser.ParseFile source metadata".to_string()));
    }

    let request = serde_json::json!({
        "path": path,
        "files": request_files,
    });
    let output = __go_types_run_bridge_helper(&request.to_string())?;
    let response: serde_json::Value = serde_json::from_slice(&output)
        .map_err(|err| __go_types_bridge_error(format!("failed to decode go/types bridge response: {}", err)))?;
    if let Some(errors) = response.get("errors").and_then(|v| v.as_array()) {
        if !errors.is_empty() {
            let message = errors.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("; ");
            return Err(__go_types_bridge_error(message));
        }
    }

    if let Some(type_facts) = response.get("types").and_then(|v| v.as_array()) {
        info.apply_go_types_bridge_facts(type_facts, &exprs_by_pos);
    }

    Ok(types_Package::default())
}

fn __go_types_run_bridge_helper(request_json: &str) -> Result<Vec<u8>, Box<dyn StdError>> {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let unique = format!(
        "go2rust-types-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default()
    );
    let dir = std::env::temp_dir().join(unique);
    std::fs::create_dir_all(&dir)
        .map_err(|err| __go_types_bridge_error(format!("failed to create go/types bridge dir: {}", err)))?;
    let helper_path = dir.join("main.go");
    std::fs::write(&helper_path, __GO_TYPES_BRIDGE_HELPER)
        .map_err(|err| __go_types_bridge_error(format!("failed to write go/types bridge helper: {}", err)))?;
    let mut child = Command::new("go")
        .arg("run")
        .arg(&helper_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| __go_types_bridge_error(format!("failed to launch go/types bridge helper: {}", err)))?;
    {
        let stdin = child.stdin.as_mut().ok_or_else(|| __go_types_bridge_error("failed to open go/types bridge stdin".to_string()))?;
        stdin.write_all(request_json.as_bytes())
            .map_err(|err| __go_types_bridge_error(format!("failed to write go/types bridge request: {}", err)))?;
    }
    let output = child.wait_with_output()
        .map_err(|err| __go_types_bridge_error(format!("failed to wait for go/types bridge helper: {}", err)))?;
    let _ = std::fs::remove_dir_all(&dir);
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(__go_types_bridge_error(format!("go/types bridge helper failed: {}", stderr)));
    }
    Ok(output.stdout)
}

fn __go_types_record_expr(exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>, expr: &ast_Expr) {
    if expr.__go_pos != 0 {
        exprs_by_pos.entry(expr.__go_pos).or_default().push(expr.clone());
    }
}

fn __go_types_collect_file_exprs(file: &ast_File, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    let decls = file.decls.borrow().as_ref().cloned().unwrap_or_default();
    for decl in decls {
        __go_types_collect_decl_exprs(&decl, exprs_by_pos);
    }
}

fn __go_types_collect_decl_exprs(decl: &ast_Decl, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(value) = decl.downcast_ref::<ast_GenDecl>() {
        let specs = value.specs.borrow().as_ref().cloned().unwrap_or_default();
        for spec in specs {
            __go_types_collect_spec_exprs(&spec, exprs_by_pos);
        }
    } else if let Some(value) = decl.downcast_ref::<ast_FuncDecl>() {
        __go_types_collect_opt_field_list(&value.recv, exprs_by_pos);
        __go_types_collect_func_type(&value.r#type, exprs_by_pos);
        __go_types_collect_opt_block(&value.body, exprs_by_pos);
    }
}

fn __go_types_collect_spec_exprs(spec: &ast_Spec, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(value) = spec.downcast_ref::<ast_ValueSpec>() {
        __go_types_collect_opt_expr(&value.r#type, exprs_by_pos);
        let values = value.values.borrow().as_ref().cloned().unwrap_or_default();
        for expr in values {
            __go_types_collect_expr(&expr, exprs_by_pos);
        }
    } else if let Some(value) = spec.downcast_ref::<ast_TypeSpec>() {
        __go_types_collect_opt_expr(&value.r#type, exprs_by_pos);
    }
}

fn __go_types_collect_opt_expr(value: &Rc<RefCell<Option<ast_Expr>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(expr) = value.borrow().as_ref().cloned() {
        __go_types_collect_expr(&expr, exprs_by_pos);
    }
}

fn __go_types_collect_expr(expr: &ast_Expr, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    __go_types_record_expr(exprs_by_pos, expr);
    if let Some(value) = expr.downcast_ref::<ast_ArrayType>() {
        __go_types_collect_opt_expr(&value.len, exprs_by_pos);
        __go_types_collect_opt_expr(&value.elt, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_BinaryExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_expr(&value.y, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_CallExpr>() {
        __go_types_collect_call_expr(value, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_CompositeLit>() {
        __go_types_collect_opt_expr(&value.r#type, exprs_by_pos);
        let elts = value.elts.borrow().as_ref().cloned().unwrap_or_default();
        for elt in elts {
            __go_types_collect_expr(&elt, exprs_by_pos);
        }
    } else if let Some(value) = expr.downcast_ref::<ast_IndexExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_expr(&value.index, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_IndexListExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        let indices = value.indices.borrow().as_ref().cloned().unwrap_or_default();
        for index in indices {
            __go_types_collect_expr(&index, exprs_by_pos);
        }
    } else if let Some(value) = expr.downcast_ref::<ast_KeyValueExpr>() {
        __go_types_collect_opt_expr(&value.key, exprs_by_pos);
        __go_types_collect_opt_expr(&value.value, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_MapType>() {
        __go_types_collect_opt_expr(&value.key, exprs_by_pos);
        __go_types_collect_opt_expr(&value.value, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_ParenExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_SelectorExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_SliceExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_expr(&value.low, exprs_by_pos);
        __go_types_collect_opt_expr(&value.high, exprs_by_pos);
        __go_types_collect_opt_expr(&value.max, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_StarExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_TypeAssertExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_expr(&value.r#type, exprs_by_pos);
    } else if let Some(value) = expr.downcast_ref::<ast_UnaryExpr>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    }
}

fn __go_types_collect_call_expr(value: &ast_CallExpr, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    __go_types_collect_opt_expr(&value.fun, exprs_by_pos);
    let args = value.args.borrow().as_ref().cloned().unwrap_or_default();
    for arg in args {
        __go_types_collect_expr(&arg, exprs_by_pos);
    }
}

fn __go_types_collect_opt_stmt(value: &Rc<RefCell<Option<ast_Stmt>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(stmt) = value.borrow().as_ref().cloned() {
        __go_types_collect_stmt_exprs(&stmt, exprs_by_pos);
    }
}

fn __go_types_collect_stmt_exprs(stmt: &ast_Stmt, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(value) = stmt.downcast_ref::<ast_AssignStmt>() {
        let lhs = value.lhs.borrow().as_ref().cloned().unwrap_or_default();
        let rhs = value.rhs.borrow().as_ref().cloned().unwrap_or_default();
        for expr in lhs.into_iter().chain(rhs.into_iter()) {
            __go_types_collect_expr(&expr, exprs_by_pos);
        }
    } else if let Some(value) = stmt.downcast_ref::<ast_DeclStmt>() {
        __go_types_collect_opt_decl(&value.decl, exprs_by_pos);
    } else if let Some(value) = stmt.downcast_ref::<ast_ExprStmt>() {
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
    } else if let Some(value) = stmt.downcast_ref::<ast_ReturnStmt>() {
        let results = value.results.borrow().as_ref().cloned().unwrap_or_default();
        for expr in results {
            __go_types_collect_expr(&expr, exprs_by_pos);
        }
    } else if let Some(value) = stmt.downcast_ref::<ast_IfStmt>() {
        __go_types_collect_opt_stmt(&value.init, exprs_by_pos);
        __go_types_collect_opt_expr(&value.cond, exprs_by_pos);
        __go_types_collect_opt_block(&value.body, exprs_by_pos);
        __go_types_collect_opt_stmt(&value.r#else, exprs_by_pos);
    } else if let Some(value) = stmt.downcast_ref::<ast_ForStmt>() {
        __go_types_collect_opt_stmt(&value.init, exprs_by_pos);
        __go_types_collect_opt_expr(&value.cond, exprs_by_pos);
        __go_types_collect_opt_stmt(&value.post, exprs_by_pos);
        __go_types_collect_opt_block(&value.body, exprs_by_pos);
    } else if let Some(value) = stmt.downcast_ref::<ast_RangeStmt>() {
        __go_types_collect_opt_expr(&value.key, exprs_by_pos);
        __go_types_collect_opt_expr(&value.value, exprs_by_pos);
        __go_types_collect_opt_expr(&value.x, exprs_by_pos);
        __go_types_collect_opt_block(&value.body, exprs_by_pos);
    }
}

fn __go_types_collect_opt_decl(value: &Rc<RefCell<Option<ast_Decl>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(decl) = value.borrow().as_ref().cloned() {
        __go_types_collect_decl_exprs(&decl, exprs_by_pos);
    }
}

fn __go_types_collect_opt_block(value: &Rc<RefCell<Option<ast_BlockStmt>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(block) = value.borrow().as_ref() {
        let list = block.list.borrow().as_ref().cloned().unwrap_or_default();
        for stmt in list {
            __go_types_collect_stmt_exprs(&stmt, exprs_by_pos);
        }
    }
}

fn __go_types_collect_func_type(value: &Rc<RefCell<Option<ast_FuncType>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(func_type) = value.borrow().as_ref() {
        __go_types_collect_opt_field_list(&func_type.params, exprs_by_pos);
        __go_types_collect_opt_field_list(&func_type.results, exprs_by_pos);
    }
}

fn __go_types_collect_opt_field_list(value: &Rc<RefCell<Option<ast_FieldList>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(field_list) = value.borrow().as_ref() {
        let fields = field_list.list.borrow().as_ref().cloned().unwrap_or_default();
        for field in fields {
            let field_guard = field.borrow();
            if let Some(field_value) = field_guard.as_ref() {
                __go_types_collect_opt_expr(&field_value.r#type, exprs_by_pos);
                __go_types_collect_opt_basic_lit(&field_value.tag, exprs_by_pos);
            }
        }
    }
}

fn __go_types_collect_opt_basic_lit(value: &Rc<RefCell<Option<ast_BasicLit>>>, exprs_by_pos: &mut BTreeMap<i32, Vec<ast_Expr>>) {
    if let Some(lit) = value.borrow().as_ref() {
        let lit_pos = lit.pos.borrow().as_ref().map(|pos| pos.0).unwrap_or_default();
        if lit_pos != 0 {
            exprs_by_pos.entry(lit_pos).or_default().push(ast_Expr::__go_from_with_pos(lit.clone(), lit_pos));
        }
    }
}



#[derive(Clone)]
pub struct types_Config {
    pub error: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<Box<dyn StdError>>>>) -> ()>>>>,
}

impl Default for types_Config {
    fn default() -> Self {
        Self {
            error: Default::default(),
        }
    }
}

impl std::fmt::Display for types_Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Config>")
    }
}


impl types_Config {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


fn main() {
    let _ = types_Config { error: Rc::new(RefCell::new(Some(Box::new(move |err: Rc<RefCell<Option<Box<dyn StdError>>>>| {
    }) as Box<dyn FnMut(Rc<RefCell<Option<Box<dyn StdError>>>>) -> ()>))), ..Default::default() };
    println!("{}", format!("{}", "ok".to_string()));
}