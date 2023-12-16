extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
///
pub fn time_day_and_add_row(item: TokenStream) -> TokenStream {
    let day_num = format!("{item:0>2}");
    let import = format!(r##"use crate::solutions::day{day_num};"##);
    let time = format!(r##"let (d1, d2) = day{day_num}::Day{day_num}.time_both();"##);
    let row_la = format!(r##"table.add_row(row!["{day_num}","##);
    let row_lb = r#"format!("{d1:?}"), format!("{d2:?}")]);"#;
    let print = format!(r##"println!("done with day {day_num}");"##);

    format!("{import}\n{time}\n{row_la}{row_lb}\n{print}")
        .parse()
        .unwrap()
}
