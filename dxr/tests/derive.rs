#[test]
fn try_build_pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/array.rs");
    t.pass("tests/trybuild/moo.rs");
    t.pass("tests/trybuild/appendix.rs");
    t.pass("tests/trybuild/recursive.rs");
    t.pass("tests/trybuild/custom_result_type.rs");
    t.pass("tests/trybuild/ownership.rs");
    t.pass("tests/trybuild/collections.rs");
}

#[rustversion::stable]
#[test]
fn try_build_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/slice.rs");
    t.compile_fail("tests/trybuild/toref.rs");
    t.compile_fail("tests/trybuild/tuple.rs");
    t.compile_fail("tests/trybuild/enum.rs");
}

#[rustversion::nightly]
#[test]
fn try_build_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/trybuild/slice.rs");
    t.compile_fail("tests/trybuild/tuple.rs");
    t.compile_fail("tests/trybuild/enum.rs");
}
