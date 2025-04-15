use serde::{Deserialize, Serialize};

use crate::tokens::Assignable;

// 入力のJsonの構造を示す型
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AzikConfig {
    pub Sequence: Vec<Assignable>,
    pub Sokuon: String,
    pub Hatsuon: String,
}
