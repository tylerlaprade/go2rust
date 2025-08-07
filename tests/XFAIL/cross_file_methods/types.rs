use std::sync::{Arc, Mutex};

/// Counter holds a numeric value
#[derive(Debug)]
struct Counter {
    value: Arc<Mutex<Option<i32>>>,
}

/// Point represents a 2D point
#[derive(Debug)]
struct Point {
    x: Arc<Mutex<Option<f64>>>,
    y: Arc<Mutex<Option<f64>>>,
}