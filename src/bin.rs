use micro_lambda::lambda;

fn main() {
    lambda(handler);
}

fn handler(event: &str) -> std::result::Result<String, String> {
    println!("{}", event);
    if event.contains("fail") {
        return Err("ERROR".to_string());
    }

    Ok("SUCCESS".to_string())
}
