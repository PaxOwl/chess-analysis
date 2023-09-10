use std::string::ToString;

pub fn table_printer<T: ToString>(headers: Vec<String>, data: Vec<Vec<T>>) {
    let outline: String = String::from("─│╭╮╰╯├┼┤┬┴"); 

    let ncols: usize = headers.len();
    let nrows: usize = data.len();

    // Building the table rows
    let mut top_row: String = String::from(outline.chars().nth(2).unwrap());
    let mut mid_row: String = String::from(outline.chars().nth(6).unwrap());
    let mut bot_row: String = String::from(outline.chars().nth(4).unwrap());

    let mut headers_row: String = String::from(outline.chars().nth(1).unwrap());
    let mut content: Vec<String> = Vec::new();
    for col in 0..ncols {
        let length: usize = headers[col].len();
        for _ in 0..(length + 2) {
           top_row.push(outline.chars().nth(0).unwrap());
           mid_row.push(outline.chars().nth(0).unwrap());
           bot_row.push(outline.chars().nth(0).unwrap());
        }
        headers_row.push(' ');
        headers_row += &headers[col];
        headers_row.push(' ');

        if col != ncols - 1 {
            top_row.push(outline.chars().nth(9).unwrap());
            mid_row.push(outline.chars().nth(7).unwrap());
            bot_row.push(outline.chars().nth(10).unwrap());
        }
        headers_row.push(outline.chars().nth(1).unwrap());
    }
    top_row.push(outline.chars().nth(3).unwrap());
    mid_row.push(outline.chars().nth(8).unwrap());
    bot_row.push(outline.chars().nth(5).unwrap());

    for row in 0..nrows {
        let mut line_content: String = String::from(outline.chars().nth(1).unwrap());
        for col in 0..ncols {
            let length: usize = headers[col].len();
            line_content.push(' ');
            line_content += &format!("{:>length$}", data[row][col].to_string());
            line_content.push(' ');
            line_content.push(outline.chars().nth(1).unwrap());
        }
        content.push(line_content.clone());
    }

    println!("rows: {}", nrows);
    println!("cols: {}", ncols);
    println!("{}", top_row);
    println!("{}", headers_row);
    println!("{}", mid_row);
    for line in content {
        println!("{}", line);
    }
    println!("{}", bot_row);
}
