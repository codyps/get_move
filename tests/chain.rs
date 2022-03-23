use get_move::{Chain, Get};

#[test]
fn chain() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];

    let c = a.chain(b);

    let mut c2 = vec![];
    for i in 0..c.len() {
        c2.push(c.get_move(i).unwrap());
    }

    assert_eq!(vec![1, 2, 3, 4, 5, 6], c2);
}

#[test]
fn iter() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];

    let c = a.chain(b);

    let mut c2 = vec![];
    for i in c.iter() {
        c2.push(i);
    }

    assert_eq!(vec![1, 2, 3, 4, 5, 6], c2);
}

#[test]
fn iter_rev() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];

    let c = a.chain(b);

    let mut c2 = vec![];
    for i in c.iter().rev() {
        c2.push(i);
    }

    assert_eq!(vec![6, 5, 4, 3, 2, 1], c2);
}

#[test]
fn len_variants() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];

    let c = a.chain(b);

    assert_eq!(c.len(), ExactSizeIterator::len(&c.iter()));
    assert_eq!(c.len(), c.iter().size_hint().0);
    assert_eq!(c.len(), c.iter().size_hint().1.unwrap());
}

const CHECK_CONST_CHAIN: Chain<[usize; 2], [usize; 2]> = get_move::chain([1, 2], [2, 3]);

#[test]
fn check_const_chain() {
    let a = [1, 2, 2, 3];

    assert!(a.iter().eq(CHECK_CONST_CHAIN.iter()));
}
