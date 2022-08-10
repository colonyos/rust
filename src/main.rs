mod colonieslib;
mod cryptolib;

use crate::colonieslib::colonies;
use crate::colonieslib::core;
use crate::colonieslib::core::Colony;
use crate::colonieslib::rpc;
//use crate::colonieslib::rpc::AddColonyRPCMsg;
use crate::cryptolib::crypto;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let p = crypto::gen_prvkey();
    let id = crypto::gen_id(&p);
    let msg = "hello".to_string();
    let s = crypto::gen_signature(&msg, &p);
    let h = crypto::gen_hash(&msg);
    let rid = crypto::recid(&msg, &s);
    colonies::say_hello();

    println!("{p}");
    println!("{id}");
    println!("{s}");
    println!("{h}");
    println!("{rid}");

    let colony = Colony {
        colonyid: "test_colony_id".to_owned(),
        name: "test_colony_name".to_owned(),
    };

    let colony = core::marshall_colony_json(&colony);

    let colony_json = match colony {
        Ok(str) => str,
        Err(error) => panic!("{error}"),
    };
    println!("{colony_json}");

    let colony2 = core::unmarshall_colony_json(colony_json).unwrap();
    println!("{:?}", colony2);

    let colony_prvkey = crypto::gen_prvkey();
    let colony_id = crypto::gen_id(&p);
    let colony = Colony {
        colonyid: colony_id,
        name: "test_colony_name".to_owned(),
    };

    let server_prvkey = "fcc79953d8a751bf41db661592dc34d30004b1a651ffa0725b03ac227641499d";
    let add_colony_rpcmsg =
        rpc::compose_add_colonyrpcmsg(colony, server_prvkey.to_owned()).unwrap();
    println!("{:?}", add_colony_rpcmsg);

    rpc::send_rpcmsg(add_colony_rpcmsg).await?;

    Ok(())
}
