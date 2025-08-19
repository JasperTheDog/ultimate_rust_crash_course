pub fn inspect(str : &String) {
    print!("Inspecting!");
    if str.ends_with("s") {
        println!(" Plural!");
    } else {
        println!(" Singular!");
    }
}

pub fn change(str: &mut String) {
    if !str.ends_with("s") {
        str.push_str("s");
    }
}

pub fn eat(str: String) -> bool {
    if str.starts_with("b") && str.contains("a") {
        return true
    }
    false
}

pub fn bedazzle(str: &mut String) {
    *str = "sparkly".to_string();
}