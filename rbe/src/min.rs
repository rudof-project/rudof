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
