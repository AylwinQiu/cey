use std::collections::HashMap;
use core::cmp::PartialEq;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
pub type CeyInt = isize;
pub type CeyFloat = f64;
pub type CeyBool = bool;
pub type CeyString = Vec<char>;
trait CeyUserDataTrait{
    // TODO
    fn hash(self: &Self) -> Vec<usize>;
}
pub enum CeyType {
    Int(CeyInt),
    Float(CeyFloat),
    Bool(CeyBool), 
    String(CeyString),
    Table(CeyTable),
    Userdata(Box<dyn CeyUserDataTrait>),
}
impl PartialEq for CeyType {
    fn eq(self: &Self, other: &Self) -> bool {
        return match self {
            CeyType::Int(x) => {
                if let CeyType::Int(y) = other {x==y} else {false}
            },
            CeyType::Float(x) => {
                if let CeyType::Float(y) = other {x==y} else {false}
            },
            CeyType::Bool(x) => {
                if let CeyType::Bool(y) = other {x==y} else {false}
            },
            CeyType::String(x) => {
                if let CeyType::String(y) = other {x==y} else {false}
            },
            CeyType::Table(x) => {
                if let CeyType::Table(y) = other {x==y} else {false}
            },
            CeyType::Userdata(x) => {
                if let CeyType::Userdata(y) = other {
                    // TODO: compare two userdata. x and y.
                    false
                }else{
                    false
                }
            }

        }
    }
}
impl Eq for CeyType {}
impl Hash for CeyType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // TODO
        match self {
            CeyType::Int(x) => {
                (*x).hash(state);
            }
            CeyType::Float(x) => {
                if x.is_nan() {
                    0x7FF8000000000000u64.hash(state);
                }else if *x == 0.0 as CeyFloat{
                    0.0f64.to_bits().hash(state);
                }else{
                    x.to_bits().hash(state);
                }
            }
            CeyType::Bool(x) => {
                (*x).hash(state);
            }
            CeyType::String(x) => {
                (*x).hash(state);
            }
            CeyType::Table(x) => {
                for each in &x.list {
                    each.hash(state);
                }
                for each in &x.hash {
                    each.0.hash(state);
                    each.1.hash(state);
                }
            }
            CeyType::Userdata(x) => {
                x.hash().hash(state);
            }

        }
    }
}

pub struct CeyTable {
    list: Vec<CeyType>,
    hash: HashMap<CeyType, CeyType>,
}
impl CeyTable {
    // TODO
}
impl PartialEq for CeyTable {
    fn eq(self: &Self, other: &Self) ->bool {
        // TODO: compare two cey table.
        if &self.list != &other.list {return false;}
        if &self.hash != &other.hash {return false;}
        return true;
    }
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
}

pub fn compile_from_string(code: CeyString) -> CeyCodeBlock {

}