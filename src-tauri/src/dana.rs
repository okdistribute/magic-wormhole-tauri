use async_std::{
    io::{prelude::WriteExt, ReadExt},
    net::{TcpListener, TcpStream},
};
use std::{fs, sync::mpsc};
use magic_wormhole::{WormholeWelcome, WormholeConnector, CodeProvider};
use log::*;

pub struct Code {
    pub welcome: WormholeWelcome,
    pub connector: WormholeConnector
}

impl Code {
    pub async fn new (code: Option<String>) -> Code {
        let code_provider = match code {
            Some(code) => CodeProvider::SetCode(code),
            None => CodeProvider::AllocateCode(2)
        };
        let (welcome, connector) = magic_wormhole::connect_to_server(
            magic_wormhole::transfer::APPID,
            magic_wormhole::transfer::AppVersion::default(),
            magic_wormhole::DEFAULT_MAILBOX_SERVER,
            code_provider,
        )
        .await
        .unwrap();
        return Code { welcome, connector };
    }

    pub fn get_code (&self) -> &String {
        return &self.welcome.code.0;
    }

    pub async fn connect (self) -> magic_wormhole::Wormhole {
        return self.connector.connect_to_client().await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::dana;

    #[async_std::test]
    async fn redeem_petnames () -> std::io::Result<()> {
        let code_a = dana::Code::new(None).await;
        let outgoing = code_a.get_code().to_owned();

        let code_b = dana::Code::new(Some(outgoing)).await;
        
        let hole1 = code_a.connect().await;
        let hole2 = code_b.connect().await;

        assert_eq!(&hole1.key.0, &hole2.key.0);
        Ok(())
    }
}

