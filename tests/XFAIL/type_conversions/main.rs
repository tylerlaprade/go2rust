use num::Complex;
use std::any::Any;
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
    let mut f: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as f64)));
    let mut i2: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some((*f.lock().unwrap().as_ref().unwrap()) as i32)));

    print!("int: {}\n", (*i.lock().unwrap().as_mut().unwrap()));
    print!("float64: {:.2}\n", (*f.lock().unwrap().as_mut().unwrap()));
    print!("back to int: {}\n", (*i2.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Integer size conversions ===".to_string());

    let mut i8: Arc<Mutex<Option<i8>>> = Arc::new(Mutex::new(Some(127)));
    let mut i16: Arc<Mutex<Option<i16>>> = Arc::new(Mutex::new(Some((*i8.lock().unwrap().as_ref().unwrap()) as i16)));
    let mut i32: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some((*i16.lock().unwrap().as_ref().unwrap()) as i32)));
    let mut i64: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some((*i32.lock().unwrap().as_ref().unwrap()) as i64)));

    print!("int8: {}\n", (*i8.lock().unwrap().as_mut().unwrap()));
    print!("int16: {}\n", (*i16.lock().unwrap().as_mut().unwrap()));
    print!("int32: {}\n", (*i32.lock().unwrap().as_mut().unwrap()));
    print!("int64: {}\n", (*i64.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Unsigned integer conversions ===".to_string());

    let mut ui: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(42)));
    let mut ui8: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some((*ui.lock().unwrap().as_ref().unwrap()) as u8)));
    let mut ui16: Arc<Mutex<Option<u16>>> = Arc::new(Mutex::new(Some((*ui8.lock().unwrap().as_ref().unwrap()) as u16)));
    let mut ui32: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some((*ui16.lock().unwrap().as_ref().unwrap()) as u32)));
    let mut ui64: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some((*ui32.lock().unwrap().as_ref().unwrap()) as u64)));

    print!("uint: {}\n", (*ui.lock().unwrap().as_mut().unwrap()));
    print!("uint8: {}\n", (*ui8.lock().unwrap().as_mut().unwrap()));
    print!("uint16: {}\n", (*ui16.lock().unwrap().as_mut().unwrap()));
    print!("uint32: {}\n", (*ui32.lock().unwrap().as_mut().unwrap()));
    print!("uint64: {}\n", (*ui64.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Float conversions ===".to_string());

    let mut f64: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(3.14159265359)));
    let mut f32: Arc<Mutex<Option<f32>>> = Arc::new(Mutex::new(Some((*f64.lock().unwrap().as_ref().unwrap()) as f32)));
    let mut backToF64: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some((*f32.lock().unwrap().as_ref().unwrap()) as f64)));

    print!("float64: %.10f\n", (*f64.lock().unwrap().as_mut().unwrap()));
    print!("float32: %.10f\n", (*f32.lock().unwrap().as_mut().unwrap()));
    print!("back to float64: %.10f\n", (*backToF64.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== String conversions ===".to_string());

    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some('A')));
    let mut b: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(65)));

    print!("rune 'A': {} ({})\n", (*r.lock().unwrap().as_mut().unwrap()), (*r.lock().unwrap().as_mut().unwrap()));
    print!("byte 65: {} ({})\n", (*b.lock().unwrap().as_mut().unwrap()), (*b.lock().unwrap().as_mut().unwrap()));

    let mut str = Arc::new(Mutex::new(Some(char::from_u32((*r.lock().unwrap().as_ref().unwrap()) as u32).unwrap().to_string())));
    print!("rune to string: {}\n", (*str.lock().unwrap().as_mut().unwrap()));

    let mut bytes = Arc::new(Mutex::new(Some(vec![72, 101, 108, 108, 111])));
    let mut strFromBytes = Arc::new(Mutex::new(Some(String::from_utf8((*bytes.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    print!("bytes to string: {}\n", (*strFromBytes.lock().unwrap().as_mut().unwrap()));

    let mut backToBytes = Arc::new(Mutex::new(Some((*strFromBytes.lock().unwrap().as_ref().unwrap()).as_bytes().to_vec())));
    print!("string to bytes: {}\n", format_slice(&backToBytes));

    let mut runes = Arc::new(Mutex::new(Some((*"Hello, 世界".to_string().lock().unwrap().as_ref().unwrap()).chars().map(|c| c as i32).collect::<Vec<_>>())));
    print!("string to runes: {}\n", format_slice(&runes));
    print!("rune count: {}\n", (*runes.lock().unwrap().as_mut().unwrap()).len());

    let mut backToString = Arc::new(Mutex::new(Some((*runes.lock().unwrap().as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>())));
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
    print!("pointer: {}\n", "0xDEADBEEF".to_string());
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

    let mut c64: Arc<Mutex<Option<num::Complex<f32>>>> = Arc::new(Mutex::new(Some(3 + 4i)));
    let mut c128: Arc<Mutex<Option<num::Complex<f64>>>> = Arc::new(Mutex::new(Some(num::Complex::<f64>::new((*c64.lock().unwrap().as_ref().unwrap()) as f64, 0.0))));

    print!("complex64: {}\n", (*c64.lock().unwrap().as_mut().unwrap()));
    print!("complex128: {}\n", (*c128.lock().unwrap().as_mut().unwrap()));

    let mut real = Arc::new(Mutex::new(Some((*(*c128.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).re)));
    let mut imag = Arc::new(Mutex::new(Some((*(*c128.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).im)));
    print!("real part: {:.2}\n", (*real.lock().unwrap().as_mut().unwrap()));
    print!("imaginary part: {:.2}\n", (*imag.lock().unwrap().as_mut().unwrap()));

    let mut newComplex = Arc::new(Mutex::new(Some(num::Complex::new(*(*real.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap(), *(*imag.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()))));
    print!("reconstructed: {}\n", (*newComplex.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Overflow examples ===".to_string());

    let mut bigInt: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(1000000)));
    let mut smallInt: Arc<Mutex<Option<i8>>> = Arc::new(Mutex::new(Some((*bigInt.lock().unwrap().as_ref().unwrap()) as i8)));
    print!("int64: {}\n", (*bigInt.lock().unwrap().as_mut().unwrap()));
    print!("int8 (overflow): {}\n", (*smallInt.lock().unwrap().as_mut().unwrap()));

    let mut preciseFloat: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(1.23456789012345)));
    let mut lessPrec: Arc<Mutex<Option<f32>>> = Arc::new(Mutex::new(Some((*preciseFloat.lock().unwrap().as_ref().unwrap()) as f32)));
    print!("float64: %.15f\n", (*preciseFloat.lock().unwrap().as_mut().unwrap()));
    print!("float32: %.15f\n", (*lessPrec.lock().unwrap().as_mut().unwrap()));

    println!("{}", "\n=== Custom type conversions ===".to_string());

    type MyInt = Arc<Mutex<Option<i32>>>;
    type MyString = Arc<Mutex<Option<String>>>;

    let mut mi: Arc<Mutex<Option<MyInt>>> = Arc::new(Mutex::new(Some(42)));
    let mut regularInt: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some((*mi.lock().unwrap().as_ref().unwrap()) as i32)));
    let mut backToMyInt: Arc<Mutex<Option<MyInt>>> = (*regularInt.lock().unwrap().as_mut().unwrap());

    print!("MyInt: {}\n", (*mi.lock().unwrap().as_mut().unwrap()));
    print!("regular int: {}\n", (*regularInt.lock().unwrap().as_mut().unwrap()));
    print!("back to MyInt: {}\n", (*backToMyInt.lock().unwrap().as_mut().unwrap()));

    let mut ms: Arc<Mutex<Option<MyString>>> = Arc::new(Mutex::new(Some("hello".to_string())));
    let mut regularString: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some((*ms.lock().unwrap().as_ref().unwrap()).to_string())));
    let mut backToMyString: Arc<Mutex<Option<MyString>>> = (*regularString.lock().unwrap().as_mut().unwrap());

    print!("MyString: {}\n", (*ms.lock().unwrap().as_mut().unwrap()));
    print!("regular string: {}\n", (*regularString.lock().unwrap().as_mut().unwrap()));
    print!("back to MyString: {}\n", (*backToMyString.lock().unwrap().as_mut().unwrap()));
}