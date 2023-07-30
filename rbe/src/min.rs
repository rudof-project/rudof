use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(PartialEq, Eq, Hash, PartialOrd, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Min{ 
    pub value: usize 
}

impl Min {
    pub fn is_0(&self) -> bool {
      self.value == 0
    }
}

impl From<usize> for Min {
    fn from(v: usize) -> Self {
       Min{value: v}
    }
}

impl From<i32> for Min {
    fn from(v: i32) -> Self {
       Min{value: v as usize}
    }
}

