use crate::object::*;
use crate::pyport::Py_ssize_t;

#[allow(unused_imports)]
use std::os::raw::{c_char, c_int, c_short, c_uchar, c_void};
#[cfg(not(PyPy))]
use std::ptr::addr_of_mut;

// skipped _Py_CODEUNIT
// skipped _Py_OPCODE
// skipped _Py_OPARG

#[cfg(all(Py_3_8, not(PyPy), not(Py_3_11)))]
opaque_struct!(_PyOpcache);

#[cfg(Py_3_12)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _PyCoCached {
    pub _co_code: *mut PyObject,
    pub _co_varnames: *mut PyObject,
    pub _co_cellvars: *mut PyObject,
    pub _co_freevars: *mut PyObject,
}

#[cfg(all(not(PyPy), not(Py_3_7)))]
opaque_struct!(PyCodeObject);

#[cfg(all(not(PyPy), Py_3_7, not(Py_3_8)))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyCodeObject {
    pub ob_base: PyObject,
    pub co_argcount: c_int,
    pub co_kwonlyargcount: c_int,
    pub co_nlocals: c_int,
    pub co_stacksize: c_int,
    pub co_flags: c_int,
    pub co_firstlineno: c_int,
    pub co_code: *mut PyObject,
    pub co_consts: *mut PyObject,
    pub co_names: *mut PyObject,
    pub co_varnames: *mut PyObject,
    pub co_freevars: *mut PyObject,
    pub co_cellvars: *mut PyObject,
    pub co_cell2arg: *mut Py_ssize_t,
    pub co_filename: *mut PyObject,
    pub co_name: *mut PyObject,
    pub co_lnotab: *mut PyObject,
    pub co_zombieframe: *mut c_void,
    pub co_weakreflist: *mut PyObject,
    pub co_extra: *mut c_void,
}

#[cfg(all(not(PyPy), Py_3_8, not(Py_3_11)))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyCodeObject {
    pub ob_base: PyObject,
    pub co_argcount: c_int,
    pub co_posonlyargcount: c_int,
    pub co_kwonlyargcount: c_int,
    pub co_nlocals: c_int,
    pub co_stacksize: c_int,
    pub co_flags: c_int,
    pub co_firstlineno: c_int,
    pub co_code: *mut PyObject,
    pub co_consts: *mut PyObject,
    pub co_names: *mut PyObject,
    pub co_varnames: *mut PyObject,
    pub co_freevars: *mut PyObject,
    pub co_cellvars: *mut PyObject,
    pub co_cell2arg: *mut Py_ssize_t,
    pub co_filename: *mut PyObject,
    pub co_name: *mut PyObject,
    #[cfg(not(Py_3_10))]
    pub co_lnotab: *mut PyObject,
    #[cfg(Py_3_10)]
    pub co_linetable: *mut PyObject,
    pub co_zombieframe: *mut c_void,
    pub co_weakreflist: *mut PyObject,
    pub co_extra: *mut c_void,
    pub co_opcache_map: *mut c_uchar,
    pub co_opcache: *mut _PyOpcache,
    pub co_opcache_flag: c_int,
    pub co_opcache_size: c_uchar,
}

#[cfg(all(not(PyPy), Py_3_11))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyCodeObject {
    pub ob_base: PyVarObject,
    pub co_consts: *mut PyObject,
    pub co_names: *mut PyObject,
    pub co_exceptiontable: *mut PyObject,
    pub co_flags: c_int,
    #[cfg(not(Py_3_12))]
    pub co_warmup: c_int,
    #[cfg(Py_3_12)]
    pub _co_linearray_entry_size: c_short,
    pub co_argcount: c_int,
    pub co_posonlyargcount: c_int,
    pub co_kwonlyargcount: c_int,
    pub co_stacksize: c_int,
    pub co_firstlineno: c_int,

    pub co_nlocalsplus: c_int,
    #[cfg(Py_3_12)]
    pub co_framesize: c_int,
    pub co_nlocals: c_int,
    pub co_nplaincellvars: c_int,
    pub co_ncellvars: c_int,
    pub co_nfreevars: c_int,

    pub co_localsplusnames: *mut PyObject,
    pub co_localspluskinds: *mut PyObject,
    pub co_filename: *mut PyObject,
    pub co_name: *mut PyObject,
    pub co_qualname: *mut PyObject,
    pub co_linetable: *mut PyObject,
    pub co_weakreflist: *mut PyObject,
    #[cfg(not(Py_3_12))]
    pub _co_code: *mut PyObject,
    #[cfg(Py_3_12)]
    pub _co_cached: *mut _PyCoCached,
    #[cfg(not(Py_3_12))]
    pub _co_linearray: *mut c_char,
    pub _co_firsttraceable: c_int,
    #[cfg(Py_3_12)]
    pub _co_linearray: *mut c_char,
    pub co_extra: *mut c_void,
    pub co_code_adaptive: [c_char; 1],
}

#[cfg(PyPy)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyCodeObject {
    pub ob_base: PyObject,
    pub co_name: *mut PyObject,
    pub co_filename: *mut PyObject,
    pub co_argcount: c_int,
    pub co_flags: c_int,
}

/* Masks for co_flags */
pub const CO_OPTIMIZED: c_int = 0x0001;
pub const CO_NEWLOCALS: c_int = 0x0002;
pub const CO_VARARGS: c_int = 0x0004;
pub const CO_VARKEYWORDS: c_int = 0x0008;
pub const CO_NESTED: c_int = 0x0010;
pub const CO_GENERATOR: c_int = 0x0020;
/* The CO_NOFREE flag is set if there are no free or cell variables.
   This information is redundant, but it allows a single flag test
   to determine whether there is any extra work to be done when the
   call frame it setup.
*/
pub const CO_NOFREE: c_int = 0x0040;
/* The CO_COROUTINE flag is set for coroutine functions (defined with
``async def`` keywords) */
pub const CO_COROUTINE: c_int = 0x0080;
pub const CO_ITERABLE_COROUTINE: c_int = 0x0100;
pub const CO_ASYNC_GENERATOR: c_int = 0x0200;

pub const CO_FUTURE_DIVISION: c_int = 0x2000;
pub const CO_FUTURE_ABSOLUTE_IMPORT: c_int = 0x4000; /* do absolute imports by default */
pub const CO_FUTURE_WITH_STATEMENT: c_int = 0x8000;
pub const CO_FUTURE_PRINT_FUNCTION: c_int = 0x1_0000;
pub const CO_FUTURE_UNICODE_LITERALS: c_int = 0x2_0000;

pub const CO_FUTURE_BARRY_AS_BDFL: c_int = 0x4_0000;
pub const CO_FUTURE_GENERATOR_STOP: c_int = 0x8_0000;
// skipped CO_FUTURE_ANNOTATIONS
// skipped CO_CELL_NOT_AN_ARG

pub const CO_MAXBLOCKS: usize = 20;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyCode_Type: PyTypeObject;
}

#[inline]
#[cfg(not(PyPy))]
pub unsafe fn PyCode_Check(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == addr_of_mut!(PyCode_Type)) as c_int
}

#[inline]
#[cfg(all(not(PyPy), Py_3_10, not(Py_3_11)))]
pub unsafe fn PyCode_GetNumFree(op: *mut PyCodeObject) -> Py_ssize_t {
    crate::PyTuple_GET_SIZE((*op).co_freevars)
}

#[inline]
#[cfg(all(not(Py_3_10), Py_3_11, not(PyPy)))]
pub unsafe fn PyCode_GetNumFree(op: *mut PyCodeObject) -> c_int {
    (*op).co_nfreevars
}

extern "C" {
    #[cfg(PyPy)]
    #[link_name = "PyPyCode_Check"]
    pub fn PyCode_Check(op: *mut PyObject) -> c_int;

    #[cfg(PyPy)]
    #[link_name = "PyPyCode_GetNumFree"]
    pub fn PyCode_GetNumFree(op: *mut PyCodeObject) -> Py_ssize_t;
}

extern "C" {
    #[cfg_attr(PyPy, link_name = "PyPyCode_New")]
    pub fn PyCode_New(
        argcount: c_int,
        kwonlyargcount: c_int,
        nlocals: c_int,
        stacksize: c_int,
        flags: c_int,
        code: *mut PyObject,
        consts: *mut PyObject,
        names: *mut PyObject,
        varnames: *mut PyObject,
        freevars: *mut PyObject,
        cellvars: *mut PyObject,
        filename: *mut PyObject,
        name: *mut PyObject,
        firstlineno: c_int,
        lnotab: *mut PyObject,
    ) -> *mut PyCodeObject;
    #[cfg(Py_3_8)]
    pub fn PyCode_NewWithPosOnlyArgs(
        argcount: c_int,
        posonlyargcount: c_int,
        kwonlyargcount: c_int,
        nlocals: c_int,
        stacksize: c_int,
        flags: c_int,
        code: *mut PyObject,
        consts: *mut PyObject,
        names: *mut PyObject,
        varnames: *mut PyObject,
        freevars: *mut PyObject,
        cellvars: *mut PyObject,
        filename: *mut PyObject,
        name: *mut PyObject,
        firstlineno: c_int,
        lnotab: *mut PyObject,
    ) -> *mut PyCodeObject;
    #[cfg_attr(PyPy, link_name = "PyPyCode_NewEmpty")]
    pub fn PyCode_NewEmpty(
        filename: *const c_char,
        funcname: *const c_char,
        firstlineno: c_int,
    ) -> *mut PyCodeObject;
    pub fn PyCode_Addr2Line(arg1: *mut PyCodeObject, arg2: c_int) -> c_int;
    // skipped PyCodeAddressRange "for internal use only"
    // skipped _PyCode_CheckLineNumber
    // skipped _PyCode_ConstantKey
    pub fn PyCode_Optimize(
        code: *mut PyObject,
        consts: *mut PyObject,
        names: *mut PyObject,
        lnotab: *mut PyObject,
    ) -> *mut PyObject;
    pub fn _PyCode_GetExtra(
        code: *mut PyObject,
        index: Py_ssize_t,
        extra: *const *mut c_void,
    ) -> c_int;
    pub fn _PyCode_SetExtra(code: *mut PyObject, index: Py_ssize_t, extra: *mut c_void) -> c_int;
}
