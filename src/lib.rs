pub mod core;
pub mod crypto;
mod crypto_test;
pub mod rpc;

pub async fn add_colony(
    colony: &core::Colony,
    prvkey: &String,
) -> Result<core::Colony, rpc::RPCError> {
    let rpcmsg = rpc::compose_add_colony_rpcmsg(colony, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let colony: core::Colony = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(colony)
}

pub async fn add_runtime(
    runtime: &core::Runtime,
    prvkey: &String,
) -> Result<core::Runtime, rpc::RPCError> {
    let rpcmsg = rpc::compose_add_runtime_rpcmsg(runtime, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let runtime: core::Runtime = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(runtime)
}

pub async fn approve_runtime(runtimeid: &String, prvkey: &String) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_approve_runtime_rpcmsg(runtimeid, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;

    Ok(())
}

pub async fn submit(
    spec: &core::ProcessSpec,
    prvkey: &String,
) -> Result<core::Process, rpc::RPCError> {
    let rpcmsg = rpc::compose_submit_processpec_rpcmsg(spec, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let process: core::Process = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(process)
}

pub async fn assign(
    colonyid: &String,
    latest: bool,
    timeout: i32,
    prvkey: &String,
) -> Result<core::Process, rpc::RPCError> {
    let rpcmsg = rpc::compose_assign_process_rpcmsg(colonyid, latest, timeout, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let process: core::Process = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(process)
}

pub async fn close(processid: &String, prvkey: &String) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_close_process_rpcmsg(processid, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;

    Ok(())
}

pub async fn fail(processid: &String, prvkey: &String) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_fail_process_rpcmsg(processid, prvkey);
    rpc::send_rpcmsg(rpcmsg).await?;

    Ok(())
}

pub async fn add_attr(
    attr: &core::Attribute,
    prvkey: &String,
) -> Result<core::Attribute, rpc::RPCError> {
    let rpcmsg = rpc::compose_add_attr_rpcmsg(attr, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let attr: core::Attribute = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(attr)
}

pub async fn get_process(
    processid: &String,
    prvkey: &String,
) -> Result<core::Process, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_process_rpcmsg(processid, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let attr: core::Process = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(attr)
}

pub async fn get_processes(
    colonyid: &String,
    count: i32,
    state: i32,
    prvkey: &String,
) -> Result<Vec<core::Process>, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_processes_rpcmsg(colonyid, count, state, prvkey);
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let processes: Result<Vec<core::Process>, serde_json::Error> =
        serde_json::from_str(reply_json.as_str());

    let processes = match processes {
        Ok(processes) => processes,
        Err(_) => Vec::new(),
    };

    Ok(processes)
}
