use crate::virtual_socket::VirtualSocket;
use rustls::{ServerConfig, ServerConnection};
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use std::{
    io::{BufReader, Read, Write},
    sync::{Arc, Mutex},
};
use wasm_bindgen::prelude::*;

struct Server {
    server: ServerConnection,
    socket: VirtualSocket,
    available_bytes: usize,
}

// Load cert.pem and key.pem at compile time
static CERT: &[u8] = include_bytes!("./cert.pem");
static KEY: &[u8] = include_bytes!("./key.pem");
static CA_CERT: &[u8] = include_bytes!("./ca-cert.pem");

// Define a global mutable variable for the server
static SERVER: Mutex<Option<Server>> = Mutex::new(None);

#[wasm_bindgen]
pub fn create_server() -> Result<(), JsValue> {
    let mut server = SERVER.lock().unwrap();
    if server.is_some() {
        return Ok(());
    }

    // Parse the certificate and private key
    let mut cert_reader = BufReader::new(CERT);
    let certs = rustls_pemfile::certs(&mut cert_reader)
        .filter_map(|result| result.ok())
        .collect::<Vec<CertificateDer<'static>>>();

    if certs.is_empty() {
        return Err(JsValue::from_str("[create_server] No certificates found"));
    }

    let mut key_reader = BufReader::new(KEY);
    let key = match rustls_pemfile::pkcs8_private_keys(&mut key_reader).next() {
        Some(Ok(key)) => PrivateKeyDer::from(key),
        _ => {
            return Err(JsValue::from_str(
                "[create_server] Failed to parse private key",
            ))
        }
    };

    // Create the server config
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| JsValue::from_str(&format!("[create_server] TLS config error: {}", e)))?;

    // Create the server connection
    let server_conn = ServerConnection::new(Arc::new(config)).map_err(|e| {
        JsValue::from_str(&format!("[create_server] Server connection error: {}", e))
    })?;

    *server = Some(Server {
        server: server_conn,
        socket: VirtualSocket::new(),
        available_bytes: 0,
    });

    Ok(())
}

#[wasm_bindgen]
pub fn server_send_data(data: &[u8]) -> Result<(), JsValue> {
    let mut server_lock = SERVER.lock().unwrap();
    let server = server_lock
        .as_mut()
        .ok_or_else(|| JsValue::from_str("[server_send_data] Server not initialized"))?;

    // Add data to the socket's read buffer (to be read by TLS server)
    server.socket.add_data(data);
    Ok(())
}

#[wasm_bindgen]
pub fn server_get_data() -> Result<Vec<u8>, JsValue> {
    let mut server_lock = SERVER.lock().unwrap();
    let server = server_lock
        .as_mut()
        .ok_or_else(|| JsValue::from_str("[server_get_data] Server not initialized"))?;

    // Get data from the socket's write buffer (written by TLS server)
    Ok(server.socket.get_written_data())
}

#[wasm_bindgen]
pub fn server_process_tls() -> Result<(), JsValue> {
    let mut server_lock = SERVER.lock().unwrap();
    let server = server_lock
        .as_mut()
        .ok_or_else(|| JsValue::from_str("[server_process_tls] Server not initialized"))?;

    // Process data based on TLS state
    if server.server.wants_read() && server.socket.has_data_to_read() {
        server
            .server
            .read_tls(&mut server.socket)
            .map_err(|e| JsValue::from_str(&format!("[server_process_tls] Read error: {}", e)))?;

        let state = server.server.process_new_packets().map_err(|e| {
            JsValue::from_str(&format!("[server_process_tls] Process error: {}", e))
        })?;
        server.available_bytes = state.plaintext_bytes_to_read();
    }

    if server.server.wants_write() {
        server
            .server
            .write_tls(&mut server.socket)
            .map_err(|e| JsValue::from_str(&format!("[server_process_tls] Write error: {}", e)))?;
    }

    Ok(())
}

#[wasm_bindgen]
pub fn server_read_request() -> Result<String, JsValue> {
    let mut server_lock = SERVER.lock().unwrap();
    let server = server_lock
        .as_mut()
        .ok_or_else(|| JsValue::from_str("[server_read_request] Server not initialized"))?;

    let mut plaintext = Vec::with_capacity(server.available_bytes);
    let reader = server.server.reader();
    let n = reader
        .take(server.available_bytes as u64)
        .read_to_end(&mut plaintext)
        .map_err(|e| JsValue::from_str(&format!("[server_read_request] Read error: {}", e)))?;

    // If n != server.available_bytes, then we have a problem
    if n != server.available_bytes {
        return Err(JsValue::from_str(
            "[server_read_request] Read error: n != server.available_bytes",
        ));
    }

    // Reset available bytes after reading
    server.available_bytes = 0;

    Ok(String::from_utf8_lossy(&plaintext).to_string())
}

#[wasm_bindgen]
pub fn server_write_response(response: &str) -> Result<(), JsValue> {
    let mut server_lock = SERVER.lock().unwrap();
    let server = server_lock
        .as_mut()
        .ok_or_else(|| JsValue::from_str("[server_write_response] Server not initialized"))?;

    let mut writer = server.server.writer();
    writer
        .write_all(response.as_bytes())
        .map_err(|e| JsValue::from_str(&format!("[server_write_response] Write error: {}", e)))?;

    Ok(())
}
