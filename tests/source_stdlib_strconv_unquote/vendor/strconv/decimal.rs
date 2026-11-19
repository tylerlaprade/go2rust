use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const UINT_SIZE: i32 = 32 << (!(0 as u64) >> 63);


pub(crate) const MAX_SHIFT: i32 = UINT_SIZE - 4;


#[derive(Debug, Clone)]
pub struct leftCheat {
    pub delta: Arc<Mutex<Option<i32>>>,
    pub cutoff: Arc<Mutex<Option<String>>>,
}

impl leftCheat {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.delta.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.cutoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            delta: __go_clone_0_0,
            cutoff: __go_clone_1_0,
        }
    }
}


impl Default for leftCheat {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            delta: __go_default_0_0,
            cutoff: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for leftCheat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.delta.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.cutoff.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) static leftcheats: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<leftCheat>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *leftcheats.lock().unwrap() = Some(vec![]);
    {
        let mut __go_slice = Vec::<leftCheat>::with_capacity(61);
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(0))), cutoff: Arc::new(Mutex::new(Some("".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(1))), cutoff: Arc::new(Mutex::new(Some("5".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(1))), cutoff: Arc::new(Mutex::new(Some("25".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(1))), cutoff: Arc::new(Mutex::new(Some("125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(2))), cutoff: Arc::new(Mutex::new(Some("625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(2))), cutoff: Arc::new(Mutex::new(Some("3125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(2))), cutoff: Arc::new(Mutex::new(Some("15625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(3))), cutoff: Arc::new(Mutex::new(Some("78125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(3))), cutoff: Arc::new(Mutex::new(Some("390625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(3))), cutoff: Arc::new(Mutex::new(Some("1953125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(4))), cutoff: Arc::new(Mutex::new(Some("9765625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(4))), cutoff: Arc::new(Mutex::new(Some("48828125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(4))), cutoff: Arc::new(Mutex::new(Some("244140625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(4))), cutoff: Arc::new(Mutex::new(Some("1220703125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(5))), cutoff: Arc::new(Mutex::new(Some("6103515625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(5))), cutoff: Arc::new(Mutex::new(Some("30517578125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(5))), cutoff: Arc::new(Mutex::new(Some("152587890625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(6))), cutoff: Arc::new(Mutex::new(Some("762939453125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(6))), cutoff: Arc::new(Mutex::new(Some("3814697265625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(6))), cutoff: Arc::new(Mutex::new(Some("19073486328125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(7))), cutoff: Arc::new(Mutex::new(Some("95367431640625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(7))), cutoff: Arc::new(Mutex::new(Some("476837158203125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(7))), cutoff: Arc::new(Mutex::new(Some("2384185791015625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(7))), cutoff: Arc::new(Mutex::new(Some("11920928955078125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(8))), cutoff: Arc::new(Mutex::new(Some("59604644775390625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(8))), cutoff: Arc::new(Mutex::new(Some("298023223876953125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(8))), cutoff: Arc::new(Mutex::new(Some("1490116119384765625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(9))), cutoff: Arc::new(Mutex::new(Some("7450580596923828125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(9))), cutoff: Arc::new(Mutex::new(Some("37252902984619140625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(9))), cutoff: Arc::new(Mutex::new(Some("186264514923095703125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(10))), cutoff: Arc::new(Mutex::new(Some("931322574615478515625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(10))), cutoff: Arc::new(Mutex::new(Some("4656612873077392578125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(10))), cutoff: Arc::new(Mutex::new(Some("23283064365386962890625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(10))), cutoff: Arc::new(Mutex::new(Some("116415321826934814453125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(11))), cutoff: Arc::new(Mutex::new(Some("582076609134674072265625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(11))), cutoff: Arc::new(Mutex::new(Some("2910383045673370361328125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(11))), cutoff: Arc::new(Mutex::new(Some("14551915228366851806640625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(12))), cutoff: Arc::new(Mutex::new(Some("72759576141834259033203125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(12))), cutoff: Arc::new(Mutex::new(Some("363797880709171295166015625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(12))), cutoff: Arc::new(Mutex::new(Some("1818989403545856475830078125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(13))), cutoff: Arc::new(Mutex::new(Some("9094947017729282379150390625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(13))), cutoff: Arc::new(Mutex::new(Some("45474735088646411895751953125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(13))), cutoff: Arc::new(Mutex::new(Some("227373675443232059478759765625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(13))), cutoff: Arc::new(Mutex::new(Some("1136868377216160297393798828125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(14))), cutoff: Arc::new(Mutex::new(Some("5684341886080801486968994140625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(14))), cutoff: Arc::new(Mutex::new(Some("28421709430404007434844970703125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(14))), cutoff: Arc::new(Mutex::new(Some("142108547152020037174224853515625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(15))), cutoff: Arc::new(Mutex::new(Some("710542735760100185871124267578125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(15))), cutoff: Arc::new(Mutex::new(Some("3552713678800500929355621337890625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(15))), cutoff: Arc::new(Mutex::new(Some("17763568394002504646778106689453125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(16))), cutoff: Arc::new(Mutex::new(Some("88817841970012523233890533447265625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(16))), cutoff: Arc::new(Mutex::new(Some("444089209850062616169452667236328125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(16))), cutoff: Arc::new(Mutex::new(Some("2220446049250313080847263336181640625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(16))), cutoff: Arc::new(Mutex::new(Some("11102230246251565404236316680908203125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(17))), cutoff: Arc::new(Mutex::new(Some("55511151231257827021181583404541015625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(17))), cutoff: Arc::new(Mutex::new(Some("277555756156289135105907917022705078125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(17))), cutoff: Arc::new(Mutex::new(Some("1387778780781445675529539585113525390625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(18))), cutoff: Arc::new(Mutex::new(Some("6938893903907228377647697925567626953125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(18))), cutoff: Arc::new(Mutex::new(Some("34694469519536141888238489627838134765625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(18))), cutoff: Arc::new(Mutex::new(Some("173472347597680709441192448139190673828125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(19))), cutoff: Arc::new(Mutex::new(Some("867361737988403547205962240695953369140625".to_string()))), ..Default::default() });
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *leftcheats.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_zero_globals() {
    *leftcheats.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_6() {
    {
        let mut __go_slice = Vec::<leftCheat>::with_capacity(61);
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(0))), cutoff: Arc::new(Mutex::new(Some("".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(1))), cutoff: Arc::new(Mutex::new(Some("5".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(1))), cutoff: Arc::new(Mutex::new(Some("25".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(1))), cutoff: Arc::new(Mutex::new(Some("125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(2))), cutoff: Arc::new(Mutex::new(Some("625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(2))), cutoff: Arc::new(Mutex::new(Some("3125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(2))), cutoff: Arc::new(Mutex::new(Some("15625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(3))), cutoff: Arc::new(Mutex::new(Some("78125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(3))), cutoff: Arc::new(Mutex::new(Some("390625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(3))), cutoff: Arc::new(Mutex::new(Some("1953125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(4))), cutoff: Arc::new(Mutex::new(Some("9765625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(4))), cutoff: Arc::new(Mutex::new(Some("48828125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(4))), cutoff: Arc::new(Mutex::new(Some("244140625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(4))), cutoff: Arc::new(Mutex::new(Some("1220703125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(5))), cutoff: Arc::new(Mutex::new(Some("6103515625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(5))), cutoff: Arc::new(Mutex::new(Some("30517578125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(5))), cutoff: Arc::new(Mutex::new(Some("152587890625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(6))), cutoff: Arc::new(Mutex::new(Some("762939453125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(6))), cutoff: Arc::new(Mutex::new(Some("3814697265625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(6))), cutoff: Arc::new(Mutex::new(Some("19073486328125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(7))), cutoff: Arc::new(Mutex::new(Some("95367431640625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(7))), cutoff: Arc::new(Mutex::new(Some("476837158203125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(7))), cutoff: Arc::new(Mutex::new(Some("2384185791015625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(7))), cutoff: Arc::new(Mutex::new(Some("11920928955078125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(8))), cutoff: Arc::new(Mutex::new(Some("59604644775390625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(8))), cutoff: Arc::new(Mutex::new(Some("298023223876953125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(8))), cutoff: Arc::new(Mutex::new(Some("1490116119384765625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(9))), cutoff: Arc::new(Mutex::new(Some("7450580596923828125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(9))), cutoff: Arc::new(Mutex::new(Some("37252902984619140625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(9))), cutoff: Arc::new(Mutex::new(Some("186264514923095703125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(10))), cutoff: Arc::new(Mutex::new(Some("931322574615478515625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(10))), cutoff: Arc::new(Mutex::new(Some("4656612873077392578125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(10))), cutoff: Arc::new(Mutex::new(Some("23283064365386962890625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(10))), cutoff: Arc::new(Mutex::new(Some("116415321826934814453125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(11))), cutoff: Arc::new(Mutex::new(Some("582076609134674072265625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(11))), cutoff: Arc::new(Mutex::new(Some("2910383045673370361328125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(11))), cutoff: Arc::new(Mutex::new(Some("14551915228366851806640625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(12))), cutoff: Arc::new(Mutex::new(Some("72759576141834259033203125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(12))), cutoff: Arc::new(Mutex::new(Some("363797880709171295166015625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(12))), cutoff: Arc::new(Mutex::new(Some("1818989403545856475830078125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(13))), cutoff: Arc::new(Mutex::new(Some("9094947017729282379150390625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(13))), cutoff: Arc::new(Mutex::new(Some("45474735088646411895751953125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(13))), cutoff: Arc::new(Mutex::new(Some("227373675443232059478759765625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(13))), cutoff: Arc::new(Mutex::new(Some("1136868377216160297393798828125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(14))), cutoff: Arc::new(Mutex::new(Some("5684341886080801486968994140625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(14))), cutoff: Arc::new(Mutex::new(Some("28421709430404007434844970703125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(14))), cutoff: Arc::new(Mutex::new(Some("142108547152020037174224853515625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(15))), cutoff: Arc::new(Mutex::new(Some("710542735760100185871124267578125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(15))), cutoff: Arc::new(Mutex::new(Some("3552713678800500929355621337890625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(15))), cutoff: Arc::new(Mutex::new(Some("17763568394002504646778106689453125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(16))), cutoff: Arc::new(Mutex::new(Some("88817841970012523233890533447265625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(16))), cutoff: Arc::new(Mutex::new(Some("444089209850062616169452667236328125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(16))), cutoff: Arc::new(Mutex::new(Some("2220446049250313080847263336181640625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(16))), cutoff: Arc::new(Mutex::new(Some("11102230246251565404236316680908203125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(17))), cutoff: Arc::new(Mutex::new(Some("55511151231257827021181583404541015625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(17))), cutoff: Arc::new(Mutex::new(Some("277555756156289135105907917022705078125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(17))), cutoff: Arc::new(Mutex::new(Some("1387778780781445675529539585113525390625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(18))), cutoff: Arc::new(Mutex::new(Some("6938893903907228377647697925567626953125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(18))), cutoff: Arc::new(Mutex::new(Some("34694469519536141888238489627838134765625".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(18))), cutoff: Arc::new(Mutex::new(Some("173472347597680709441192448139190673828125".to_string()))), ..Default::default() });
        __go_slice.push(leftCheat { delta: Arc::new(Mutex::new(Some(19))), cutoff: Arc::new(Mutex::new(Some("867361737988403547205962240695953369140625".to_string()))), ..Default::default() });
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *leftcheats.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for leftCheat {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
