use std::{thread, time::Duration};

use cursive::{
    traits::Identifiable,
    views::{Dialog, LinearLayout, Panel},
    Cursive, Vec2,
};
use file_common::{file_bytes, AcknowledgementBoardView, Options};

use crate::client::Client;

mod client;

fn main() {
    let bytes = file_bytes().unwrap();

    let mut client = Client::new(bytes, 1200);

    let mut cursive = setup_cursive(client.number_of_chunks());

    loop {
        client.tick().unwrap();
        step_ui(&mut cursive, &client);
        thread::sleep(Duration::from_millis(20));
    }
}

/// Sets up an cursive environment.
fn setup_cursive(number_of_chunks: usize) -> Cursive {
    let x = 20.;
    let y = (number_of_chunks as f64 / x).ceil();

    let table = AcknowledgementBoardView::new(Options {
        size: Vec2::new(x as usize, y as usize),
        acknowledged_fields: number_of_chunks,
    });

    let mut cursive = Cursive::crossterm().unwrap();

    cursive.add_layer(
        Dialog::new()
            .title("File client")
            .content(LinearLayout::horizontal().child(Panel::new(table.with_id("table"))))
            .button("Quit game", |s| {
                s.pop_layer();
            }),
    );

    cursive
}

/// Refreshes the UI.
fn step_ui(cursive: &mut Cursive, client: &Client) {
    if let Some(ref mut table_view) = cursive.find_id::<AcknowledgementBoardView>("table") {
        for x in client.sent_packets() {
            table_view.mark_sent(*x);
        }

        for x in client.acked_packets() {
            table_view.acknowledge(*x);
        }
    }

    cursive.step();
    cursive.refresh();
}
