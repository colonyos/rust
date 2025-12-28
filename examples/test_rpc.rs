use colonies::core::Executor;

fn main() {
    let executor = Executor::new("test-executor", "exec-id", "cli", "test-colony");

    // Show the executor JSON
    let json = serde_json::to_string_pretty(&executor).unwrap();
    println!("Executor JSON:");
    println!("{}", json);
}
