extern crate ramp;

use ramp::Int;
use std::mem::replace;
use std::{thread, time};

use pyo3::prelude::*;

pub fn fib(n: usize) -> Int {
    let mut f0: Int = Int::from(0);
    let mut f1: Int = Int::from(1);
    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_0() {
        assert_eq!(Int::from(0), fib(0));
    }

    #[test]
    fn fibonacci_1() {
        assert_eq!(Int::from(55), fib(10));
    }
}

#[pyclass]
struct MyType {
    #[pyo3(get, set)] // no need to specify getter/setter
    number: usize
}

#[pymethods]
impl MyType {

    #[new]
    fn new(obj: &PyRawObject, number: usize) {
        obj.init({ MyType { number } });
    }

    fn get_fibonacci(&mut self) -> PyResult<String> {
        let res: Int = fib(self.number);
        Ok(res.to_string())
    }

    fn get_my_struct(&mut self) -> Py<MyStruct> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Py::new(py, MyStruct { number1: 1, number2: 2 }).unwrap()
    }

    fn wait(&mut self, seconds: u64) -> PyResult<u64> {
        let waiting_time = time::Duration::from_secs(seconds);
        thread::sleep(waiting_time);
        
        Ok(seconds*2)
    }

    fn wait_mt(&mut self, seconds: u64, py: Python) -> PyResult<u64> {
        let res = py.allow_threads(move || {
            let waiting_time = time::Duration::from_secs(seconds);
            thread::sleep(waiting_time);
            seconds
        });
        Ok(res*2)
    }
}

#[pyclass]
struct MyStruct {
    #[pyo3(get, set)] // no need to specify getter/setter
    number1: usize,

    #[pyo3(get, set)] // no need to specify getter/setter
    number2: usize
}

#[pymodule]
fn myrustlib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyType>()?;
    m.add_class::<MyStruct>()?;
    Ok(())
}