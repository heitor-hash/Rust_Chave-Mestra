use aes_gcm::{
    AeadCore, Aes256Gcm, Error, KeyInit, Nonce,
    aead::{AeadMut, OsRng},
};
use sha2::Digest;

pub fn key_into_hash(key: &str) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();
    hasher.update(key);

    let hash = hasher.finalize();
    hash.into()
}

pub fn encrypt(data: &[u8], hash: &[u8; 32]) -> Result<Vec<u8>, Error> {
    let mut cipher = Aes256Gcm::new_from_slice(hash).unwrap();
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let mut data_pac = Vec::new();
    data_pac.extend_from_slice("123456789".as_bytes());
    data_pac.extend_from_slice(data);
    let data_pac: &[u8] = &data_pac;
    let ciphertext = match cipher.encrypt(&nonce, data_pac) {
        Ok(a) => a,
        Err(e) => {
            println!("Erro ao criptografar, {}", &e);
            return Err(e);
        }
    };
    let mut pac = Vec::new();
    pac.extend_from_slice(&nonce);
    pac.extend_from_slice(&ciphertext);
    Ok(pac)
}

pub fn decrypt(data: &[u8], hash: &[u8; 32]) -> Result<Vec<u8>, Error> {
    let mut cipher = Aes256Gcm::new_from_slice(hash).unwrap();

    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let ret = match cipher.decrypt(nonce, ciphertext) {
        Ok(a) => a,
        Err(e) => {
            println!("Falha ao descriptografar {}", e);
            return Err(e);
        }
    };
    let (test, deciphertext) = ret.split_at(9);
    let f = { "123456789".as_bytes() == test };
    if f {
        return Ok(deciphertext.to_vec());
    }

    println!("{}, teste = {:?} \n{:?}", f, test, "123456789".as_bytes());
    return Err(Error);
}
