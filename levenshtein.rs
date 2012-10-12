extern mod std;
use std::map::HashMap;
use option::{Some,None};

export edit_distance;
export edit_distance_str;

fn minimum(args: ~[uint]) -> uint {
    vec::foldl(uint::max_value, args, |a, b| uint::min(a, *b))
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

pub fn edit_distance(s: ~[u8], t: ~[u8]) -> uint {
    edit_distance_str(str::from_bytes(s), str::from_bytes(t))
}

pub fn edit_distance_str(s: ~str, t: ~str) -> uint {
    _edit_distance_str(s, t, HashMap())
}

fn _edit_distance_str(s: ~str, t: ~str, table: HashMap<~str, uint>) -> uint {
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

#[test]
fn test_distance() {
    let s = ~"kitten";
    let t = ~"sitting";
    assert edit_distance_str(s, t) == 3;
}

