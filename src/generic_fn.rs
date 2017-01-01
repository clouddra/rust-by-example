struct A;
struct S(A);

struct SGen<T>(T);

fn main() {
    let _s = S(A);

    let _char: SGen<char> = SGen('a');

    let _char = SGen('a');

    let _t = SGen(A);

    let _i32 = SGen(6);

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    generic::<char>(SGen('a'));
    generic(SGen('c'));
}

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}
