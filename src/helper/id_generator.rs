use sha1::{Digest, Sha1};

pub fn id_generator(title: &str, description: &str) -> String {
    let mut hash = Sha1::new();
    hash.update(title);
    hash.update(description);
    let result = hash.finalize();

    let hex_str = hex::encode(&result[..4]);
    return hex_str;
}
