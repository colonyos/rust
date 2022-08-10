#[cfg(test)]
use crate::crypto::cryptolib;

#[test]
fn test_gen_prvkey() {
    let p = cryptolib::gen_prvkey();
    assert_eq!(64, p.len())
}

#[test]
fn test_gen_id() {
    let p = cryptolib::gen_prvkey();
    let p = cryptolib::gen_id(&p);
    assert_eq!(64, p.len())
}

#[test]
fn test_gen_signature() {
    let p = cryptolib::gen_prvkey();
    let msg = "test".to_string();
    let s = cryptolib::gen_signature(&msg, &p);
    assert_eq!(130, s.len())
}

#[test]
fn test_gen_hash() {
    let msg = "test".to_string();
    let s = cryptolib::gen_hash(&msg);
    assert_eq!(64, s.len())
}

#[test]
fn test_recid() {
    let p = cryptolib::gen_prvkey();
    let id = cryptolib::gen_id(&p);
    let msg = "hello".to_string();
    let s = cryptolib::gen_signature(&msg, &p);
    let rid = cryptolib::recid(&msg, &s);
    assert_eq!(id, rid)
}
