use std::path::PathBuf;

use rust2py;

#[test]
fn test_excel_reader() {
    let xlxs = PathBuf::from("sample.xlxs");

    print!("{:?}", xlxs);
}
