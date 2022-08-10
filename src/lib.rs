pub mod core;
pub mod crypto;
mod crypto_test;
pub mod rpc;

pub async fn add_colony(
    colony: &core::Colony,
    prvkey: &String,
) -> Result<core::Colony, rpc::RPCError> {
    let add_colony_rpcmsg = rpc::compose_add_colony_rpcmsg(colony, prvkey);
    let reply_json = rpc::send_rpcmsg(add_colony_rpcmsg).await?;
    let colony: core::Colony = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(colony)
}

pub async fn add_runtime(
    runtime: &core::Runtime,
    prvkey: &String,
) -> Result<core::Runtime, rpc::RPCError> {
    let add_runtime_rpcmsg = rpc::compose_add_runtime_rpcmsg(runtime, prvkey);
    let reply_json = rpc::send_rpcmsg(add_runtime_rpcmsg).await?;
    let runtime: core::Runtime = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(runtime)
}

pub async fn approve_runtime(runtimeid: &String, prvkey: &String) -> Result<(), rpc::RPCError> {
    let add_runtime_rpcmsg = rpc::compose_approve_runtime_rpcmsg(runtimeid, prvkey);
    rpc::send_rpcmsg(add_runtime_rpcmsg).await?;

    Ok(())
}

pub async fn submit_processspec(
    spec: core::ProcessSpec,
    prvkey: String,
) -> Result<core::Process, rpc::RPCError> {
    let add_runtime_rpcmsg = rpc::compose_submit_processpec_rpcmsg(spec, prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(add_runtime_rpcmsg).await?;
    let process: core::Process = serde_json::from_str(reply_json.as_str()).unwrap();

    Ok(process)
}
