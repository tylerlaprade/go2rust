use std::fmt::{Display};
use std::sync::{Arc, Mutex};

fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.lock().unwrap();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

struct GoAtomicPointer<T> {
    value: Arc<Mutex<Option<Arc<Mutex<Option<T>>>>>>,
}

impl<T> GoAtomicPointer<T> {
    fn load(&self) -> Arc<Mutex<Option<T>>> {
        self.value.lock().unwrap().as_ref().cloned().unwrap_or_else(|| Arc::new(Mutex::new(None)))
    }

    fn store(&self, value: Arc<Mutex<Option<T>>>) {
        *self.value.lock().unwrap() = Some(value);
    }

    fn swap(&self, value: Arc<Mutex<Option<T>>>) -> Arc<Mutex<Option<T>>> {
        self.value.lock().unwrap().replace(value).unwrap_or_else(|| Arc::new(Mutex::new(None)))
    }

    fn compare_and_swap(&self, old: Arc<Mutex<Option<T>>>, new: Arc<Mutex<Option<T>>>) -> bool {
        let mut current = self.value.lock().unwrap();
        let matched = match current.as_ref() {
            Some(value) if Arc::ptr_eq(value, &old) => true,
            Some(value) => value.lock().unwrap().is_none() && old.lock().unwrap().is_none(),
            None => old.lock().unwrap().is_none(),
        };
        if matched {
            *current = Some(new);
        }
        matched
    }
}

impl<T> Clone for GoAtomicPointer<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

impl<T> Default for GoAtomicPointer<T> {
    fn default() -> Self {
        Self { value: Arc::new(Mutex::new(None)) }
    }
}

impl<T> std::fmt::Debug for GoAtomicPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoAtomicPointer")
    }
}

impl<T> std::fmt::Display for GoAtomicPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoAtomicPointer")
    }
}
