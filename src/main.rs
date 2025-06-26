#[derive(Debug)]
struct GroupSchedule {
    group_name: String,
    details: Vec<GroupDetail>,
}
#[derive(Debug)]
struct GroupDetail {
    class: String,
    time: String,
}
struct Credentials {
    url: String,
    user: String,
    password: String,
}

impl GroupSchedule {
    fn string_from_cell(cell: &docx_rs::TableCell) -> String {
        cell.children
            .iter()
            .filter_map(|c| match c {
                docx_rs::TableCellContent::Paragraph(p) => Some(p),
                _ => None,
            })
            .map(|p| format!("{}\n", p.raw_text()))
            .collect()
    }
    fn schedule_from_vec(v: Vec<Vec<String>>) -> Vec<GroupSchedule> {
        todo!()
    }
    fn vec_from_table(
        table: Box<docx_rs::Table>,
    ) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let mut converted_table: Vec<Vec<String>> = Vec::new();
        let row_count = table.rows.len();

        //checks
        if (row_count % 2 == 1) & (row_count < 4) {
            return Err(format!("row count {row_count} incompatible!").into());
        };
        // TODO: use table.rows.split_at(row_count / 2)! The table is formed like a union
        let rows: Vec<Vec<docx_rs::TableRowChild>> = table
            .rows
            .iter()
            .filter_map(|row| match row {
                docx_rs::TableChild::TableRow(r) => Some(r),
            })
            .map(|r| r.cells.clone())
            .collect();
        for row in rows {
            let cells: Vec<&docx_rs::TableCell> = row
                .iter()
                .filter_map(|cell| match cell {
                    docx_rs::TableRowChild::TableCell(c) => Some(c),
                })
                .collect();
            let converted_row: Vec<String> = cells
                .iter()
                .map(|c| GroupSchedule::string_from_cell(c))
                .collect();
            converted_table.push(converted_row);
        }
        match converted_table.len() {
            0 => return Err("Coundn't form a vec from table!".into()),
            _ => return Ok(converted_table),
        }
    }
    fn vec_from_docx(docx: docx_rs::Docx) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let mut table: Option<Box<docx_rs::Table>> = None;

        //searching through the docx to find docx_rs::Table
        //idk who made these files to have 2 tables
        for c in docx.document.children {
            if let docx_rs::DocumentChild::Table(t) = c {
                const MINIMAL_ROWS: usize = 4;
                if t.rows.len() <= MINIMAL_ROWS {
                    continue;
                }
                table = Some(t);
                break;
            }
        }

        //converting docx_rs::Table to Vec<cargo::GroupSchedule>
        if let Some(table) = table {
            return GroupSchedule::vec_from_table(table);
        }
        Err("Problem converting docx to Vec<Vec<String>>!".into())
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
fn list_directories(credentials: &Credentials) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    todo!()
}
fn list_docx_files(
    credentials: &Credentials,
    directory: &String,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
