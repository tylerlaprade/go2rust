use std::cell::{RefCell};
use std::rc::{Rc};


#[derive(Clone, Debug)]
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
    let mut localBase = Rc::new(RefCell::new(Some(GoTime::from_unix(1700000000 as i64, 0 as i64))));
    let mut base = (*localBase.borrow().as_ref().unwrap()).u_t_c();
    println!("{} {}", "Base time:".to_string(), (*base.borrow().as_ref().unwrap()));

    let mut future = (*base.borrow().as_ref().unwrap()).add(Rc::new(RefCell::new(Some(std::time::Duration::from_secs(3600)))));
    println!("{} {}", "One hour later:".to_string(), (*future.borrow().as_ref().unwrap()));

    println!("{} {}", "Unix timestamp:".to_string(), (*(*base.borrow().as_ref().unwrap()).unix().borrow().as_ref().unwrap()));
}