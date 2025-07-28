pub fn basic_switch(day: i32) {
    
}

pub fn switch_with_expression() {
    let mut x = 10;
    
}

pub fn switch_without_expression() {
    let mut score = 85;
    
}

pub fn switch_with_fallthrough(num: i32) {
    
}

pub fn type_switch(value: Unknown) {
    
}

fn main() {
    println!("{}", "=== Basic switch ===".to_string());
    basic_switch(1);
    basic_switch(6);
    basic_switch(10);
    println!("{}", "\n=== Switch with expression ===".to_string());
    switch_with_expression();
    println!("{}", "\n=== Switch without expression ===".to_string());
    switch_without_expression();
    println!("{}", "\n=== Switch with fallthrough ===".to_string());
    switch_with_fallthrough(1);
    println!("{}", "---".to_string());
    switch_with_fallthrough(4);
    println!("{}", "\n=== Type switch ===".to_string());
    type_switch(42);
    type_switch("hello".to_string());
    type_switch(true);
    type_switch(3.14);
    type_switch(vec![1, 2, 3]);
}