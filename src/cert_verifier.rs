use rustls::client::{ServerCertVerified, ServerCertVerifier};

pub struct CustomServerCertVerifier {}

impl ServerCertVerifier for CustomServerCertVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &rustls::Certificate,
        intermediates: &[rustls::Certificate],
        server_name: &rustls::ServerName,
        scts: &mut dyn Iterator<Item = &[u8]>,
        ocsp_response: &[u8],
        now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        eprintln!("verify_server_cert: NOT VERIFYING");
        // dbg!(end_entity, intermediates, server_name, ocsp_response, now);
        Ok(ServerCertVerified::assertion())
    }
}
