use colonies;
use colonies::core::Colony;
use colonies::core::Runtime;
use colonies::crypto;
use random_string::generate;

async fn create_test_colony() -> (Colony, Colony, String) {
    let colony_prvkey = crypto::gen_prvkey();
    let colony_id = crypto::gen_id(&colony_prvkey);
    let colony = Colony {
        colonyid: colony_id,
        name: "test_colony_name".to_owned(),
    };

    let server_prvkey = "fcc79953d8a751bf41db661592dc34d30004b1a651ffa0725b03ac227641499d";
    let added_colony = colonies::add_colony(&colony, &server_prvkey.to_owned())
        .await
        .unwrap();

    (colony, added_colony, colony_prvkey)
}

async fn create_test_runtime(colonyid: String, prvkey: String) -> (Runtime, Runtime, String) {
    let runtime_prvkey = crypto::gen_prvkey();
    let runtime_id = crypto::gen_id(&runtime_prvkey);
    let charset = "abcdefghijklmnopqrstuvwxyz";
    let name = generate(64, charset);

    let runtime = Runtime {
        runtimeid: runtime_id.to_owned(),
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
    let _ = colonies::approve_runtime(runtime_id, prvkey).await;

    (runtime.clone(), added_runtime, runtime_prvkey)
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
    let colony_prvkey = t.2;

    let t = create_test_runtime(colony.colonyid, colony_prvkey).await;
    let runtime = t.0;
    let added_runtime = t.1;

    assert_eq!(runtime.runtimeid, added_runtime.runtimeid)
}
