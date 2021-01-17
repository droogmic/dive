use std::hash::Hash;

use ed25519::signature::{Signer, Verifier};
use ed25519_dalek::PublicKey;
use rand_core::OsRng;
use uuid::Uuid;

pub struct Biometric {
    data: Vec<u64>,
}

pub struct Secret {
    pub data: String,
}

impl Secret {
    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_bytes()
    }
}

//pub struct UserVerifier<V> {
pub struct UserVerifier {
    pub verify_key: ed25519_dalek::PublicKey,
}

//impl<V> UserVerifier<V>
//where
//    V: Verifier<ed25519::Signature>
impl UserVerifier {
    pub fn verify(
        &self,
        secret: &Secret,
        signature: &ed25519::Signature,
    ) -> Result<(), ed25519::Error> {
        self.verify_key.verify(secret.as_bytes(), signature)
    }
}

//pub struct User<S>
//where
//    S: Signer<ed25519::Signature>
pub struct User {
    pub id: Uuid,
    //signing_key: S,
    signing_key: ed25519_dalek::Keypair,
}

//impl<S> User<S>
//where
//    S: Signer<ed25519::Signature>
impl User {
    //pub fn new_dalek() -> User<ed25519_dalek::Keypair> {
    pub fn new_dalek() -> User {
        let id = Uuid::new_v4();
        let signing_key = ed25519_dalek::Keypair::generate(&mut OsRng);
        User { id, signing_key }
    }

    pub fn get_public_key(&self) -> ed25519_dalek::PublicKey {
        self.signing_key.public
    }

    pub fn sign(&self, secret: &Secret) -> ed25519::Signature {
        self.signing_key.sign(secret.as_bytes())
    }
}
