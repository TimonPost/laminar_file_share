use crate::{
    server::ClientEntry,
    table_view::{TableView, TableViewItem},
};
use cursive::align::HAlign;
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum BasicColumn {
    Address,
    Uploaded,
}

impl BasicColumn {
    fn as_str(&self) -> &str {
        match *self {
            BasicColumn::Address => "Address",
            BasicColumn::Uploaded => "Uploaded",
        }
    }
}

impl TableViewItem<BasicColumn> for ClientEntry {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Address => self.endpoint.to_string(),
            BasicColumn::Uploaded => ((100.0 / self.total_bytes as f32) as f32
                * self.received_bytes.len() as f32)
                .to_string(),
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        Ordering::Equal
    }
}

pub fn table_view() -> TableView<ClientEntry, BasicColumn> {
    TableView::<ClientEntry, BasicColumn>::new()
        .column(BasicColumn::Address, "Client Addr", |c| c.width_percent(50))
        .column(BasicColumn::Uploaded, "Uploaded", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(50)
        })
}
