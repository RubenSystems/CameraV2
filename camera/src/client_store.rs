use rsct::client::Client;
use std::collections::BTreeMap;

fn hash_client(client: &Client) -> String {
    format!("{}{}", client.ip_string(), client.port_string())
}

const DEFAULT_CLIENT_TTL: u8 = 8;

struct ClientStore {
    client: Client,
    ttl: u8,
}

pub struct ClientManager {
    clients: BTreeMap<String, ClientStore>,
}

impl ClientManager {
    pub fn new() -> Self {
        ClientManager {
            clients: BTreeMap::<String, ClientStore>::new(),
        }
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.insert(
            hash_client(&client),
            ClientStore {
                client,
                ttl: DEFAULT_CLIENT_TTL,
            },
        );
    }

    pub fn update_client(&mut self, client: Client) {
        if let Some(cli) = self.clients.get_mut(&hash_client(&client)) {
            cli.ttl = DEFAULT_CLIENT_TTL;
        }
    }

    pub fn broadcast_iter(&mut self) -> impl Iterator<Item = (&String, &Client)> {
        self.clients.iter_mut().map(|(key, value)| {
            value.ttl -= 1;
            (key, &value.client)
        })
    }
}
