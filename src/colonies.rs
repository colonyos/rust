use crate::colonieslib::core;
use crate::colonieslib::rpc;

pub async fn add_colony(
    colony: core::Colony,
    prvkey: String,
) -> Result<core::Colony, rpc::RPCError> {
    let add_colony_rpcmsg = rpc::compose_add_colonyrpcmsg(colony, prvkey.to_owned());
    let reply_json = rpc::send_rpcmsg(add_colony_rpcmsg).await?;

    let colony: core::Colony = serde_json::from_str(reply_json.as_str()).unwrap();
    Ok(colony)
}
