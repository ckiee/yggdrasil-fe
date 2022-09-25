use rustls::client::{ServerCertVerified, ServerCertVerifier};

pub struct CustomServerCertVerifier {}

impl ServerCertVerifier for CustomServerCertVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        eprintln!("verify_server_cert: NOT VERIFYING");
        // dbg!(end_entity, intermediates, server_name, ocsp_response, now);
        Ok(ServerCertVerified::assertion())
    }
}
