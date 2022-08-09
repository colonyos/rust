mod crypto;

use crate::crypto::cryptolib;

fn main() {
    let p = cryptolib::gen_prvkey();
    let id = cryptolib::gen_id(&p);
    let msg = "hello".to_string();
    let s = cryptolib::gen_signature(&msg, &p);
    let h = cryptolib::gen_hash(&msg);
    let rid = cryptolib::recid(&msg, &s);

    println!("{p}");
    println!("{id}");
    println!("{s}");
    println!("{h}");
    println!("{rid}");
}
