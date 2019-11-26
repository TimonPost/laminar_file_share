use std::{collections::HashMap, net::SocketAddr};

use cursive::{
    traits::{Identifiable, Scrollable, With},
    views::{Dialog, EditView, ListView, SelectView},
    Cursive,
};

use crate::server::{ClientEntry, Server};

mod server;

fn main() {
    let mut server = Server::new();

    let mut cursive = setup_cursive();

    loop {
        server.tick().unwrap();
        step_ui(&mut cursive, &server.clients);
    }
}

fn step_ui(cursive: &mut Cursive, clients: &HashMap<SocketAddr, ClientEntry>) {
    cursive.step();
    cursive.refresh();

    if let Some(ref mut list) = cursive.find_id::<ListView>("table") {
        list.clear();

        for client in clients {
            list.add_child(
                &format!(
                    "{} | {}%",
                    client.0,
                    ((100.0 / client.1.total_bytes as f32) as f32
                        * client.1.received_bytes.len() as f32)
                ),
                EditView::new(),
            );
        }
    }
}

fn setup_cursive() -> Cursive {
    let mut siv = Cursive::crossterm().unwrap();

    let list_view = ListView::new()
        .child(
            "Connection",
            // Popup-mode SelectView are small enough to fit here
            SelectView::new().popup().item_str("0-18"),
        )
        .with(|list| {
            // We can also add children procedurally
            for i in 0..50 {
                list.add_child(&format!("{}", i), EditView::new());
            }
        })
        .with_id("table")
        .scrollable();

    siv.add_layer(
        Dialog::new()
            .title("Connections")
            .button("Ok", |s| s.quit())
            .content(list_view),
    );

    siv
}
