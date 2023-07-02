use colonies;
use tokio::time::{sleep, Duration};

fn fib(n: i64) -> i64 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

#[tokio::main]
async fn main() {
    let colonyid = "d051bc9306f3b9addf3c735a7e12b4abb0310be5e47086306c4136d8bca7331c";
    let executorprvkey = "20452339785f6793bde92befb1b8f8bcb3394bb4f8e0b7899c5c48fbd98c7f7b";

    loop {
        println!("trying to get an assignment");
        let assigned_process = colonies::assign(&colonyid, false, 10, &executorprvkey).await;
        let assigned_process = match assigned_process {
            Ok(process) => process,
            Err(err) => {
                if err.conn_err() {
                    println!("connection problem, re-trying in 1 second");
                    sleep(Duration::from_millis(1000)).await;
                }
                println!("{}", err);
                println!("timeout, or another worker got the assignment, re-trying ...");
                continue;
            }
        };

        println!(
            "yippi, we are assigned to process id={}",
            assigned_process.processid
        );

        match assigned_process.spec.funcname.as_str() {
            "calc_fibonacci" => {
                //println!("{}", assigned_process.spec.args);
                let f = fib(45);
                println!("fibonacci of {} is {}", 40, f);
                let res = colonies::close(&assigned_process.processid, executorprvkey).await;
                if res.is_err() {
                    println!("failed to close process");
                }
            }
            _ => {
                println!("invalid function name");
                let res = colonies::fail(&assigned_process.processid, executorprvkey).await;
                if res.is_err() {
                    println!("failed to fail process");
                }
            }
        };
    }
}
