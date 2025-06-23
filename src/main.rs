use rustydav::client;
use serde::{Deserialize, Serialize};
//Everything is going to user, so everything is a String
struct Class /*Занятие*/ {
    name: String,
    teacher: String,
    time: String,
}
struct GroupSchedule {
    name: String,
    classes: Vec<Class>,
}
struct Credentials {
    url: String,
    user: String,
    password: String,
}
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
fn parce_docx(docx_file: Vec<u8>) -> Result<Vec<GroupSchedule>, ()> {
    todo!()
}
fn download_group_schedule(
    credentials: &Credentials,
    directory: &String,
    filename: &String,
    group_name: &String,
) -> Result<GroupSchedule, ()> {
    let docx = download_docx_file(credentials, directory, filename)?;
    let groups = parce_docx(docx)?;
    for group in groups {
        if group.name == *group_name {
            return Ok(group);
        }
    }
    Err(())
}
fn main() {}
