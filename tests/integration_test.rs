use colonies;
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

    let runtime = Runtime {
        runtimeid: runtimeid.to_owned(),
        runtimetype: "test_runtime_type".to_owned(),
        name: name,
        colonyid: colonyid.to_owned(),
        cpu: "".to_owned(),
        cores: 0,
        mem: 0,
        gpu: "".to_owned(),
        gpus: 0,
        state: 0,
        commissiontime: "2022-08-08T10:22:25.819199495+02:00".to_owned(),
        lastheardfromtime: "2022-08-08T10:22:25.819199495+02:00".to_owned(),
    };

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
async fn test_submit_processspec() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_runtime(&colony.colonyid, &colonyprvkey).await;
    let runtimeprvkey = t.2;

    let spec = create_test_process_spec(colony.colonyid.as_str());

    let process = colonies::submit_processspec(spec, runtimeprvkey)
        .await
        .unwrap();
    assert_eq!(64, process.processid.len())
}
