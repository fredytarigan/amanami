use comfy_table::{Cell, Table};

pub struct OutputTable {
    header: Vec<Cell>,
    rows: Vec<Vec<Cell>>,
}

impl OutputTable {
    pub fn new(header: Vec<Cell>, rows: Vec<Vec<Cell>>) -> OutputTable {
        Self { header, rows }
    }

    pub fn display_output(&self) {
        let mut table = Table::new();

        table.set_header(self.header.clone());

        for item in &self.rows {
            table.add_row(item.clone());
        }

        println!("{}", table);
    }
}
