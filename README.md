# Introduction
This repo contains a Rust implementation of a subset of the Colonies API, making it possible to implement Colonies Executors in Rust.

Note: this code has only been tested on Linux.

## Example code
Just a simple helloworld! For full example with error handling, click [here](examples/assign/src/main.rs).

```rust
let colonyid = "4787a5071856a4acf702b2ffcea422e3237a679c681314113d86139461290cf4";
let executorprvkey = "ddf7f7791208083b6a9ed975a72684f6406a269cfa36f1b1c32045c0a71fff05";

loop {
    let assigned_process = colonies::assign(&colonyid, false, 10, &executorprvkey).await.unwrap();
    match assigned_process.spec.func.as_str() {
        "say" => {
            let attr = Attribute::new(
                &colonyid,
                &assigned_process.processid,
                "output",
                &assigned_process.spec.args[0],
            );
            colonies::add_attr(&attr, executorprvkey).await;
            colonies::close(&assigned_process.processid, executorprvkey).await;
            }
        }
        _ => {
            colonies::fail(&assigned_process.processid, executorprvkey).await;
        }
    };
}
```

# To test it ... 
## Colonies server
First start a Colonies server.

```console
source devenv
colonies dev
```

## Start the Rust helloworld executor.
```console
cd examples/assign
cargo run
```

## Submit a process spec 
```console
colonies process run --func say --args hello  --targettype cli --wait 
```

Output:
```console
hello
```
