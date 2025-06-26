#[derive(Debug, Clone)]
struct GroupSchedule {
    group_name: String,
    details: Vec<GroupDetail>,
}
#[derive(Debug, Clone)]
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
    fn schedule_from_vec(v: Vec<Vec<String>>) -> Vec<GroupSchedule> {
        if (v.len() % 2 == 0) | (v.len() <= 2) {
            println!("{}", v.len());
            return Vec::new();
        }
        let mut groups: Vec<GroupSchedule> = Vec::with_capacity(v[0][1..].len());
        for (i, cell) in v[0][1..].iter().enumerate() {
            groups.push(GroupSchedule {
                group_name: cell.to_string(),
                details: Vec::with_capacity(v[1..].len()),
            });
        }
        for (i, row) in v[1..].iter().enumerate() {
            for (j, cell) in row[1..].iter().enumerate() {
                groups[j].details.push(GroupDetail {
                    class: v[0][j + 1].to_string(),
                    time: v[i + 1][0].to_string(),
                })
            }
        }
        println!("{:#?}", groups);
        return groups;
    }
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
    fn vec_from_table(
        table: Box<docx_rs::Table>,
    ) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let row_count = table.rows.len();
        let mut converted_table: Vec<Vec<String>> = Vec::with_capacity(row_count);

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
#[test]
fn schedule_from_vec_test() {
    let buf = std::fs::read("schedule.docx").unwrap();
    let docx = docx_rs::read_docx(&buf).unwrap();

    let groups = GroupSchedule::vec_from_docx(docx);
    assert!(groups.is_ok());
    let groups = groups.unwrap();
    let (left, right) = groups.split_at(groups.len() / 2);
    let (left, right) = (
        GroupSchedule::schedule_from_vec(Vec::from(left)),
        GroupSchedule::schedule_from_vec(Vec::from(right)),
    );
    println!("right: {:#?}\nleft: {:#?}", right, left);
}
