use std::time::{SystemTime, UNIX_EPOCH};

use azalea_buf::{McBuf, McBufWritable};
use rsa::{
    signature::{RandomizedSigner, SignatureEncoding},
    RsaPrivateKey,
};
use sha2::Sha256;
use uuid::Uuid;

#[derive(Debug, Clone, McBuf)]
pub struct SaltSignaturePair {
    pub salt: u64,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, McBuf)]
pub struct MessageSignature {
    pub bytes: [u8; 256],
}

#[derive(Clone, Debug, McBuf, PartialEq)]
pub struct SignedMessageHeader {
    pub previous_signature: Option<MessageSignature>,
    pub sender: Uuid,
}

/// Generates a random u64 to use as a salt
pub fn make_salt() -> u64 {
    rand::random()
}

pub struct SignChatMessageOptions {
    pub account_uuid: Uuid,
    pub chat_session_uuid: Uuid,

    pub message_index: u32,

    /// Can be acquired with [`make_salt`].
    pub salt: u64,

    /// The current time that we're sending the message at.
    pub timestamp: SystemTime,

    /// The message that we're sending in chat.
    pub message: String,

    pub private_key: RsaPrivateKey,
}

pub fn sign_chat_message(opts: &SignChatMessageOptions) -> MessageSignature {
    let mut data_to_sign = Vec::new();
    // always 1 for some reason
    1i32.write_into(&mut data_to_sign).unwrap();
    // player uuid
    opts.account_uuid.write_into(&mut data_to_sign).unwrap();
    // chat session uuid
    opts.chat_session_uuid
        .write_into(&mut data_to_sign)
        .unwrap();
    // message index
    opts.message_index.write_into(&mut data_to_sign).unwrap();
    // salt
    opts.salt.write_into(&mut data_to_sign).unwrap();

    // timestamp as seconds
    let seconds_since_epoch = opts
        .timestamp
        .duration_since(UNIX_EPOCH)
        .expect("timestamp must be after epoch")
        .as_secs();
    seconds_since_epoch.write_into(&mut data_to_sign).unwrap();

    // message length as u32
    let message_len: u32 = opts.message.len().try_into().unwrap();
    message_len.write_into(&mut data_to_sign).unwrap();
    // message bytes
    data_to_sign.extend_from_slice(opts.message.as_bytes());

    // last seen messages length
    0i32.write_into(&mut data_to_sign).unwrap();
    // signatures of last seen messages
    // ... not implemented yet

    // hash with sha256 and then sign with rsa

    // let mut hasher = Sha256::new();
    // hasher.update(data_to_sign.as_slice());
    // let hash = hasher.finalize();

    // println!("hash: {:?}", hash);

    let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new(opts.private_key.clone());
    let mut rng = rand::thread_rng();
    let signature = signing_key
        .sign_with_rng(&mut rng, &data_to_sign)
        .to_bytes();

    println!("signature: {:?}", signature);

    MessageSignature {
        bytes: signature
            .as_ref()
            .try_into()
            .expect("signature must be 256 bytes"),
    }
}
