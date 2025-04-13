use crate::model::transfer_log::TransferLog;
use crate::ui::strings::GetName;
use crate::ui::views::transfer_view::TransferView;
use cursive::view::ViewWrapper;
use cursive::views::Dialog;
use cursive_table_view::{TableView, TableViewItem};
use device_controller::peripheral::transfer::Transfer;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

pub struct TransferLogView {
    inner: TableType,
    transfers: Arc<Mutex<Vec<Transfer>>>,
}

impl TransferLogView {
    pub fn new() -> Self {
        let transfers = Arc::new(Mutex::new(Vec::new()));
        let dialog_transfers = transfers.clone();
        Self {
            inner: TableView::new()
                .column(LogColumns::Index, "Seq", |c| c)
                .column(LogColumns::Type, "Type", |c| c)
                .column(LogColumns::Name, "Name", |c| c)
                .on_submit(move |c, r, _| {
                    let t = dialog_transfers.lock().unwrap();
                    c.add_layer(Dialog::around(TransferView::new(&t[r])).button("OK", |c| {
                        c.pop_layer();
                    }))
                }),
            transfers,
        }
    }

    pub fn set_items_from_log(&mut self, log: TransferLog) {
        self.inner.set_items(
            log.get_transfers()
                .iter()
                .map(raw_transfer_to_table_row)
                .collect(),
        );

        let mut t = self.transfers.lock().unwrap();
        *t = log.get_transfers().iter().map(|i| i.1.clone()).collect();
    }
}

impl ViewWrapper for TransferLogView {
    cursive::wrap_impl!(self.inner: TableType);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogColumns {
    Index,
    Type,
    Name,
}

#[derive(Clone)]
pub struct LogColumnData {
    index: u64,
    transfer_type: &'static str,
    name: &'static str,
}

impl TableViewItem<LogColumns> for LogColumnData {
    fn to_column(&self, column: LogColumns) -> String {
        match column {
            LogColumns::Index => self.index.to_string(),
            LogColumns::Type => self.transfer_type.to_string(),
            LogColumns::Name => self.name.to_string(),
        }
    }

    fn cmp(&self, other: &Self, column: LogColumns) -> Ordering
    where
        Self: Sized,
    {
        match column {
            LogColumns::Index => self.index.cmp(&other.index),
            LogColumns::Type => self.transfer_type.cmp(other.transfer_type),
            LogColumns::Name => self.name.cmp(other.name),
        }
    }
}

pub type TableType = TableView<LogColumnData, LogColumns>;

fn raw_transfer_to_table_row(t: &(u64, Transfer)) -> LogColumnData {
    match &t.1 {
        Transfer::Notification(n) => LogColumnData {
            index: t.0,
            transfer_type: "N",
            name: n.get_name(),
        },
        Transfer::Command(c) => LogColumnData {
            index: t.0,
            transfer_type: "C",
            name: c.get_name(),
        },
    }
}
