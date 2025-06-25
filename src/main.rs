#[derive(Debug)]
struct GroupSchedule {
    group_name: String,
    details: Vec<GroupDetail>,
}
#[derive(Debug)]
struct GroupDetail {
    class: Vec<String>,
    time: String,
}
struct Credentials {
    url: String,
    user: String,
    password: String,
}

impl GroupSchedule {
    fn vec_from_docx(
        docx: docx_rs::Docx,
    ) -> Result<Vec<GroupSchedule>, Box<dyn std::error::Error>> {
        let mut table: Option<Box<docx_rs::Table>> = None;

        //searching through the docx to find docx_rs::Table
        for c in docx.document.children {
            if let docx_rs::DocumentChild::Table(t) = c {
                const MINIMAL_ROWS: usize = 2; //idk who made these files to have 2 tables
                if t.rows.len() <= MINIMAL_ROWS {
                    continue;
                }
                table = Some(t);
                break;
            }
        }

        //converting docx_rs::Table to Vec<cargo::GroupSchedule>
        let mut groups: Vec<GroupSchedule>;

        if let Some(table) = table {
            let row_count = table.rows.len();

            //checks
            if row_count % 2 == 1 {
                return Err(format!("row count {row_count} incompatible!").into());
            };
            let mut reformatted_table: Vec<Vec<String>> = Vec::new();
            for (i, row) in table.rows.iter().enumerate() {}

            //convert docx_rs::Table to a more sane Vec<Vec<>>
            //row
            for row in table.rows {
                let docx_rs::TableChild::TableRow(row) = row;
                //cell
                for cell in row.cells {
                    print!("\t[");
                    let mut cell_text = String::new();
                    let docx_rs::TableRowChild::TableCell(cell) = cell;
                    //cell data
                    for (i, child) in cell.children.iter().enumerate() {
                        let cell_subtext = match child {
                            docx_rs::TableCellContent::Paragraph(text) => text.raw_text(),
                            _ => continue,
                        };
                        if cell_text.is_empty() {
                            cell_text = cell_subtext;
                        } else {
                            cell_text = format!("{}, {}", cell_text, cell_subtext);
                        }
                    }
                    print!("{}]", cell_text);
                }
                println!();
            }
        }
        Err("Group not formed!".into())
    }
    fn get_group_by_name(
        groups: Vec<GroupSchedule>,
        group_name: &String,
    ) -> Result<GroupSchedule, ()> {
        for group in groups {
            if group.group_name == *group_name {
                return Ok(group);
            }
        }
        Err(())
    }
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
fn main() {}
#[test]
fn vec_from_docx_is_err_test() {
    let buf = std::fs::read("schedule.docx").unwrap();
    let docx = docx_rs::read_docx(&buf).unwrap();

    let groups = GroupSchedule::vec_from_docx(docx);
    println!("{:#?}", groups);
    assert!(groups.is_ok());
}
