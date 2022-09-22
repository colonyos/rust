use colonies;
use colonies::core::Colony;
use colonies::crypto;
use colonies::rpc;

// main
#[tokio::main]
async fn main() -> Result<(), rpc::RPCError> {
    let colony_prvkey = crypto::gen_prvkey();
    let colony_id = crypto::gen_id(&colony_prvkey);
    let colony = Colony {
        colonyid: colony_id,
        name: "test_colony_name".to_owned(),
    };

    let server_prvkey = "fcc79953d8a751bf41db661592dc34d30004b1a651ffa0725b03ac227641499d";
    let added_colony = colonies::add_colony(&colony, &server_prvkey.to_owned()).await?;
    println!("{:?}", colony_prvkey);
    println!("{:?}", added_colony);

    Ok(())
}
