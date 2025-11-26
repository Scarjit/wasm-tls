use crate::virtual_socket::VirtualSocket;
use rustls::{pki_types::TrustAnchor, ClientConfig, RootCertStore};
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use std::{
    io::{BufReader, Read, Write},
    sync::{Arc, Mutex},
};
use wasm_bindgen::prelude::*;
struct Client {
    client: rustls::ClientConnection,
    socket: VirtualSocket,
    available_bytes: usize,
}

// Load ca-cert.pem at compile time
static CA_CERT: &[u8] = include_bytes!("./ca-cert.pem");

// Define a global mutable variable for the client
static CLIENT: Mutex<Option<Client>> = Mutex::new(None);

#[wasm_bindgen]
pub fn create_client(remote_host: &str) {
    let mut client = CLIENT.lock().unwrap();
    if client.is_some() {
        return;
    }

    let mut ca_cert_reader = BufReader::new(CA_CERT);
    let der = rustls_pemfile::certs(&mut ca_cert_reader)
        .filter_map(|result| result.ok())
        .next()
        .unwrap();

    let mut root_cert_store = RootCertStore::empty();
    root_cert_store.add(der);

    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    let server_name = remote_host.to_string().try_into().unwrap();
    let client_connection = rustls::ClientConnection::new(Arc::new(config), server_name);
    if let Ok(v) = client_connection {
        *client = Some(Client {
            client: v,
            socket: VirtualSocket::new(),
            available_bytes: 0,
        });
    } else {
        crate::log("[create_client] Failed to create client");
    }
}

#[wasm_bindgen]
pub fn send_data(data: &[u8]) -> Result<(), JsValue> {
    let mut client_lock = CLIENT.lock().unwrap();
    let client = client_lock
        .as_mut()
        .ok_or_else(|| JsValue::from_str("[send_data] Client not initialized"))?;

    // Add data to the socket's read buffer (to be read by TLS client)
    client.socket.add_data(data);
    Ok(())
}

#[wasm_bindgen]
pub fn get_data() -> Result<Vec<u8>, JsValue> {
    let mut client_lock = CLIENT.lock().unwrap();
    let client = client_lock
        .as_mut()
        .ok_or_else(|| JsValue::from_str("[get_data] Client not initialized"))?;

    // Get data from the socket's write buffer (written by TLS client)
    Ok(client.socket.get_written_data())
}

#[wasm_bindgen]
pub fn process_tls() -> Result<(), JsValue> {
    let mut client_lock = CLIENT.lock().unwrap();
    let client = client_lock
        .as_mut()
        .ok_or_else(|| JsValue::from_str("[process_tls] Client not initialized"))?;

    // Process data based on the example provided
    if client.client.wants_read() && client.socket.has_data_to_read() {
        client
            .client
            .read_tls(&mut client.socket)
            .map_err(|e| JsValue::from_str(&format!("[process_tls] Read error: {}", e)))?;

        let state = client
            .client
            .process_new_packets()
            .map_err(|e| JsValue::from_str(&format!("[process_tls] Process error: {}", e)))?;
        client.available_bytes = state.plaintext_bytes_to_read();
    }

    if client.client.wants_write() {
        client
            .client
            .write_tls(&mut client.socket)
            .map_err(|e| JsValue::from_str(&format!("[process_tls] Write error: {}", e)))?;
    }

    Ok(())
}

#[wasm_bindgen]
pub fn write_request(request: &str) -> Result<(), JsValue> {
    let mut client_lock = CLIENT.lock().unwrap();
    let client = client_lock
        .as_mut()
        .ok_or_else(|| JsValue::from_str("[write_request] Client not initialized"))?;

    let mut writer = client.client.writer();
    writer
        .write_all(request.as_bytes())
        .map_err(|e| JsValue::from_str(&format!("[write_request] Write error: {}", e)))?;

    Ok(())
}

#[wasm_bindgen]
pub fn read_response() -> Result<String, JsValue> {
    let mut client_lock = CLIENT.lock().unwrap();
    let client = client_lock
        .as_mut()
        .ok_or_else(|| JsValue::from_str("[read_response] Client not initialized"))?;

    let mut plaintext = Vec::with_capacity(client.available_bytes);
    let reader = client.client.reader();
    let n = reader
        .take(client.available_bytes as u64)
        .read_to_end(&mut plaintext)
        .map_err(|e| JsValue::from_str(&format!("[read_response] Read error: {}", e)))?;

    // If n != client.available_bytes, then we have a problem
    if n != client.available_bytes {
        return Err(JsValue::from_str(
            "[read_response] Read error: n != client.available_bytes",
        ));
    }

    // Reset available bytes after reading
    client.available_bytes = 0;

    Ok(String::from_utf8_lossy(&plaintext).to_string())
}
