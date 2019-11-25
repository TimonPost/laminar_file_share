use crate::server::{ClientEntry, Server};
use crossterm::{cursor, execute, screen::RawScreen};
use cursive::{
    traits::{Boxable, Identifiable, Scrollable, With},
    views::{Dialog, EditView, LinearLayout, ListView, SelectView, TextView},
    Cursive,
};
use std::{
    collections::HashMap,
    io::{stdout, Write},
    net::SocketAddr,
    thread,
    time::Duration,
};
mod server;

fn main() {
    let _screen = RawScreen::into_raw_mode();
    execute!(stdout(), cursor::Hide);

    let mut server = Server::new();

    let mut cursive = setup_cursive();

    loop {
        server.poll().unwrap();
        step_ui(&mut cursive, &server.clients);
        thread::sleep(Duration::from_millis(10));
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
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::crossterm().unwrap();

    //    let mut table = view::table_view();
    //    table.set_items(vec![ClientEntry::new("127.0.0.1:12355".parse().unwrap(), 10000)]);

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
        });

    let sc = list_view.with_id("table").scrollable();

    let dialog = Dialog::new()
        .title("Connections")
        .button("Ok", |s| s.quit())
        .content(sc);

    siv.add_layer(dialog);

    siv
}
