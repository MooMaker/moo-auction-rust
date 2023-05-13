use bincode;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    // ... Code to populate the 'clients' HashMap ...

    save_clients_to_file(&clients, "clients.bin");

    println!("Clients saved to file.");
}

fn save_clients_to_file(clients: &Clients, file_path: &str) {
    // Serialize the 'clients' HashMap to binary
    let encoded =
        bincode::serialize(&*clients.lock().unwrap()).expect("Failed to serialize to binary");

    // Open the file in write mode
    let mut file = File::create(file_path).expect("Failed to create file");

    // Write the encoded data to the file
    file.write_all(&encoded).expect("Failed to write to file");
}
