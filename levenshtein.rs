use std;

fn minimum(args: ~[uint]) -> uint {
    vec::foldl(uint::max_value, args, |a, b| { uint::min(a, b) })
}

fn distance(s: ~str, t: ~str) -> uint {
    io::println(fmt!("'%s', '%s'", s, t));
    let slen = str::len(s);
    let tlen = str::len(t);
    let mut cost : uint = 0u;

    io::println(fmt!("'%s':%u, '%s':%u", s, slen, t, tlen));

    if slen > 0u && tlen > 0u { // not having this here causes bounds check assertion failed
        if str::slice(s, 0u, 1u) != str::slice(t, 0u, 1u) {
            cost = 1u;
        }
    }

    if slen == 0u { io::println("slen == 0, returning tlen"); return tlen; }
    else if tlen == 0u { io::println("tlen == 0, returning slen"); return slen; }
    else { io::println("slen > 0, tlen > 0, returning minimum"); return minimum(
              ~[distance(copy str::slice(s, 1u, slen - 1u), copy t) + 1u,
                distance(copy s, copy str::slice(t, 1u, tlen - 1u)) + 1u,
                distance(copy str::slice(s, 1u, slen - 1u), copy str::slice(t, 1u, tlen - 1u)) + cost]
            );
    }
}

#[test]
fn test_minimum() {
    let a = ~[5, 8, 2 , 0];
    assert 0 == minimum(a);
}

#[test]
fn test_distance() {
    let s = ~"kitten";
    let t = ~"sitting";
    distance(s, t);
}

