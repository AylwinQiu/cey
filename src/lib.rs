use std::collections::HashMap;
use std::ffi::c_void;

pub type CeyInt = isize;
pub type CeyFloat = f64;
pub type CeyBool = bool;
pub type CeyString = Vec<char>;
pub type CeyUserData = *mut c_void;


pub enum CeyBasic {
    Int(CeyInt),
    Float(CeyFloat),
    Bool(CeyBool), 
    String(CeyString),
    Table(CeyTable),
    Userdata(CeyUserData),
}

pub struct CeyType{
    data: CeyBasic,
    ref_cont: usize,
}
impl CeyType {
    // TODO
}

pub struct CeyTable {
    list: Vec<CeyType>,
    hash: HashMap<CeyType, CeyType>
    // TODO
}
impl CeyTable {
    // TODO
}

pub struct CeyCodeBlock {
    // TODO
}
impl CeyCodeBlock{
    // TODO
}

pub struct CeyVM {
    var_stack: Vec<CeyTable>,
    // TODO
}
impl CeyVM {
    pub fn new(self: &Self) {
        // This function will return the 
    }
    pub fn compile_from_string(self: &Self, code: CeyString) -> CeyCodeBlock {
        // TODO
    }
}

