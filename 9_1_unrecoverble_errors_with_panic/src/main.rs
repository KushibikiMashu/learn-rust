fn main() {
    panic!("crash and burn");

    let v = vec![1, 2, 3];

//    存在しない要素のインデックスにアクセスしているため
//    エラーが発生する
//    v[99];

//    $ RUST_BACKTRACE=1 cargo run
//    Compiling unrecoverble_errors_with_panic v0.1.0 (/rust/unrecoverble_errors_with_panic)
//
//    Finished dev [unoptimized + debuginfo] target(s) in 1.55s
//    Running `target/debug/unrecoverble_errors_with_panic`
//    thread 'main' panicked at 'crash and burn', src/main.rs:2:5
//    stack backtrace:
//    0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
//    at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
//    1: std::sys_common::backtrace::_print
//    at src/libstd/sys_common/backtrace.rs:70
//    2: std::panicking::default_hook::{{closure}}
//    at src/libstd/sys_common/backtrace.rs:58
//    at src/libstd/panicking.rs:200
//    3: std::panicking::default_hook
//    at src/libstd/panicking.rs:215
//    4: std::panicking::rust_panic_with_hook
//    at src/libstd/panicking.rs:478
//    5: std::panicking::begin_panic
//    at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libstd/panicking.rs:412
//    6: unrecoverble_errors_with_panic::main
//    at src/main.rs:2
//    7: std::rt::lang_start::{{closure}}
//    at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libstd/rt.rs:64
//    8: std::panicking::try::do_call
//    at src/libstd/rt.rs:49
//    at src/libstd/panicking.rs:297
//    9: __rust_maybe_catch_panic
//    at src/libpanic_unwind/lib.rs:92
//    10: std::rt::lang_start_internal
//    at src/libstd/panicking.rs:276
//    at src/libstd/panic.rs:388
//    at src/libstd/rt.rs:48
//    11: std::rt::lang_start
//    at /rustc/2aa4c46cfdd726e97360c2734835aa3515e8c858/src/libstd/rt.rs:64
//    12: main
//    13: __libc_start_main
//    14: _start
}
