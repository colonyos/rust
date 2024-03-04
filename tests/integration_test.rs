use colonies;
use colonies::core::Attribute;
use colonies::core::Colony;
use colonies::core::Conditions;
use colonies::core::Executor;
use colonies::core::FunctionSpec;
use colonies::crypto;
use random_string::generate;
use std::collections::HashMap;


const SERVER_PRVKEY: &str = "fcc79953d8a751bf41db661592dc34d30004b1a651ffa0725b03ac227641499d";

async fn create_test_colony() -> (Colony, Colony, String) {
    // randomly generate a colony name
    let charset = "abcdefghijklmnopqrstuvwxyz";
    let name = generate(64, charset);

    let colonyprvkey = crypto::gen_prvkey();
    let colonyid = crypto::gen_id(&colonyprvkey);
    let colony = Colony {
        colonyid: colonyid,
        name: "test_colony_name_".to_owned() +&name,
    };

    let added_colony = colonies::add_colony(&colony, &SERVER_PRVKEY.to_owned())
        .await
        .unwrap();

    (colony, added_colony, colonyprvkey)
}

async fn create_test_executor(colonyname: &String, prvkey: &String) -> (Executor, Executor, String) {
    let executorprvkey = crypto::gen_prvkey();
    let executorid = crypto::gen_id(&executorprvkey);
    let charset = "abcdefghijklmnopqrstuvwxyz";
    let name = generate(64, charset);

    let executor = Executor::new(&name, &executorid, "test_executor_type", &colonyname);
    let added_executor = colonies::add_executor(&executor, &prvkey).await.unwrap();
    let _ = colonies::approve_executor(&colonyname, &name, &prvkey).await;

    (executor.clone(), added_executor, executorprvkey)
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct Conditions {
//     pub colonyname: String,
//     pub executornames: Vec<String>,
//     pub executortype: String,
//     pub dependencies: Vec<String>,
//     pub nodes: i32,
//     pub cpu: String,
//     pub processes: i32,
//     #[serde(rename = "processes-per-node")]
//     pub processes_per_node: i32,  
//     pub mem: String,
//     pub storage: String,
//     pub gpu: GPU,
//     pub walltime: i64,
// }

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct GPU {
//     pub name: String,
//     pub mem: String,
//     pub count: i32,
//     pub nodecount: i32,
// }

//
fn create_test_function_spec(colonyname: &str) -> FunctionSpec {
    let conditions = Conditions{
        colonyname: colonyname.to_owned(),
        executornames: vec![],
        executortype: "test_executor_type".to_owned(),
        dependencies: vec![],
        nodes: -1,
        cpu: "".to_owned(),
        processes: -1,
        processes_per_node: -1,
        mem: "".to_owned(),
        storage: "".to_owned(),
        gpu: colonies::core::GPU{
            name: "".to_owned(),
            mem: "".to_owned(),
            count: -1,
            nodecount: -1,
        },
        walltime: -1,
    };
    let mut args: Vec<String> = Vec::new();
    args.push("test_args".to_owned());
    let mut env: HashMap<String, String> = HashMap::new();
    env.insert("test_key".to_owned(), "test_value".to_owned());
    FunctionSpec::new(
        "test_name",
        "test_func",
        args,
        -1,
        -1,
        -1,
        conditions,
        "label",
        env,
    )
}

#[tokio::test]
async fn test_add_colony() {
    let t = create_test_colony().await;
    let colony = t.0;
    let added_colony = t.1;

    assert_eq!(colony.colonyid, added_colony.colonyid);

    let r = colonies::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonies::remove_colony to succeed");
}

#[tokio::test]
async fn test_add_executor() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executor = t.0;
    let added_executor = t.1;
   
    assert_eq!(executor.executorid, added_executor.executorid);
    
    let r = colonies::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonies::remove_colony to succeed");
}

#[tokio::test]
async fn test_submit() {
    println!("---------------------------- 1");
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    println!("---------------------------- 3");
    let spec = create_test_function_spec(colony.name.as_str());

    let process = colonies::submit(&spec, &executorprvkey).await.unwrap();
    assert_eq!(64, process.processid.len());
    
    let r = colonies::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonies::remove_colony to succeed");
}

#[tokio::test]
async fn test_assign() {
    println!("---------------------------- 1");
    let t = create_test_colony().await;
    println!("---------------------------- 2");
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

     println!("---------------------------- 3");

     let spec = create_test_function_spec(colony.name.as_str());
     let submitted_process = colonies::submit(&spec, &executorprvkey).await.unwrap();
     println!("submitted_process: {:?}", submitted_process);
     let assigned_process = colonies::assign(&colony.name, 10, &executorprvkey)
         .await
         .unwrap();
     assert_eq!(submitted_process.processid, assigned_process.processid);
     
     let r = colonies::remove_colony(&colony.name, &SERVER_PRVKEY).await;
     assert!(r.is_ok(), "Expected colonies::remove_colony to succeed");
}

#[tokio::test]
async fn test_close() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_function_spec(colony.name.as_str());
    let submitted_process = colonies::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.name, 10, &executorprvkey)
        .await
        .unwrap();
    assert_eq!(submitted_process.processid, assigned_process.processid);

    let _ = colonies::close(&assigned_process.processid, &executorprvkey).await;
     
    let r = colonies::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonies::remove_colony to succeed");
}

#[tokio::test]
async fn test_failed() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_function_spec(colony.name.as_str());
    let submitted_process = colonies::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.name, 10, &executorprvkey)
        .await
        .unwrap();
    assert_eq!(submitted_process.processid, assigned_process.processid);

    let _ = colonies::fail(&assigned_process.processid, &executorprvkey).await;
    
    let r = colonies::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonies::remove_colony to succeed");
}

#[tokio::test]
async fn test_add_attr() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_function_spec(colony.name.as_str());
    let submitted_process = colonies::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.name, 10, &executorprvkey)
        .await
        .unwrap();

    assert_eq!(submitted_process.processid, assigned_process.processid);

    let attr = Attribute::new(
        &colony.name,
        &assigned_process.processid,
        "test_key",
        "test_value",
    );
    let added_attr = colonies::add_attr(&attr, &executorprvkey).await.unwrap();
    assert_eq!(64, added_attr.attributeid.len());
    assert_eq!(added_attr.targetcolonyname, attr.targetcolonyname);
    assert_eq!(added_attr.key, attr.key);
    assert_eq!(added_attr.value, attr.value);

    let _ = colonies::close(&assigned_process.processid, &executorprvkey).await;
    
    let r = colonies::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonies::remove_colony to succeed");
}

#[tokio::test]
async fn test_get_process() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_function_spec(colony.name.as_str());
    let submitted_process = colonies::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.name, 10, &executorprvkey)
        .await
        .unwrap();

    assert_eq!(submitted_process.processid, assigned_process.processid);
    let _ = colonies::close(&assigned_process.processid, &executorprvkey).await;

    let process_from_server = colonies::get_process(&assigned_process.processid, &executorprvkey)
        .await
        .unwrap();

    assert_eq!(process_from_server.processid, assigned_process.processid);
    
    let r = colonies::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonies::remove_colony to succeed");
}

#[tokio::test]
async fn test_get_processes() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_function_spec(colony.name.as_str());
    let submitted_process1 = colonies::submit(&spec, &executorprvkey).await.unwrap();
    let submitted_process2 = colonies::submit(&spec, &executorprvkey).await.unwrap();

    let processes = colonies::get_processes(
        &colony.name,
        100,
        colonies::core::PENDING,
        &executorprvkey,
    )
    .await
    .unwrap();

    let mut counter = 0;
    let iter = processes.iter();
    for p in iter {
        if p.processid == submitted_process1.processid {
            counter = counter + 1;
        }
        if p.processid == submitted_process2.processid {
            counter = counter + 1;
        }
    }

    assert_eq!(counter, 2);
    
    let r = colonies::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonies::remove_colony to succeed");
}

#[tokio::test]
async fn test_get_processes_empty() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let _ = colonies::get_processes(
        &colony.name,
        100,
        colonies::core::PENDING,
        &executorprvkey,
    )
    .await
    .unwrap();
}
