pub fn hash(clear_pwd: &str) -> Result<String, pwhash::error::Error> {
    pwhash::bcrypt::hash(clear_pwd)
}

pub fn verify(pass: &str, hash_pwd: &str) -> bool {
    pwhash::bcrypt::verify(pass, hash_pwd)
}
