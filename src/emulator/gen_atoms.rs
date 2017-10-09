//! Generated by codegen/vm_gen_op.py
//! Creates array of predefined atoms
//! Config used: OTP20 
#![allow(dead_code)]

use term::immediate;
use term::lterm::LTerm;


pub const APPLY: LTerm = LTerm { value: immediate::make_atom_raw_const(1) };
pub const BADARG: LTerm = LTerm { value: immediate::make_atom_raw_const(2) };
pub const BADARITH: LTerm = LTerm { value: immediate::make_atom_raw_const(3) };
pub const BADARITY: LTerm = LTerm { value: immediate::make_atom_raw_const(4) };
pub const BADFUN: LTerm = LTerm { value: immediate::make_atom_raw_const(5) };
pub const BADMATCH: LTerm = LTerm { value: immediate::make_atom_raw_const(6) };
pub const CASE_CLAUSE: LTerm = LTerm { value: immediate::make_atom_raw_const(7) };
pub const ERLANG: LTerm = LTerm { value: immediate::make_atom_raw_const(8) };
pub const ERROR: LTerm = LTerm { value: immediate::make_atom_raw_const(9) };
pub const EXIT: LTerm = LTerm { value: immediate::make_atom_raw_const(10) };
pub const FALSE: LTerm = LTerm { value: immediate::make_atom_raw_const(11) };
pub const FUNCTION_CLAUSE: LTerm = LTerm { value: immediate::make_atom_raw_const(12) };
pub const HIGH: LTerm = LTerm { value: immediate::make_atom_raw_const(13) };
pub const IF_CLAUSE: LTerm = LTerm { value: immediate::make_atom_raw_const(14) };
pub const INIT: LTerm = LTerm { value: immediate::make_atom_raw_const(15) };
pub const KILL: LTerm = LTerm { value: immediate::make_atom_raw_const(16) };
pub const KILLED: LTerm = LTerm { value: immediate::make_atom_raw_const(17) };
pub const LOW: LTerm = LTerm { value: immediate::make_atom_raw_const(18) };
pub const NOCATCH: LTerm = LTerm { value: immediate::make_atom_raw_const(19) };
pub const NORMAL: LTerm = LTerm { value: immediate::make_atom_raw_const(20) };
pub const OK: LTerm = LTerm { value: immediate::make_atom_raw_const(21) };
pub const SYSTEM_LIMIT: LTerm = LTerm { value: immediate::make_atom_raw_const(22) };
pub const THROW: LTerm = LTerm { value: immediate::make_atom_raw_const(23) };
pub const TRAP_EXIT: LTerm = LTerm { value: immediate::make_atom_raw_const(24) };
pub const TRUE: LTerm = LTerm { value: immediate::make_atom_raw_const(25) };
pub const UNDEF: LTerm = LTerm { value: immediate::make_atom_raw_const(26) };
pub const UNDEFINED: LTerm = LTerm { value: immediate::make_atom_raw_const(27) };
pub const G: LTerm = LTerm { value: immediate::make_atom_raw_const(28) };
pub const B: LTerm = LTerm { value: immediate::make_atom_raw_const(29) };
pub const U: LTerm = LTerm { value: immediate::make_atom_raw_const(30) };