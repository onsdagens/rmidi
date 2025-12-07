use std::{
    process::Output,
    sync::{Arc, Mutex},
};

use core_foundation::runloop::CFRunLoop;
use coremidi::{
    Client, Destination, Destinations, Endpoint, EventList, InputPort, Notification, Object,
    OutputPort, Protocol, Source, Sources,
};

use rmidi::midi_connections::*;

fn main() {
    let midi_connections = Arc::new(Mutex::new(MidiConnections::new()));

    let midi_con_callback = midi_connections.clone();
    let client =
        Client::new_with_notifications("Example Client", move |notification: &Notification| {
            println!("notification: {:?}", notification);
            let mut midi_connections = midi_con_callback.lock().unwrap();
            midi_connections.update_connections();
        })
        .unwrap();

    {
        let mut midi_connections = midi_connections.lock().unwrap();
        midi_connections.opt_client = Some(client);
        midi_connections.update_connections();
    }

    CFRunLoop::run_current();

    loop {}
}
