use snow::{
    Builder,
};

fn get_inc_key(start: u8) -> [u8; 32] {
    let mut k = [0u8; 32];
    for i in 0..32 {
        k[i] = start + i as u8;
    }
    k
}

fn create_keypair () {
    let params: NoiseParams = "Noise_XXpsk3_25519_ChaChaPoly_SHA256".parse().unwrap();
    return Builder::new(params.clone())
            .local_private_key(&get_inc_key(0))
            .build_initiator()
            .unwrap();
}


fn step_1 (key) {
    // XX(s, rs):
    // -> e
    let len = key.write_message(&[], &mut buf).unwrap();
    //let _ = h_r.read_message(&buf[..len], &mut buf2).unwrap();
}

fn step_2 (key) {
    // <- e, ee s, es
    let len = key.write_message(&[], &mut buf).unwrap();
    //let _ = h_i.read_message(&buf[..len], &mut buf2).unwrap();
}

fn step_3 (key, psk: [u8; 32]) {
    key.set_psk(3, &psk).unwrap();
}

fn test_set_psk() {
    let mut h_r =
        Builder::new(params).local_private_key(&get_inc_key(1)).build_responder().unwrap();

    let mut buf = [0u8; 1024];
    let mut buf2 = [0u8; 1024];

    let psk = get_inc_key(3);


    // -> s, se, psk
    let len = h_i.write_message(&[], &mut buf).unwrap();
    let _ = h_r.read_message(&buf[..len], &mut buf2).unwrap();
}
