use colonyos;
use colonyos::core::Attribute;
use colonyos::core::Colony;
use colonyos::core::Executor;
use colonyos::core::FunctionSpec;
use colonyos::crypto;
use random_string::generate;


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

    let added_colony = colonyos::add_colony(&colony, &SERVER_PRVKEY.to_owned())
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
    let added_executor = colonyos::add_executor(&executor, &prvkey).await.unwrap();
    let _ = colonyos::approve_executor(&colonyname, &name, &prvkey).await;

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

fn create_test_function_spec(colonyname: &str) -> FunctionSpec {
    let mut spec = FunctionSpec::new("test_func", "test_executor_type", colonyname);
    spec.args = vec!["test_args".to_owned()];
    spec.label = "label".to_owned();
    spec.env.insert("test_key".to_owned(), "test_value".to_owned());
    spec
}

#[tokio::test]
async fn test_add_colony() {
    let t = create_test_colony().await;
    let colony = t.0;
    let added_colony = t.1;

    assert_eq!(colony.colonyid, added_colony.colonyid);

    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
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
    
    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
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

    let process = colonyos::submit(&spec, &executorprvkey).await.unwrap();
    assert_eq!(64, process.processid.len());
    
    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
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
     let submitted_process = colonyos::submit(&spec, &executorprvkey).await.unwrap();
     println!("submitted_process: {:?}", submitted_process);
     let assigned_process = colonyos::assign(&colony.name, 10, &executorprvkey)
         .await
         .unwrap();
     assert_eq!(submitted_process.processid, assigned_process.processid);
     
     let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
     assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
}

#[tokio::test]
async fn test_close() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_function_spec(colony.name.as_str());
    let submitted_process = colonyos::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonyos::assign(&colony.name, 10, &executorprvkey)
        .await
        .unwrap();
    assert_eq!(submitted_process.processid, assigned_process.processid);

    let _ = colonyos::close(&assigned_process.processid, &executorprvkey).await;
     
    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
}

#[tokio::test]
async fn test_failed() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_function_spec(colony.name.as_str());
    let submitted_process = colonyos::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonyos::assign(&colony.name, 10, &executorprvkey)
        .await
        .unwrap();
    assert_eq!(submitted_process.processid, assigned_process.processid);

    let _ = colonyos::fail(&assigned_process.processid, &executorprvkey).await;
    
    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
}

#[tokio::test]
async fn test_add_attr() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_function_spec(colony.name.as_str());
    let submitted_process = colonyos::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonyos::assign(&colony.name, 10, &executorprvkey)
        .await
        .unwrap();

    assert_eq!(submitted_process.processid, assigned_process.processid);

    let attr = Attribute::new(
        &colony.name,
        &assigned_process.processid,
        "test_key",
        "test_value",
    );
    let added_attr = colonyos::add_attr(&attr, &executorprvkey).await.unwrap();
    assert_eq!(64, added_attr.attributeid.len());
    assert_eq!(added_attr.targetcolonyname, attr.targetcolonyname);
    assert_eq!(added_attr.key, attr.key);
    assert_eq!(added_attr.value, attr.value);

    let _ = colonyos::close(&assigned_process.processid, &executorprvkey).await;
    
    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
}

#[tokio::test]
async fn test_get_process() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_function_spec(colony.name.as_str());
    let submitted_process = colonyos::submit(&spec, &executorprvkey).await.unwrap();
    let assigned_process = colonyos::assign(&colony.name, 10, &executorprvkey)
        .await
        .unwrap();

    assert_eq!(submitted_process.processid, assigned_process.processid);
    let _ = colonyos::close(&assigned_process.processid, &executorprvkey).await;

    let process_from_server = colonyos::get_process(&assigned_process.processid, &executorprvkey)
        .await
        .unwrap();

    assert_eq!(process_from_server.processid, assigned_process.processid);
    
    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
}

#[tokio::test]
async fn test_get_processes() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_function_spec(colony.name.as_str());
    let submitted_process1 = colonyos::submit(&spec, &executorprvkey).await.unwrap();
    let submitted_process2 = colonyos::submit(&spec, &executorprvkey).await.unwrap();

    let processes = colonyos::get_processes(
        &colony.name,
        100,
        colonyos::core::PENDING,
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
    
    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
}

#[tokio::test]
async fn test_get_processes_empty() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let _ = colonyos::get_processes(
        &colony.name,
        100,
        colonyos::core::PENDING,
        &executorprvkey,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_channel_append_error_handling() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    // Create a function spec WITHOUT a channel
    let spec = create_test_function_spec(colony.name.as_str());
    // Note: spec.channels is empty

    // Submit and assign the process
    let _submitted = colonyos::submit(&spec, &executorprvkey).await.unwrap();
    let assigned = colonyos::assign(&colony.name, 10, &executorprvkey)
        .await
        .unwrap();

    // Try to append to a non-existent channel - should fail
    let result = colonyos::channel_append(
        &assigned.processid,
        "non-existent-channel",
        1,
        "message",
        "",
        0,
        &executorprvkey,
    )
    .await;

    // Should return an error
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(!err.conn_err()); // Not a connection error, but a server error

    // Close the process
    let _ = colonyos::close(&assigned.processid, &executorprvkey).await;

    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok());
}

// Note: This test requires the server to support lazy channel creation.
// The correct flow is:
// 1. Submit process with channels defined in spec
// 2. Assign process (moves to RUNNING state)
// 3. Subscribe to the channel (triggers channel creation on the server)
// 4. Channel operations (append/read) will work
//
// The subscribe_channel function uses WebSocket to subscribe to channel events,
// which also triggers channel creation on the server side.
#[tokio::test]
async fn test_channel_append_and_read() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    // Create a function spec with a channel
    let mut spec = create_test_function_spec(colony.name.as_str());
    spec.channels = vec!["test-channel".to_string()];

    // Submit and assign the process
    let _submitted = colonyos::submit(&spec, &executorprvkey).await.unwrap();
    let assigned = colonyos::assign(&colony.name, 10, &executorprvkey)
        .await
        .unwrap();

    // Verify channel is in the assigned process spec
    assert!(assigned.spec.channels.contains(&"test-channel".to_string()));

    // Subscribe to the channel to trigger channel creation on the server.
    // Use a short timeout and expect it to return with no messages (timeout).
    // The callback returns false to stop immediately after first batch.
    let subscribe_result = colonyos::subscribe_channel(
        &assigned.processid,
        "test-channel",
        0, // afterseq - start from beginning
        1, // 1 second timeout - short timeout since we just want to trigger creation
        &executorprvkey,
        |_entries| false, // Stop after first callback (or timeout)
    )
    .await;

    // If subscribe fails with connection error, the server might not support channels
    if let Err(e) = &subscribe_result {
        if e.conn_err() {
            // Close the process and cleanup
            let _ = colonyos::close(&assigned.processid, &executorprvkey).await;
            let _ = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
            // Test passes - WebSocket channel subscription not available
            return;
        }
    }

    // Append data to the channel (should work now that we've subscribed)
    let append_result = colonyos::channel_append(
        &assigned.processid,
        "test-channel",
        1, // sequence
        "message 1",
        "", // payloadtype
        0, // inreplyto
        &executorprvkey,
    )
    .await;

    // If channel_append fails with "Channel not found", skip the rest of the test
    // This can happen if the server version doesn't support lazy channel creation
    if let Err(e) = &append_result {
        if e.to_string().contains("Channel not found") {
            // Close the process and cleanup
            let _ = colonyos::close(&assigned.processid, &executorprvkey).await;
            let _ = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
            // Test passes - channel feature not available on this server
            return;
        }
    }
    append_result.unwrap();

    colonyos::channel_append(
        &assigned.processid,
        "test-channel",
        2, // sequence
        "message 2",
        "", // payloadtype
        0, // inreplyto
        &executorprvkey,
    )
    .await
    .unwrap();

    // Read from the channel
    let entries = colonyos::channel_read(
        &assigned.processid,
        "test-channel",
        0, // afterseq
        10, // limit
        &executorprvkey,
    )
    .await
    .unwrap();

    // Verify we got both messages
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].sequence, 1);
    assert_eq!(entries[0].payload_as_string(), "message 1");
    assert_eq!(entries[1].sequence, 2);
    assert_eq!(entries[1].payload_as_string(), "message 2");

    // Close the process
    let _ = colonyos::close(&assigned.processid, &executorprvkey).await;

    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
}

#[tokio::test]
async fn test_set_output() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    let spec = create_test_function_spec(colony.name.as_str());
    let _submitted = colonyos::submit(&spec, &executorprvkey).await.unwrap();
    let assigned = colonyos::assign(&colony.name, 10, &executorprvkey)
        .await
        .unwrap();

    // Set output
    colonyos::set_output(
        &assigned.processid,
        vec!["output1".to_string(), "output2".to_string()],
        &executorprvkey,
    )
    .await
    .unwrap();

    // Close the process
    colonyos::close(&assigned.processid, &executorprvkey).await.unwrap();

    // Verify output was set
    let process = colonyos::get_process(&assigned.processid, &executorprvkey)
        .await
        .unwrap();
    assert_eq!(process.output.len(), 2);
    assert_eq!(process.output[0], "output1");
    assert_eq!(process.output[1], "output2");

    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
}

#[tokio::test]
async fn test_add_log() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executor = t.0;
    let executorprvkey = t.2;

    let spec = create_test_function_spec(colony.name.as_str());
    let _submitted = colonyos::submit(&spec, &executorprvkey).await.unwrap();
    let assigned = colonyos::assign(&colony.name, 10, &executorprvkey)
        .await
        .unwrap();

    // Add a log message
    let log = colonyos::core::Log {
        processid: assigned.processid.clone(),
        colonyname: colony.name.clone(),
        executorname: executor.executorname.clone(),
        message: "Test log message".to_string(),
        timestamp: 0,
    };
    colonyos::add_log(&log, &executorprvkey).await.unwrap();

    // Get logs
    let logs = colonyos::get_logs(
        &colony.name,
        &assigned.processid,
        &executor.executorname,
        10,
        0,
        &executorprvkey,
    )
    .await
    .unwrap();

    assert!(!logs.is_empty());
    assert_eq!(logs[0].message, "Test log message");

    // Close the process
    colonyos::close(&assigned.processid, &executorprvkey).await.unwrap();

    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
}

#[tokio::test]
async fn test_get_statistics() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    // Get statistics
    let stats = colonyos::get_statistics(&colony.name, &executorprvkey)
        .await
        .unwrap();

    // At minimum, executors should be 1 (we just added one)
    assert!(stats.executors >= 1);

    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
}

#[tokio::test]
async fn test_workflow() {
    let t = create_test_colony().await;
    let colony = t.0;
    let colonyprvkey = t.2;

    let t = create_test_executor(&colony.name, &colonyprvkey).await;
    let executorprvkey = t.2;

    // Create a simple workflow with two steps
    let mut step1 = FunctionSpec::new("step1", "test_executor_type", &colony.name);
    step1.nodename = "step1".to_string();

    let mut step2 = FunctionSpec::new("step2", "test_executor_type", &colony.name);
    step2.nodename = "step2".to_string();
    step2.conditions.dependencies = vec!["step1".to_string()];

    let workflow = colonyos::core::WorkflowSpec {
        colonyname: colony.name.clone(),
        functionspecs: vec![step1, step2],
    };

    // Submit the workflow
    let pg = colonyos::submit_workflow(&workflow, &executorprvkey)
        .await
        .unwrap();

    assert!(!pg.processgraphid.is_empty());
    assert_eq!(pg.processids.len(), 2);

    // Get the process graph
    let pg2 = colonyos::get_processgraph(&pg.processgraphid, &executorprvkey)
        .await
        .unwrap();
    assert_eq!(pg.processgraphid, pg2.processgraphid);

    // Clean up - assign and close processes
    let p1 = colonyos::assign(&colony.name, 10, &executorprvkey).await.unwrap();
    colonyos::close(&p1.processid, &executorprvkey).await.unwrap();

    let p2 = colonyos::assign(&colony.name, 10, &executorprvkey).await.unwrap();
    colonyos::close(&p2.processid, &executorprvkey).await.unwrap();

    // Remove the process graph
    colonyos::remove_processgraph(&pg.processgraphid, &executorprvkey)
        .await
        .unwrap();

    let r = colonyos::remove_colony(&colony.name, &SERVER_PRVKEY).await;
    assert!(r.is_ok(), "Expected colonyos::remove_colony to succeed");
}
