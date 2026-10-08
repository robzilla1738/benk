fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args == ["--capabilities"] {
        println!(
            "{{\"product\":\"Benk\",\"mode\":\"disabled\",\"capabilities\":[],\"sandboxImplemented\":false}}"
        );
        return;
    }
    eprintln!("Benk broker: execution disabled. No action performed.");
    std::process::exit(77);
}
