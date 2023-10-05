use rsct::client::Client;
use std::collections::HashMap;
use std::sync::Arc;

fn hash_client(client: &Client) -> String {
    format!("{}{}", client.ip_string(), client.port_string())
}

const DEFAULT_CLIENT_TTL: i8 = 8;

pub struct ClientStore {
    pub client: Arc<Client>,
    ttl: i8,
}

pub struct ClientManager {
    pub clients: HashMap<String, ClientStore>,
}

impl ClientManager {
    pub fn new() -> Self {
        ClientManager {
            clients: HashMap::<String, ClientStore>::new(),
        }
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.insert(
            hash_client(&client),
            ClientStore {
                client: Arc::new(client),
                ttl: DEFAULT_CLIENT_TTL,
            },
        );
    }

    pub fn update_client(&mut self, client: Client) {
        if let Some(cli) = self.clients.get_mut(&hash_client(&client)) {
            cli.ttl = DEFAULT_CLIENT_TTL;
        }
    }
}
