use docx_rs::*;
use rustydav::client;
use serde::{Deserialize, Serialize};
struct GroupSchedule {
    group_name: String,
    data: Vec<Vec<String>>, /*Schedule formatting is inconsistent, don't bother structuring*/
    time: Vec<String>,
}
struct Credentials {
    url: String,
    user: String,
    password: String,
}

// NOTE:list_directories -> user interaction -> list_docx_files -> user interaction ->
// download_docx
// TODO:'()' as Err doen't look right, fix it(?)
fn list_directories(credentials: &Credentials) -> Result<Vec<String>, ()> {
    todo!()
}
fn list_docx_files(credentials: &Credentials, directory: &String) -> Result<Vec<String>, ()> {
    todo!()
}
fn download_docx_file(
    credentials: &Credentials,
    directory: &String,
    filename: &String,
) -> Result<Vec<u8>, ()> {
    todo!()
}
fn parse_docx(docx_file: Vec<u8>) -> Result<Vec<GroupSchedule>, ()> {
    todo!()
}
fn get_group_by_name(groups: Vec<GroupSchedule>, group_name: &String) -> Result<GroupSchedule, ()> {
    for group in groups {
        if group.group_name == *group_name {
            return Ok(group);
        }
    }
    Err(())
}
fn main() {}
#[test]
fn parse_test() {
    let buf = std::fs::read("schedule.docx").unwrap();
    let docx = docx_rs::read_docx(&buf).unwrap();

    // TODO:Reformat. Too nested
    for child in docx.document.children {
        match child{
            docx_rs::DocumentChild::Table(table) /*full table!*/=> {
                println!("Found a table!");
                if table.rows.len() == 1{
                    println!("\tIt's empty!");
                    continue;
                }
                for row in table.rows{
                    match row{
                        TableChild::TableRow(row) => for cell in row.cells{
                            print!("\t[");
                            let mut cell_text = String::new();
                            match cell{
                                TableRowChild::TableCell(cell) => for (i, child) in cell.children.iter().enumerate(){
                                    let cell_subtext = match child{
                                        TableCellContent::Paragraph(text) => text.raw_text(),
                                        _ => continue,
                                    };
                                    if cell_text.is_empty() {
                                        cell_text = cell_subtext;
                                    }else{
                                        cell_text = format!("{}, {}", cell_text, cell_subtext);
                                    }
                                },
                            };
                            print!("{}]", cell_text);
                        }
                    }
                    println!();
                }
            },
            _ => println!("Found not a table!"),
        }
    }
}
