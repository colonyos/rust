use colonies;
use colonies::core::Attribute;
use colonies::core::Colony;
use colonies::core::Conditions;
use colonies::core::ProcessSpec;
use colonies::core::Runtime;
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

async fn create_test_runtime(colonyid: &String, prvkey: &String) -> (Runtime, Runtime, String) {
    let runtimeprvkey = crypto::gen_prvkey();
    let runtimeid = crypto::gen_id(&runtimeprvkey);
    let charset = "abcdefghijklmnopqrstuvwxyz";
    let name = generate(64, charset);

    let runtime = Runtime::new(&name, &runtimeid, "test_runtime_type", &colonyid);
    let added_runtime = colonies::add_runtime(&runtime, &prvkey).await.unwrap();
    let _ = colonies::approve_runtime(&runtimeid, &prvkey).await;

    (runtime.clone(), added_runtime, runtimeprvkey)
}

fn create_test_process_spec(colonyid: &str) -> ProcessSpec {
    let conditions = Conditions::new(colonyid, "test_runtime_type");
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
async fn test_add_runtime() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_runtime(&colony.colonyid, &colonyprvkey).await;
    let runtime = t.0;
    let added_runtime = t.1;

    assert_eq!(runtime.runtimeid, added_runtime.runtimeid)
}

#[tokio::test]
async fn test_submit() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_runtime(&colony.colonyid, &colonyprvkey).await;
    let runtimeprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());

    let process = colonies::submit(&spec, &runtimeprvkey).await.unwrap();
    assert_eq!(64, process.processid.len())
}

#[tokio::test]
async fn test_assign() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_runtime(&colony.colonyid, &colonyprvkey).await;
    let runtimeprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());
    let submitted_process = colonies::submit(&spec, &runtimeprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.colonyid, false, 10, &runtimeprvkey)
        .await
        .unwrap();
    assert_eq!(submitted_process.processid, assigned_process.processid)
}

#[tokio::test]
async fn test_close() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_runtime(&colony.colonyid, &colonyprvkey).await;
    let runtimeprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());
    let submitted_process = colonies::submit(&spec, &runtimeprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.colonyid, false, 10, &runtimeprvkey)
        .await
        .unwrap();
    assert_eq!(submitted_process.processid, assigned_process.processid);

    let _ = colonies::close(&assigned_process.processid, &runtimeprvkey).await;
}

#[tokio::test]
async fn test_failed() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_runtime(&colony.colonyid, &colonyprvkey).await;
    let runtimeprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());
    let submitted_process = colonies::submit(&spec, &runtimeprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.colonyid, false, 10, &runtimeprvkey)
        .await
        .unwrap();
    assert_eq!(submitted_process.processid, assigned_process.processid);

    let _ = colonies::fail(&assigned_process.processid, &runtimeprvkey).await;
}

#[tokio::test]
async fn test_add_attr() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_runtime(&colony.colonyid, &colonyprvkey).await;
    let runtimeprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());
    let submitted_process = colonies::submit(&spec, &runtimeprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.colonyid, false, 10, &runtimeprvkey)
        .await
        .unwrap();

    assert_eq!(submitted_process.processid, assigned_process.processid);

    let attr = Attribute::new(
        &colony.colonyid,
        &assigned_process.processid,
        "test_key",
        "test_value",
    );
    let added_attr = colonies::add_attr(&attr, &runtimeprvkey).await.unwrap();
    assert_eq!(64, added_attr.attributeid.len());
    assert_eq!(added_attr.targetcolonyid, attr.targetcolonyid);
    assert_eq!(added_attr.key, attr.key);
    assert_eq!(added_attr.value, attr.value);

    let _ = colonies::close(&assigned_process.processid, &runtimeprvkey).await;
}

#[tokio::test]
async fn test_get_process() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_runtime(&colony.colonyid, &colonyprvkey).await;
    let runtimeprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());
    let submitted_process = colonies::submit(&spec, &runtimeprvkey).await.unwrap();
    let assigned_process = colonies::assign(&colony.colonyid, false, 10, &runtimeprvkey)
        .await
        .unwrap();

    assert_eq!(submitted_process.processid, assigned_process.processid);
    let _ = colonies::close(&assigned_process.processid, &runtimeprvkey).await;

    let process_from_server = colonies::get_process(&assigned_process.processid, &runtimeprvkey)
        .await
        .unwrap();

    assert_eq!(process_from_server.processid, assigned_process.processid);
}

#[tokio::test]
async fn test_get_processes() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_runtime(&colony.colonyid, &colonyprvkey).await;
    let runtimeprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());
    let submitted_process1 = colonies::submit(&spec, &runtimeprvkey).await.unwrap();
    let submitted_process2 = colonies::submit(&spec, &runtimeprvkey).await.unwrap();

    let processes = colonies::get_processes(
        &colony.colonyid,
        100,
        colonies::core::PENDING,
        &runtimeprvkey,
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

    let t = create_test_runtime(&colony.colonyid, &colonyprvkey).await;
    let runtimeprvkey = t.2;

    let _ = colonies::get_processes(
        &colony.colonyid,
        100,
        colonies::core::PENDING,
        &runtimeprvkey,
    )
    .await
    .unwrap();
}
