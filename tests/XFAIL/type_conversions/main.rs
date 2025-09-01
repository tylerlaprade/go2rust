use num::Complex;
use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T>(slice: &Rc<RefCell<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_any(value: &dyn Any) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else {
        "<unknown>".to_string()
    }
}

fn main() {
        // Basic numeric type conversions
    println!("{}", "=== Basic numeric conversions ===".to_string());

    let mut i: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(42)));
    let mut f: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some((*i.as_ref().unwrap().as_ref().unwrap()) as f64)));
    let mut i2: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some((*f.as_ref().unwrap().as_ref().unwrap()) as i32)));

    print!("int: {}\n", (*i.borrow_mut().as_mut().unwrap()));
    print!("float64: {:.2}\n", (*f.borrow_mut().as_mut().unwrap()));
    print!("back to int: {}\n", (*i2.borrow_mut().as_mut().unwrap()));

        // Different integer sizes
    println!("{}", "\n=== Integer size conversions ===".to_string());

    let mut i8: Rc<RefCell<Option<i8>>> = Rc::new(RefCell::new(Some(127)));
    let mut i16: Rc<RefCell<Option<i16>>> = Rc::new(RefCell::new(Some((*i8.as_ref().unwrap().as_ref().unwrap()) as i16)));
    let mut i32: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some((*i16.as_ref().unwrap().as_ref().unwrap()) as i32)));
    let mut i64: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some((*i32.as_ref().unwrap().as_ref().unwrap()) as i64)));

    print!("int8: {}\n", (*i8.borrow_mut().as_mut().unwrap()));
    print!("int16: {}\n", (*i16.borrow_mut().as_mut().unwrap()));
    print!("int32: {}\n", (*i32.borrow_mut().as_mut().unwrap()));
    print!("int64: {}\n", (*i64.borrow_mut().as_mut().unwrap()));

        // Unsigned integers
    println!("{}", "\n=== Unsigned integer conversions ===".to_string());

    let mut ui: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(Some(42)));
    let mut ui8: Rc<RefCell<Option<u8>>> = Rc::new(RefCell::new(Some((*ui.as_ref().unwrap().as_ref().unwrap()) as u8)));
    let mut ui16: Rc<RefCell<Option<u16>>> = Rc::new(RefCell::new(Some((*ui8.as_ref().unwrap().as_ref().unwrap()) as u16)));
    let mut ui32: Rc<RefCell<Option<u32>>> = Rc::new(RefCell::new(Some((*ui16.as_ref().unwrap().as_ref().unwrap()) as u32)));
    let mut ui64: Rc<RefCell<Option<u64>>> = Rc::new(RefCell::new(Some((*ui32.as_ref().unwrap().as_ref().unwrap()) as u64)));

    print!("uint: {}\n", (*ui.borrow_mut().as_mut().unwrap()));
    print!("uint8: {}\n", (*ui8.borrow_mut().as_mut().unwrap()));
    print!("uint16: {}\n", (*ui16.borrow_mut().as_mut().unwrap()));
    print!("uint32: {}\n", (*ui32.borrow_mut().as_mut().unwrap()));
    print!("uint64: {}\n", (*ui64.borrow_mut().as_mut().unwrap()));

        // Float conversions
    println!("{}", "\n=== Float conversions ===".to_string());

    let mut f64: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(3.14159265359)));
    let mut f32: Rc<RefCell<Option<f32>>> = Rc::new(RefCell::new(Some((*f64.as_ref().unwrap().as_ref().unwrap()) as f32)));
    let mut backToF64: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some((*f32.as_ref().unwrap().as_ref().unwrap()) as f64)));

    print!("float64: {:.10}\n", (*f64.borrow_mut().as_mut().unwrap()));
    print!("float32: {:.10}\n", (*f32.borrow_mut().as_mut().unwrap()));
    print!("back to float64: {:.10}\n", (*backToF64.borrow_mut().as_mut().unwrap()));

        // String conversions
    println!("{}", "\n=== String conversions ===".to_string());

    let mut r: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(('A' as i32))));
    let mut b: Rc<RefCell<Option<u8>>> = Rc::new(RefCell::new(Some(65)));

    print!("rune 'A': {} ({})\n", (*r.borrow_mut().as_mut().unwrap()), (*r.borrow_mut().as_mut().unwrap()));
    print!("byte 65: {} ({})\n", (*b.borrow_mut().as_mut().unwrap()), (*b.borrow_mut().as_mut().unwrap()));

        // Rune to string
    let mut str = Rc::new(RefCell::new(Some(char::from_u32((*r.borrow().as_ref().unwrap()) as u32).unwrap().to_string())));
    print!("rune to string: {}\n", (*str.borrow_mut().as_mut().unwrap()));

        // Byte slice to string
    let mut bytes = Rc::new(RefCell::new(Some(vec![72, 101, 108, 108, 111])));
    let mut strFromBytes = Rc::new(RefCell::new(Some(String::from_utf8((*bytes.borrow().as_ref().unwrap()).clone()).unwrap())));
    print!("bytes to string: {}\n", (*strFromBytes.borrow_mut().as_mut().unwrap()));

        // String to byte slice
    let mut backToBytes = Rc::new(RefCell::new(Some((*strFromBytes.borrow().as_ref().unwrap()).as_bytes().to_vec())));
    print!("string to bytes: {}\n", format_slice(&backToBytes));

        // String to rune slice
    let mut runes = Rc::new(RefCell::new(Some((*"Hello, 世界".to_string().borrow().as_ref().unwrap()).chars().map(|c| c as i32).collect::<Vec<_>>())));
    print!("string to runes: {}\n", format_slice(&runes));
    print!("rune count: {}\n", (*runes.borrow().as_ref().unwrap()).len());

        // Rune slice back to string
    let mut backToString = Rc::new(RefCell::new(Some((*runes.borrow().as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap()).collect::<String>())));
    print!("runes to string: {}\n", (*backToString.borrow_mut().as_mut().unwrap()));

        // Boolean conversions (not direct, but showing concept)
    println!("{}", "\n=== Boolean-like conversions ===".to_string());

    let mut zero: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut nonZero: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(42)));

        // Go doesn't have direct bool conversion, but we can demonstrate the concept
    print!("zero == 0: {}\n", (*zero.borrow_mut().as_mut().unwrap()) == 0);
    print!("nonZero != 0: {}\n", (*nonZero.borrow_mut().as_mut().unwrap()) != 0);

        // Pointer conversions
    println!("{}", "\n=== Pointer conversions ===".to_string());

    let mut num: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(100)));
    let mut ptr: Rc<RefCell<Option<i32>>> = num.clone();

    print!("value: {}\n", (*num.borrow_mut().as_mut().unwrap()));
    print!("pointer: {}\n", "0xDEADBEEF".to_string());
    print!("dereferenced: {}\n", (*ptr.borrow().as_ref().unwrap()));

        // Interface conversions (basic)
    println!("{}", "\n=== Interface conversions ===".to_string());

    let mut any: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(42) as Box<dyn Any>)));
    print!("interface{{}} value: {}\n", format_any(any.borrow().as_ref().unwrap().as_ref()));
    print!("interface{{}} type: <type>\n");

        // Type assertion
    let (mut intVal, mut ok) = ({
        let val = any.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i32>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow_mut().as_mut().unwrap()) {
        print!("asserted as int: {}\n", (*intVal.borrow_mut().as_mut().unwrap()));
    }

        // Change interface value
    { let new_val = Box::new("hello".to_string()) as Box<dyn Any>; *any.borrow_mut() = Some(new_val); };
    print!("new interface{{}} value: {}\n", format_any(any.borrow().as_ref().unwrap().as_ref()));
    print!("new interface{{}} type: <type>\n");

    let (mut strVal, mut ok) = ({
        let val = any.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<String>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(String::new()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(String::new()))), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow_mut().as_mut().unwrap()) {
        print!("asserted as string: {}\n", (*strVal.borrow_mut().as_mut().unwrap()));
    }

        // Complex number conversions
    println!("{}", "\n=== Complex number conversions ===".to_string());

    let mut c64: Rc<RefCell<Option<num::Complex<f32>>>> = Rc::new(RefCell::new(Some(3 + 4i)));
    let mut c128: Rc<RefCell<Option<num::Complex<f64>>>> = Rc::new(RefCell::new(Some(num::Complex::<f64>::new((*c64.borrow().as_ref().unwrap()) as f64, 0.0))));

    print!("complex64: {}\n", (*c64.borrow_mut().as_mut().unwrap()));
    print!("complex128: {}\n", (*c128.borrow_mut().as_mut().unwrap()));

        // Extract real and imaginary parts
    let mut real = Rc::new(RefCell::new(Some((*(*c128.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()).re)));
    let mut imag = Rc::new(RefCell::new(Some((*(*c128.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()).im)));
    print!("real part: {:.2}\n", (*real.borrow_mut().as_mut().unwrap()));
    print!("imaginary part: {:.2}\n", (*imag.borrow_mut().as_mut().unwrap()));

        // Create complex from parts
    let mut newComplex = Rc::new(RefCell::new(Some(num::Complex::new(*(*real.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap(), *(*imag.borrow_mut().as_mut().unwrap()).borrow().as_ref().unwrap()))));
    print!("reconstructed: {}\n", (*newComplex.borrow_mut().as_mut().unwrap()));

        // Overflow demonstration (be careful!)
    println!("{}", "\n=== Overflow examples ===".to_string());

    let mut bigInt: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(1000000)));
    let mut smallInt: Rc<RefCell<Option<i8>>> = Rc::new(RefCell::new(Some((*bigInt.as_ref().unwrap().as_ref().unwrap()) as i8)));
    print!("int64: {}\n", (*bigInt.borrow_mut().as_mut().unwrap()));
    print!("int8 (overflow): {}\n", (*smallInt.borrow_mut().as_mut().unwrap()));

        // Precision loss in float conversion
    let mut preciseFloat: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(1.23456789012345)));
    let mut lessPrec: Rc<RefCell<Option<f32>>> = Rc::new(RefCell::new(Some((*preciseFloat.as_ref().unwrap().as_ref().unwrap()) as f32)));
    print!("float64: {:.15}\n", (*preciseFloat.borrow_mut().as_mut().unwrap()));
    print!("float32: {:.15}\n", (*lessPrec.borrow_mut().as_mut().unwrap()));

        // Custom type conversions
    println!("{}", "\n=== Custom type conversions ===".to_string());

    type MyInt = Rc<RefCell<Option<i32>>>;
    type MyString = Rc<RefCell<Option<String>>>;

    let mut mi: Rc<RefCell<Option<MyInt>>> = Rc::new(RefCell::new(Some(42)));
    let mut regularInt: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some((*mi.as_ref().unwrap().as_ref().unwrap()) as i32)));
    let mut backToMyInt: Rc<RefCell<Option<MyInt>>> = (*regularInt.borrow_mut().as_mut().unwrap());

    print!("MyInt: {}\n", (*mi.borrow_mut().as_mut().unwrap()));
    print!("regular int: {}\n", (*regularInt.borrow_mut().as_mut().unwrap()));
    print!("back to MyInt: {}\n", (*backToMyInt.borrow_mut().as_mut().unwrap()));

    let mut ms: Rc<RefCell<Option<MyString>>> = Rc::new(RefCell::new(Some("hello".to_string())));
    let mut regularString: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some((*ms.borrow().as_ref().unwrap()).to_string())));
    let mut backToMyString: Rc<RefCell<Option<MyString>>> = (*regularString.borrow_mut().as_mut().unwrap());

    print!("MyString: {}\n", (*ms.borrow_mut().as_mut().unwrap()));
    print!("regular string: {}\n", (*regularString.borrow_mut().as_mut().unwrap()));
    print!("back to MyString: {}\n", (*backToMyString.borrow_mut().as_mut().unwrap()));
}