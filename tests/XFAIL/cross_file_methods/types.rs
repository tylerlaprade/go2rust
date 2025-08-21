use std::sync::{Arc, Mutex};

/// Counter holds a numeric value
#[derive(Debug, Clone, Default)]
struct Counter {
    value: Arc<Mutex<Option<i32>>>,
}

/// Point represents a 2D point
#[derive(Debug, Clone, Default)]
struct Point {
    x: Arc<Mutex<Option<f64>>>,
    y: Arc<Mutex<Option<f64>>>,
}