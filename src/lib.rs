use calamine::{open_workbook, Reader, Xlsx};
use cpython::{py_fn, py_module_initializer, PyDict, PyResult, Python};

// add bindings to the generated python module
// N.B: names: "rust2py" must be the name of the `.so` or `.pyd` file
py_module_initializer!(rust2py, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(
        py,
        "excel_reader",
        py_fn!(py, excel_reader_py(path: String)),
    )?;
    Ok(())
});

//TODO: GET PATH
fn excel_reader_py(py: Python, path: String) -> PyResult<PyDict> {
    println!("{}", path);
    let mut excel: Xlsx<_> = open_workbook(path).unwrap();
    let mut pydict = PyDict::new(py);
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            pydict.set_item(py, key: K, value: V);
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    }
    pydict
}
