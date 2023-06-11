use serde::{Serialize, Deserialize};
use std::{ops::{BitAnd, Add, Sub, Mul, Div, Rem, BitOr, Shl, Shr, BitXor}, hash::{Hash, Hasher}};


#[derive(Debug,Clone,Copy,Serialize,Deserialize)]
pub enum Word{
    U64(usize),
    I64(isize),
    F64(f64),
    CHAR(char),
    BOOL(bool),
}
impl Word {
    pub fn init() -> Self{
        return Word::U64(0x0);
    }
    pub fn is_zero(self) -> bool {
        match self {
            Word::U64(w) => w == 0,
            Word::I64(w) => w == 0,
            Word::F64(w) => w == 0.0,
            Word::CHAR(w) => w as u8 == 0,
            Word::BOOL(w) => !w,
        }
    }

    pub fn as_usize(self) -> usize {
        match self {
            Word::U64(w) => w ,
            Word::I64(w) => w as usize,
            Word::F64(w) => w as usize,
            Word::CHAR(w) => (w as u8) as usize,
            Word::BOOL(w) => w as usize,
        }
    }
    pub fn and(&self, other: &Self) -> Self {
        return Word::BOOL(!self.is_zero() && !other.is_zero()); 
    }
    pub fn or(&self, other: &Self) -> Self {
        return Word::BOOL(!self.is_zero() || !other.is_zero()); 
    }
    pub fn neg(&self) -> Self {
        return Word::BOOL(self.is_zero()); 
    }
}
impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::U64(l0), Self::U64(r0)) => *l0 == *r0,
            (Self::I64(l0), Self::I64(r0)) => *l0 == *r0,
            (Self::F64(l0), Self::F64(r0)) => *l0 == *r0,
            (Self::CHAR(l0), Self::CHAR(r0)) => *l0 == *r0,
            (Self::BOOL(l0), Self::BOOL(r0)) => *l0 == *r0,
            _ => false,
        }
    }
}
impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::U64(l0), Self::U64(r0)) => l0.partial_cmp(r0),
            (Self::I64(l0), Self::I64(r0)) => l0.partial_cmp(r0),
            (Self::F64(l0), Self::F64(r0)) => l0.partial_cmp(r0),
            (Self::CHAR(l0), Self::CHAR(r0)) => l0.partial_cmp(r0),
            (Self::BOOL(l0), Self::BOOL(r0)) => l0.partial_cmp(r0),
            _ => None,
        }
    }
}

impl Eq for Word{}

impl Hash for Word {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Word::U64(w) => {
                state.write_u8(1);
                w.hash(state);
            }
            Word::I64(w) => {
                state.write_u8(2);
                w.hash(state);
            }
            Word::F64(w) => {
                state.write_u8(4);
                &(*w as u64).hash(state);
            }
            Word::CHAR(w) => {
                state.write_u8(8);
                w.hash(state);
            }
            Word::BOOL(w) => {
                state.write_u8(16);
                w.hash(state);
            }
        }
    }
}
impl Add for Word{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Word::U64(x) => match rhs {
                Word::U64(y) => Word::U64(x+y),
                Word::I64(y) => Word::I64(x as  isize + y),
                Word::F64(y) => Word::F64(x as f64 + y),
                Word::CHAR(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
                Word::BOOL(y) => Word::U64(x + (y as usize)),
            },
            Word::I64(x) =>  match rhs {
                Word::U64(y) => Word::I64(x+y as isize),
                Word::I64(y) => Word::I64(x + y),
                Word::F64(y) => Word::F64(x as f64 + y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
                Word::BOOL(y) => Word::I64(x + (y as isize)),
            },
            Word::F64(x) => match rhs {
                Word::U64(y) => Word::F64(x + y as f64),
                Word::I64(y) => Word::F64(x + y as f64),
                Word::F64(y) => Word::F64(x + y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
                Word::BOOL(y) => Word::F64(x + ((y as usize) as f64)),
            },
            Word::CHAR(x) => match rhs {
                Word::U64(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
                Word::I64(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
                Word::F64(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
                Word::CHAR(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
                Word::BOOL(y) => Word::CHAR(((x as u8) + (y as u8)) as char ),
            },
            Word::BOOL(x) => match rhs {
                _ => Word::BOOL(x),
            },
        }
    }
}

impl Sub for Word{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Word::U64(x) => match rhs {
                Word::U64(y) => Word::U64(x-y),
                Word::I64(y) => Word::I64(x as  isize - y),
                Word::F64(y) => Word::F64(x as f64 - y),
                Word::CHAR(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
                Word::BOOL(y) => Word::U64(x - (y as usize)),
            },
            Word::I64(x) =>  match rhs {
                Word::U64(y) => Word::I64(x-y as isize),
                Word::I64(y) => Word::I64(x - y),
                Word::F64(y) => Word::F64(x as f64 - y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
                Word::BOOL(y) => Word::I64(x - (y as isize)),
            },
            Word::F64(x) => match rhs {
                Word::U64(y) => Word::F64(x - y as f64),
                Word::I64(y) => Word::F64(x - y as f64),
                Word::F64(y) => Word::F64(x - y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
                Word::BOOL(y) => Word::F64(x - ((y as usize) as f64)),
            },
            Word::CHAR(x) => match rhs {
                Word::U64(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
                Word::I64(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
                Word::F64(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
                Word::CHAR(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
                Word::BOOL(y) => Word::CHAR(((x as u8) - (y as u8)) as char ),
            },

            Word::BOOL(x) => match rhs {
                _ => Word::BOOL(x),
            },
        }
    }
}

impl Mul for Word{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Word::U64(x) => match rhs {
                Word::U64(y) => Word::U64(x*y),
                Word::I64(y) => Word::I64(x as  isize * y),
                Word::F64(y) => Word::F64(x as f64 * y),
                Word::CHAR(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
                Word::BOOL(y) => Word::U64(x * (y as usize)),
            },
            Word::I64(x) =>  match rhs {
                Word::U64(y) => Word::I64(x*y as isize),
                Word::I64(y) => Word::I64(x * y),
                Word::F64(y) => Word::F64(x as f64 * y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
                Word::BOOL(y) => Word::I64(x * (y as isize)),
            },
            Word::F64(x) => match rhs {
                Word::U64(y) => Word::F64(x * y as f64),
                Word::I64(y) => Word::F64(x * y as f64),
                Word::F64(y) => Word::F64(x * y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
                Word::BOOL(y) => Word::F64(x * ((y as usize) as f64)),
            },
            Word::CHAR(x) => match rhs {
                Word::U64(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
                Word::I64(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
                Word::F64(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
                Word::CHAR(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
                Word::BOOL(y) => Word::CHAR(((x as u8) * (y as u8)) as char ),
            },
            Word::BOOL(x) => match rhs {
                _ => Word::BOOL(x),
            },
        }
    }
}

impl Div for Word{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Word::U64(x) => match rhs {
                Word::U64(y) => Word::U64(x/y),
                Word::I64(y) => Word::I64(x as  isize / y),
                Word::F64(y) => Word::F64(x as f64 / y),
                Word::CHAR(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
                Word::BOOL(y) => Word::U64(x / (y as usize)),
            },
            Word::I64(x) =>  match rhs {
                Word::U64(y) => Word::I64(x/y as isize),
                Word::I64(y) => Word::I64(x / y),
                Word::F64(y) => Word::F64(x as f64 / y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
                Word::BOOL(y) => Word::I64(x / (y as isize)),
            },
            Word::F64(x) => match rhs {
                Word::U64(y) => Word::F64(x / y as f64),
                Word::I64(y) => Word::F64(x / y as f64),
                Word::F64(y) => Word::F64(x / y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
                Word::BOOL(y) => Word::F64(x / ((y as usize) as f64)),
            },
            Word::CHAR(x) => match rhs {
                Word::U64(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
                Word::I64(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
                Word::F64(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
                Word::CHAR(y) => Word::CHAR(((x as u8) / (y as u8)) as char ),
                Word::BOOL(y) => Word::CHAR(x),
            },
            Word::BOOL(x) => match rhs {
                _ => Word::BOOL(x),
            },
        }
    }
}

impl Rem for Word{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Word::U64(x) => match rhs {
                Word::U64(y) => Word::U64(x%y),
                Word::I64(y) => Word::I64(x as  isize % y),
                Word::F64(y) => Word::F64(x as f64 % y),
                Word::CHAR(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
                Word::BOOL(y) => Word::U64(x % (y as usize)),
            },
            Word::I64(x) =>  match rhs {
                Word::U64(y) => Word::I64(x%y as isize),
                Word::I64(y) => Word::I64(x % y),
                Word::F64(y) => Word::F64(x as f64 % y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
                Word::BOOL(y) => Word::I64(x % (y as isize)),
            },
            Word::F64(x) => match rhs {
                Word::U64(y) => Word::F64(x % y as f64),
                Word::I64(y) => Word::F64(x % y as f64),
                Word::F64(y) => Word::F64(x % y ),
                Word::CHAR(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
                Word::BOOL(y) => Word::F64(x % ((y as usize) as f64)),
            },
            Word::CHAR(x) => match rhs {
                Word::U64(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
                Word::I64(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
                Word::F64(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
                Word::CHAR(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
                Word::BOOL(y) => Word::CHAR(((x as u8) % (y as u8)) as char ),
            },
            Word::BOOL(x) => match rhs {
                _ => Word::BOOL(x),
            },
        }
    }
}

impl BitAnd for Word {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let cmp_bytes = {
            match rhs {
                Word::U64(w) => w,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
                Word::BOOL(w) => w as usize,
            }
        };
        let self_bytes = {
            match self {
                Word::U64(w) => w,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
                Word::BOOL(w) => w as usize,
            }
        };
        let res = self_bytes & cmp_bytes;
        return Word::U64(res);
    }
}

impl BitOr for Word {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let cmp_bytes = {
            match rhs {
                Word::U64(w) => w,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
                Word::BOOL(w) => w as usize,
            }
        };
        let self_bytes = {
            match self {
                Word::U64(w) => w,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
                Word::BOOL(w) => w as usize,
            }
        };
        let res = self_bytes | cmp_bytes;
        return Word::U64(res);
    }
}


impl BitXor for Word {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let cmp_bytes = {
            match rhs {
                Word::U64(w) => w,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
                Word::BOOL(w) => w as usize,
            }
        };
        let self_bytes = {
            match self {
                Word::U64(w) => w,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
                Word::BOOL(w) => w as usize,
            }
        };
        let res = self_bytes ^ cmp_bytes;
        return Word::U64(res);
    }
}
impl Shl for Word {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        let cmp_bytes = {
            match rhs {
                Word::U64(w) => w,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
                Word::BOOL(w) => w as usize,
            }
        };
        let self_bytes = {
            match self {
                Word::U64(w) => w,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
                Word::BOOL(w) => w as usize,
            }
        };
        let res = self_bytes << cmp_bytes;
        return Word::U64(res);
    }
}

impl Shr for Word {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        let cmp_bytes = {
            match rhs {
                Word::U64(w) => w,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
                Word::BOOL(w) => w as usize,
            }
        };
        let self_bytes = {
            match self {
                Word::U64(w) => w,
                Word::I64(w) => w as usize,
                Word::F64(w) => w as usize,
                Word::CHAR(w) => w as usize,
                Word::BOOL(w) => w as usize,
            }
        };
        let res = self_bytes >> cmp_bytes;
        return Word::U64(res);
    }
}