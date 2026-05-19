use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
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

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
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

fn go_strconv_format_int(value: i64, base: i32) -> String {
    if base == 10 {
        return value.to_string();
    }
    if !(2..=36).contains(&base) {
        return value.to_string();
    }

    let negative = value < 0;
    let mut n = if negative {
        value.wrapping_neg() as u64
    } else {
        value as u64
    };
    let base = base as u64;
    let digits = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut out = Vec::new();
    if n == 0 {
        out.push(b'0');
    }
    while n > 0 {
        out.push(digits[(n % base) as usize]);
        n /= base;
    }
    if negative {
        out.push(b'-');
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

fn go_strconv_format_float(value: f64, fmt: char, precision: i32) -> String {
    let precision = if precision < 0 { 6 } else { precision as usize };
    match fmt {
        'e' => format!("{:.*e}", precision, value),
        'E' => format!("{:.*E}", precision, value),
        'f' => format!("{:.*}", precision, value),
        'g' | 'G' => {
            if precision == 0 {
                format!("{:.0}", value)
            } else {
                format!("{:.*}", precision, value)
            }
        }
        _ => value.to_string(),
    }
}

#[derive(Clone, Debug, Default)]
struct GoTime {
    seconds: i64,
    nanos: i32,
}

fn go_time_civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    (year, m as u32, d as u32)
}

impl GoTime {
    fn now() -> Self {
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        GoTime {
            seconds: duration.as_secs() as i64,
            nanos: duration.subsec_nanos() as i32,
        }
    }

    fn from_unix(seconds: i64, nanos: i64) -> Self {
        let seconds = seconds + nanos.div_euclid(1_000_000_000);
        let nanos = nanos.rem_euclid(1_000_000_000);
        GoTime {
            seconds,
            nanos: nanos as i32,
        }
    }

    fn add(&self, duration: Rc<RefCell<Option<std::time::Duration>>>) -> Rc<RefCell<Option<GoTime>>> {
        let duration = *duration.borrow().as_ref().unwrap();
        Rc::new(RefCell::new(Some(GoTime::from_unix(
            self.seconds + duration.as_secs() as i64,
            self.nanos as i64 + duration.subsec_nanos() as i64,
        ))))
    }

    fn u_t_c(&self) -> Rc<RefCell<Option<GoTime>>> {
        Rc::new(RefCell::new(Some(self.clone())))
    }

    fn unix(&self) -> Rc<RefCell<Option<i64>>> {
        Rc::new(RefCell::new(Some(self.seconds)))
    }

    fn unix_nano(&self) -> Rc<RefCell<Option<i64>>> {
        Rc::new(RefCell::new(Some(
            self.seconds * 1_000_000_000 + self.nanos as i64,
        )))
    }

    fn is_zero(&self) -> Rc<RefCell<Option<bool>>> {
        Rc::new(RefCell::new(Some(self.seconds == 0 && self.nanos == 0)))
    }

    fn format(&self, _layout: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(self.to_string())))
    }
}

impl std::fmt::Display for GoTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let days = self.seconds.div_euclid(86_400);
        let secs_of_day = self.seconds.rem_euclid(86_400);
        let (year, month, day) = go_time_civil_from_days(days);
        let hour = secs_of_day / 3_600;
        let minute = (secs_of_day % 3_600) / 60;
        let second = secs_of_day % 60;
        if self.nanos == 0 {
            write!(
                f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02} +0000 UTC",
                year, month, day, hour, minute, second
            )
        } else {
            let mut fraction = format!("{:09}", self.nanos);
            while fraction.ends_with('0') {
                fraction.pop();
            }
            write!(
                f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{} +0000 UTC",
                year, month, day, hour, minute, second, fraction
            )
        }
    }
}

fn main() {
    let __go_os_args = Rc::new(RefCell::new(Some(std::env::args().collect::<Vec<String>>())));

        // Testing multiple standard library imports
    println!("{}", format!("{}", "=== Testing multiple stdlib imports ===".to_string()));

        // strings package
    println!("{}", format!("{}", "\n--- strings package ---".to_string()));
    let mut upper = Rc::new(RefCell::new(Some({ let __s = "hello world".to_string(); __s.to_uppercase() })));
    println!("{} {}", format!("{}", "Upper:".to_string()), format!("{}", { let __v = (*upper.borrow().as_ref().unwrap()).clone(); __v }));

    let mut lower = Rc::new(RefCell::new(Some({ let __s = "HELLO WORLD".to_string(); __s.to_lowercase() })));
    println!("{} {}", format!("{}", "Lower:".to_string()), format!("{}", { let __v = (*lower.borrow().as_ref().unwrap()).clone(); __v }));

    let mut trimmed = Rc::new(RefCell::new(Some({ let __s = "  hello  ".to_string(); __s.trim().to_string() })));
    println!("{} {}", format!("{}", "Trimmed:".to_string()), format!("{}", { let __v = (*trimmed.borrow().as_ref().unwrap()).clone(); __v }));

    let mut split = Rc::new(RefCell::new(Some({ let __s = "a,b,c".to_string(); let __sep = ",".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() })));
    print!("Split: {}\n", format_slice(&split));

        // strconv package
    println!("{}", format!("{}", "\n--- strconv package ---".to_string()));
    let mut num = Rc::new(RefCell::new(Some(42)));
    let mut str = Rc::new(RefCell::new(Some((*num.borrow().as_ref().unwrap()).to_string())));
    println!("{} {}", format!("{}", "Number as string:".to_string()), format!("{}", { let __v = (*str.borrow().as_ref().unwrap()).clone(); __v }));

    let (mut parsed, mut err) = { let __atoi_input = "123".to_string().clone(); match __atoi_input.parse::<i32>() { Ok(n) => (Rc::new(RefCell::new(Some(n))), Rc::new(RefCell::new(None))), Err(e) => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input)))))) } };
    if (*err.borrow()).is_some() {
        println!("{} {}", format!("{}", "Parse error:".to_string()), format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
    } else {
        println!("{} {}", format!("{}", "Parsed number:".to_string()), format!("{}", { let __v = (*parsed.borrow().as_ref().unwrap()).clone(); __v }));
    }

    let mut floatStr = Rc::new(RefCell::new(Some(go_strconv_format_float(3.14159 as f64, char::from_u32((('f' as i32)) as u32).unwrap_or('f'), 2 as i32))));
    println!("{} {}", format!("{}", "Float as string:".to_string()), format!("{}", { let __v = (*floatStr.borrow().as_ref().unwrap()).clone(); __v }));

        // math package
    println!("{}", format!("{}", "\n--- math package ---".to_string()));
    print!("Pi: {:.6}\n", std::f64::consts::PI);
    print!("E: {:.6}\n", std::f64::consts::E);
    print!("Sqrt(16): {:.2}\n", (*Rc::new(RefCell::new(Some((16.0 as f64).sqrt()))).borrow().as_ref().unwrap()));
    print!("Pow(2, 8): {:.0}\n", (*Rc::new(RefCell::new(Some((2.0 as f64).powf(8.0 as f64)))).borrow().as_ref().unwrap()));
    print!("Max(10, 20): {:.0}\n", (*Rc::new(RefCell::new(Some((10.0 as f64).max(20.0 as f64)))).borrow().as_ref().unwrap()));
    print!("Min(10, 20): {:.0}\n", (*Rc::new(RefCell::new(Some((10.0 as f64).min(20.0 as f64)))).borrow().as_ref().unwrap()));

        // time package
    println!("{}", format!("{}", "\n--- time package ---".to_string()));
    let mut localFixed = Rc::new(RefCell::new(Some(GoTime::from_unix(1700000000 as i64, 0 as i64))));
    let mut fixed = (*localFixed.borrow().as_ref().unwrap()).u_t_c();
    println!("{} {}", format!("{}", "Fixed timestamp:".to_string()), format!("{}", (*(*fixed.borrow().as_ref().unwrap()).unix().borrow().as_ref().unwrap())));
    let mut later = (*fixed.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some(std::time::Duration::from_secs(3600)))));
    println!("{} {}", format!("{}", "One hour later:".to_string()), format!("{}", (*(*later.borrow().as_ref().unwrap()).unix().borrow().as_ref().unwrap())));

        // os package
    println!("{}", format!("{}", "\n--- os package ---".to_string()));
    println!("{} {}", format!("{}", "Argument count:".to_string()), format!("{}", (*__go_os_args.clone().borrow().as_ref().unwrap()).len()));

        // Combined usage
    println!("{}", format!("{}", "\n--- Combined usage ---".to_string()));
    let mut timestamp = (*fixed.borrow().as_ref().unwrap()).unix();
    let mut timestampStr = Rc::new(RefCell::new(Some(go_strconv_format_int((*timestamp.borrow().as_ref().unwrap()) as i64, 10 as i32))));
    let mut message = Rc::new(RefCell::new(Some(format!("{}{}", "Timestamp: ".to_string(), (*timestampStr.borrow().as_ref().unwrap())))));
    println!("{}", format!("{}", { let __v = (*message.borrow().as_ref().unwrap()).clone(); __v }));

        // Mathematical calculation with string formatting
    let mut result = Rc::new(RefCell::new(Some(((*Rc::new(RefCell::new(Some((3.0 as f64).powf(2.0 as f64)))).borrow().as_ref().unwrap()) + (*Rc::new(RefCell::new(Some((4.0 as f64).powf(2.0 as f64)))).borrow().as_ref().unwrap()) as f64).sqrt())));
    let mut resultStr = Rc::new(RefCell::new(Some(go_strconv_format_float((*result.borrow().as_ref().unwrap()) as f64, char::from_u32((('f' as i32)) as u32).unwrap_or('f'), 2 as i32))));
    print!("Hypotenuse of 3,4 triangle: {}\n", { let __v = (*resultStr.borrow().as_ref().unwrap()).clone(); __v });
}