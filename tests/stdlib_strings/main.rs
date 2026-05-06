use std::fmt::{Display};
use std::sync::{Arc, Mutex};

fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
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

fn main() {
        // Basic string operations
    println!("{}", "=== Basic string operations ===".to_string());

    let mut str = Arc::new(Mutex::new(Some("Hello, World!".to_string())));
    print!("Original string: {}\n", { let __v = (*str.lock().unwrap().as_ref().unwrap()).clone(); __v });
    print!("Length: {}\n", (*str.lock().unwrap().as_ref().unwrap()).len());

        // String indexing and slicing
    print!("First character: {}\n", ({ let __s = (*str.lock().unwrap().as_ref().unwrap()).clone(); __s.as_bytes()[0 as usize] }) as u8 as char);
    print!("Last character: {}\n", ({ let __s = (*str.lock().unwrap().as_ref().unwrap()).clone(); __s.as_bytes()[{ let __tmp_x = (*str.lock().unwrap().as_ref().unwrap()).len(); let __tmp_y = 1; __tmp_x - __tmp_y } as usize] }) as u8 as char);
    print!("Substring [0:5]: {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*str.lock().unwrap().as_ref().unwrap()).clone(); __s[0 as usize..5 as usize].to_string() }))).lock().unwrap().as_ref().unwrap()));
    print!("Substring [7:]: {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*str.lock().unwrap().as_ref().unwrap()).clone(); __s[7 as usize..].to_string() }))).lock().unwrap().as_ref().unwrap()));

        // String concatenation
    println!("{}", "\n=== String concatenation ===".to_string());
    let mut first = Arc::new(Mutex::new(Some("Hello".to_string())));
    let mut second = Arc::new(Mutex::new(Some("World".to_string())));
    let mut combined = Arc::new(Mutex::new(Some(format!("{}{}", format!("{}{}", format!("{}{}", { let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v }, ", ".to_string()), { let __v = (*second.lock().unwrap().as_ref().unwrap()).clone(); __v }), "!".to_string()))));
    print!("Concatenated: {}\n", { let __v = (*combined.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Using strings package
    println!("{}", "\n=== strings package functions ===".to_string());

    let mut text = Arc::new(Mutex::new(Some("  Go is awesome for systems programming  ".to_string())));
    print!("Original: '{}'\n", { let __v = (*text.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Trimming
    let mut trimmed = Arc::new(Mutex::new(Some({ let __v = (*text.lock().unwrap().as_ref().unwrap()).clone(); __v }.trim().to_string())));
    print!("Trimmed: '{}'\n", { let __v = (*trimmed.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Case conversion
    print!("Upper: {}\n", (*Arc::new(Mutex::new(Some({ let __v = (*trimmed.lock().unwrap().as_ref().unwrap()).clone(); __v }.to_uppercase()))).lock().unwrap().as_ref().unwrap()));
    print!("Lower: {}\n", (*Arc::new(Mutex::new(Some({ let __v = (*trimmed.lock().unwrap().as_ref().unwrap()).clone(); __v }.to_lowercase()))).lock().unwrap().as_ref().unwrap()));
    print!("Title: {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*trimmed.lock().unwrap().as_ref().unwrap()).clone(); let mut __out = String::new(); let mut __new_word = true; for __ch in __s.chars() { if __ch.is_alphanumeric() { if __new_word { for __upper in __ch.to_uppercase() { __out.push(__upper); } } else { __out.push(__ch); } __new_word = false; } else { __out.push(__ch); __new_word = true; } } __out }))).lock().unwrap().as_ref().unwrap()));

        // String searching
    println!("{}", "\n=== String searching ===".to_string());
    let mut searchText = Arc::new(Mutex::new(Some("The quick brown fox jumps over the lazy dog".to_string())));
    print!("Text: {}\n", { let __v = (*searchText.lock().unwrap().as_ref().unwrap()).clone(); __v });

    print!("Contains 'fox': {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*searchText.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "fox".to_string(); __s.contains(&__arg) }))).lock().unwrap().as_ref().unwrap()));
    print!("Contains 'cat': {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*searchText.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "cat".to_string(); __s.contains(&__arg) }))).lock().unwrap().as_ref().unwrap()));

    print!("Index of 'fox': {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*searchText.lock().unwrap().as_ref().unwrap()).clone(); let __substr = "fox".to_string(); __s.find(&__substr).map(|__i| __i as i32).unwrap_or(-1) }))).lock().unwrap().as_ref().unwrap()));
    print!("Index of 'cat': {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*searchText.lock().unwrap().as_ref().unwrap()).clone(); let __substr = "cat".to_string(); __s.find(&__substr).map(|__i| __i as i32).unwrap_or(-1) }))).lock().unwrap().as_ref().unwrap()));

    print!("Last index of 'the': {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*searchText.lock().unwrap().as_ref().unwrap()).clone(); let __substr = "the".to_string(); __s.rfind(&__substr).map(|__i| __i as i32).unwrap_or(-1) }))).lock().unwrap().as_ref().unwrap()));

    print!("Count of 'the': {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*searchText.lock().unwrap().as_ref().unwrap()).clone(); let __substr = "the".to_string(); if __substr.is_empty() { __s.chars().count() as i32 + 1 } else { __s.matches(&__substr).count() as i32 } }))).lock().unwrap().as_ref().unwrap()));

        // String prefixes and suffixes
    println!("{}", "\n=== Prefixes and suffixes ===".to_string());
    let mut filename = Arc::new(Mutex::new(Some("document.txt".to_string())));
    print!("Filename: {}\n", { let __v = (*filename.lock().unwrap().as_ref().unwrap()).clone(); __v });
    print!("Has .txt suffix: {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*filename.lock().unwrap().as_ref().unwrap()).clone(); let __arg = ".txt".to_string(); __s.ends_with(&__arg) }))).lock().unwrap().as_ref().unwrap()));
    print!("Has .pdf suffix: {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*filename.lock().unwrap().as_ref().unwrap()).clone(); let __arg = ".pdf".to_string(); __s.ends_with(&__arg) }))).lock().unwrap().as_ref().unwrap()));
    print!("Has 'doc' prefix: {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*filename.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "doc".to_string(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap()));

        // String splitting and joining
    println!("{}", "\n=== Splitting and joining ===".to_string());
    let mut csv = Arc::new(Mutex::new(Some("apple,banana,cherry,date".to_string())));
    print!("CSV: {}\n", { let __v = (*csv.lock().unwrap().as_ref().unwrap()).clone(); __v });

    let mut fruits = Arc::new(Mutex::new(Some({ let __s = (*csv.lock().unwrap().as_ref().unwrap()).clone(); let __sep = ",".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() })));
    print!("Split result: {}\n", format_slice(&fruits));

    let mut rejoined = Arc::new(Mutex::new(Some({ let __parts = (*fruits.lock().unwrap().as_ref().unwrap()).clone(); let __sep = " | ".to_string(); __parts.join(&__sep) })));
    print!("Rejoined: {}\n", { let __v = (*rejoined.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // Fields (split on whitespace)
    let mut sentence = Arc::new(Mutex::new(Some("The quick brown fox".to_string())));
    let mut words = Arc::new(Mutex::new(Some({ let __s = (*sentence.lock().unwrap().as_ref().unwrap()).clone(); __s.split_whitespace().map(|__part| __part.to_string()).collect::<Vec<String>>() })));
    print!("Words: {}\n", format_slice(&words));

        // String replacement
    println!("{}", "\n=== String replacement ===".to_string());
    let mut original = Arc::new(Mutex::new(Some("I like cats and cats like me".to_string())));
    print!("Original: {}\n", { let __v = (*original.lock().unwrap().as_ref().unwrap()).clone(); __v });

    let mut replaced = Arc::new(Mutex::new(Some({ let __s = (*original.lock().unwrap().as_ref().unwrap()).clone(); let __old = "cats".to_string(); let __new = "dogs".to_string(); let __n = 1; if __n < 0 { __s.replace(&__old, &__new) } else { __s.replacen(&__old, &__new, __n as usize) } })));
    print!("Replace first 'cats': {}\n", { let __v = (*replaced.lock().unwrap().as_ref().unwrap()).clone(); __v });

    let mut replacedAll = Arc::new(Mutex::new(Some({ let __s = (*original.lock().unwrap().as_ref().unwrap()).clone(); let __old = "cats".to_string(); let __new = "dogs".to_string(); __s.replace(&__old, &__new) })));
    print!("Replace all 'cats': {}\n", { let __v = (*replacedAll.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // String repetition
    println!("{}", "\n=== String repetition ===".to_string());
    let mut pattern = Arc::new(Mutex::new(Some("Go! ".to_string())));
    let mut repeated = Arc::new(Mutex::new(Some({ let __s = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); let __count = 3; __s.repeat(__count as usize) })));
    print!("Repeated: {}\n", { let __v = (*repeated.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // String comparison
    println!("{}", "\n=== String comparison ===".to_string());
    let mut str1 = Arc::new(Mutex::new(Some("apple".to_string())));
    let mut str2 = Arc::new(Mutex::new(Some("banana".to_string())));
    let mut str3 = Arc::new(Mutex::new(Some("apple".to_string())));

    print!("'{}' == '{}': {}\n", { let __v = (*str1.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*str2.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __tmp_x = { let __v = (*str1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*str2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y });
    print!("'{}' == '{}': {}\n", { let __v = (*str1.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*str3.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __tmp_x = { let __v = (*str1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*str3.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y });
    print!("'{}' < '{}': {}\n", { let __v = (*str1.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*str2.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __tmp_x = { let __v = (*str1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*str2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y });

        // Case-insensitive comparison
    print!("EqualFold('Apple', 'APPLE'): {}\n", (*Arc::new(Mutex::new(Some({ let __a = "Apple".to_string(); let __b = "APPLE".to_string(); __a.to_lowercase() == __b.to_lowercase() }))).lock().unwrap().as_ref().unwrap()));

        // String building with strings.Builder
    println!("{}", "\n=== String building ===".to_string());
    let mut builder: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    (*builder.lock().unwrap().as_mut().unwrap()).push_str("Building ");
    (*builder.lock().unwrap().as_mut().unwrap()).push_str("a ");
    (*builder.lock().unwrap().as_mut().unwrap()).push_str("string ");
    (*builder.lock().unwrap().as_mut().unwrap()).push_str("efficiently");

    let mut built = Arc::new(Mutex::new(Some((*builder.lock().unwrap().as_ref().unwrap()).clone())));
    print!("Built string: {}\n", { let __v = (*built.lock().unwrap().as_ref().unwrap()).clone(); __v });
    print!("Builder length: {}\n", (*Arc::new(Mutex::new(Some((*builder.lock().unwrap().as_ref().unwrap()).len() as i32))).lock().unwrap().as_ref().unwrap()));

        // Rune iteration (Unicode support)
    println!("{}", "\n=== Unicode and runes ===".to_string());
    let mut unicode = Arc::new(Mutex::new(Some("Hello, 世界! 🌍".to_string())));
    print!("Unicode string: {}\n", { let __v = (*unicode.lock().unwrap().as_ref().unwrap()).clone(); __v });
    print!("Byte length: {}\n", (*unicode.lock().unwrap().as_ref().unwrap()).len());

    let mut runeCount = Arc::new(Mutex::new(Some(0)));
    for (_, r) in (*unicode.lock().unwrap().as_ref().unwrap()).char_indices() {
        { let mut guard = runeCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        print!("Rune: {} (U+{})\n", r, format!("{:04X}", r as u32));
    }
    print!("Rune count: {}\n", { let __v = (*runeCount.lock().unwrap().as_ref().unwrap()).clone(); __v });

        // String trimming variations
    println!("{}", "\n=== String trimming variations ===".to_string());
    let mut messy = Arc::new(Mutex::new(Some("!!!Hello World!!!".to_string())));
    print!("Original: {}\n", { let __v = (*messy.lock().unwrap().as_ref().unwrap()).clone(); __v });
    print!("TrimLeft '!': {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*messy.lock().unwrap().as_ref().unwrap()).clone(); let __cutset = "!".to_string(); __s.trim_start_matches(|__ch| __cutset.contains(__ch)).to_string() }))).lock().unwrap().as_ref().unwrap()));
    print!("TrimRight '!': {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*messy.lock().unwrap().as_ref().unwrap()).clone(); let __cutset = "!".to_string(); __s.trim_end_matches(|__ch| __cutset.contains(__ch)).to_string() }))).lock().unwrap().as_ref().unwrap()));
    print!("Trim '!': {}\n", (*Arc::new(Mutex::new(Some({ let __s = (*messy.lock().unwrap().as_ref().unwrap()).clone(); let __cutset = "!".to_string(); __s.trim_matches(|__ch| __cutset.contains(__ch)).to_string() }))).lock().unwrap().as_ref().unwrap()));

        // String formatting with different verbs
    println!("{}", "\n=== String formatting ===".to_string());
    let mut name = Arc::new(Mutex::new(Some("Alice".to_string())));
    let mut age = Arc::new(Mutex::new(Some(30)));
    let mut height = Arc::new(Mutex::new(Some(5.6)));

    print!("Name: {}, Age: {}, Height: {:.1}\n", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*age.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*height.lock().unwrap().as_ref().unwrap()).clone(); __v });
    print!("Quoted string: {:?}\n", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v });
    print!("String with width: '{:>10}'\n", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v });
    print!("Left-aligned: '{:<10}'\n", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v });
}