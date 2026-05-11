use std::error::Error as StdError;
use std::sync::{Arc, Mutex};


#[derive(Debug, Clone, Default)]
struct GoUrl {
    scheme: Arc<Mutex<Option<String>>>,
    host: Arc<Mutex<Option<String>>>,
    path: Arc<Mutex<Option<String>>>,
    raw_query: Arc<Mutex<Option<String>>>,
}

fn go_url_parse(input: &str) -> GoUrl {
    let (scheme, rest) = input.split_once("://").unwrap_or(("", input));
    let (before_query, raw_query) = rest.split_once('?').unwrap_or((rest, ""));
    let slash = before_query.find('/').unwrap_or(before_query.len());
    let host = &before_query[..slash];
    let path = if slash < before_query.len() { &before_query[slash..] } else { "" };
    GoUrl {
        scheme: Arc::new(Mutex::new(Some(scheme.to_string()))),
        host: Arc::new(Mutex::new(Some(host.to_string()))),
        path: Arc::new(Mutex::new(Some(path.to_string()))),
        raw_query: Arc::new(Mutex::new(Some(raw_query.to_string()))),
    }
}

fn main() {
    let (mut u, mut err) = { let __url_input = "https://example.com/path?key=value".to_string(); (Arc::new(Mutex::new(Some(go_url_parse(&__url_input)))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))) };
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        return;
    }

    println!("{} {}", "Scheme:".to_string(), (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).scheme.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone());
    println!("{} {}", "Host:".to_string(), (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).host.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone());
    println!("{} {}", "Path:".to_string(), (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).path.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone());
    println!("{} {}", "Query:".to_string(), (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).raw_query.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone());
}