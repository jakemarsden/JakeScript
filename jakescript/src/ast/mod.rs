pub use crate::token::{SourceLocation, SourcePosition};
pub use declaration::*;
pub use expression::*;
pub use identifier::*;
pub use literal::*;
pub use op::*;
use serde::{de, ser, Deserialize, Serialize};
pub use statement::*;
use std::fmt;

mod declaration;
mod expression;
mod identifier;
mod literal;
mod op;
mod statement;

#[macro_export(crate)]
macro_rules! ast_node {
    (
        $(#[$attribute:meta] )*
        $vis:vis struct $type_name:ident {$(
            $(#[$member_attribute:meta] )*
            $member_vis:vis $member:ident: $member_type:ty,
        )*}
    ) => {
        ast_node!(
            $(#[$attribute] )*
            ##[source_location = |self| &self.loc]
            $vis struct $type_name {$(
                $(#[$member_attribute] )*
                $member_vis $member: $member_type,
            )*}
        );
    };
    (
        $(#[$attribute:meta] )*
        ##[source_location = |$loc_self:ident| $loc_expr:expr]
        $vis:vis struct $type_name:ident {$(
            $(#[$member_attribute:meta] )*
            $member_vis:vis $member:ident: $member_type:ty,
        )*}
    ) => {
        #[derive(Clone, Debug, PartialEq, ::serde::Deserialize, ::serde::Serialize)]
        $(#[$attribute] )*
        $vis struct $type_name {$(
            $(#[$member_attribute] )*
            $member_vis $member: $member_type,
        )*}

        impl $crate::ast::Node for $type_name {
            fn source_location(&$loc_self) -> &$crate::ast::SourceLocation {
                $loc_expr
            }
        }
    };
    (
        $(#[$attribute:meta] )*
        $vis:vis enum $type_name:ident {$(
            $(#[$variant_attribute:meta] )*
            $variant:ident($inner:ty),
        )*}
    ) => {
        #[derive(Clone, Debug, PartialEq, ::serde::Deserialize, ::serde::Serialize)]
        $(#[$attribute] )*
        $vis enum $type_name {$(
            $(#[$variant_attribute] )*
            $variant($inner),
        )*}

        impl $crate::ast::Node for $type_name {
            fn source_location(&self) -> &$crate::ast::SourceLocation {
                match self {$(
                    Self::$variant(inner) => inner.source_location(),
                )*}
            }
        }

        $(impl ::std::convert::From<$inner> for $type_name {
            fn from(inner: $inner) -> Self {
                Self::$variant(inner)
            }
        }

        impl ::std::convert::TryFrom<$type_name> for $inner {
            type Error = $type_name;

            fn try_from(node: $type_name) -> ::std::result::Result<Self, Self::Error> {
                if let $type_name::$variant(inner) = node {
                    ::std::result::Result::Ok(inner)
                } else {
                    ::std::result::Result::Err(node)
                }
            }
        })*
    };
}

pub trait Node: Clone + fmt::Debug + PartialEq + de::DeserializeOwned + ser::Serialize {
    fn source_location(&self) -> &SourceLocation;
}

ast_node!(
    #[derive(Default)]
    pub struct Script {
        loc: SourceLocation,
        body: Block,
    }
);

impl Script {
    pub fn new(loc: SourceLocation, body: Block) -> Self {
        Self { loc, body }
    }

    pub fn body(&self) -> &Block {
        &self.body
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Block {
    hoisted_declarations: Vec<Declaration>,
    body: Vec<Statement>,
}

impl Default for Block {
    fn default() -> Self {
        Self::empty()
    }
}

impl Block {
    pub fn empty() -> Self {
        Self {
            hoisted_declarations: Vec::default(),
            body: Vec::default(),
        }
    }

    pub fn single_statement(statement: Statement) -> Self {
        Self {
            hoisted_declarations: Vec::default(),
            body: vec![statement],
        }
    }

    pub fn new(hoisted_declarations: Vec<Declaration>, body: Vec<Statement>) -> Self {
        Self {
            hoisted_declarations,
            body,
        }
    }

    pub fn hoisted_declarations(&self) -> &[Declaration] {
        &self.hoisted_declarations
    }

    pub fn body(&self) -> &[Statement] {
        &self.body
    }
}
