#![no_main]

use libfuzzer_sys::fuzz_target;
use rustand_fuzz::{run_fuzz, Operation};

fuzz_target!(|ops: Vec<Operation>| {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    rt.block_on(run_fuzz(ops));
});
