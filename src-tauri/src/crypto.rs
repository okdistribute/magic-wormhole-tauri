use snow::{
    Builder,
};


fn create_noise (psk: &[u8]) {
    let params: NoiseParams = "Noise_XXpsk3_25519_ChaChaPoly_SHA256".parse().unwrap();
    let builder = Builder::new(params.clone());
    let static_key = builder.generate.keypair().unwrap();
    let mut noise = builder.local_private_key(&static_key.private).psk(3, psk).build_responder().unwrap();
    return noise;
}

