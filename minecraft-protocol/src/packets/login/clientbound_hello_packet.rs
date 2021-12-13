use async_trait::async_trait;
use std::hash::Hash;
use tokio::io::BufReader;

use crate::{
    mc_buf,
    packets::{Packet, PacketTrait},
};

#[derive(Hash, Clone, Debug)]
pub struct ClientboundHelloPacket {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub nonce: Vec<u8>,
}

#[async_trait]
impl PacketTrait for ClientboundHelloPacket {
    fn get(self) -> Packet {
        Packet::ClientboundHelloPacket(self)
    }
    fn write(&self, _buf: &mut Vec<u8>) {
        panic!("ClientboundHelloPacket::write not implemented")
    }

    async fn read<T: tokio::io::AsyncRead + std::marker::Unpin + std::marker::Send>(
        buf: &mut BufReader<T>,
    ) -> Result<Packet, String> {
        let server_id = mc_buf::read_utf_with_len(buf, 20).await?;
        let public_key = mc_buf::read_byte_array(buf).await?;
        let nonce = mc_buf::read_byte_array(buf).await?;

        Ok(ClientboundHelloPacket {
            server_id,
            public_key,
            nonce,
        }
        .get())
    }
}
