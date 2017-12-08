#![allow(warnings)]

extern crate jiyunet_core as core;
extern crate jiyunet_dag as dag;
extern crate jiyunet_db as db;

use dag::block;

pub mod ck;
pub mod io;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ValidationError {

    // Problem decoding data.
    DecodeError(core::Address),

    // Node not found in db, try again later?
    NodeNotFound(core::Address),

    // If something is too big to be allowed.
    ComponentTooLarge(core::sig::Hash),

    // Identitiy doesn't have credits for some action.
    InsufficientCredits

}
