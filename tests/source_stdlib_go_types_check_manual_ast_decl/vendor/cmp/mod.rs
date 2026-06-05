use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::fmt::{Display};

/// Ordered is a constraint that permits any ordered type: any type
/// that supports the operators < <= >= >.
/// If future releases of Go add new ordered types,
/// this constraint will be modified to include them.
///
/// Note that floating-point types may contain NaN ("not-a-number") values.
/// An operator such as == or < will always report false when
/// comparing a NaN value with any other value, NaN or not.
/// See the [Compare] function for a consistent way to compare NaN values.
pub trait Ordered: std::fmt::Display + Any {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool;
}

impl Clone for Box<dyn Ordered + Send + Sync> {
    fn clone(&self) -> Self {
        Ordered::__go_clone_box_ordered(self.as_ref())
    }
}

impl Ordered for i8 {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i8>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ordered for i16 {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i16>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ordered for i32 {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ordered for i64 {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<i64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ordered for isize {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<isize>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ordered for u8 {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u8>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ordered for u16 {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u16>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ordered for u32 {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ordered for u64 {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<u64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ordered for usize {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<usize>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ordered for f32 {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<f32>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ordered for f64 {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<f64>() {
            self == __other
        } else {
            false
        }
    }
}

impl Ordered for String {
    fn __go_clone_box_ordered(&self) -> Box<dyn Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<String>() {
            self == __other
        } else {
            false
        }
    }
}

/// Less reports whether x is less than y.
/// For floating-point types, a NaN is considered less than any non-NaN,
/// and -0.0 is not less than (is equal to) 0.0.
pub fn less<T: Ordered + Clone + PartialOrd + Send + Sync + 'static>(x: T, y: T) -> bool {
    (is_na_n::<T>(x.clone()) && !is_na_n::<T>(y.clone())) || { let __tmp_x = x.clone(); let __tmp_y = y.clone(); __tmp_x < __tmp_y }
}

/// Compare returns
///
///	-1 if x is less than y,
///	 0 if x equals y,
///	+1 if x is greater than y.
///
/// For floating-point types, a NaN is considered less than any non-NaN,
/// a NaN is considered equal to a NaN, and -0.0 is equal to 0.0.
pub fn compare<T: Ordered + Clone + PartialOrd + Send + Sync + 'static>(x: T, y: T) -> i32 {
    let mut xNaN = is_na_n::<T>(x.clone());
    let mut yNaN = is_na_n::<T>(y.clone());
    if xNaN {
        if yNaN {
        return 0;
    }
        return -(1);
    }
    if yNaN {
        return 1;
    }
    if { let __tmp_x = x.clone(); let __tmp_y = y.clone(); __tmp_x < __tmp_y } {
        return -(1);
    }
    if { let __tmp_x = x.clone(); let __tmp_y = y.clone(); __tmp_x > __tmp_y } {
        return 1;
    }
    0
}

/// isNaN reports whether x is a NaN without requiring the math package.
/// This will always return false if T is not floating-point.
pub fn is_na_n<T: Ordered + Clone + PartialOrd + Send + Sync + 'static>(x: T) -> bool {
    return { let __tmp_x = x.clone(); let __tmp_y = x.clone(); __tmp_x != __tmp_y };
}