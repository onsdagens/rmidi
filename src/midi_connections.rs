use core_foundation::runloop::CFRunLoop;
use coremidi::{
    Client, Destination, Destinations, Endpoint, EventList, InputPort, Notification, Object,
    OutputPort, Protocol, Source, Sources,
};

pub struct MidiConnections {
    pub opt_client: Option<Client>,
    pub in_ports: Vec<InputPort>,
    pub out_ports: Vec<OutputPort>,
}

impl MidiConnections {
    /// Create MidiConnections without a client. Client will be set later.
    pub fn new() -> Self {
        Self {
            opt_client: None,
            in_ports: Vec::new(),
            out_ports: Vec::new(),
        }
    }

    /// We rebuild in and out ports on every notification for simplicity.
    /// Later, we can optimize this to only add/remove ports as needed.
    pub fn update_connections(&mut self) {
        if let Some(client) = &self.opt_client {
            let in_ports: Vec<InputPort> = Sources
                .into_iter()
                .map(|s| {
                    client
                        .input_port("input", |data| println!("Received MIDI data: {:?}", data))
                        .unwrap()
                })
                .collect();
            in_ports
                .iter()
                .zip(Sources.into_iter())
                .for_each(|(port, source)| {
                    port.connect_source(&source).unwrap();
                    println!("Connected to source: {}", source.display_name().unwrap());
                });
            self.in_ports = in_ports;

            let out_ports: Vec<OutputPort> = Destinations
                .into_iter()
                .map(|s| client.output_port("output").unwrap())
                .collect();
            self.out_ports = out_ports;
        }
    }
}
