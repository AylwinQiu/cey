use std::collections::HashMap;

pub type CeyInt = isize;
pub type CeyFloat = f64;
pub type CeyBool = bool;
pub type CeyString = Vec<char>;
pub enum CeyType {
    Int(CeyInt),
    Float(CeyFloat),
    Bool(CeyBool), 
    String(CeyString),
    Table(CeyTable),
    Userdata()
}
pub struct CeyTable {
    list: Vec<CeyType>,
    hash: HashMap<CeyType, CeyType>
    // TODO
}

pub struct CeyVM {
    // TODO
}

