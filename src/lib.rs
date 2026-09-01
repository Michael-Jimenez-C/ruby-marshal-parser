use num_bigint::{BigInt, Sign};
use num_traits::Signed;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyBytes, PyDict, PyInt, PyList, PyString};
use std::collections::HashMap;

#[pyclass(dict)]
#[derive(Clone)]
struct RubySymbol {
    #[pyo3(get)]
    name: String,
    raw: Option<Vec<u8>>,
}

#[pymethods]
impl RubySymbol {
    #[new]
    #[pyo3(signature = (name, raw=None))]
    fn new(name: String, raw: Option<Vec<u8>>) -> Self {
        RubySymbol { name, raw }
    }

    fn __str__(&self) -> String {
        self.name.clone()
    }

    fn __repr__(&self) -> String {
        format!("RubySymbol({:?})", self.name)
    }

    fn __eq__(&self, other: &Bound<'_, PyAny>) -> PyResult<bool> {
        if let Ok(other_sym) = other.downcast::<RubySymbol>() {
            Ok(self.name == other_sym.borrow().name)
        } else if let Ok(s) = other.extract::<String>() {
            Ok(self.name == s)
        } else {
            Ok(false)
        }
    }

    fn __ne__(&self, other: &Bound<'_, PyAny>) -> PyResult<bool> {
        Ok(!self.__eq__(other)?)
    }

    fn __hash__(&self) -> isize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.name.hash(&mut hasher);
        hasher.finish() as isize
    }
}

#[pyclass(dict)]
struct RubyString {
    #[pyo3(get, set)]
    text: String,
    #[pyo3(get, set)]
    raw: Vec<u8>,
    #[pyo3(get, set)]
    encoding: Option<PyObject>,
    #[pyo3(get, set)]
    ivars: Py<PyDict>,
}

#[pymethods]
impl RubyString {
    #[new]
    #[pyo3(signature = (text, raw, encoding=None, ivars=None))]
    fn new(
        py: Python<'_>,
        text: String,
        raw: Vec<u8>,
        encoding: Option<PyObject>,
        ivars: Option<Py<PyDict>>,
    ) -> Self {
        RubyString {
            text,
            raw,
            encoding,
            ivars: ivars.unwrap_or_else(|| PyDict::new(py).unbind()),
        }
    }

    fn __repr__(&self) -> String {
        format!("RubyString({:?})", self.text)
    }
}

#[pyclass(dict)]
struct RubyFloat {
    #[pyo3(get, set)]
    value: f64,
    #[pyo3(get, set)]
    raw: Vec<u8>,
    #[pyo3(get, set)]
    ivars: Py<PyDict>,
}

#[pymethods]
impl RubyFloat {
    #[new]
    #[pyo3(signature = (value, raw, ivars=None))]
    fn new(py: Python<'_>, value: f64, raw: Vec<u8>, ivars: Option<Py<PyDict>>) -> Self {
        RubyFloat {
            value,
            raw,
            ivars: ivars.unwrap_or_else(|| PyDict::new(py).unbind()),
        }
    }

    fn __float__(&self) -> f64 {
        self.value
    }

    fn __repr__(&self) -> String {
        format!("RubyFloat({:?})", self.value)
    }
}

#[pyclass(dict)]
struct RubyObject {
    #[pyo3(get, set)]
    class_name: Option<String>,
    #[pyo3(get, set)]
    attrs: Py<PyDict>,
}

#[pymethods]
impl RubyObject {
    #[new]
    #[pyo3(signature = (class_name=None, attrs=None))]
    fn new(
        py: Python<'_>,
        class_name: Option<String>,
        attrs: Option<Py<PyDict>>,
    ) -> Self {
        RubyObject {
            class_name,
            attrs: attrs.unwrap_or_else(|| PyDict::new(py).unbind()),
        }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let attrs = self.attrs.bind(py);
        Ok(format!(
            "RubyObject({:?}, {:?})",
            self.class_name,
            attrs.repr()?.to_string_lossy()
        ))
    }
}

#[pyclass(dict)]
struct RubyStruct {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    members: Py<PyDict>,
}

#[pymethods]
impl RubyStruct {
    #[new]
    #[pyo3(signature = (name, members=None))]
    fn new(py: Python<'_>, name: String, members: Option<Py<PyDict>>) -> Self {
        RubyStruct {
            name,
            members: members.unwrap_or_else(|| PyDict::new(py).unbind()),
        }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let members = self.members.bind(py);
        Ok(format!(
            "RubyStruct({:?}, {:?})",
            self.name,
            members.repr()?.to_string_lossy()
        ))
    }
}

#[pyclass(dict)]
struct RubyUserClass {
    #[pyo3(get, set)]
    class_name: String,
    #[pyo3(get, set)]
    value: PyObject,
    #[pyo3(get, set)]
    ivars: Py<PyDict>,
}

#[pymethods]
impl RubyUserClass {
    #[new]
    #[pyo3(signature = (class_name, value, ivars=None))]
    fn new(
        py: Python<'_>,
        class_name: String,
        value: PyObject,
        ivars: Option<Py<PyDict>>,
    ) -> Self {
        RubyUserClass {
            class_name,
            value,
            ivars: ivars.unwrap_or_else(|| PyDict::new(py).unbind()),
        }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "RubyUserClass({:?}, {:?})",
            self.class_name,
            self.value.bind(py).repr()?.to_string_lossy()
        ))
    }
}

#[pyclass(dict)]
struct RubyUserDefined {
    #[pyo3(get, set)]
    class_name: String,
    #[pyo3(get, set)]
    data: Vec<u8>,
    #[pyo3(get, set)]
    ivars: Py<PyDict>,
}

#[pymethods]
impl RubyUserDefined {
    #[new]
    #[pyo3(signature = (class_name, data, ivars=None))]
    fn new(
        py: Python<'_>,
        class_name: String,
        data: Vec<u8>,
        ivars: Option<Py<PyDict>>,
    ) -> Self {
        RubyUserDefined {
            class_name,
            data,
            ivars: ivars.unwrap_or_else(|| PyDict::new(py).unbind()),
        }
    }

    fn __repr__(&self) -> String {
        format!("RubyUserDefined({:?}, {:?})", self.class_name, self.data)
    }
}

#[pyclass(dict)]
struct RubyUserMarshal {
    #[pyo3(get, set)]
    class_name: String,
    #[pyo3(get, set)]
    data: PyObject,
    #[pyo3(get, set)]
    ivars: Py<PyDict>,
}

#[pymethods]
impl RubyUserMarshal {
    #[new]
    #[pyo3(signature = (class_name, data, ivars=None))]
    fn new(
        py: Python<'_>,
        class_name: String,
        data: PyObject,
        ivars: Option<Py<PyDict>>,
    ) -> Self {
        RubyUserMarshal {
            class_name,
            data,
            ivars: ivars.unwrap_or_else(|| PyDict::new(py).unbind()),
        }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "RubyUserMarshal({:?}, {:?})",
            self.class_name,
            self.data.bind(py).repr()?.to_string_lossy()
        ))
    }
}

#[pyclass(dict)]
struct RubyData {
    #[pyo3(get, set)]
    class_name: String,
    #[pyo3(get, set)]
    state: PyObject,
    #[pyo3(get, set)]
    ivars: Py<PyDict>,
}

#[pymethods]
impl RubyData {
    #[new]
    #[pyo3(signature = (class_name, state, ivars=None))]
    fn new(
        py: Python<'_>,
        class_name: String,
        state: PyObject,
        ivars: Option<Py<PyDict>>,
    ) -> Self {
        RubyData {
            class_name,
            state,
            ivars: ivars.unwrap_or_else(|| PyDict::new(py).unbind()),
        }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "RubyData({:?}, {:?})",
            self.class_name,
            self.state.bind(py).repr()?.to_string_lossy()
        ))
    }
}

#[pyclass(dict)]
struct RubyExtended {
    #[pyo3(get, set)]
    module: String,
    #[pyo3(get, set)]
    obj: PyObject,
}

#[pymethods]
impl RubyExtended {
    #[new]
    fn new(module: String, obj: PyObject) -> Self {
        RubyExtended { module, obj }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "RubyExtended({:?}, {:?})",
            self.module,
            self.obj.bind(py).repr()?.to_string_lossy()
        ))
    }
}

#[pyclass(dict)]
struct RubyClass {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    type_byte: u8,
}

#[pymethods]
impl RubyClass {
    #[new]
    #[pyo3(signature = (name, type_byte=99))]
    fn new(name: String, type_byte: u8) -> Self {
        RubyClass { name, type_byte }
    }

    fn __repr__(&self) -> String {
        format!("RubyClass({:?})", self.name)
    }
}

#[pyclass(dict)]
struct RubyRegexp {
    #[pyo3(get, set)]
    source: Vec<u8>,
    #[pyo3(get, set)]
    options: u8,
    #[pyo3(get, set)]
    ivars: Py<PyDict>,
}

#[pymethods]
impl RubyRegexp {
    #[new]
    #[pyo3(signature = (source, options, ivars=None))]
    fn new(
        py: Python<'_>,
        source: Vec<u8>,
        options: u8,
        ivars: Option<Py<PyDict>>,
    ) -> Self {
        RubyRegexp {
            source,
            options,
            ivars: ivars.unwrap_or_else(|| PyDict::new(py).unbind()),
        }
    }

    fn __repr__(&self) -> String {
        format!("RubyRegexp({:?}, {})", self.source, self.options)
    }
}

#[pyclass(dict)]
struct RubyList {
    #[pyo3(get, set)]
    items: Vec<PyObject>,
    #[pyo3(get, set)]
    ivars: Py<PyDict>,
}

#[pymethods]
impl RubyList {
    #[new]
    #[pyo3(signature = (items=None, ivars=None))]
    fn new(
        py: Python<'_>,
        items: Option<Vec<PyObject>>,
        ivars: Option<Py<PyDict>>,
    ) -> Self {
        RubyList {
            items: items.unwrap_or_default(),
            ivars: ivars.unwrap_or_else(|| PyDict::new(py).unbind()),
        }
    }

    fn __len__(&self) -> usize {
        self.items.len()
    }

    fn __getitem__(&self, index: isize, py: Python<'_>) -> PyResult<PyObject> {
        let idx = if index < 0 {
            self.items.len() as isize + index
        } else {
            index
        } as usize;
        self.items
            .get(idx)
            .map(|o| o.clone_ref(py))
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("list index out of range"))
    }

    fn __setitem__(&mut self, index: isize, value: PyObject, py: Python<'_>) -> PyResult<()> {
        let idx = if index < 0 {
            self.items.len() as isize + index
        } else {
            index
        } as usize;
        if let Some(slot) = self.items.get_mut(idx) {
            *slot = value.clone_ref(py);
            Ok(())
        } else {
            Err(pyo3::exceptions::PyIndexError::new_err("list assignment index out of range"))
        }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let parts: PyResult<Vec<String>> = self
            .items
            .iter()
            .map(|o| Ok(o.bind(py).repr()?.to_string_lossy().to_string()))
            .collect();
        Ok(format!("[{}]", parts?.join(", ")))
    }
}

#[pyclass(dict)]
struct RubyDict {
    #[pyo3(get, set)]
    items: Vec<(PyObject, PyObject)>,
    #[pyo3(get, set)]
    ivars: Py<PyDict>,
}

#[pymethods]
impl RubyDict {
    #[new]
    #[pyo3(signature = (items=None, ivars=None))]
    fn new(
        py: Python<'_>,
        items: Option<Vec<(PyObject, PyObject)>>,
        ivars: Option<Py<PyDict>>,
    ) -> Self {
        RubyDict {
            items: items.unwrap_or_default(),
            ivars: ivars.unwrap_or_else(|| PyDict::new(py).unbind()),
        }
    }

    fn keys(&self, py: Python<'_>) -> PyResult<Vec<PyObject>> {
        Ok(self.items.iter().map(|(k, _)| k.clone_ref(py)).collect())
    }

    fn __len__(&self) -> usize {
        self.items.len()
    }

    fn __getitem__(&self, key: PyObject, py: Python<'_>) -> PyResult<PyObject> {
        for (k, v) in &self.items {
            if k.bind(py).eq(&key.bind(py))? {
                return Ok(v.clone_ref(py));
            }
        }
        Err(pyo3::exceptions::PyKeyError::new_err("key not found"))
    }

    fn __setitem__(&mut self, key: PyObject, value: PyObject, py: Python<'_>) -> PyResult<()> {
        for (k, v) in self.items.iter_mut() {
            if k.bind(py).eq(&key.bind(py))? {
                *v = value.clone_ref(py);
                return Ok(());
            }
        }
        self.items.push((key.clone_ref(py), value.clone_ref(py)));
        Ok(())
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let parts: PyResult<Vec<String>> = self
            .items
            .iter()
            .map(|(k, v)| {
                Ok(format!(
                    "{}: {}",
                    k.bind(py).repr()?.to_string_lossy(),
                    v.bind(py).repr()?.to_string_lossy()
                ))
            })
            .collect();
        Ok(format!("{{{}}}", parts?.join(", ")))
    }
}

#[pyclass(dict)]
struct RubyDictWithDefault {
    #[pyo3(get, set)]
    items: Vec<(PyObject, PyObject)>,
    #[pyo3(get, set)]
    default: PyObject,
    #[pyo3(get, set)]
    ivars: Py<PyDict>,
}

#[pymethods]
impl RubyDictWithDefault {
    #[new]
    #[pyo3(signature = (items=None, default=None, ivars=None))]
    fn new(
        py: Python<'_>,
        items: Option<Vec<(PyObject, PyObject)>>,
        default: Option<PyObject>,
        ivars: Option<Py<PyDict>>,
    ) -> Self {
        RubyDictWithDefault {
            items: items.unwrap_or_default(),
            default: default.unwrap_or_else(|| py.None()),
            ivars: ivars.unwrap_or_else(|| PyDict::new(py).unbind()),
        }
    }

    fn __getitem__(&self, key: PyObject, py: Python<'_>) -> PyResult<PyObject> {
        for (k, v) in &self.items {
            if k.bind(py).eq(&key.bind(py))? {
                return Ok(v.clone_ref(py));
            }
        }
        Ok(self.default.clone_ref(py))
    }

    fn __setitem__(&mut self, key: PyObject, value: PyObject, py: Python<'_>) -> PyResult<()> {
        for (k, v) in self.items.iter_mut() {
            if k.bind(py).eq(&key.bind(py))? {
                *v = value.clone_ref(py);
                return Ok(());
            }
        }
        self.items.push((key.clone_ref(py), value.clone_ref(py)));
        Ok(())
    }
}

struct MarshalReader<'a> {
    py: Python<'a>,
    data: &'a [u8],
    pos: usize,
    objects: Vec<PyObject>,
    symbols: Vec<Py<RubySymbol>>,
}

impl<'a> MarshalReader<'a> {
    fn new(py: Python<'a>, data: &'a [u8]) -> Self {
        MarshalReader {
            py,
            data,
            pos: 0,
            objects: Vec::new(),
            symbols: Vec::new(),
        }
    }

    fn read_byte(&mut self) -> PyResult<u8> {
        if self.pos >= self.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "unexpected EOF at offset {}",
                self.pos
            )));
        }
        let b = self.data[self.pos];
        self.pos += 1;
        Ok(b)
    }

    fn peek_byte(&mut self) -> PyResult<u8> {
        if self.pos >= self.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "unexpected EOF at offset {}",
                self.pos
            )));
        }
        Ok(self.data[self.pos])
    }

    fn read_bytes(&mut self, n: usize) -> PyResult<&'a [u8]> {
        let end = self.pos + n;
        if end > self.data.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "unexpected EOF reading {} bytes at offset {}",
                n, self.pos
            )));
        }
        let bytes = &self.data[self.pos..end];
        self.pos = end;
        Ok(bytes)
    }

    fn read_long(&mut self) -> PyResult<i64> {
        let b = self.read_byte()? as i64;
        match b {
            0x00 => Ok(0),
            0x01 => Ok(self.read_byte()? as i64),
            0xFF => Ok((self.read_byte()? as i64) - 256),
            0x02 => {
                let bytes = self.read_bytes(2)?;
                Ok(u16::from_le_bytes([bytes[0], bytes[1]]) as i64)
            }
            0xFE => {
                let bytes = self.read_bytes(2)?;
                Ok(u16::from_le_bytes([bytes[0], bytes[1]]) as i64 - 0x10000)
            }
            0x03 => {
                let bytes = self.read_bytes(3)?;
                Ok(((bytes[2] as i64) << 16)
                    | ((bytes[1] as i64) << 8)
                    | (bytes[0] as i64))
            }
            0xFD => {
                let bytes = self.read_bytes(3)?;
                Ok((((bytes[2] as i64) << 16)
                    | ((bytes[1] as i64) << 8)
                    | (bytes[0] as i64))
                    - 0x1000000)
            }
            0x04 => {
                let bytes = self.read_bytes(4)?;
                Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64)
            }
            0xFC => {
                let bytes = self.read_bytes(4)?;
                Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as i64
                    - 0x100000000)
            }
            _ if b > 127 => Ok((b - 256) + 5),
            _ => Ok(b - 5),
        }
    }

    fn read_byte_sequence(&mut self) -> PyResult<&'a [u8]> {
        let len = self.read_long()? as usize;
        self.read_bytes(len)
    }

    fn load(&mut self) -> PyResult<PyObject> {
        let header = self.read_bytes(2)?;
        if header != b"\x04\x08" {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "unsupported Marshal version (expected 4.8)",
            ));
        }
        self.read_object()
    }

    fn read_object(&mut self) -> PyResult<PyObject> {
        let type_byte = self.read_byte()?;
        self.read_object_with_type(type_byte, true)
    }

    fn read_object_with_type(
        &mut self,
        type_byte: u8,
        register: bool,
    ) -> PyResult<PyObject> {
        match type_byte {
            b'T' => Ok(true.to_object(self.py)),
            b'F' => Ok(false.to_object(self.py)),
            b'0' => Ok(self.py.None()),
            b'i' => Ok(self.read_long()?.to_object(self.py)),
            b':' => self.read_symbol_real(),
            b';' => self.read_symbol_link(),
            b'l' => self.read_bignum(register),
            b'f' => self.read_float(register),
            b'"' => self.read_string_object(),
            b'I' => self.read_instance_variables(register),
            b'[' => self.read_array(register),
            b'{' => self.read_hash(register),
            b'}' => self.read_hash_with_default(register),
            b'o' => self.read_object_generic(register),
            b'S' => self.read_struct(register),
            b'c' => self.read_class(register, b'c'),
            b'm' => self.read_class(register, b'm'),
            b'M' => self.read_class(register, b'M'),
            b'C' => self.read_user_class(register),
            b'u' => self.read_user_defined(register),
            b'U' => self.read_user_marshal(register),
            b'd' => self.read_data(register),
            b'e' => self.read_extended(register),
            b'/' => self.read_regexp(register),
            b'@' => self.read_object_link(),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "unknown Marshal type byte 0x{:02x} ({:?}) at offset {}",
                type_byte,
                type_byte as char,
                self.pos - 1
            ))),
        }
    }

    fn register(&mut self, obj: PyObject) -> PyObject {
        self.objects.push(obj.clone_ref(self.py));
        obj
    }

    fn read_symbol(&mut self) -> PyResult<Py<RubySymbol>> {
        let obj = self.read_object()?;
        if let Ok(sym) = obj.bind(self.py).downcast::<RubySymbol>() {
            Ok(sym.clone().unbind())
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(format!(
                "expected symbol but got {:?}",
                obj.bind(self.py).get_type().name()
            )))
        }
    }

    fn read_symbol_real(&mut self) -> PyResult<PyObject> {
        let raw = self.read_byte_sequence()?;
        let name = decode_symbol_bytes(raw);
        let sym = RubySymbol::new(name, Some(raw.to_vec()));
        let py_sym = Py::new(self.py, sym)?;
        self.symbols.push(py_sym.clone_ref(self.py));
        Ok(py_sym.into_py(self.py))
    }

    fn read_symbol_link(&mut self) -> PyResult<PyObject> {
        let index = self.read_long()? as usize;
        if index >= self.symbols.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "symbol link index {} out of range",
                index
            )));
        }
        Ok(self.symbols[index].clone_ref(self.py).into_py(self.py))
    }


    fn read_bignum(&mut self, register: bool) -> PyResult<PyObject> {
        let sign_byte = self.read_byte()?;
        let word_len = self.read_long()? as usize;
        let data = self.read_bytes(word_len * 2)?;
        let mag = BigInt::from_bytes_le(Sign::Plus, data);
        let value = if sign_byte == b'-' {
            -mag
        } else {
            mag
        };
        let obj = value.to_object(self.py);
        if register {
            Ok(self.register(obj))
        } else {
            Ok(obj)
        }
    }

    fn read_float(&mut self, register: bool) -> PyResult<PyObject> {
        let raw = self.read_byte_sequence()?.to_vec();
        let token: Vec<u8> = raw.splitn(2, |&b| b == 0).next().unwrap_or(&[]).to_vec();
        let value = match token.as_slice() {
            b"inf" => f64::INFINITY,
            b"-inf" => f64::NEG_INFINITY,
            b"nan" => f64::NAN,
            _ => {
                let s = String::from_utf8_lossy(&token);
                s.parse::<f64>().map_err(|e| {
                    pyo3::exceptions::PyValueError::new_err(format!(
                        "invalid float representation {:?}: {}",
                        raw, e
                    ))
                })?
            }
        };
        let float = RubyFloat::new(self.py, value, raw, None);
        let obj = Py::new(self.py, float)?.into_py(self.py);
        if register {
            Ok(self.register(obj))
        } else {
            Ok(obj)
        }
    }


    fn read_string_object(&mut self) -> PyResult<PyObject> {
        let raw = self.read_byte_sequence()?.to_vec();
        let obj = self.decode_bytes(&raw, None)?;
        Ok(self.register(obj))
    }

    fn decode_bytes(&mut self, raw: &[u8], enc: Option<Bound<'_, PyAny>>) -> PyResult<PyObject> {
        let (text, encoding) = match enc {
            None => (decode_to_string(raw), None),
            Some(e) if e.is_none() => (decode_to_string(raw), None),
            Some(e) => {
                if e.is_instance_of::<PyBool>() {
                    if e.is_truthy()? {
                        (decode_to_string(raw), Some(true.to_object(self.py)))
                    } else {
                        (latin1_to_string(raw), Some(false.to_object(self.py)))
                    }
                } else if let Ok(n) = e.extract::<i64>() {
                    if n == 0 {
                        (latin1_to_string(raw), Some(0i64.to_object(self.py)))
                    } else {
                        (decode_to_string(raw), Some(n.to_object(self.py)))
                    }
                } else if let Ok(s) = e.extract::<String>() {
                    let name = s.to_lowercase();
                    if name == "ascii-8bit" || name == "binary" {
                        (latin1_to_string(raw), Some(e.to_object(self.py)))
                    } else if name == "utf-8" {
                        (decode_to_string(raw), Some(e.to_object(self.py)))
                    } else if name == "us-ascii" {
                        (String::from_utf8_lossy(raw).to_string(), Some(e.to_object(self.py)))
                    } else {
                        (latin1_to_string(raw), Some(e.to_object(self.py)))
                    }
                } else {
                    (decode_to_string(raw), None)
                }
            }
        };
        let s = RubyString::new(self.py, text, raw.to_vec(), encoding, None);
        Ok(Py::new(self.py, s)?.into_py(self.py))
    }

    fn read_instance_variables(&mut self, register: bool) -> PyResult<PyObject> {
        let type_byte = self.peek_byte()?;
        let obj = if type_byte == b'"' {
            self.read_byte()?;
            let raw = self.read_byte_sequence()?.to_vec();
            let s = RubyString::new(self.py, decode_to_string(&raw), raw, None, None);
            let obj = Py::new(self.py, s)?.into_py(self.py);
            if register {
                self.register(obj.clone_ref(self.py));
            }
            obj
        } else {
            self.read_byte()?;
            self.read_object_with_type(type_byte, register)?
        };
        let ivars = self.read_ivars()?;
        self.apply_ivars(obj, ivars)
    }

    fn sym_name(&self, sym: &Py<RubySymbol>) -> String {
        sym.bind(self.py).borrow().name.clone()
    }

    fn read_ivars(&mut self) -> PyResult<Py<PyDict>> {
        let count = self.read_long()? as usize;
        let dict = PyDict::new(self.py);
        for _ in 0..count {
            let name = self.read_symbol()?;
            let value = self.read_object()?;
            dict.set_item(self.sym_name(&name), value)?;
        }
        Ok(dict.unbind())
    }

    fn apply_ivars(&mut self, obj: PyObject, ivars: Py<PyDict>) -> PyResult<PyObject> {
        let binding = obj.bind(self.py);
        macro_rules! apply_to {
            ($ty:ty, $field:ident) => {
                if let Ok(o) = binding.downcast::<$ty>() {
                    let mut inner = o.borrow_mut();
                    merge_dicts(self.py, &mut inner.$field, &ivars)?;
                    return Ok(obj);
                }
            };
        }

        apply_to!(RubyObject, attrs);
        apply_to!(RubyStruct, members);
        if let Ok(o) = binding.downcast::<RubyString>() {
            let mut inner = o.borrow_mut();
            if let Some(enc) = ivars.bind(self.py).get_item("E")? {
                inner.encoding = Some(enc.to_object(self.py));
            }
            merge_dicts(self.py, &mut inner.ivars, &ivars)?;
            return Ok(obj);
        }
        if let Ok(o) = binding.downcast::<RubyRegexp>() {
            let mut inner = o.borrow_mut();
            if let Some(enc) = ivars.bind(self.py).get_item("E")? {
                if let Ok(s) = enc.extract::<String>() {
                    inner.source = decode_bytes_to_vec(&inner.source, &s);
                }
            }
            merge_dicts(self.py, &mut inner.ivars, &ivars)?;
            return Ok(obj);
        }
        apply_to!(RubyUserClass, ivars);
        apply_to!(RubyUserDefined, ivars);
        apply_to!(RubyUserMarshal, ivars);
        apply_to!(RubyData, ivars);
        apply_to!(RubyFloat, ivars);
        apply_to!(RubyList, ivars);
        apply_to!(RubyDict, ivars);

        if let Ok(list) = binding.downcast::<PyList>() {
            let items: Vec<PyObject> = list.iter().map(|x| x.to_object(self.py)).collect();
            let wrapped = RubyList::new(self.py, Some(items), Some(ivars));
            return Ok(Py::new(self.py, wrapped)?.into_py(self.py));
        }
        if let Ok(dict) = binding.downcast::<PyDict>() {
            let items = dict_to_items(self.py, dict)?;
            let wrapped = RubyDict::new(self.py, Some(items), Some(ivars));
            return Ok(Py::new(self.py, wrapped)?.into_py(self.py));
        }
        let wrapped = RubyObject::new(self.py, None, None);
        let py_wrapped = Py::new(self.py, wrapped)?;
        {
            let mut attrs = py_wrapped.borrow_mut(self.py);
            attrs.attrs = PyDict::new(self.py).unbind();
            merge_dicts(self.py, &mut attrs.attrs, &ivars)?;
            attrs.attrs.bind(self.py).set_item("_value", obj)?;
        }
        Ok(py_wrapped.into_py(self.py))
    }

    fn read_array(&mut self, register: bool) -> PyResult<PyObject> {
        let count = self.read_long()? as usize;
        let list = RubyList::new(self.py, None, None);
        let py_list = Py::new(self.py, list)?;
        if register {
            self.register(py_list.clone_ref(self.py).into_py(self.py));
        }
        {
            let mut inner = py_list.borrow_mut(self.py);
            for _ in 0..count {
                inner.items.push(self.read_object()?);
            }
        }
        Ok(py_list.into_py(self.py))
    }

    fn read_hash(&mut self, register: bool) -> PyResult<PyObject> {
        let count = self.read_long()? as usize;
        let dict = RubyDict::new(self.py, None, None);
        let py_dict = Py::new(self.py, dict)?;
        if register {
            self.register(py_dict.clone_ref(self.py).into_py(self.py));
        }
        {
            let mut inner = py_dict.borrow_mut(self.py);
            for _ in 0..count {
                let key = self.read_object()?;
                let value = self.read_object()?;
                inner.items.push((key, value));
            }
        }
        Ok(py_dict.into_py(self.py))
    }

    fn read_hash_with_default(&mut self, register: bool) -> PyResult<PyObject> {
        let count = self.read_long()? as usize;
        let dict = RubyDictWithDefault::new(self.py, None, None, None);
        let py_dict = Py::new(self.py, dict)?;
        if register {
            self.register(py_dict.clone_ref(self.py).into_py(self.py));
        }
        let default;
        {
            let mut inner = py_dict.borrow_mut(self.py);
            for _ in 0..count {
                let key = self.read_object()?;
                let value = self.read_object()?;
                inner.items.push((key, value));
            }
            default = self.read_object()?;
            inner.default = default;
        }
        Ok(py_dict.into_py(self.py))
    }

    fn read_object_generic(&mut self, register: bool) -> PyResult<PyObject> {
        let class_name = self.read_symbol()?;
        let obj = RubyObject::new(self.py, Some(self.sym_name(&class_name)), None);
        let py_obj = Py::new(self.py, obj)?;
        if register {
            self.register(py_obj.clone_ref(self.py).into_py(self.py));
        }
        let count = self.read_long()? as usize;
        {
            let inner = py_obj.borrow(self.py);
            for _ in 0..count {
                let name = self.read_symbol()?;
                let value = self.read_object()?;
                inner.attrs.bind(self.py).set_item(self.sym_name(&name), value)?;
            }
        }
        Ok(py_obj.into_py(self.py))
    }

    fn read_struct(&mut self, register: bool) -> PyResult<PyObject> {
        let name = self.read_symbol()?;
        let obj = RubyStruct::new(self.py, self.sym_name(&name), None);
        let py_obj = Py::new(self.py, obj)?;
        if register {
            self.register(py_obj.clone_ref(self.py).into_py(self.py));
        }
        let count = self.read_long()? as usize;
        {
            let inner = py_obj.borrow(self.py);
            for _ in 0..count {
                let member = self.read_symbol()?;
                let value = self.read_object()?;
                inner.members.bind(self.py).set_item(self.sym_name(&member), value)?;
            }
        }
        Ok(py_obj.into_py(self.py))
    }

    fn read_class(&mut self, register: bool, type_byte: u8) -> PyResult<PyObject> {
        let raw = self.read_byte_sequence()?;
        let name = String::from_utf8_lossy(raw).to_string();
        let obj = RubyClass::new(name, type_byte);
        let py_obj = Py::new(self.py, obj)?;
        if register {
            Ok(self.register(py_obj.into_py(self.py)))
        } else {
            Ok(py_obj.into_py(self.py))
        }
    }

    fn read_user_class(&mut self, _register: bool) -> PyResult<PyObject> {
        let class_name = self.read_symbol()?;
        let value = self.read_object()?;
        self.attach_user_class(value, &self.sym_name(&class_name))
    }

    fn read_user_defined(&mut self, register: bool) -> PyResult<PyObject> {
        let class_name = self.read_symbol()?;
        let data = self.read_byte_sequence()?.to_vec();
        let obj = RubyUserDefined::new(self.py, self.sym_name(&class_name), data, None);
        let py_obj = Py::new(self.py, obj)?;
        if register {
            Ok(self.register(py_obj.into_py(self.py)))
        } else {
            Ok(py_obj.into_py(self.py))
        }
    }

    fn read_user_marshal(&mut self, register: bool) -> PyResult<PyObject> {
        let class_name = self.read_symbol()?;
        let data = self.read_object()?;
        let obj = RubyUserMarshal::new(self.py, self.sym_name(&class_name), data, None);
        let py_obj = Py::new(self.py, obj)?;
        if register {
            Ok(self.register(py_obj.into_py(self.py)))
        } else {
            Ok(py_obj.into_py(self.py))
        }
    }

    fn read_data(&mut self, register: bool) -> PyResult<PyObject> {
        let class_name = self.read_symbol()?;
        let state = self.read_object()?;
        let obj = RubyData::new(self.py, self.sym_name(&class_name), state, None);
        let py_obj = Py::new(self.py, obj)?;
        if register {
            Ok(self.register(py_obj.into_py(self.py)))
        } else {
            Ok(py_obj.into_py(self.py))
        }
    }

    fn read_extended(&mut self, _register: bool) -> PyResult<PyObject> {
        let module = self.read_symbol()?;
        let obj = self.read_object()?;
        self.attach_module(obj, &self.sym_name(&module))
    }

    fn read_regexp(&mut self, register: bool) -> PyResult<PyObject> {
        let source = self.read_byte_sequence()?.to_vec();
        let options = self.read_byte()?;
        let obj = RubyRegexp::new(self.py, source, options, None);
        let py_obj = Py::new(self.py, obj)?;
        if register {
            Ok(self.register(py_obj.into_py(self.py)))
        } else {
            Ok(py_obj.into_py(self.py))
        }
    }

    fn read_object_link(&mut self) -> PyResult<PyObject> {
        let index = self.read_long()? as usize;
        if index == 0 || index > self.objects.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "object link index {} out of range",
                index
            )));
        }
        Ok(self.objects[index - 1].clone_ref(self.py))
    }

    fn attach_module(&mut self, obj: PyObject, module: &str) -> PyResult<PyObject> {
        obj.bind(self.py).setattr("_extended_module", module)?;
        Ok(obj)
    }

    fn attach_user_class(&mut self, obj: PyObject, class_name: &str) -> PyResult<PyObject> {
        obj.bind(self.py).setattr("_user_class", class_name)?;
        Ok(obj)
    }
}

fn decode_to_string(raw: &[u8]) -> String {
    String::from_utf8(raw.to_vec())
        .unwrap_or_else(|_| latin1_to_string(raw))
}

fn latin1_to_string(raw: &[u8]) -> String {
    raw.iter().map(|&b| b as char).collect()
}

fn decode_symbol_bytes(raw: &[u8]) -> String {
    decode_to_string(raw)
}

fn decode_bytes_to_vec(raw: &[u8], _enc: &str) -> Vec<u8> {
    raw.to_vec()
}

fn merge_dicts(
    py: Python<'_>,
    target: &mut Py<PyDict>,
    source: &Py<PyDict>,
) -> PyResult<()> {
    for (k, v) in source.bind(py).iter() {
        target.bind(py).set_item(k, v)?;
    }
    Ok(())
}

fn dict_to_items(py: Python<'_>, dict: &Bound<'_, PyDict>) -> PyResult<Vec<(PyObject, PyObject)>> {
    let mut items = Vec::new();
    for (k, v) in dict.iter() {
        items.push((k.to_object(py), v.to_object(py)));
    }
    Ok(items)
}

struct MarshalWriter<'a> {
    py: Python<'a>,
    data: Vec<u8>,
    symbols: HashMap<String, usize>,
    objects: HashMap<usize, usize>,
}

impl<'a> MarshalWriter<'a> {
    fn new(py: Python<'a>) -> Self {
        MarshalWriter {
            py,
            data: Vec::new(),
            symbols: HashMap::new(),
            objects: HashMap::new(),
        }
    }

    fn dump(&mut self, root: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
        self.data.extend_from_slice(b"\x04\x08");
        self.write_object(root)?;
        Ok(self.data.clone())
    }

    fn write_byte(&mut self, value: u8) {
        self.data.push(value);
    }

    fn write_bytes(&mut self, value: &[u8]) {
        self.data.extend_from_slice(value);
    }

    fn write_long(&mut self, x: i64) -> PyResult<()> {
        if x == 0 {
            self.write_byte(0x00);
        } else if (1..=122).contains(&x) {
            self.write_byte((x + 5) as u8);
        } else if (-123..=-1).contains(&x) {
            self.write_byte(((x - 5) & 0xFF) as u8);
        } else if (123..=255).contains(&x) {
            self.write_byte(0x01);
            self.write_byte(x as u8);
        } else if (-256..=-124).contains(&x) {
            self.write_byte(0xFF);
            self.write_byte(((x + 256) & 0xFF) as u8);
        } else if (256..=65535).contains(&x) {
            self.write_byte(0x02);
            self.write_bytes(&(x as u16).to_le_bytes());
        } else if (-65536..=-257).contains(&x) {
            self.write_byte(0xFE);
            self.write_bytes(&((x + 65536) as u16).to_le_bytes());
        } else if (65536..=16777215).contains(&x) {
            self.write_byte(0x03);
            self.write_bytes(&[
                (x & 0xFF) as u8,
                ((x >> 8) & 0xFF) as u8,
                ((x >> 16) & 0xFF) as u8,
            ]);
        } else if (-16777216..=-65537).contains(&x) {
            self.write_byte(0xFD);
            let y = x + 16777216;
            self.write_bytes(&[
                (y & 0xFF) as u8,
                ((y >> 8) & 0xFF) as u8,
                ((y >> 16) & 0xFF) as u8,
            ]);
        } else if (16777216..=4294967295).contains(&x) {
            self.write_byte(0x04);
            self.write_bytes(&(x as u32).to_le_bytes());
        } else if (-4294967296..=-16777217).contains(&x) {
            self.write_byte(0xFC);
            let y = x + 4294967296i64;
            self.write_bytes(&(y as u32).to_le_bytes());
        } else {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "long value out of encodable range: {}",
                x
            )));
        }
        Ok(())
    }

    fn write_byte_sequence(&mut self, raw: &[u8]) -> PyResult<()> {
        self.write_long(raw.len() as i64)?;
        self.write_bytes(raw);
        Ok(())
    }

    fn write_object(&mut self, obj: &Bound<'_, PyAny>) -> PyResult<()> {
        if self.is_immediate(obj) {
            return self.write_immediate(obj);
        }

        let ptr = obj.as_ptr() as usize;
        if let Some(&index) = self.objects.get(&ptr) {
            self.write_byte(b'@');
            self.write_long(index as i64)?;
            return Ok(());
        }

        let ext = obj.getattr("_extended_module").ok().filter(|o| !o.is_none());
        let uc = obj.getattr("_user_class").ok().filter(|o| !o.is_none());

        if let Some(module) = ext {
            self.write_byte(b'e');
            self.write_symbol(&module)?;
            self.objects.insert(ptr, self.objects.len() + 1);
            return self.write_body(obj);
        }

        if let Some(class_name) = uc {
            self.write_byte(b'C');
            self.write_symbol(&class_name)?;
            self.objects.insert(ptr, self.objects.len() + 1);
            return self.write_body(obj);
        }

        self.objects.insert(ptr, self.objects.len() + 1);
        self.write_body(obj)
    }

    fn is_immediate(&self, obj: &Bound<'_, PyAny>) -> bool {
        obj.is_none()
            || obj.is_instance_of::<PyBool>()
            || obj.is_instance_of::<PyInt>()
            || obj.is_instance_of::<RubySymbol>()
    }

    fn write_immediate(&mut self, obj: &Bound<'_, PyAny>) -> PyResult<()> {
        if obj.is_none() {
            self.write_byte(b'0');
        } else if let Ok(b) = obj.downcast::<PyBool>() {
            self.write_byte(if b.is_truthy()? { b'T' } else { b'F' });
        } else if obj.is_instance_of::<PyInt>() {
            let x = obj.extract::<i64>()?;
            self.write_int(x)?;
        } else if let Ok(sym) = obj.downcast::<RubySymbol>() {
            self.write_symbol_obj(&*sym.borrow())?;
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "unexpected immediate object: {:?}",
                obj.repr()?.to_string_lossy()
            )));
        }
        Ok(())
    }

    fn write_body(&mut self, obj: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(s) = obj.downcast::<RubyString>() {
            self.write_ruby_string(&*s.borrow())
        } else if let Ok(b) = obj.downcast::<PyBytes>() {
            self.write_bytes_string(b.as_bytes())
        } else if let Ok(s) = obj.downcast::<PyString>() {
            self.write_plain_string(s.to_str()?)
        } else if let Ok(f) = obj.downcast::<RubyFloat>() {
            self.write_float(&*f.borrow())
        } else if let Ok(d) = obj.downcast::<RubyDictWithDefault>() {
            self.write_hash_with_default(&*d.borrow())
        } else if let Ok(d) = obj.downcast::<RubyDict>() {
            self.write_hash_dict(&*d.borrow())
        } else if let Ok(d) = obj.downcast::<PyDict>() {
            self.write_hash_py(d)
        } else if let Ok(l) = obj.downcast::<RubyList>() {
            self.write_list(&*l.borrow())
        } else if let Ok(l) = obj.downcast::<PyList>() {
            self.write_list_py(l)
        } else if let Ok(o) = obj.downcast::<RubyObject>() {
            self.write_object_generic(&*o.borrow())
        } else if let Ok(s) = obj.downcast::<RubyStruct>() {
            self.write_struct(&*s.borrow())
        } else if let Ok(o) = obj.downcast::<RubyUserClass>() {
            self.write_user_class(&*o.borrow())
        } else if let Ok(o) = obj.downcast::<RubyUserDefined>() {
            self.write_user_defined(&*o.borrow())
        } else if let Ok(o) = obj.downcast::<RubyUserMarshal>() {
            self.write_user_marshal(&*o.borrow())
        } else if let Ok(o) = obj.downcast::<RubyData>() {
            self.write_data(&*o.borrow())
        } else if let Ok(o) = obj.downcast::<RubyExtended>() {
            self.write_extended(&*o.borrow())
        } else if let Ok(r) = obj.downcast::<RubyRegexp>() {
            self.write_regexp(&*r.borrow())
        } else if let Ok(c) = obj.downcast::<RubyClass>() {
            self.write_class(&*c.borrow())
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "unsupported object for Marshal dump: {:?}",
                obj.get_type().name()
            )))
        }
    }

    fn write_int(&mut self, x: i64) -> PyResult<()> {
        if (-2_147_483_648..=2_147_483_647).contains(&x) {
            self.write_byte(b'i');
            self.write_long(x)?;
        } else {
            self.write_bignum(BigInt::from(x))?;
        }
        Ok(())
    }

    fn write_bignum(&mut self, x: BigInt) -> PyResult<()> {
        let sign = if x.sign() == Sign::Minus { b'-' } else { b'+' };
        let mag = x.abs().to_bytes_le().1;
        let mut bytes = mag;
        if bytes.len() % 2 == 1 {
            bytes.push(0);
        }
        if bytes.is_empty() {
            bytes.extend_from_slice(&[0, 0]);
        }
        self.write_byte(b'l');
        self.write_byte(sign);
        self.write_long((bytes.len() / 2) as i64)?;
        self.write_bytes(&bytes);
        Ok(())
    }

    fn write_float(&mut self, obj: &RubyFloat) -> PyResult<()> {
        let has_ivars = !obj.ivars.bind(self.py).is_empty();
        if has_ivars {
            self.write_byte(b'I');
        }
        self.write_byte(b'f');
        self.write_byte_sequence(&obj.raw)?;
        if has_ivars {
            self.write_ivars(&obj.ivars.bind(self.py))?;
        }
        Ok(())
    }

    fn write_symbol(&mut self, obj: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(sym) = obj.downcast::<RubySymbol>() {
            self.write_symbol_obj(&*sym.borrow())
        } else {
            let name = obj.extract::<String>()?;
            self.write_symbol_by_name(&name)
        }
    }

    fn write_symbol_obj(&mut self, sym: &RubySymbol) -> PyResult<()> {
        self.write_symbol_by_name(&sym.name)
    }

    fn write_symbol_by_name(&mut self, name: &str) -> PyResult<()> {
        if let Some(&index) = self.symbols.get(name) {
            self.write_byte(b';');
            self.write_long(index as i64)?;
            return Ok(());
        }
        let index = self.symbols.len();
        self.symbols.insert(name.to_string(), index);
        self.write_byte(b':');
        self.write_byte_sequence(name.as_bytes())?;
        Ok(())
    }

    fn write_ruby_string(&mut self, obj: &RubyString) -> PyResult<()> {
        let py = self.py;
        let ivars = obj.ivars.bind(py);
        let has_encoding = obj.encoding.is_some();
        let has_ivars = !ivars.is_empty() || has_encoding;

        if has_ivars {
            self.write_byte(b'I');
        }
        self.write_byte(b'"');
        self.write_byte_sequence(&obj.raw)?;

        if has_ivars {
            let merged = PyDict::new(py);
            for (k, v) in ivars.iter() {
                merged.set_item(k, v)?;
            }
            if let Some(ref enc) = obj.encoding {
                merged.set_item("E", enc.bind(py))?;
            }
            self.write_ivars(&merged)?;
        }
        Ok(())
    }

    fn write_bytes_string(&mut self, raw: &[u8]) -> PyResult<()> {
        self.write_byte(b'I');
        self.write_byte(b'"');
        self.write_byte_sequence(raw)?;
        self.write_byte(0x06);
        self.write_symbol_by_name("E")?;
        self.write_byte(b'F');
        Ok(())
    }

    fn write_plain_string(&mut self, text: &str) -> PyResult<()> {
        self.write_byte(b'"');
        self.write_byte_sequence(text.as_bytes())?;
        Ok(())
    }

    fn write_list(&mut self, obj: &RubyList) -> PyResult<()> {
        let has_ivars = !obj.ivars.bind(self.py).is_empty();
        if has_ivars {
            self.write_byte(b'I');
        }
        self.write_byte(b'[');
        self.write_long(obj.items.len() as i64)?;
        for item in &obj.items {
            self.write_object(&item.bind(self.py))?;
        }
        if has_ivars {
            self.write_ivars(&obj.ivars.bind(self.py))?;
        }
        Ok(())
    }

    fn write_list_py(&mut self, obj: &Bound<'_, PyList>) -> PyResult<()> {
        self.write_byte(b'[');
        self.write_long(obj.len() as i64)?;
        for item in obj.iter() {
            self.write_object(&item)?;
        }
        Ok(())
    }

    fn write_hash_dict(&mut self, obj: &RubyDict) -> PyResult<()> {
        let has_ivars = !obj.ivars.bind(self.py).is_empty();
        if has_ivars {
            self.write_byte(b'I');
        }
        self.write_byte(b'{');
        self.write_long(obj.items.len() as i64)?;
        for (k, v) in &obj.items {
            self.write_object(&k.bind(self.py))?;
            self.write_object(&v.bind(self.py))?;
        }
        if has_ivars {
            self.write_ivars(&obj.ivars.bind(self.py))?;
        }
        Ok(())
    }

    fn write_hash_py(&mut self, obj: &Bound<'_, PyDict>) -> PyResult<()> {
        self.write_byte(b'{');
        self.write_long(obj.len() as i64)?;
        for (k, v) in obj.iter() {
            self.write_object(&k)?;
            self.write_object(&v)?;
        }
        Ok(())
    }

    fn write_hash_with_default(&mut self, obj: &RubyDictWithDefault) -> PyResult<()> {
        let has_ivars = !obj.ivars.bind(self.py).is_empty();
        if has_ivars {
            self.write_byte(b'I');
        }
        self.write_byte(b'}');
        self.write_long(obj.items.len() as i64)?;
        for (k, v) in &obj.items {
            self.write_object(&k.bind(self.py))?;
            self.write_object(&v.bind(self.py))?;
        }
        self.write_object(&obj.default.bind(self.py))?;
        if has_ivars {
            self.write_ivars(&obj.ivars.bind(self.py))?;
        }
        Ok(())
    }

    fn write_ivars(&mut self, ivars: &Bound<'_, PyDict>) -> PyResult<()> {
        self.write_long(ivars.len() as i64)?;
        for (k, v) in ivars.iter() {
            self.write_symbol(&k)?;
            self.write_object(&v)?;
        }
        Ok(())
    }

    fn write_object_generic(&mut self, obj: &RubyObject) -> PyResult<()> {
        self.write_byte(b'o');
        self.write_symbol_by_name(obj.class_name.as_deref().unwrap_or(""))?;
        let attrs = obj.attrs.bind(self.py);
        self.write_long(attrs.len() as i64)?;
        for (k, v) in attrs.iter() {
            self.write_symbol(&k)?;
            self.write_object(&v)?;
        }
        Ok(())
    }

    fn write_struct(&mut self, obj: &RubyStruct) -> PyResult<()> {
        self.write_byte(b'S');
        self.write_symbol_by_name(&obj.name)?;
        let members = obj.members.bind(self.py);
        self.write_long(members.len() as i64)?;
        for (k, v) in members.iter() {
            self.write_symbol(&k)?;
            self.write_object(&v)?;
        }
        Ok(())
    }

    fn write_class(&mut self, obj: &RubyClass) -> PyResult<()> {
        self.write_byte(obj.type_byte);
        self.write_byte_sequence(obj.name.as_bytes())?;
        Ok(())
    }

    fn write_user_class(&mut self, obj: &RubyUserClass) -> PyResult<()> {
        let has_ivars = !obj.ivars.bind(self.py).is_empty();
        if has_ivars {
            self.write_byte(b'I');
        }
        self.write_byte(b'C');
        self.write_symbol_by_name(&obj.class_name)?;
        self.write_object(&obj.value.bind(self.py))?;
        if has_ivars {
            self.write_ivars(&obj.ivars.bind(self.py))?;
        }
        Ok(())
    }

    fn write_user_defined(&mut self, obj: &RubyUserDefined) -> PyResult<()> {
        let has_ivars = !obj.ivars.bind(self.py).is_empty();
        if has_ivars {
            self.write_byte(b'I');
        }
        self.write_byte(b'u');
        self.write_symbol_by_name(&obj.class_name)?;
        self.write_byte_sequence(&obj.data)?;
        if has_ivars {
            self.write_ivars(&obj.ivars.bind(self.py))?;
        }
        Ok(())
    }

    fn write_user_marshal(&mut self, obj: &RubyUserMarshal) -> PyResult<()> {
        let has_ivars = !obj.ivars.bind(self.py).is_empty();
        if has_ivars {
            self.write_byte(b'I');
        }
        self.write_byte(b'U');
        self.write_symbol_by_name(&obj.class_name)?;
        self.write_object(&obj.data.bind(self.py))?;
        if has_ivars {
            self.write_ivars(&obj.ivars.bind(self.py))?;
        }
        Ok(())
    }

    fn write_data(&mut self, obj: &RubyData) -> PyResult<()> {
        let has_ivars = !obj.ivars.bind(self.py).is_empty();
        if has_ivars {
            self.write_byte(b'I');
        }
        self.write_byte(b'd');
        self.write_symbol_by_name(&obj.class_name)?;
        self.write_object(&obj.state.bind(self.py))?;
        if has_ivars {
            self.write_ivars(&obj.ivars.bind(self.py))?;
        }
        Ok(())
    }

    fn write_extended(&mut self, obj: &RubyExtended) -> PyResult<()> {
        self.write_byte(b'e');
        self.write_symbol_by_name(&obj.module)?;
        self.write_object(&obj.obj.bind(self.py))
    }

    fn write_regexp(&mut self, obj: &RubyRegexp) -> PyResult<()> {
        let has_ivars = !obj.ivars.bind(self.py).is_empty();
        if has_ivars {
            self.write_byte(b'I');
        }
        self.write_byte(b'/');
        self.write_byte_sequence(&obj.source)?;
        self.write_byte(obj.options);
        if has_ivars {
            self.write_ivars(&obj.ivars.bind(self.py))?;
        }
        Ok(())
    }
}

#[pyfunction]
fn load<'py>(py: Python<'py>, data: &[u8]) -> PyResult<Bound<'py, PyAny>> {
    let mut reader = MarshalReader::new(py, data);
    let obj = reader.load()?;
    Ok(obj.into_bound(py))
}

#[pyfunction]
fn load_file<'py>(py: Python<'py>, path: &str) -> PyResult<Bound<'py, PyAny>> {
    let data = std::fs::read(path)?;
    load(py, &data)
}

#[pyfunction]
fn dump<'py>(py: Python<'py>, obj: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyBytes>> {
    let mut writer = MarshalWriter::new(py);
    let bytes = writer.dump(obj)?;
    Ok(PyBytes::new(py, &bytes))
}

#[pyfunction]
fn dump_file<'py>(py: Python<'py>, obj: &Bound<'py, PyAny>, path: &str) -> PyResult<()> {
    let bytes = dump(py, obj)?;
    std::fs::write(path, bytes.as_bytes())?;
    Ok(())
}

#[pyfunction]
fn load_all<'py>(py: Python<'py>, data: &[u8]) -> PyResult<Vec<Bound<'py, PyAny>>> {
    let mut pos = 0;
    let mut out = Vec::new();
    while pos < data.len() {
        let mut reader = MarshalReader::new(py, &data[pos..]);
        let obj = reader.load()?;
        pos += reader.pos;
        out.push(obj.into_bound(py));
    }
    Ok(out)
}

#[pyfunction]
fn load_file_all<'py>(py: Python<'py>, path: &str) -> PyResult<Vec<Bound<'py, PyAny>>> {
    let data = std::fs::read(path)?;
    load_all(py, &data)
}

#[pyfunction]
fn dump_all<'py>(py: Python<'py>, objects: &Bound<'py, PyList>) -> PyResult<Bound<'py, PyBytes>> {
    let mut out = Vec::new();
    for obj in objects.iter() {
        let mut writer = MarshalWriter::new(py);
        out.extend(writer.dump(&obj)?);
    }
    Ok(PyBytes::new(py, &out))
}

#[pyfunction]
fn dump_file_all<'py>(
    py: Python<'py>,
    objects: &Bound<'py, PyList>,
    path: &str,
) -> PyResult<()> {
    let bytes = dump_all(py, objects)?;
    std::fs::write(path, bytes.as_bytes())?;
    Ok(())
}

#[pymodule]
fn r_marshal(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RubySymbol>()?;
    m.add_class::<RubyString>()?;
    m.add_class::<RubyFloat>()?;
    m.add_class::<RubyObject>()?;
    m.add_class::<RubyStruct>()?;
    m.add_class::<RubyUserClass>()?;
    m.add_class::<RubyUserDefined>()?;
    m.add_class::<RubyUserMarshal>()?;
    m.add_class::<RubyData>()?;
    m.add_class::<RubyExtended>()?;
    m.add_class::<RubyClass>()?;
    m.add_class::<RubyRegexp>()?;
    m.add_class::<RubyList>()?;
    m.add_class::<RubyDict>()?;
    m.add_class::<RubyDictWithDefault>()?;
    m.add_function(wrap_pyfunction!(load, m)?)?;
    m.add_function(wrap_pyfunction!(load_file, m)?)?;
    m.add_function(wrap_pyfunction!(dump, m)?)?;
    m.add_function(wrap_pyfunction!(dump_file, m)?)?;
    m.add_function(wrap_pyfunction!(load_all, m)?)?;
    m.add_function(wrap_pyfunction!(load_file_all, m)?)?;
    m.add_function(wrap_pyfunction!(dump_all, m)?)?;
    m.add_function(wrap_pyfunction!(dump_file_all, m)?)?;
    Ok(())
}
