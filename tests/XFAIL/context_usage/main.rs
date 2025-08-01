fn main() {
    let (mut ctx, mut cancel) = (*context.lock().unwrap().as_mut().unwrap()).with_timeout(std::sync::Arc::new(std::sync::Mutex::new(Some((*context.lock().unwrap().as_mut().unwrap()).background()))), std::sync::Arc::new(std::sync::Mutex::new(Some(2 * (*time.lock().unwrap().as_mut().unwrap()).second))));
    // defer cancel() // TODO: defer not yet supported
    // TODO: Unhandled statement type: SelectStmt
}