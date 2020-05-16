extern crate cpython;
use cpython::*;

use std::cell::RefCell;
use std::collections::HashSet;

type Inner = HashSet<u32>;

py_class!(class RustSet |py| {
    data hs: RefCell<Inner>;

    def __new__(_cls) -> PyResult<RustSet> {
        Self::create_instance(py, RefCell::new(Inner::new()))
    }

    def __contains__(&self, v: u32) -> PyResult<bool> {
        Ok(self.hs(py).borrow().contains(&v))
    }

    def add(&self, v: u32) -> PyResult<PyObject> {
        self.borrow_mut(py)?.insert(v);
        Ok(py.None())
    }

    def extend(&self, iterable: &PyObject) -> PyResult<PyObject> {
        let mut hs = self.hs(py).borrow_mut(py)?;
        for vobj in iterable.iter(py)? {
            hs.insert(vobj?.extract::<u32>(py)?);
        }
        Ok(py.None())
    }
});
