fn main() {
    const manifest: &'static str = "[package]\nname = \"demo\"\n";


    println!("{}", format!("{}", "raw\\ntext".to_string()));
    print!("[package]\nname = \"{}\"\n", "demo".to_string());
    print!("{}", format!("{}", manifest));
}