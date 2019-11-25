use crate::{
    acknowledgement_board_view::{AcknowledgementBoardView, Options},
    client::Client,
};
use cursive::{
    traits::Identifiable,
    views::{Dialog, LinearLayout, Panel},
    Cursive, Vec2,
};
use file_common::file_bytes;
use std::{thread, time::Duration};

mod client;

fn main() {
    let bytes = file_bytes().unwrap();

    let mut client = Client::new(bytes, 1200);

    let mut cursive = setup_cursive(client.packet_chunks.len());

    loop {
        client.poll().unwrap();
        step_ui(&mut cursive, &client.acked_packets);
        thread::sleep(Duration::from_millis(20));
    }
}

fn setup_cursive(number_of_chunks: usize) -> Cursive {
    let x = 20.;
    let y = (number_of_chunks as f64 / x).ceil();

    let table = AcknowledgementBoardView::new(Options {
        size: Vec2::new(x as usize, y as usize),
        acknowledge_fields: number_of_chunks,
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

fn step_ui(cursive: &mut Cursive, acked_packets: &Vec<usize>) {
    if let Some(ref mut table_view) = cursive.find_id::<AcknowledgementBoardView>("table") {
        for x in acked_packets {
            table_view.acknowledge(*x);
        }
    }

    cursive.step();
    cursive.refresh();
}
