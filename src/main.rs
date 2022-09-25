#![feature(iter_intersperse)]
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
    time::Duration,
};

use anyhow::{Context, Result};
use ed25519_dalek::{Keypair, PublicKey};
use rustls::client::ServerCertVerifier;

use crate::{
    cert_verifier::CustomServerCertVerifier,
    handler::PacketHandler,
    proto::{PacketType, PeerMeta, TreeInfo, PEER_META_BYTE_SIZE},
};
mod cert_verifier;
mod handler;
mod proto;

fn main() -> Result<()> {
    let root_store = rustls::RootCertStore::empty();
    let mut config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    // XXX: Bad bad bad bad.
    config
        .dangerous()
        .set_certificate_verifier(Arc::new(CustomServerCertVerifier {}));

    let rc_config = Arc::new(config);
    let ext_ip = "127.0.0.1";
    let port: u16 = 39575;
    let mut conn = rustls::ClientConnection::new(rc_config, ext_ip.try_into()?)?;
    let mut sock = TcpStream::connect((ext_ip, port)).unwrap(); // TODO: Nodelay if double because of tun
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
    let mut buf = [0u8; PEER_META_BYTE_SIZE];
    let mut csprng = rand::thread_rng();
    let keypair = Keypair::generate(&mut csprng);
    let format_key = |bytes: &[u8]| {
        bytes
            .iter()
            .map(|x| format!("{x:0>2x}"))
            .collect::<String>()
    };

    tls.read_exact(&mut buf).context("failed tls read")?;
    tls.write(&PeerMeta::new_with_key(keypair.public).to_bytes())?;
    let remote_meta = PeerMeta::from_bytes(&buf).context("PeerMeta parse rx")?;
    dbg!(remote_meta);

    let handler = PacketHandler {};

    // Introduction's over, let's start processing packets.
    loop {
        // Intrapacket timeout
        tls.sock
            .set_read_timeout(Some(Duration::from_millis(10000)))?;
        let length = {
            let mut buf = [0u8; 2];
            tls.read_exact(&mut buf)
                .context("packet len rx; no keepalives?")?;
            u16::from_be_bytes(buf)
        };
        // Innerpacket timeout
        tls.sock
            .set_read_timeout(Some(Duration::from_millis(1000)))?;

        assert!(length > 0); // At least one byte for typ
        let typ: PacketType = {
            let mut buf = [0u8; 1];
            tls.read_exact(&mut buf).context("packet typ rx")?;
            buf[0].try_into()?
        };

        let data = {
            let mut buf = Vec::with_capacity(65535);
            buf.resize((length - 1).into(), 0u8);
            tls.read_exact(&mut buf).context("packet data rx")?;
            buf
        };

        match typ {
            PacketType::Tree => PacketHandler::handle_tree_info(
                TreeInfo::from_bytes(&data).context("while decoding TreeInfo")?,
            ),
            PacketType::KeepAlive => {
                // Look away, mom! (TODO: use the fancy types?)
                tls.write(&[1, 0])?;
            }
            _ => eprintln!("don't know how to handle {typ:?}"),
        };
    }

    // Ok(())
}
