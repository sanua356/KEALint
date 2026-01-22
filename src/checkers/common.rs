use tabled::{
    Table, Tabled,
    settings::{Modify, Style, Width, object::Columns},
};

#[derive(Tabled)]
#[tabled(display(Option, "tabled::derive::display::option", ""))]
pub struct Problem {
    pub name: String,
    pub config_type: String,
    pub importance: String,
    pub description: String,
    pub snapshot: Option<String>,
    pub links: Option<String>,
}

pub fn tabled_print_problems(problems: Vec<Problem>) {
    let mut table = Table::new(problems);
    table.with(Style::modern());
    table.with(Modify::new(Columns::one(0)).with(Width::wrap(20)));
    table.with(Modify::new(Columns::one(3)).with(Width::wrap(20)));
    table.with(Modify::new(Columns::one(4)).with(Width::wrap(20)));
    table.with(Modify::new(Columns::one(5)).with(Width::wrap(20)));
    println!("{}", table);

    println!("{} problem(s) found.", &table.count_rows() - 1);
}
