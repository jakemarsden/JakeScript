pub use block::*;
pub use declaration::*;
pub use expression::*;
pub use identifier::*;
pub use literal::*;
pub use statement::*;

use serde::{de, ser};
use std::fmt;

mod block;
mod declaration;
mod expression;
mod identifier;
mod literal;
mod statement;

pub trait Node: Clone + fmt::Debug + PartialEq + de::DeserializeOwned + ser::Serialize {}
