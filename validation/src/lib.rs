#![allow(warnings)]

extern crate jiyunet_core as core;
extern crate jiyunet_dag as dag;
extern crate jiyunet_db as db;

use dag::block;

pub mod ck;
pub mod io;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ValidationError {
    DecodeError(core::Address), // Problem decoding data.
    NodeNotFound(core::Address), // Node not found in db, try again later?
    ComponentTooLarge(core::sig::Hash), // If something is too big to be allowed.
    InsufficientCredits, // Identitiy doesn't have credits for some action.
}
