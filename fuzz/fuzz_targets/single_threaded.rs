#![no_main]

use libfuzzer_sys::fuzz_target;
use rustand_fuzz::{Operation, run_fuzz};

fuzz_target!(|ops: Vec<Operation>| {
    run_fuzz(ops);
});
