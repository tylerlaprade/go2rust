fn format_map<K: std::fmt::Display + std::cmp::Ord + Clone, V>(map: &std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<K, std::sync::Arc<std::sync::Mutex<Option<V>>>>>>>) -> String 
where
    V: std::fmt::Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());
        
        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();
        
        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T>(slice: &std::sync::Arc<std::sync::Mutex<Option<Vec<T>>>>) -> String 
where
    T: std::fmt::Display,
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
    println!("{}", "=== Basic numeric conversions ===".to_string());
    let mut i: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    let mut f: std::sync::Arc<std::sync::Mutex<Option<f64>>> = float64(i.clone());
    let mut i2: std::sync::Arc<std::sync::Mutex<Option<i32>>> = int(f.clone());
    print!("int: {}\n", (*i.lock().unwrap().as_mut().unwrap()));
    print!("float64: {:.2}\n", (*f.lock().unwrap().as_mut().unwrap()));
    print!("back to int: {}\n", (*i2.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== Integer size conversions ===".to_string());
    let mut i8: std::sync::Arc<std::sync::Mutex<Option<int8>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(127)));
    let mut i16: std::sync::Arc<std::sync::Mutex<Option<int16>>> = int16(i8.clone());
    let mut i32: std::sync::Arc<std::sync::Mutex<Option<int32>>> = int32(i16.clone());
    let mut i64: std::sync::Arc<std::sync::Mutex<Option<i64>>> = int64(i32.clone());
    print!("int8: {}\n", (*i8.lock().unwrap().as_mut().unwrap()));
    print!("int16: {}\n", (*i16.lock().unwrap().as_mut().unwrap()));
    print!("int32: {}\n", (*i32.lock().unwrap().as_mut().unwrap()));
    print!("int64: {}\n", (*i64.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== Unsigned integer conversions ===".to_string());
    let mut ui: std::sync::Arc<std::sync::Mutex<Option<uint>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    let mut ui8: std::sync::Arc<std::sync::Mutex<Option<uint8>>> = uint8(ui.clone());
    let mut ui16: std::sync::Arc<std::sync::Mutex<Option<uint16>>> = uint16(ui8.clone());
    let mut ui32: std::sync::Arc<std::sync::Mutex<Option<uint32>>> = uint32(ui16.clone());
    let mut ui64: std::sync::Arc<std::sync::Mutex<Option<uint64>>> = uint64(ui32.clone());
    print!("uint: {}\n", (*ui.lock().unwrap().as_mut().unwrap()));
    print!("uint8: {}\n", (*ui8.lock().unwrap().as_mut().unwrap()));
    print!("uint16: {}\n", (*ui16.lock().unwrap().as_mut().unwrap()));
    print!("uint32: {}\n", (*ui32.lock().unwrap().as_mut().unwrap()));
    print!("uint64: {}\n", (*ui64.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== Float conversions ===".to_string());
    let mut f64: std::sync::Arc<std::sync::Mutex<Option<f64>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(3.14159265359)));
    let mut f32: std::sync::Arc<std::sync::Mutex<Option<float32>>> = float32(f64.clone());
    let mut backToF64: std::sync::Arc<std::sync::Mutex<Option<f64>>> = float64(f32.clone());
    print!("float64: %.10f\n", (*f64.lock().unwrap().as_mut().unwrap()));
    print!("float32: %.10f\n", (*f32.lock().unwrap().as_mut().unwrap()));
    print!("back to float64: %.10f\n", (*backToF64.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== String conversions ===".to_string());
    let mut r: std::sync::Arc<std::sync::Mutex<Option<rune>>> = std::sync::Arc::new(std::sync::Mutex::new(Some('A')));
    let mut b: std::sync::Arc<std::sync::Mutex<Option<byte>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(65)));
    print!("rune 'A': {} ({})\n", (*r.lock().unwrap().as_mut().unwrap()), (*r.lock().unwrap().as_mut().unwrap()));
    print!("byte 65: {} ({})\n", (*b.lock().unwrap().as_mut().unwrap()), (*b.lock().unwrap().as_mut().unwrap()));
    let mut str = string(r.clone());
    print!("rune to string: {}\n", (*str.lock().unwrap().as_mut().unwrap()));
    let mut bytes = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![72, 101, 108, 108, 111])));
    let mut strFromBytes = string(bytes.clone());
    print!("bytes to string: {}\n", (*strFromBytes.lock().unwrap().as_mut().unwrap()));
    let mut backToBytes = (strFromBytes.clone());
    print!("string to bytes: {}\n", (*backToBytes.lock().unwrap().as_mut().unwrap()));
    let mut runes = (std::sync::Arc::new(std::sync::Mutex::new(Some("Hello, 世界".to_string()))));
    print!("string to runes: {}\n", (*runes.lock().unwrap().as_mut().unwrap()));
    print!("rune count: {}\n", (*runes.lock().unwrap().as_mut().unwrap()).len());
    let mut backToString = string(runes.clone());
    print!("runes to string: {}\n", (*backToString.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== Boolean-like conversions ===".to_string());
    let mut zero: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    let mut nonZero: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    print!("zero == 0: {}\n", (*zero.lock().unwrap().as_mut().unwrap()) == 0);
    print!("nonZero != 0: {}\n", (*nonZero.lock().unwrap().as_mut().unwrap()) != 0);
    println!("{}", "\n=== Pointer conversions ===".to_string());
    let mut num: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(100)));
    let mut ptr: std::sync::Arc<std::sync::Mutex<Option<i32>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(num.clone())));
    print!("value: {}\n", (*num.lock().unwrap().as_mut().unwrap()));
    print!("pointer: %p\n", (*ptr.lock().unwrap().as_mut().unwrap()));
    print!("dereferenced: {}\n", (*ptr.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== Interface conversions ===".to_string());
    let mut any: std::sync::Arc<std::sync::Mutex<Option<Box<dyn std::any::Any>>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    print!("interface{} value: {}\n", (*any.lock().unwrap().as_mut().unwrap()));
    print!("interface{} type: %T\n", (*any.lock().unwrap().as_mut().unwrap()));
    let (mut intVal, mut ok) = match (*any.lock().unwrap().as_mut().unwrap()).downcast_ref::<i32>() { Some(v) => (v.clone(), true), None => (0, false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("asserted as int: {}\n", (*intVal.lock().unwrap().as_mut().unwrap()));
    }
    { let new_val = "hello".to_string(); *any.lock().unwrap() = Some(new_val); };
    print!("new interface{} value: {}\n", (*any.lock().unwrap().as_mut().unwrap()));
    print!("new interface{} type: %T\n", (*any.lock().unwrap().as_mut().unwrap()));
    let (mut strVal, mut ok) = match (*any.lock().unwrap().as_mut().unwrap()).downcast_ref::<String>() { Some(v) => (v.clone(), true), None => (String::new(), false) };
    if (*ok.lock().unwrap().as_mut().unwrap()) {
        print!("asserted as string: {}\n", (*strVal.lock().unwrap().as_mut().unwrap()));
    }
    println!("{}", "\n=== Complex number conversions ===".to_string());
    let mut c64: std::sync::Arc<std::sync::Mutex<Option<complex64>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(3 + 4i)));
    let mut c128: std::sync::Arc<std::sync::Mutex<Option<complex128>>> = complex128(c64.clone());
    print!("complex64: {}\n", (*c64.lock().unwrap().as_mut().unwrap()));
    print!("complex128: {}\n", (*c128.lock().unwrap().as_mut().unwrap()));
    let mut real = real(c128.clone());
    let mut imag = imag(c128.clone());
    print!("real part: {:.2}\n", (*real.lock().unwrap().as_mut().unwrap()));
    print!("imaginary part: {:.2}\n", (*imag.lock().unwrap().as_mut().unwrap()));
    let mut newComplex = complex(real.clone(), imag.clone());
    print!("reconstructed: {}\n", (*newComplex.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== Overflow examples ===".to_string());
    let mut bigInt: std::sync::Arc<std::sync::Mutex<Option<i64>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(1000000)));
    let mut smallInt: std::sync::Arc<std::sync::Mutex<Option<int8>>> = int8(bigInt.clone());
    print!("int64: {}\n", (*bigInt.lock().unwrap().as_mut().unwrap()));
    print!("int8 (overflow): {}\n", (*smallInt.lock().unwrap().as_mut().unwrap()));
    let mut preciseFloat: std::sync::Arc<std::sync::Mutex<Option<f64>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(1.23456789012345)));
    let mut lessPrec: std::sync::Arc<std::sync::Mutex<Option<float32>>> = float32(preciseFloat.clone());
    print!("float64: %.15f\n", (*preciseFloat.lock().unwrap().as_mut().unwrap()));
    print!("float32: %.15f\n", (*lessPrec.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== Custom type conversions ===".to_string());
    
    
    let mut mi: std::sync::Arc<std::sync::Mutex<Option<MyInt>>> = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    let mut regularInt: std::sync::Arc<std::sync::Mutex<Option<i32>>> = int(mi.clone());
    let mut backToMyInt: std::sync::Arc<std::sync::Mutex<Option<MyInt>>> = my_int(regularInt.clone());
    print!("MyInt: {}\n", (*mi.lock().unwrap().as_mut().unwrap()));
    print!("regular int: {}\n", (*regularInt.lock().unwrap().as_mut().unwrap()));
    print!("back to MyInt: {}\n", (*backToMyInt.lock().unwrap().as_mut().unwrap()));
    let mut ms: std::sync::Arc<std::sync::Mutex<Option<MyString>>> = std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string())));
    let mut regularString: std::sync::Arc<std::sync::Mutex<Option<String>>> = string(ms.clone());
    let mut backToMyString: std::sync::Arc<std::sync::Mutex<Option<MyString>>> = my_string(regularString.clone());
    print!("MyString: {}\n", (*ms.lock().unwrap().as_mut().unwrap()));
    print!("regular string: {}\n", (*regularString.lock().unwrap().as_mut().unwrap()));
    print!("back to MyString: {}\n", (*backToMyString.lock().unwrap().as_mut().unwrap()));
}