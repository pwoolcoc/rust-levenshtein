use std::map::hashmap;
use std::time;
use option::{Some,None};

export edit_distance;
export edit_distance_str;

fn minimum(args: ~[uint]) -> uint {
    vec::foldl(uint::max_value, args, |a, b| {uint::min(a, b)})
}

fn tail(s: ~str) -> ~str {
    str::slice(s, 1, str::len(s))
}

fn head(s: ~str) -> ~str {
    if str::len(s) == 0 {
        ~""
    } else {
        str::slice(s, 0, 1)
    }
}

fn edit_distance(s: ~[u8], t: ~[u8]) -> uint {
    edit_distance_str(str::from_bytes(s), str::from_bytes(t))
}

fn edit_distance_str(s: ~str, t: ~str) -> uint {
    _edit_distance_str(s, t, hashmap())
}

fn _edit_distance_str(s: ~str, t: ~str, table: hashmap<~str, uint>) -> uint {
    match table.find(s + t) {
        Some(value) => { value }
        None => {
            let slen = str::len(s);
            let tlen = str::len(t);
            let mut cost : uint = 0u;
            let mut result : uint;

            if head(copy s) != head(copy t) {
                cost = 1u;
            }

            if slen == 0u { result = tlen; }
            else if tlen == 0u { result = slen; }
            else {
                result = minimum(
                      ~[_edit_distance_str(tail(copy s), copy t, table) + 1u,
                        _edit_distance_str(copy s, tail(copy t), table) + 1u,
                        _edit_distance_str(tail(copy s), tail(copy t), table) + cost]
                );
            }

            table.insert(s + t, result);
            result
        }
    }

}

fn time_me (blk: fn() -> ()) -> float {
    let before = time::precise_time_s();
    blk();
    let after = time::precise_time_s();
    after - before
}


#[test]
fn test_distance() {
    let s = ~"kitten";
    let t = ~"sitting";
    assert edit_distance_str(s, t) == 3;
}

#[test]
fn test_timing() {
    let s = ~"kitten";
    let t = ~"sitting";
    let runtime = time_me(|| {
            for iter::repeat(10) {
                assert edit_distance_str(s,t) == 3;
            }
    });
    io::println(fmt!("total time: %f", runtime));
}

