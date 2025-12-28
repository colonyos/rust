use colonyos;
use colonyos::core::Attribute;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let colonyname = "dev";
    let executorprvkey = "ddf7f7791208083b6a9ed975a72684f6406a269cfa36f1b1c32045c0a71fff05";

    loop {
        println!("trying to get an assignment");
        let assigned_process = colonyos::assign(&colonyname, 10, &executorprvkey).await;
        let assigned_process = match assigned_process {
            Ok(process) => process,
            Err(err) => {
                if err.conn_err() {
                    println!("connection problem, re-trying in 1 second");
                    sleep(Duration::from_millis(1000)).await;
                }
                println!("timeout, or another worker got the assignment, re-trying ...");
                continue;
            }
        };

        println!(
            "yippi, we are assigned to process id={}",
            assigned_process.processid
        );

        match assigned_process.spec.funcname.as_str() {
            "say" => {
                let attr = Attribute::new(
                    &colonyid,
                    &assigned_process.processid,
                    "output",
                    &assigned_process.spec.args[0],
                );
                let res = colonyos::add_attr(&attr, executorprvkey).await;
                if res.is_err() {
                    println!("failed to add attribute");
                }
                let res = colonyos::close(&assigned_process.processid, executorprvkey).await;
                if res.is_err() {
                    println!("failed to close process");
                }
            }
            _ => {
                println!("invalid function name");
                let res = colonyos::fail(&assigned_process.processid, executorprvkey).await;
                if res.is_err() {
                    println!("failed to fail process");
                }
            }
        };
    }
}
