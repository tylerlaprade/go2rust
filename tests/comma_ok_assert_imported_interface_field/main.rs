use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::rc::{Rc};

/// Comma-ok type assertion on a field whose type is an interface imported from
/// another package (ast.Expr). The assertion operand must stay the wrapped
/// interface handle so the downcast can unwrap it; go/parser does this on
/// ast.Expr/ast.Stmt fields throughout (`typ, ok := x.(*ast.ChanType)`).
pub fn describe(f: Rc<RefCell<Option<example_com_commaok_ast::r#mod::Field>>>) -> Rc<RefCell<Option<String>>> {
    {
        let (mut ct, mut ok) = ({
        let val = (*f.borrow().as_ref().unwrap()).value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn example_com_commaok_ast::r#mod::Expr>::__go_as_any(any_val.as_ref()).downcast_ref::<example_com_commaok_ast::r#mod::ChanTypePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<example_com_commaok_ast::r#mod::ChanType>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<example_com_commaok_ast::r#mod::ChanType>)), false)
        }
    });;
        if ok {
            return Rc::new(RefCell::new(Some(format!("chan {}", (*(*ct.borrow().as_ref().unwrap()).dir.borrow().as_ref().unwrap())))));;
        }
    }
    Rc::new(RefCell::new(Some("other".to_string())))
}

fn main() {
    ::example_com_commaok_ast::__go_init_all();

    println!("{}", format!("{}", (*describe(Rc::new(RefCell::new(Some(example_com_commaok_ast::r#mod::Field { value: Rc::new(RefCell::new(Some(Box::new(example_com_commaok_ast::r#mod::ChanTypePtr(Rc::new(RefCell::new(Some(example_com_commaok_ast::r#mod::ChanType { dir: Rc::new(RefCell::new(Some(3))), ..Default::default() }))).clone())) as Box<dyn example_com_commaok_ast::r#mod::Expr>))), ..Default::default() })))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*describe(Rc::new(RefCell::new(Some(example_com_commaok_ast::r#mod::Field { value: Rc::new(RefCell::new(Some(Box::new(example_com_commaok_ast::r#mod::IdentPtr(Rc::new(RefCell::new(Some(example_com_commaok_ast::r#mod::Ident { name: Rc::new(RefCell::new(Some("x".to_string()))), ..Default::default() }))).clone())) as Box<dyn example_com_commaok_ast::r#mod::Expr>))), ..Default::default() })))).borrow().as_ref().unwrap())));
}