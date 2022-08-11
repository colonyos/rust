pub mod core;
pub mod crypto;
mod crypto_test;
pub mod rpc;

pub async fn add_colony(
    colony: &core::Colony,
    prvkey: &str,
) -> Result<core::Colony, rpc::RPCError> {
    let rpcmsg = rpc::compose_add_colony_rpcmsg(colony, &prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let colony: core::Colony = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(colony)
}

pub async fn add_runtime(
    runtime: &core::Runtime,
    prvkey: &str,
) -> Result<core::Runtime, rpc::RPCError> {
    let rpcmsg = rpc::compose_add_runtime_rpcmsg(runtime, &prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let runtime: core::Runtime = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(runtime)
}

pub async fn approve_runtime(runtimeid: &str, prvkey: &str) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_approve_runtime_rpcmsg(&runtimeid.to_owned(), &prvkey.to_owned());
    rpc::send_rpcmsg(rpcmsg).await?;

    Ok(())
}

pub async fn submit(
    spec: &core::ProcessSpec,
    prvkey: &str,
) -> Result<core::Process, rpc::RPCError> {
    let rpcmsg = rpc::compose_submit_processpec_rpcmsg(spec, &prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let process: core::Process = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(process)
}

pub async fn assign(
    colonyid: &str,
    latest: bool,
    timeout: i32,
    prvkey: &str,
) -> Result<core::Process, rpc::RPCError> {
    let rpcmsg = rpc::compose_assign_process_rpcmsg(
        &colonyid.to_owned(),
        latest,
        timeout,
        &prvkey.to_owned(),
    );
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let process: core::Process = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(process)
}

pub async fn close(processid: &str, prvkey: &str) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_close_process_rpcmsg(&processid.to_owned(), &prvkey.to_owned());
    rpc::send_rpcmsg(rpcmsg).await?;

    Ok(())
}

pub async fn fail(processid: &str, prvkey: &str) -> Result<(), rpc::RPCError> {
    let rpcmsg = rpc::compose_fail_process_rpcmsg(&processid.to_owned(), &prvkey.to_owned());
    rpc::send_rpcmsg(rpcmsg).await?;

    Ok(())
}

pub async fn add_attr(
    attr: &core::Attribute,
    prvkey: &str,
) -> Result<core::Attribute, rpc::RPCError> {
    let rpcmsg = rpc::compose_add_attr_rpcmsg(attr, &prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let attr: core::Attribute = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(attr)
}

pub async fn get_process(processid: &str, prvkey: &str) -> Result<core::Process, rpc::RPCError> {
    let rpcmsg = rpc::compose_get_process_rpcmsg(&processid.to_owned(), &prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let attr: core::Process = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(attr)
}

pub async fn get_processes(
    colonyid: &str,
    count: i32,
    state: i32,
    prvkey: &str,
) -> Result<Vec<core::Process>, rpc::RPCError> {
    let rpcmsg =
        rpc::compose_get_processes_rpcmsg(&colonyid.to_owned(), count, state, &prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(rpcmsg).await?;
    let processes: Result<Vec<core::Process>, serde_json::Error> =
        serde_json::from_str(reply_json.as_str());

    let processes = match processes {
        Ok(processes) => processes,
        Err(_) => Vec::new(),
    };

    Ok(processes)
}
