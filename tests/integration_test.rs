use colonies;
use colonies::core::Attribute;
use colonies::core::Colony;
use colonies::core::Conditions;
use colonies::core::Executor;
use colonies::core::Function;
use colonies::core::ProcessSpec;
use colonies::crypto;
use random_string::generate;
use std::collections::HashMap;

async fn create_test_colony() -> (Colony, Colony, String) {
    let colonyprvkey = crypto::gen_prvkey();
    let colonyid = crypto::gen_id(&colonyprvkey);
    let colony = Colony {
        colonyid: colonyid,
        name: "test_colony_name".to_owned(),
    };

    let serverprvkey = "fcc79953d8a751bf41db661592dc34d30004b1a651ffa0725b03ac227641499d";
    let added_colony = colonies::add_colony(&colony, &serverprvkey.to_owned())
        .await
        .unwrap();

    (colony, added_colony, colonyprvkey)
}

async fn create_test_executor(colonyid: &String, prvkey: &String) -> (Executor, Executor, String) {
    let executorprvkey = crypto::gen_prvkey();
    let executorid = crypto::gen_id(&executorprvkey);
    let charset = "abcdefghijklmnopqrstuvwxyz";
    let name = generate(64, charset);

    let functions: Vec<Function> = Vec::new();
    let executor = Executor::new(
        &name,
        &executorid,
        "test_executor_type",
        &colonyid,
        functions,
    );
    let added_executor = colonies::add_executor(&executor, &prvkey).await.unwrap();
    let _ = colonies::approve_executor(&executorid, &prvkey).await;

    (executor.clone(), added_executor, executorprvkey)
}

fn create_test_process_spec(colonyid: &str) -> ProcessSpec {
    let conditions = Conditions::new(colonyid, "test_executor_type");
    let mut args: Vec<String> = Vec::new();
    args.push("test_args".to_owned());
    let mut env: HashMap<String, String> = HashMap::new();
    env.insert("test_key".to_owned(), "test_value".to_owned());
    ProcessSpec::new("test_name", "test_func", args, -1, -1, -1, conditions, env)
}

#[tokio::test]
async fn test_add_colony() {
    let t = create_test_colony().await;
    let colony = t.0;
    let added_colony = t.1;

    assert_eq!(colony.colonyid, added_colony.colonyid)
}

#[tokio::test]
async fn test_add_executor() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.colonyid, &colonyprvkey).await;
    let executor = t.0;
    let added_executor = t.1;

    assert_eq!(executor.executorid, added_executor.executorid)
}

#[tokio::test]
async fn test_submit() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.colonyid, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());

    let process = colonies::submit(&spec, &executorprvkey).await.unwrap();
    assert_eq!(64, process.processid.len())
}

#[tokio::test]
async fn test_assign() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.colonyid, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());
    let submitted_process = colonies::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.colonyid, false, 10, &executorprvkey)
        .await
        .unwrap();
    assert_eq!(submitted_process.processid, assigned_process.processid)
}

#[tokio::test]
async fn test_close() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.colonyid, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());
    let submitted_process = colonies::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.colonyid, false, 10, &executorprvkey)
        .await
        .unwrap();
    assert_eq!(submitted_process.processid, assigned_process.processid);

    let _ = colonies::close(&assigned_process.processid, &executorprvkey).await;
}

#[tokio::test]
async fn test_failed() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.colonyid, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());
    let submitted_process = colonies::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.colonyid, false, 10, &executorprvkey)
        .await
        .unwrap();
    assert_eq!(submitted_process.processid, assigned_process.processid);

    let _ = colonies::fail(&assigned_process.processid, &executorprvkey).await;
}

#[tokio::test]
async fn test_add_attr() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.colonyid, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());
    let submitted_process = colonies::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.colonyid, false, 10, &executorprvkey)
        .await
        .unwrap();

    assert_eq!(submitted_process.processid, assigned_process.processid);

    let attr = Attribute::new(
        &colony.colonyid,
        &assigned_process.processid,
        "test_key",
        "test_value",
    );
    let added_attr = colonies::add_attr(&attr, &executorprvkey).await.unwrap();
    assert_eq!(64, added_attr.attributeid.len());
    assert_eq!(added_attr.targetcolonyid, attr.targetcolonyid);
    assert_eq!(added_attr.key, attr.key);
    assert_eq!(added_attr.value, attr.value);

    let _ = colonies::close(&assigned_process.processid, &executorprvkey).await;
}

#[tokio::test]
async fn test_get_process() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.colonyid, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());
    let submitted_process = colonies::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.colonyid, false, 10, &executorprvkey)
        .await
        .unwrap();

    assert_eq!(submitted_process.processid, assigned_process.processid);
    let _ = colonies::close(&assigned_process.processid, &executorprvkey).await;

    let process_from_server = colonies::get_process(&assigned_process.processid, &executorprvkey)
        .await
        .unwrap();

    assert_eq!(process_from_server.processid, assigned_process.processid);
}

#[tokio::test]
async fn test_get_processes() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.colonyid, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());
    let submitted_process1 = colonies::submit(&spec, &executorprvkey).await.unwrap();
    let submitted_process2 = colonies::submit(&spec, &executorprvkey).await.unwrap();

    let processes = colonies::get_processes(
        &colony.colonyid,
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
}

#[tokio::test]
async fn test_get_processes_empty() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.colonyid, &colonyprvkey).await;
    let executorprvkey = t.2;

    let _ = colonies::get_processes(
        &colony.colonyid,
        100,
        colonies::core::PENDING,
        &executorprvkey,
    )
    .await
    .unwrap();
}
