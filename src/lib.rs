#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

pub mod rules;

mod ext;
mod gas;
#[cfg(feature = "cli")]
pub mod logger;
mod optimizer;
mod symbols;

pub mod stack_height;

pub use ext::{
    externalize, externalize_mem, shrink_unknown_stack, underscore_funcs, ununderscore_funcs,
};
pub use gas::inject_gas_counter;
pub use optimizer::{optimize, Error as OptimizerError};

pub struct TargetSymbols {
    pub create: &'static str,
    pub call: &'static str,
    pub ret: &'static str,
}

pub enum TargetRuntime {
    Substrate(TargetSymbols),
    PWasm(TargetSymbols),
}

impl TargetRuntime {
    pub fn substrate() -> TargetRuntime {
        TargetRuntime::Substrate(TargetSymbols {
            create: "deploy",
            call: "call",
            ret: "ext_return",
        })
    }

    pub fn pwasm() -> TargetRuntime {
        TargetRuntime::PWasm(TargetSymbols {
            create: "deploy",
            call: "call",
            ret: "ret",
        })
    }

    pub fn symbols(&self) -> &TargetSymbols {
        match self {
            TargetRuntime::Substrate(s) => s,
            TargetRuntime::PWasm(s) => s,
        }
    }
}

#[cfg(not(feature = "std"))]
mod std {
    pub use ::alloc::{borrow, boxed, string, vec};
    pub use core::*;

    pub mod rc {
        pub use alloc::rc::Rc;
    }

    pub mod collections {
        pub use alloc::collections::{BTreeMap, BTreeSet};
    }
}

#[cfg(feature = "std")]
mod std {
    pub use std::*;
}
