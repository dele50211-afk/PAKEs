fn mix_bytes(input: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (index, byte) in input.iter().enumerate() {
        let slot = index % out.len();
        out[slot] = out[slot].wrapping_add(*byte).wrapping_mul(31u8).wrapping_add((index % 17) as u8);
    }

    let mut final_out = [0u8; 32];
    for i in 0..32 {
        final_out[i] = out[i].wrapping_add((i as u8).wrapping_mul(17));
    }

    final_out
}

pub fn derive_shared_secret(
    password: &[u8],
    salt: &[u8],
    local_ephemeral: &[u8],
    peer_ephemeral: &[u8],
) -> [u8; 32] {
    let local_digest = mix_bytes(local_ephemeral);
    let peer_digest = mix_bytes(peer_ephemeral);

    let mut out = [0u8; 32];
    for i in 0..32 {
        let password_byte = password[i % password.len()];
        let salt_byte = salt[i % salt.len()];
        let combined = local_digest[i].wrapping_add(peer_digest[i]).wrapping_add(password_byte).wrapping_add(salt_byte);
        out[i] = combined.wrapping_mul(31u8).wrapping_add((i as u8).wrapping_mul(17));
    }

    out
}

pub fn parse_request(request: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>), String> {
    let parts: Vec<&str> = request.trim().split('|').collect();
    if parts.len() != 4 {
        return Err("expected 4 pipe-delimited fields: password|salt|local_ephemeral|peer_ephemeral".to_string());
    }

    Ok((
        parts[0].as_bytes().to_vec(),
        parts[1].as_bytes().to_vec(),
        parts[2].as_bytes().to_vec(),
        parts[3].as_bytes().to_vec(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_secret_is_consistent_for_both_sides() {
        let password = b"correct horse battery staple";
        let salt = b"demo-salt";
        let client_ephemeral = b"client-ephemeral";
        let server_ephemeral = b"server-ephemeral";

        let client_secret = derive_shared_secret(password, salt, client_ephemeral, server_ephemeral);
        let server_secret = derive_shared_secret(password, salt, server_ephemeral, client_ephemeral);

        assert_eq!(client_secret, server_secret);
    }

    #[test]
    fn parse_request_accepts_pipe_delimited_fields() {
        let parsed = parse_request("secret|salt|client|server").unwrap();
        assert_eq!(parsed.0, b"secret".to_vec());
        assert_eq!(parsed.1, b"salt".to_vec());
        assert_eq!(parsed.2, b"client".to_vec());
        assert_eq!(parsed.3, b"server".to_vec());
    }
}
