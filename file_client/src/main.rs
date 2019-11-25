use crate::client::Client;
use std::time::Duration;
use std::thread;
use std::io::{stdout, Write};
use crossterm::{
    Output,
    cursor, queue, execute,
    style::{Colorize, PrintStyledContent},
    terminal,
    terminal::ClearType,
};
use cursive::{traits::{Boxable, Identifiable}};
use crossterm::screen::RawScreen;
use cursive::{Cursive, Vec2};
use crate::board_view::{AcknowledgementBoardView, Options};
use cursive::views::{Dialog, LinearLayout, Panel};
use file_common::file_bytes;
use std::collections::VecDeque;

mod board_view;
mod client;

fn main() {
    execute!(stdout(), cursor::Hide);

    let bytes = file_bytes()
        .unwrap();

    let mut client = Client::new(bytes, 1200);

    let mut cursive = setup_cursive(client.packet_chunks.len());

    loop {
        client.poll().unwrap();
        step_ui(&mut cursive, &client.acked_packets);
        thread::sleep(Duration::from_millis(20));
    }
}

fn setup_cursive(number_of_chunks: usize) -> Cursive  {
    let x = 20.;
    let y = (number_of_chunks as f64 / x).ceil();

    let mut table = AcknowledgementBoardView::new(Options {size: Vec2::new(x as usize, y as usize), acknowledge_cells: number_of_chunks});

    let mut cursive = Cursive::crossterm().unwrap();

    cursive.add_layer(
        Dialog::new()
            .title("File client")
            .content(
                LinearLayout::horizontal()
                    .child(Panel::new(table.with_id("table"))),
            )
            .button("Quit game", |s| {
                s.pop_layer();
            }),
    );

    cursive
}

fn step_ui(cursive: &mut Cursive, acked_packets: &Vec<usize>) {
    if let Some(ref mut table_view) = cursive.find_id::<AcknowledgementBoardView>("table")
    {
        for x in acked_packets {
            table_view.acknowledge(*x);
        }
    }
    
    cursive.step();
    cursive.refresh();
}

