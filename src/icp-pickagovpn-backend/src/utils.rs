use bytes::{BytesMut, BufMut};

pub struct Server {
    ip: String,
    username: String,
    pass: String
}

pub fn encrypt_server(
    _ip: String,
    _username: String,
    _pass: String,
    _salt: String
) -> BytesMut {
    let mut encrypted = BytesMut::with_capacity(1024);
    encrypted.put(&b"salt"[..]);

    encrypted
}

pub fn decrypt_server(data: BytesMut) -> Server {
    Server {
       ip: "10.10.10.101".to_string(),
       pass: "funky".to_string(),
       username: "me".to_string()
    }
}
