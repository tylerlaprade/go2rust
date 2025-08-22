/// Counter holds a numeric value
#[derive(Debug, Clone, Default)]
struct Counter {
    value: Rc<RefCell<Option<i32>>>,
}

/// Point represents a 2D point
#[derive(Debug, Clone, Default)]
struct Point {
    x: Rc<RefCell<Option<f64>>>,
    y: Rc<RefCell<Option<f64>>>,
}