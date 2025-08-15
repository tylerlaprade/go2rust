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

fn main() {
        // Basic string operations
println!("{}", "=== Basic string operations ===".to_string());

    let mut str = Arc::new(Mutex::new(Some("Hello, World!".to_string())));
    print!("Original string: {}\n", (*str.lock().unwrap().as_mut().unwrap()));
    print!("Length: {}\n", (*str.lock().unwrap().as_ref().unwrap()).len());

        // String indexing and slicing
print!("First character: {}\n", (*(*str.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).as_bytes()[0 as usize]);
    print!("Last character: {}\n", (*(*str.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).as_bytes()[(*str.lock().unwrap().as_ref().unwrap()).len() - 1 as usize]);
    print!("Substring [0:5]: {}\n", Arc::new(Mutex::new(Some((*str.lock().unwrap().as_ref().unwrap())[0 as usize..5 as usize].to_vec()))));
    print!("Substring [7:]: {}\n", Arc::new(Mutex::new(Some((*str.lock().unwrap().as_ref().unwrap())[7 as usize..].to_vec()))));

        // String concatenation
println!("{}", "\n=== String concatenation ===".to_string());
    let mut first = Arc::new(Mutex::new(Some("Hello".to_string())));
    let mut second = Arc::new(Mutex::new(Some("World".to_string())));
    let mut combined = Arc::new(Mutex::new(Some(format!("{}{}", format!("{}{}", (*first.lock().unwrap().as_mut().unwrap()), ", ".to_string()) + (*second.lock().unwrap().as_mut().unwrap()), "!".to_string()))));
    print!("Concatenated: {}\n", (*combined.lock().unwrap().as_mut().unwrap()));

        // Using strings package
println!("{}", "\n=== strings package functions ===".to_string());

    let mut text = Arc::new(Mutex::new(Some("  Go is awesome for systems programming  ".to_string())));
    print!("Original: '{}'\n", (*text.lock().unwrap().as_mut().unwrap()));

        // Trimming
let mut trimmed = (*text.lock().unwrap().as_mut().unwrap()).trim();
    print!("Trimmed: '{}'\n", (*trimmed.lock().unwrap().as_mut().unwrap()));

        // Case conversion
print!("Upper: {}\n", (*(*trimmed.lock().unwrap().as_mut().unwrap()).to_uppercase().lock().unwrap().as_ref().unwrap()));
    print!("Lower: {}\n", (*(*trimmed.lock().unwrap().as_mut().unwrap()).to_lowercase().lock().unwrap().as_ref().unwrap()));
    print!("Title: {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).title(Arc::new(Mutex::new(Some((*trimmed.lock().unwrap().as_mut().unwrap()))))).lock().unwrap().as_ref().unwrap()));

        // String searching
println!("{}", "\n=== String searching ===".to_string());
    let mut searchText = Arc::new(Mutex::new(Some("The quick brown fox jumps over the lazy dog".to_string())));
    print!("Text: {}\n", (*searchText.lock().unwrap().as_mut().unwrap()));

    print!("Contains 'fox': {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).contains(Arc::new(Mutex::new(Some((*searchText.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("fox".to_string())))).lock().unwrap().as_ref().unwrap()));
    print!("Contains 'cat': {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).contains(Arc::new(Mutex::new(Some((*searchText.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("cat".to_string())))).lock().unwrap().as_ref().unwrap()));

    print!("Index of 'fox': {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).index(Arc::new(Mutex::new(Some((*searchText.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("fox".to_string())))).lock().unwrap().as_ref().unwrap()));
    print!("Index of 'cat': {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).index(Arc::new(Mutex::new(Some((*searchText.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("cat".to_string())))).lock().unwrap().as_ref().unwrap()));

    print!("Last index of 'the': {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).last_index(Arc::new(Mutex::new(Some((*searchText.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("the".to_string())))).lock().unwrap().as_ref().unwrap()));

    print!("Count of 'the': {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).count(Arc::new(Mutex::new(Some((*searchText.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("the".to_string())))).lock().unwrap().as_ref().unwrap()));

        // String prefixes and suffixes
println!("{}", "\n=== Prefixes and suffixes ===".to_string());
    let mut filename = Arc::new(Mutex::new(Some("document.txt".to_string())));
    print!("Filename: {}\n", (*filename.lock().unwrap().as_mut().unwrap()));
    print!("Has .txt suffix: {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).has_suffix(Arc::new(Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some(".txt".to_string())))).lock().unwrap().as_ref().unwrap()));
    print!("Has .pdf suffix: {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).has_suffix(Arc::new(Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some(".pdf".to_string())))).lock().unwrap().as_ref().unwrap()));
    print!("Has 'doc' prefix: {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).has_prefix(Arc::new(Mutex::new(Some((*filename.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("doc".to_string())))).lock().unwrap().as_ref().unwrap()));

        // String splitting and joining
println!("{}", "\n=== Splitting and joining ===".to_string());
    let mut csv = Arc::new(Mutex::new(Some("apple,banana,cherry,date".to_string())));
    print!("CSV: {}\n", (*csv.lock().unwrap().as_mut().unwrap()));

    let mut fruits = (*strings.lock().unwrap().as_mut().unwrap()).split(Arc::new(Mutex::new(Some((*csv.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some(",".to_string()))));
    print!("Split result: {}\n", format_slice(&fruits));

    let mut rejoined = (*strings.lock().unwrap().as_mut().unwrap()).join(Arc::new(Mutex::new(Some((*fruits.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some(" | ".to_string()))));
    print!("Rejoined: {}\n", (*rejoined.lock().unwrap().as_mut().unwrap()));

        // Fields (split on whitespace)
let mut sentence = Arc::new(Mutex::new(Some("The quick brown fox".to_string())));
    let mut words = (*strings.lock().unwrap().as_mut().unwrap()).fields(Arc::new(Mutex::new(Some((*sentence.lock().unwrap().as_mut().unwrap())))));
    print!("Words: {}\n", format_slice(&words));

        // String replacement
println!("{}", "\n=== String replacement ===".to_string());
    let mut original = Arc::new(Mutex::new(Some("I like cats and cats like me".to_string())));
    print!("Original: {}\n", (*original.lock().unwrap().as_mut().unwrap()));

    let mut replaced = (*strings.lock().unwrap().as_mut().unwrap()).replace(Arc::new(Mutex::new(Some((*original.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("cats".to_string()))), Arc::new(Mutex::new(Some("dogs".to_string()))), Arc::new(Mutex::new(Some(1))));
    print!("Replace first 'cats': {}\n", (*replaced.lock().unwrap().as_mut().unwrap()));

    let mut replacedAll = (*strings.lock().unwrap().as_mut().unwrap()).replace_all(Arc::new(Mutex::new(Some((*original.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("cats".to_string()))), Arc::new(Mutex::new(Some("dogs".to_string()))));
    print!("Replace all 'cats': {}\n", (*replacedAll.lock().unwrap().as_mut().unwrap()));

        // String repetition
println!("{}", "\n=== String repetition ===".to_string());
    let mut pattern = Arc::new(Mutex::new(Some("Go! ".to_string())));
    let mut repeated = (*strings.lock().unwrap().as_mut().unwrap()).repeat(Arc::new(Mutex::new(Some((*pattern.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some(3))));
    print!("Repeated: {}\n", (*repeated.lock().unwrap().as_mut().unwrap()));

        // String comparison
println!("{}", "\n=== String comparison ===".to_string());
    let mut str1 = Arc::new(Mutex::new(Some("apple".to_string())));
    let mut str2 = Arc::new(Mutex::new(Some("banana".to_string())));
    let mut str3 = Arc::new(Mutex::new(Some("apple".to_string())));

    print!("'{}' == '{}': {}\n", (*str1.lock().unwrap().as_mut().unwrap()), (*str2.lock().unwrap().as_mut().unwrap()), (*str1.lock().unwrap().as_mut().unwrap()) == (*str2.lock().unwrap().as_mut().unwrap()));
    print!("'{}' == '{}': {}\n", (*str1.lock().unwrap().as_mut().unwrap()), (*str3.lock().unwrap().as_mut().unwrap()), (*str1.lock().unwrap().as_mut().unwrap()) == (*str3.lock().unwrap().as_mut().unwrap()));
    print!("'{}' < '{}': {}\n", (*str1.lock().unwrap().as_mut().unwrap()), (*str2.lock().unwrap().as_mut().unwrap()), (*str1.lock().unwrap().as_mut().unwrap()) < (*str2.lock().unwrap().as_mut().unwrap()));

        // Case-insensitive comparison
print!("EqualFold('Apple', 'APPLE'): {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).equal_fold(Arc::new(Mutex::new(Some("Apple".to_string()))), Arc::new(Mutex::new(Some("APPLE".to_string())))).lock().unwrap().as_ref().unwrap()));

        // String building with strings.Builder
println!("{}", "\n=== String building ===".to_string());
    let mut builder: Arc<Mutex<Option</* TODO: Unhandled type *ast.SelectorExpr */ Arc<Mutex<Option<()>>>>>>;

    (*builder.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("Building ".to_string()))));
    (*builder.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("a ".to_string()))));
    (*builder.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("string ".to_string()))));
    (*builder.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("efficiently".to_string()))));

    let mut built = (*builder.lock().unwrap().as_mut().unwrap()).string();
    print!("Built string: {}\n", (*built.lock().unwrap().as_mut().unwrap()));
    print!("Builder length: {}\n", (*(*builder.lock().unwrap().as_mut().unwrap()).len().lock().unwrap().as_ref().unwrap()));

        // Rune iteration (Unicode support)
println!("{}", "\n=== Unicode and runes ===".to_string());
    let mut unicode = Arc::new(Mutex::new(Some("Hello, 世界! 🌍".to_string())));
    print!("Unicode string: {}\n", (*unicode.lock().unwrap().as_mut().unwrap()));
    print!("Byte length: {}\n", (*unicode.lock().unwrap().as_ref().unwrap()).len());

    let mut runeCount = Arc::new(Mutex::new(Some(0)));
    for (_, r) in (*(*unicode.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).chars().enumerate() {
        { let mut guard = runeCount.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        print!("Rune: {} (U+%04X)\n", r, r);
    }
    print!("Rune count: {}\n", (*runeCount.lock().unwrap().as_mut().unwrap()));

        // String trimming variations
println!("{}", "\n=== String trimming variations ===".to_string());
    let mut messy = Arc::new(Mutex::new(Some("!!!Hello World!!!".to_string())));
    print!("Original: {}\n", (*messy.lock().unwrap().as_mut().unwrap()));
    print!("TrimLeft '!': {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).trim_left(Arc::new(Mutex::new(Some((*messy.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("!".to_string())))).lock().unwrap().as_ref().unwrap()));
    print!("TrimRight '!': {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).trim_right(Arc::new(Mutex::new(Some((*messy.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("!".to_string())))).lock().unwrap().as_ref().unwrap()));
    print!("Trim '!': {}\n", (*(*strings.lock().unwrap().as_mut().unwrap()).trim(Arc::new(Mutex::new(Some((*messy.lock().unwrap().as_mut().unwrap())))), Arc::new(Mutex::new(Some("!".to_string())))).lock().unwrap().as_ref().unwrap()));

        // String formatting with different verbs
println!("{}", "\n=== String formatting ===".to_string());
    let mut name = Arc::new(Mutex::new(Some("Alice".to_string())));
    let mut age = Arc::new(Mutex::new(Some(30)));
    let mut height = Arc::new(Mutex::new(Some(5.6)));

    print!("Name: {}, Age: {}, Height: {:.1}\n", (*name.lock().unwrap().as_mut().unwrap()), (*age.lock().unwrap().as_mut().unwrap()), (*height.lock().unwrap().as_mut().unwrap()));
    print!("Quoted string: %q\n", (*name.lock().unwrap().as_mut().unwrap()));
    print!("String with width: '%10s'\n", (*name.lock().unwrap().as_mut().unwrap()));
    print!("Left-aligned: '%-10s'\n", (*name.lock().unwrap().as_mut().unwrap()));
}