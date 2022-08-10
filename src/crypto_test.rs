#[cfg(test)]
use crate::crypto;

#[test]
fn test_gen_prvkey() {
    let p = crypto::gen_prvkey();
    assert_eq!(64, p.len())
}

#[test]
fn test_gen_id() {
    let p = crypto::gen_prvkey();
    let p = crypto::gen_id(&p);
    assert_eq!(64, p.len())
}

#[test]
fn test_gen_signature() {
    let p = crypto::gen_prvkey();
    let msg = "test".to_string();
    let s = crypto::gen_signature(&msg, &p);
    assert_eq!(130, s.len())
}

#[test]
fn test_gen_hash() {
    let msg = "test".to_string();
    let s = crypto::gen_hash(&msg);
    assert_eq!(64, s.len())
}

#[test]
fn test_recid() {
    let p = crypto::gen_prvkey();
    let id = crypto::gen_id(&p);
    let msg = "hello".to_string();
    let s = crypto::gen_signature(&msg, &p);
    let rid = crypto::recid(&msg, &s);
    assert_eq!(id, rid)
}
