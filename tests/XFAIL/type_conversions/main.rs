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
    println!("{}", "=== Basic numeric conversions ===".to_string());

    let mut i: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(42)));
    let mut f: Arc<Mutex<Option<f64>>> = (float64.lock().unwrap().as_ref().unwrap())(i.clone());
    let mut i2: Arc<Mutex<Option<i32>>> = (int.lock().unwrap().as_ref().unwrap())(f.clone());

    print!("int: {}\n", (*i.lock().unwrap().as_mut().unwrap()));
    print!("float64: {:.2}\n", (*f.lock().unwrap().as_mut().unwrap()));
    print!("back to int: {}\n", (*i2.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Integer size conversions ===".to_string());

    let mut i8: Arc<Mutex<Option<int8>>> = Arc::new(Mutex::new(Some(127)));
    let mut i16: Arc<Mutex<Option<int16>>> = (int16.lock().unwrap().as_ref().unwrap())(i8.clone());
    let mut i32: Arc<Mutex<Option<int32>>> = (int32.lock().unwrap().as_ref().unwrap())(i16.clone());
    let mut i64: Arc<Mutex<Option<i64>>> = (int64.lock().unwrap().as_ref().unwrap())(i32.clone());

    print!("int8: {}\n", (*i8.lock().unwrap().as_mut().unwrap()));
    print!("int16: {}\n", (*i16.lock().unwrap().as_mut().unwrap()));
    print!("int32: {}\n", (*i32.lock().unwrap().as_mut().unwrap()));
    print!("int64: {}\n", (*i64.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Unsigned integer conversions ===".to_string());

    let mut ui: Arc<Mutex<Option<uint>>> = Arc::new(Mutex::new(Some(42)));
    let mut ui8: Arc<Mutex<Option<uint8>>> = (uint8.lock().unwrap().as_ref().unwrap())(ui.clone());
    let mut ui16: Arc<Mutex<Option<uint16>>> = (uint16.lock().unwrap().as_ref().unwrap())(ui8.clone());
    let mut ui32: Arc<Mutex<Option<uint32>>> = (uint32.lock().unwrap().as_ref().unwrap())(ui16.clone());
    let mut ui64: Arc<Mutex<Option<uint64>>> = (uint64.lock().unwrap().as_ref().unwrap())(ui32.clone());

    print!("uint: {}\n", (*ui.lock().unwrap().as_mut().unwrap()));
    print!("uint8: {}\n", (*ui8.lock().unwrap().as_mut().unwrap()));
    print!("uint16: {}\n", (*ui16.lock().unwrap().as_mut().unwrap()));
    print!("uint32: {}\n", (*ui32.lock().unwrap().as_mut().unwrap()));
    print!("uint64: {}\n", (*ui64.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Float conversions ===".to_string());

    let mut f64: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(3.14159265359)));
    let mut f32: Arc<Mutex<Option<float32>>> = (float32.lock().unwrap().as_ref().unwrap())(f64.clone());
    let mut backToF64: Arc<Mutex<Option<f64>>> = (float64.lock().unwrap().as_ref().unwrap())(f32.clone());

    print!("float64: %.10f\n", (*f64.lock().unwrap().as_mut().unwrap()));
    print!("float32: %.10f\n", (*f32.lock().unwrap().as_mut().unwrap()));
    print!("back to float64: %.10f\n", (*backToF64.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== String conversions ===".to_string());

    let mut r: Arc<Mutex<Option<rune>>> = Arc::new(Mutex::new(Some('A')));
    let mut b: Arc<Mutex<Option<byte>>> = Arc::new(Mutex::new(Some(65)));

    print!("rune 'A': {} ({})\n", (*r.lock().unwrap().as_mut().unwrap()), (*r.lock().unwrap().as_mut().unwrap()));
    print!("byte 65: {} ({})\n", (*b.lock().unwrap().as_mut().unwrap()), (*b.lock().unwrap().as_mut().unwrap()));

    let mut str = (string.lock().unwrap().as_ref().unwrap())(r.clone());
    print!("rune to string: {}\n", (*str.lock().unwrap().as_mut().unwrap()));

    let mut bytes = Arc::new(Mutex::new(Some(vec![72, 101, 108, 108, 111])));
    let mut strFromBytes = (string.lock().unwrap().as_ref().unwrap())(bytes.clone());
    print!("bytes to string: {}\n", (*strFromBytes.lock().unwrap().as_mut().unwrap()));

    let mut backToBytes = (/* TODO: Unhandled expression type: ArrayType */ Arc::new(Mutex::new(Some(()))).lock().unwrap().as_ref().unwrap())(strFromBytes.clone());
    print!("string to bytes: {}\n", format_slice(&backToBytes));

    let mut runes = (/* TODO: Unhandled expression type: ArrayType */ Arc::new(Mutex::new(Some(()))).lock().unwrap().as_ref().unwrap())(Arc::new(Mutex::new(Some("Hello, 世界".to_string()))));
    print!("string to runes: {}\n", format_slice(&runes));
    print!("rune count: {}\n", (*runes.lock().unwrap().as_mut().unwrap()).len());

    let mut backToString = (string.lock().unwrap().as_ref().unwrap())(runes.clone());
    print!("runes to string: {}\n", (*backToString.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Boolean-like conversions ===".to_string());

    let mut zero: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut nonZero: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(42)));

    print!("zero == 0: {}\n", (*zero.lock().unwrap().as_mut().unwrap()) == 0);
    print!("nonZero != 0: {}\n", (*nonZero.lock().unwrap().as_mut().unwrap()) != 0);

    println!("{}", "\n=== Pointer conversions ===".to_string());

    let mut num: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(100)));
    let mut ptr: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(num.clone())));

    print!("value: {}\n", (*num.lock().unwrap().as_mut().unwrap()));
    print!("pointer: %p\n", (*ptr.lock().unwrap().as_mut().unwrap()));
    print!("dereferenced: {}\n", (*ptr.lock().unwrap().as_ref().unwrap()));

    println!("{}", "\n=== Interface conversions ===".to_string());

    let mut any: Arc<Mutex<Option<Box<dyn Any>>>> = Arc::new(Mutex::new(Some(42)));
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

    let mut c64: Arc<Mutex<Option<complex64>>> = Arc::new(Mutex::new(Some(3 + 4i)));
    let mut c128: Arc<Mutex<Option<complex128>>> = (complex128.lock().unwrap().as_ref().unwrap())(c64.clone());

    print!("complex64: {}\n", (*c64.lock().unwrap().as_mut().unwrap()));
    print!("complex128: {}\n", (*c128.lock().unwrap().as_mut().unwrap()));

    let mut real = (real.lock().unwrap().as_ref().unwrap())(c128.clone());
    let mut imag = (imag.lock().unwrap().as_ref().unwrap())(c128.clone());
    print!("real part: {:.2}\n", (*real.lock().unwrap().as_mut().unwrap()));
    print!("imaginary part: {:.2}\n", (*imag.lock().unwrap().as_mut().unwrap()));

    let mut newComplex = (complex.lock().unwrap().as_ref().unwrap())(real.clone(), imag.clone());
    print!("reconstructed: {}\n", (*newComplex.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Overflow examples ===".to_string());

    let mut bigInt: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(1000000)));
    let mut smallInt: Arc<Mutex<Option<int8>>> = (int8.lock().unwrap().as_ref().unwrap())(bigInt.clone());
    print!("int64: {}\n", (*bigInt.lock().unwrap().as_mut().unwrap()));
    print!("int8 (overflow): {}\n", (*smallInt.lock().unwrap().as_mut().unwrap()));

    let mut preciseFloat: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(1.23456789012345)));
    let mut lessPrec: Arc<Mutex<Option<float32>>> = (float32.lock().unwrap().as_ref().unwrap())(preciseFloat.clone());
    print!("float64: %.15f\n", (*preciseFloat.lock().unwrap().as_mut().unwrap()));
    print!("float32: %.15f\n", (*lessPrec.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Custom type conversions ===".to_string());

    
    

    let mut mi: Arc<Mutex<Option<MyInt>>> = Arc::new(Mutex::new(Some(42)));
    let mut regularInt: Arc<Mutex<Option<i32>>> = (int.lock().unwrap().as_ref().unwrap())(mi.clone());
    let mut backToMyInt: Arc<Mutex<Option<MyInt>>> = (MyInt.lock().unwrap().as_ref().unwrap())(regularInt.clone());

    print!("MyInt: {}\n", (*mi.lock().unwrap().as_mut().unwrap()));
    print!("regular int: {}\n", (*regularInt.lock().unwrap().as_mut().unwrap()));
    print!("back to MyInt: {}\n", (*backToMyInt.lock().unwrap().as_mut().unwrap()));

    let mut ms: Arc<Mutex<Option<MyString>>> = Arc::new(Mutex::new(Some("hello".to_string())));
    let mut regularString: Arc<Mutex<Option<String>>> = (string.lock().unwrap().as_ref().unwrap())(ms.clone());
    let mut backToMyString: Arc<Mutex<Option<MyString>>> = (MyString.lock().unwrap().as_ref().unwrap())(regularString.clone());

    print!("MyString: {}\n", (*ms.lock().unwrap().as_mut().unwrap()));
    print!("regular string: {}\n", (*regularString.lock().unwrap().as_mut().unwrap()));
    print!("back to MyString: {}\n", (*backToMyString.lock().unwrap().as_mut().unwrap()));
}