use crate::crypto::{encrypt, decrypt};
use anyhow::Result;
use bytes::{Buf, BufMut, BytesMut};
use serde::{de::DeserializeOwned, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct EncryptedFrameCodec {
    key: [u8; 32],
    read_buf: BytesMut,
    write_buf: BytesMut,
}

impl EncryptedFrameCodec {
    pub fn new(key: [u8; 32]) -> Self {
        Self {
            key,
            read_buf: BytesMut::with_capacity(8192),
            write_buf: BytesMut::with_capacity(8192),
        }
    }

    pub async fn write_message<T: Serialize, W: AsyncWriteExt + Unpin>(
        &mut self,
        writer: &mut W,
        msg: &T,
    ) -> Result<()> {
        let plaintext = rmp_serde::to_vec_named(msg)?;
        let mut nonce = [0u8; 24];
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut nonce);

        let ciphertext = encrypt(&self.key, &nonce, &plaintext)?;
        
        // Frame: [4-byte len][24-byte nonce][ciphertext]
        let total_len = (24 + ciphertext.len()) as u32;
        self.write_buf.put_u32(total_len);
        self.write_buf.put_slice(&nonce);
        self.write_buf.put_slice(&ciphertext);

        writer.write_all(&self.write_buf).await?;
        self.write_buf.clear();
        Ok(())
    }

    pub async fn read_message<T: DeserializeOwned, R: AsyncReadExt + Unpin>(
        &mut self,
        reader: &mut R,
    ) -> Result<Option<T>> {
        loop {
            if self.read_buf.len() >= 4 {
                let mut len_bytes = [0u8; 4];
                len_bytes.copy_from_slice(&self.read_buf[..4]);
                let total_len = u32::from_be_bytes(len_bytes) as usize;

                if self.read_buf.len() >= 4 + total_len {
                    self.read_buf.advance(4);
                    let mut nonce = [0u8; 24];
                    nonce.copy_from_slice(&self.read_buf[..24]);
                    self.read_buf.advance(24);
                    
                    let ciphertext_len = total_len - 24;
                    let ciphertext = &self.read_buf[..ciphertext_len];
                    
                    let plaintext = decrypt(&self.key, &nonce, ciphertext)?;
                    let msg = rmp_serde::from_slice(&plaintext)?;
                    
                    self.read_buf.advance(ciphertext_len);
                    return Ok(Some(msg));
                }
            }

            if reader.read_buf(&mut self.read_buf).await? == 0 {
                if self.read_buf.is_empty() {
                    return Ok(None);
                } else {
                    return Err(anyhow::anyhow!("Connection closed unexpectedly"));
                }
            }
        }
    }
}
