use calamine::{open_workbook, Reader, Xlsx};
use cpython::{
    py_fn, py_module_initializer, PyDict, PyErr, PyList, PyResult, Python, PythonObject, ToPyObject,
};

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
    let pydict = PyDict::new(py);
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        let excel_headers = PyList::new(py, &[]);
        for headers in r.rows().take(1) {
            for header in 0..headers.len() {
                //println!("{} {} ", header, headers[header]);
                // let value = PyString::new(py,  (&headers[header])).to_py_object(py);
                excel_headers.insert(
                    py,
                    header,
                    headers[header].to_string().to_py_object(py).into_object(),
                )
            }
        }
        let mut i = 0;
        for row in r.rows().skip(1) {
            let excel_values = PyList::new(py, &[]);
            let kv = PyDict::new(py);
            for field in 0..row.len() {
                kv.set_item(
                    py,
                    excel_headers.get_item(py, field),
                    row[field].to_string().to_py_object(py).into_object(),
                );
            }
            excel_values.insert(py, i, kv.to_py_object(py).into_object());
            pydict.set_item(py, i, excel_values);
            // println!("increment {} {:?}", i, row[1]);
            i = i + 1;
        }
        Ok(pydict)
    } else {
        // let err = PyErr::new(py, "Reference Error");
        let dict = PyDict::new(py);

        Ok(dict)
    }
}
