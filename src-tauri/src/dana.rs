use std::{fs, sync::mpsc};
use magic_wormhole::{WormholeWelcome, WormholeConnector, CodeProvider};
use log::*;
use std::collections::HashMap;
use anyhow;

#[derive(Debug)]
pub enum DanaError {
    Wormhole { error: anyhow::Error},
}

impl std::error::Error for DanaError {}

impl std::fmt::Display for DanaError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            DanaError::Wormhole { error } => write!(f, "{}" , error),
        }
    }
}

pub struct Peer {
    pub welcome: WormholeWelcome, 
    pub code: String,
    pub connector: WormholeConnector
}

pub async fn redeem_code(code: String) -> Result<Peer, DanaError> {
    let code_provider = CodeProvider::SetCode(code);
    let peer = create_peer(code_provider).await;
    return peer;
}

pub async fn create_code() -> Result<Peer, DanaError> {
    let code_provider = CodeProvider::AllocateCode(2);
    let result= create_peer(code_provider).await;
    return result;
} 

async fn create_peer (code_provider: CodeProvider) -> Result<Peer, DanaError> {
    let result = magic_wormhole::connect_to_server(
        magic_wormhole::transfer::APPID,
        magic_wormhole::transfer::AppVersion::default(),
        magic_wormhole::DEFAULT_MAILBOX_SERVER,
        code_provider,
    )
    .await;

    match result {
        Ok((welcome, connector)) => {
            let code = welcome.code.0.clone();
            Ok(Peer { 
                welcome, 
                connector, 
                code
            })
        }
        Err(error) => Err(DanaError::Wormhole { error })
    }
}


#[cfg(test)]
mod tests {
    use crate::dana::*;

    #[async_std::test]
    async fn redeem_petnames () -> std::io::Result<()> {
        let peer_a: Peer = create_code().await.unwrap();
        let share_out_of_band = peer_a.welcome.code.0.to_owned();

        let peer_b: Peer = redeem_code(share_out_of_band).await.unwrap();
        
        let hole1 = connect(peer_a).await.unwrap();
        let hole2 = connect(peer_b).await.unwrap();

        assert_eq!(&hole1.key.0, &hole2.key.0);
        Ok(())
    }
}

