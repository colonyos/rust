//! ColonyOS WASM Executor Example
//!
//! This example demonstrates how to build a ColonyOS executor that runs
//! in WebAssembly (browser environment). The executor connects to a
//! ColonyOS server, polls for processes, and executes them.
//!
//! # Building
//!
//! ```bash
//! wasm-pack build --target web
//! ```
//!
//! # Usage
//!
//! See the accompanying index.html file for browser integration.

use colonyos::core::{Executor, Process};
use colonyos::crypto;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::prelude::*;
use web_sys::console;

/// Log a message to the browser console
fn log(msg: &str) {
    console::log_1(&JsValue::from_str(msg));
}

/// Configuration for the WASM executor
#[wasm_bindgen]
pub struct WasmExecutorConfig {
    server_url: String,
    colony_name: String,
    executor_name: String,
    executor_type: String,
    colony_prvkey: String,
}

#[wasm_bindgen]
impl WasmExecutorConfig {
    /// Create a new configuration
    #[wasm_bindgen(constructor)]
    pub fn new(
        server_url: &str,
        colony_name: &str,
        executor_name: &str,
        executor_type: &str,
        colony_prvkey: &str,
    ) -> Self {
        WasmExecutorConfig {
            server_url: server_url.to_string(),
            colony_name: colony_name.to_string(),
            executor_name: executor_name.to_string(),
            executor_type: executor_type.to_string(),
            colony_prvkey: colony_prvkey.to_string(),
        }
    }
}

/// The WASM Executor
#[wasm_bindgen]
pub struct WasmExecutor {
    config: WasmExecutorConfig,
    private_key: String,
    executor_id: String,
    running: bool,
}

#[wasm_bindgen]
impl WasmExecutor {
    /// Create a new WASM executor
    #[wasm_bindgen(constructor)]
    pub fn new(config: WasmExecutorConfig) -> Self {
        // Initialize console logging
        console_log::init_with_level(log::Level::Debug).ok();

        // Configure the server URL
        let api_url = format!("{}/api", config.server_url.trim_end_matches('/'));
        colonyos::set_server_url(&api_url);
        log(&format!("Server URL set to: {}", api_url));

        // Generate a new private key for this executor
        let private_key = crypto::gen_prvkey();
        let executor_id = crypto::gen_id(&private_key);

        log(&format!(
            "Created WASM executor: name={}, id={}",
            config.executor_name, executor_id
        ));

        WasmExecutor {
            config,
            private_key,
            executor_id,
            running: false,
        }
    }

    /// Register the executor with the ColonyOS server and approve it
    #[wasm_bindgen]
    pub async fn register(&self) -> Result<(), JsValue> {
        log(&format!("Registering executor '{}' with ID: {}",
            self.config.executor_name, self.executor_id));
        log(&format!("Colony: {}", self.config.colony_name));

        let executor = Executor::new(
            &self.config.executor_name,
            &self.executor_id,
            &self.config.executor_type,
            &self.config.colony_name,
        );

        // Register the executor (requires colony owner key)
        match colonyos::add_executor(&executor, &self.config.colony_prvkey).await {
            Ok(_) => {
                log("Executor registered successfully");
            }
            Err(e) => {
                let err_msg = e.to_string();
                // If already registered, continue to approval
                if err_msg.contains("already exists") || err_msg.contains("already registered") {
                    log("Executor already registered, continuing to approval...");
                } else {
                    let msg = format!("Failed to register executor: {}", err_msg);
                    log(&msg);
                    return Err(JsValue::from_str(&msg));
                }
            }
        }

        // Approve the executor using the colony owner key
        log("Approving executor with colony owner key...");
        log(&format!("Colony key ID: {}", colonyos::crypto::gen_id(&self.config.colony_prvkey)));

        match colonyos::approve_executor(
            &self.config.colony_name,
            &self.config.executor_name,
            &self.config.colony_prvkey,
        ).await {
            Ok(_) => {
                log("Executor approved successfully");
                Ok(())
            }
            Err(e) => {
                let err_msg = e.to_string();
                // If already approved, continue
                if err_msg.contains("already approved") {
                    log("Executor already approved, continuing...");
                    Ok(())
                } else {
                    let msg = format!("Failed to approve executor: {}", err_msg);
                    log(&msg);
                    Err(JsValue::from_str(&msg))
                }
            }
        }
    }

    /// Start the executor loop
    #[wasm_bindgen]
    pub async fn start(&mut self) -> Result<(), JsValue> {
        self.running = true;
        log("Starting executor loop...");

        while self.running {
            match self.poll_and_execute().await {
                Ok(executed) => {
                    if executed {
                        log("Process executed successfully");
                    }
                }
                Err(e) => {
                    log(&format!("Error in executor loop: {}", e));
                    // Wait before retrying on error
                    TimeoutFuture::new(1000).await;
                }
            }

            // Small delay between polls
            TimeoutFuture::new(100).await;
        }

        Ok(())
    }

    /// Stop the executor
    #[wasm_bindgen]
    pub fn stop(&mut self) {
        log("Stopping executor...");
        self.running = false;
    }

    /// Poll for a process and execute it
    async fn poll_and_execute(&self) -> Result<bool, String> {
        // Long-poll for a process (10 second timeout)
        let result = colonyos::assign(&self.config.colony_name, 10, &self.private_key).await;

        match result {
            Ok(process) => {
                log(&format!(
                    "Assigned process: id={}, func={}",
                    process.processid, process.spec.funcname
                ));
                self.execute_process(&process).await?;
                Ok(true)
            }
            Err(e) => {
                if e.conn_err() {
                    Err(format!("Connection error: {}", e))
                } else {
                    // Timeout or no process available - continue polling
                    Ok(false)
                }
            }
        }
    }

    /// Execute a process based on its function name
    async fn execute_process(&self, process: &Process) -> Result<(), String> {
        let result = match process.spec.funcname.as_str() {
            "echo" => self.handle_echo(process).await,
            "add" => self.handle_add(process).await,
            "multiply" => self.handle_multiply(process).await,
            "greet" => self.handle_greet(process).await,
            "browser_info" => self.handle_browser_info(process).await,
            _ => {
                log(&format!("Unknown function: {}", process.spec.funcname));
                self.fail_process(&process.processid, "Unknown function").await
            }
        };

        result
    }

    /// Handle the 'echo' function - returns the first argument
    async fn handle_echo(&self, process: &Process) -> Result<(), String> {
        let output = process
            .spec
            .args
            .first()
            .cloned()
            .unwrap_or_else(|| "".to_string());

        self.complete_process(&process.processid, vec![output]).await
    }

    /// Handle the 'add' function - adds two numbers
    async fn handle_add(&self, process: &Process) -> Result<(), String> {
        if process.spec.args.len() < 2 {
            return self.fail_process(&process.processid, "add requires 2 arguments").await;
        }

        let a: f64 = process.spec.args[0]
            .parse()
            .map_err(|_| "Invalid first argument")?;
        let b: f64 = process.spec.args[1]
            .parse()
            .map_err(|_| "Invalid second argument")?;

        let result = a + b;
        self.complete_process(&process.processid, vec![result.to_string()]).await
    }

    /// Handle the 'multiply' function - multiplies two numbers
    async fn handle_multiply(&self, process: &Process) -> Result<(), String> {
        if process.spec.args.len() < 2 {
            return self.fail_process(&process.processid, "multiply requires 2 arguments").await;
        }

        let a: f64 = process.spec.args[0]
            .parse()
            .map_err(|_| "Invalid first argument")?;
        let b: f64 = process.spec.args[1]
            .parse()
            .map_err(|_| "Invalid second argument")?;

        let result = a * b;
        self.complete_process(&process.processid, vec![result.to_string()]).await
    }

    /// Handle the 'greet' function - returns a greeting
    async fn handle_greet(&self, process: &Process) -> Result<(), String> {
        let name = process
            .spec
            .args
            .first()
            .cloned()
            .unwrap_or_else(|| "World".to_string());

        let greeting = format!("Hello, {}! Greetings from the WASM executor.", name);
        self.complete_process(&process.processid, vec![greeting]).await
    }

    /// Handle the 'browser_info' function - returns browser information
    async fn handle_browser_info(&self, process: &Process) -> Result<(), String> {
        let window = web_sys::window().ok_or("No window object")?;
        let navigator = window.navigator();
        let user_agent = navigator.user_agent().unwrap_or_default();
        let language = navigator.language().unwrap_or_default();

        let info = format!(
            "User-Agent: {}, Language: {}, Executor: {}",
            user_agent, language, self.config.executor_name
        );

        self.complete_process(&process.processid, vec![info]).await
    }

    /// Complete a process with output
    async fn complete_process(&self, processid: &str, output: Vec<String>) -> Result<(), String> {
        colonyos::set_output(processid, output, &self.private_key)
            .await
            .map_err(|e| format!("Failed to set output: {}", e))?;

        colonyos::close(processid, &self.private_key)
            .await
            .map_err(|e| format!("Failed to close process: {}", e))?;

        log(&format!("Process {} completed", processid));
        Ok(())
    }

    /// Fail a process with an error message
    async fn fail_process(&self, processid: &str, error: &str) -> Result<(), String> {
        log(&format!("Process {} failed: {}", processid, error));

        colonyos::fail(processid, &self.private_key)
            .await
            .map_err(|e| format!("Failed to fail process: {}", e))?;

        Ok(())
    }

    /// Get the executor ID
    #[wasm_bindgen(getter)]
    pub fn executor_id(&self) -> String {
        self.executor_id.clone()
    }

    /// Check if the executor is running
    #[wasm_bindgen(getter)]
    pub fn is_running(&self) -> bool {
        self.running
    }
}

/// Generate a new private key (utility function)
#[wasm_bindgen]
pub fn generate_private_key() -> String {
    crypto::gen_prvkey()
}

/// Generate an ID from a private key (utility function)
#[wasm_bindgen]
pub fn generate_id(private_key: &str) -> String {
    crypto::gen_id(private_key)
}

/// Verify crypto works correctly by signing and recovering (debug function)
#[wasm_bindgen]
pub fn verify_crypto(private_key: &str, message: &str) -> String {
    let id = crypto::gen_id(private_key);
    let signature = colonyos::crypto::gen_signature(message, private_key);
    let recovered = colonyos::crypto::recid(message, &signature);

    format!(
        "ID: {}\nMessage: {}\nSignature: {}\nRecovered: {}\nMatch: {}",
        id, message, signature, recovered, id == recovered
    )
}

/// Initialize WASM module
#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Debug).ok();
    log("ColonyOS WASM Executor module loaded");
}
