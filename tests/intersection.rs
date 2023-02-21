extern crate intervals;

use intervals::Interval;

macro_rules! i {
    (Open[$left:expr, $right:expr]) => { Interval::open_unchecked($left, $right) };
    (Closed[$left:expr, $right:expr]) => { Interval::closed_unchecked($left, $right) };

    (LCRO[$left:expr, $right:expr]) => { Interval::lcro_unchecked($left, $right) };
    (LORC[$left:expr, $right:expr]) => { Interval::lorc_unchecked($left, $right) };

    (LO[$left:expr]) => { Interval::left_open($left) };
    (LC[$left:expr]) => { Interval::left_closed($left) };

    (RO[$right:expr]) => { Interval::right_open($right) };
    (RC[$right:expr]) => { Interval::right_closed($right) };

    (Degenerate[$x:expr]) => { Interval::degenerate($x) };
}

macro_rules! test_intersects {
    ($x:expr; [$(($y:expr, |$z:ident| $test:expr)),+]) => {{
        let x = $x;
        $({
            let y = $y;
            let $z = x.intersect(y);

            { $test }

            let $z = y.intersect(x);

            { $test }
        })+
    }};
}

#[test]
fn unbounded() {
    test_intersects!(
        Interval::unbounded();
        [
            (i!(Closed[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0]))),
            (i!(Open[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LORC[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(LCRO[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(LO[0.0]), |z| assert_eq!(z.unwrap(), i!(LO[0.0]))),
            (i!(LC[0.0]), |z| assert_eq!(z.unwrap(), i!(LC[0.0]))),
            (i!(RO[0.0]), |z| assert_eq!(z.unwrap(), i!(RO[0.0]))),
            (i!(RC[0.0]), |z| assert_eq!(z.unwrap(), i!(RC[0.0])))
        ]
    );
}

#[test]
fn closed() {
    test_intersects!(
        i!(Closed[0.0, 1.0]);
        [
            (i!(Closed[-2.0, -1.0]), |z| assert!(z.is_none())),
            (i!(Closed[-1.0, 0.0]), |z| assert_eq!(z.unwrap(), i!(Degenerate[0.0]))),
            (i!(Closed[-0.5, 0.5]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 0.5]))),
            (i!(Closed[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0]))),
            (i!(Closed[0.5, 1.5]), |z| assert_eq!(z.unwrap(), i!(Closed[0.5, 1.0]))),
            (i!(Closed[1.0, 2.0]), |z| assert_eq!(z.unwrap(), i!(Degenerate[1.0]))),
            (i!(Closed[2.0, 3.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(Closed[0.0, 1.0]);
        [
            (i!(Open[-2.0, -1.0]), |z| assert!(z.is_none())),
            (i!(Open[-1.0, 0.0]), |z| assert!(z.is_none())),
            (i!(Open[-0.5, 0.5]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 0.5]))),
            (i!(Open[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(Open[0.5, 1.5]), |z| assert_eq!(z.unwrap(), i!(LORC[0.5, 1.0]))),
            (i!(Open[1.0, 2.0]), |z| assert!(z.is_none())),
            (i!(Open[2.0, 3.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(Closed[0.0, 1.0]);
        [
            (i!(LORC[-2.0, -1.0]), |z| assert!(z.is_none())),
            (i!(LORC[-1.0, 0.0]), |z| assert_eq!(z.unwrap(), i!(Degenerate[0.0]))),
            (i!(LORC[-0.5, 0.5]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 0.5]))),
            (i!(LORC[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(LORC[0.5, 1.5]), |z| assert_eq!(z.unwrap(), i!(LORC[0.5, 1.0]))),
            (i!(LORC[1.0, 2.0]), |z| assert!(z.is_none())),
            (i!(LORC[2.0, 3.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(Closed[0.0, 1.0]);
        [
            (i!(LCRO[-2.0, -1.0]), |z| assert!(z.is_none())),
            (i!(LCRO[-1.0, 0.0]), |z| assert!(z.is_none())),
            (i!(LCRO[-0.5, 0.5]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 0.5]))),
            (i!(LCRO[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(LCRO[0.5, 1.5]), |z| assert_eq!(z.unwrap(), i!(Closed[0.5, 1.0]))),
            (i!(LCRO[1.0, 2.0]), |z| assert_eq!(z.unwrap(), i!(Degenerate[1.0]))),
            (i!(LCRO[2.0, 3.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(Closed[0.0, 1.0]);
        [
            (i!(LO[-2.0]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0]))),
            (i!(LO[-1.0]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0]))),
            (i!(LO[-0.5]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0]))),
            (i!(LO[0.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(LO[0.5]), |z| assert_eq!(z.unwrap(), i!(LORC[0.5, 1.0]))),
            (i!(LO[1.0]), |z| assert!(z.is_none())),
            (i!(LO[2.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(Closed[0.0, 1.0]);
        [
            (i!(LC[-2.0]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0]))),
            (i!(LC[-1.0]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0]))),
            (i!(LC[-0.5]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0]))),
            (i!(LC[0.0]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0]))),
            (i!(LC[0.5]), |z| assert_eq!(z.unwrap(), i!(Closed[0.5, 1.0]))),
            (i!(LC[1.0]), |z| assert_eq!(z.unwrap(), i!(Degenerate[1.0]))),
            (i!(LC[2.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(Closed[0.0, 1.0]);
        [
            (i!(RO[-2.0]), |z| assert!(z.is_none())),
            (i!(RO[-1.0]), |z| assert!(z.is_none())),
            (i!(RO[-0.5]), |z| assert!(z.is_none())),
            (i!(RO[0.0]), |z| assert!(z.is_none())),
            (i!(RO[0.5]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 0.5]))),
            (i!(RO[1.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0])))
        ]
    );

    test_intersects!(
        i!(Closed[0.0, 1.0]);
        [
            (i!(RC[-2.0]), |z| assert!(z.is_none())),
            (i!(RC[-1.0]), |z| assert!(z.is_none())),
            (i!(RC[-0.5]), |z| assert!(z.is_none())),
            (i!(RC[0.0]), |z| assert_eq!(z.unwrap(), i!(Degenerate[0.0]))),
            (i!(RC[0.5]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 0.5]))),
            (i!(RC[1.0]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0])))
        ]
    );
}

#[test]
fn open() {
    test_intersects!(
        i!(Open[0.0, 1.0]);
        [
            (Interval::unbounded(), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0])))
        ]
    );

    test_intersects!(
        i!(Open[0.0, 1.0]);
        [
            (i!(Open[-2.0, -1.0]), |z| assert!(z.is_none())),
            (i!(Open[-1.0, 0.0]), |z| assert!(z.is_none())),
            (i!(Open[-0.5, 0.5]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 0.5]))),
            (i!(Open[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(Open[0.5, 1.5]), |z| assert_eq!(z.unwrap(), i!(Open[0.5, 1.0]))),
            (i!(Open[1.0, 2.0]), |z| assert!(z.is_none())),
            (i!(Open[2.0, 3.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(Open[0.0, 1.0]);
        [
            (i!(LORC[-2.0, -1.0]), |z| assert!(z.is_none())),
            (i!(LORC[-1.0, 0.0]), |z| assert!(z.is_none())),
            (i!(LORC[-0.5, 0.5]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 0.5]))),
            (i!(LORC[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LORC[0.5, 1.5]), |z| assert_eq!(z.unwrap(), i!(Open[0.5, 1.0]))),
            (i!(LORC[1.0, 2.0]), |z| assert!(z.is_none())),
            (i!(LORC[2.0, 3.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(Open[0.0, 1.0]);
        [
            (i!(LCRO[-2.0, -1.0]), |z| assert!(z.is_none())),
            (i!(LCRO[-1.0, 0.0]), |z| assert!(z.is_none())),
            (i!(LCRO[-0.5, 0.5]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 0.5]))),
            (i!(LCRO[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LCRO[0.5, 1.5]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.5, 1.0]))),
            (i!(LCRO[1.0, 2.0]), |z| assert!(z.is_none())),
            (i!(LCRO[2.0, 3.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(Open[0.0, 1.0]);
        [
            (i!(LO[-2.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LO[-1.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LO[-0.5]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LO[0.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LO[0.5]), |z| assert_eq!(z.unwrap(), i!(Open[0.5, 1.0]))),
            (i!(LO[1.0]), |z| assert!(z.is_none())),
            (i!(LO[2.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(Open[0.0, 1.0]);
        [
            (i!(LC[-2.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LC[-1.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LC[-0.5]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LC[0.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LC[0.5]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.5, 1.0]))),
            (i!(LC[1.0]), |z| assert!(z.is_none())),
            (i!(LC[2.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(Open[0.0, 1.0]);
        [
            (i!(RO[-2.0]), |z| assert!(z.is_none())),
            (i!(RO[-1.0]), |z| assert!(z.is_none())),
            (i!(RO[-0.5]), |z| assert!(z.is_none())),
            (i!(RO[0.0]), |z| assert!(z.is_none())),
            (i!(RO[0.5]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 0.5]))),
            (i!(RO[1.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0])))
        ]
    );

    test_intersects!(
        i!(Open[0.0, 1.0]);
        [
            (i!(RC[-2.0]), |z| assert!(z.is_none())),
            (i!(RC[-1.0]), |z| assert!(z.is_none())),
            (i!(RC[-0.5]), |z| assert!(z.is_none())),
            (i!(RC[0.0]), |z| assert!(z.is_none())),
            (i!(RC[0.5]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 0.5]))),
            (i!(RC[1.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0])))
        ]
    );
}

#[test]
fn lorc() {
    test_intersects!(
        i!(LORC[0.0, 1.0]);
        [
            (i!(LORC[-2.0, -1.0]), |z| assert!(z.is_none())),
            (i!(LORC[-1.0, 0.0]), |z| assert!(z.is_none())),
            (i!(LORC[-0.5, 0.5]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 0.5]))),
            (i!(LORC[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(LORC[0.5, 1.5]), |z| assert_eq!(z.unwrap(), i!(LORC[0.5, 1.0]))),
            (i!(LORC[1.0, 2.0]), |z| assert!(z.is_none())),
            (i!(LORC[2.0, 3.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(LORC[0.0, 1.0]);
        [
            (i!(LCRO[-2.0, -1.0]), |z| assert!(z.is_none())),
            (i!(LCRO[-1.0, 0.0]), |z| assert!(z.is_none())),
            (i!(LCRO[-0.5, 0.5]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 0.5]))),
            (i!(LCRO[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LCRO[0.5, 1.5]), |z| assert_eq!(z.unwrap(), i!(Closed[0.5, 1.0]))),
            (i!(LCRO[1.0, 2.0]), |z| assert_eq!(z.unwrap(), i!(Degenerate[1.0]))),
            (i!(LCRO[2.0, 3.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(LORC[0.0, 1.0]);
        [
            (i!(LO[-2.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(LO[-1.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(LO[-0.5]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(LO[0.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(LO[0.5]), |z| assert_eq!(z.unwrap(), i!(LORC[0.5, 1.0]))),
            (i!(LO[1.0]), |z| assert!(z.is_none())),
            (i!(LO[2.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(LORC[0.0, 1.0]);
        [
            (i!(LC[-2.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(LC[-1.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(LC[-0.5]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(LC[0.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(LC[0.5]), |z| assert_eq!(z.unwrap(), i!(Closed[0.5, 1.0]))),
            (i!(LC[1.0]), |z| assert_eq!(z.unwrap(), i!(Degenerate[1.0]))),
            (i!(LC[2.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(LORC[0.0, 1.0]);
        [
            (i!(RO[-2.0]), |z| assert!(z.is_none())),
            (i!(RO[-1.0]), |z| assert!(z.is_none())),
            (i!(RO[-0.5]), |z| assert!(z.is_none())),
            (i!(RO[0.0]), |z| assert!(z.is_none())),
            (i!(RO[0.5]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 0.5]))),
            (i!(RO[1.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0])))
        ]
    );

    test_intersects!(
        i!(LORC[0.0, 1.0]);
        [
            (i!(RC[-2.0]), |z| assert!(z.is_none())),
            (i!(RC[-1.0]), |z| assert!(z.is_none())),
            (i!(RC[-0.5]), |z| assert!(z.is_none())),
            (i!(RC[0.0]), |z| assert!(z.is_none())),
            (i!(RC[0.5]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 0.5]))),
            (i!(RC[1.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0])))
        ]
    );
}

#[test]
fn lcro() {
    test_intersects!(
        i!(LCRO[0.0, 1.0]);
        [
            (i!(LCRO[-2.0, -1.0]), |z| assert!(z.is_none())),
            (i!(LCRO[-1.0, 0.0]), |z| assert!(z.is_none())),
            (i!(LCRO[-0.5, 0.5]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 0.5]))),
            (i!(LCRO[0.0, 1.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(LCRO[0.5, 1.5]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.5, 1.0]))),
            (i!(LCRO[1.0, 2.0]), |z| assert!(z.is_none())),
            (i!(LCRO[2.0, 3.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(LCRO[0.0, 1.0]);
        [
            (i!(LO[-2.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(LO[-1.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(LO[-0.5]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(LO[0.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(LO[0.5]), |z| assert_eq!(z.unwrap(), i!(Open[0.5, 1.0]))),
            (i!(LO[1.0]), |z| assert!(z.is_none())),
            (i!(LO[2.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(LCRO[0.0, 1.0]);
        [
            (i!(LC[-2.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(LC[-1.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(LC[-0.5]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(LC[0.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(LC[0.5]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.5, 1.0]))),
            (i!(LC[1.0]), |z| assert!(z.is_none())),
            (i!(LC[2.0]), |z| assert!(z.is_none()))
        ]
    );

    test_intersects!(
        i!(LCRO[0.0, 1.0]);
        [
            (i!(RO[-2.0]), |z| assert!(z.is_none())),
            (i!(RO[-1.0]), |z| assert!(z.is_none())),
            (i!(RO[-0.5]), |z| assert!(z.is_none())),
            (i!(RO[0.0]), |z| assert!(z.is_none())),
            (i!(RO[0.5]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 0.5]))),
            (i!(RO[1.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0])))
        ]
    );

    test_intersects!(
        i!(LCRO[0.0, 1.0]);
        [
            (i!(RC[-2.0]), |z| assert!(z.is_none())),
            (i!(RC[-1.0]), |z| assert!(z.is_none())),
            (i!(RC[-0.5]), |z| assert!(z.is_none())),
            (i!(RC[0.0]), |z| assert_eq!(z.unwrap(), i!(Degenerate[0.0]))),
            (i!(RC[0.5]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 0.5]))),
            (i!(RC[1.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0])))
        ]
    );
}

#[test]
fn lo() {
    test_intersects!(
        i!(LO[0.0]);
        [
            (i!(LO[-2.0]), |z| assert_eq!(z.unwrap(), i!(LO[0.0]))),
            (i!(LO[-1.0]), |z| assert_eq!(z.unwrap(), i!(LO[0.0]))),
            (i!(LO[-0.5]), |z| assert_eq!(z.unwrap(), i!(LO[0.0]))),
            (i!(LO[0.0]), |z| assert_eq!(z.unwrap(), i!(LO[0.0]))),
            (i!(LO[0.5]), |z| assert_eq!(z.unwrap(), i!(LO[0.5]))),
            (i!(LO[1.0]), |z| assert_eq!(z.unwrap(), i!(LO[1.0]))),
            (i!(LO[2.0]), |z| assert_eq!(z.unwrap(), i!(LO[2.0])))
        ]
    );

    test_intersects!(
        i!(LO[0.0]);
        [
            (i!(LC[-2.0]), |z| assert_eq!(z.unwrap(), i!(LO[0.0]))),
            (i!(LC[-1.0]), |z| assert_eq!(z.unwrap(), i!(LO[0.0]))),
            (i!(LC[-0.5]), |z| assert_eq!(z.unwrap(), i!(LO[0.0]))),
            (i!(LC[0.0]), |z| assert_eq!(z.unwrap(), i!(LO[0.0]))),
            (i!(LC[0.5]), |z| assert_eq!(z.unwrap(), i!(LC[0.5]))),
            (i!(LC[1.0]), |z| assert_eq!(z.unwrap(), i!(LC[1.0]))),
            (i!(LC[2.0]), |z| assert_eq!(z.unwrap(), i!(LC[2.0])))
        ]
    );

    test_intersects!(
        i!(LO[0.0]);
        [
            (i!(RO[-2.0]), |z| assert!(z.is_none())),
            (i!(RO[-1.0]), |z| assert!(z.is_none())),
            (i!(RO[-0.5]), |z| assert!(z.is_none())),
            (i!(RO[0.0]), |z| assert!(z.is_none())),
            (i!(RO[0.5]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 0.5]))),
            (i!(RO[1.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 1.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z.unwrap(), i!(Open[0.0, 2.0])))
        ]
    );

    test_intersects!(
        i!(LO[0.0]);
        [
            (i!(RC[-2.0]), |z| assert!(z.is_none())),
            (i!(RC[-1.0]), |z| assert!(z.is_none())),
            (i!(RC[-0.5]), |z| assert!(z.is_none())),
            (i!(RC[0.0]), |z| assert!(z.is_none())),
            (i!(RC[0.5]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 0.5]))),
            (i!(RC[1.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 1.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z.unwrap(), i!(LORC[0.0, 2.0])))
        ]
    );
}

#[test]
fn lc() {
    test_intersects!(
        i!(LC[0.0]);
        [
            (i!(LC[-2.0]), |z| assert_eq!(z.unwrap(), i!(LC[0.0]))),
            (i!(LC[-1.0]), |z| assert_eq!(z.unwrap(), i!(LC[0.0]))),
            (i!(LC[-0.5]), |z| assert_eq!(z.unwrap(), i!(LC[0.0]))),
            (i!(LC[0.0]), |z| assert_eq!(z.unwrap(), i!(LC[0.0]))),
            (i!(LC[0.5]), |z| assert_eq!(z.unwrap(), i!(LC[0.5]))),
            (i!(LC[1.0]), |z| assert_eq!(z.unwrap(), i!(LC[1.0]))),
            (i!(LC[2.0]), |z| assert_eq!(z.unwrap(), i!(LC[2.0])))
        ]
    );

    test_intersects!(
        i!(LC[0.0]);
        [
            (i!(RO[-2.0]), |z| assert!(z.is_none())),
            (i!(RO[-1.0]), |z| assert!(z.is_none())),
            (i!(RO[-0.5]), |z| assert!(z.is_none())),
            (i!(RO[0.0]), |z| assert!(z.is_none())),
            (i!(RO[0.5]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 0.5]))),
            (i!(RO[1.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 1.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z.unwrap(), i!(LCRO[0.0, 2.0])))
        ]
    );

    test_intersects!(
        i!(LC[0.0]);
        [
            (i!(RC[-2.0]), |z| assert!(z.is_none())),
            (i!(RC[-1.0]), |z| assert!(z.is_none())),
            (i!(RC[-0.5]), |z| assert!(z.is_none())),
            (i!(RC[0.0]), |z| assert_eq!(z.unwrap(), i!(Degenerate[0.0]))),
            (i!(RC[0.5]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 0.5]))),
            (i!(RC[1.0]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 1.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z.unwrap(), i!(Closed[0.0, 2.0])))
        ]
    );
}

#[test]
fn ro() {
    test_intersects!(
        i!(RO[0.0]);
        [
            (i!(RO[-2.0]), |z| assert_eq!(z.unwrap(), i!(RO[-2.0]))),
            (i!(RO[-1.0]), |z| assert_eq!(z.unwrap(), i!(RO[-1.0]))),
            (i!(RO[-0.5]), |z| assert_eq!(z.unwrap(), i!(RO[-0.5]))),
            (i!(RO[0.0]), |z| assert_eq!(z.unwrap(), i!(RO[0.0]))),
            (i!(RO[0.5]), |z| assert_eq!(z.unwrap(), i!(RO[0.0]))),
            (i!(RO[1.0]), |z| assert_eq!(z.unwrap(), i!(RO[0.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z.unwrap(), i!(RO[0.0])))
        ]
    );

    test_intersects!(
        i!(RO[0.0]);
        [
            (i!(RC[-2.0]), |z| assert_eq!(z.unwrap(), i!(RC[-2.0]))),
            (i!(RC[-1.0]), |z| assert_eq!(z.unwrap(), i!(RC[-1.0]))),
            (i!(RC[-0.5]), |z| assert_eq!(z.unwrap(), i!(RC[-0.5]))),
            (i!(RC[0.0]), |z| assert_eq!(z.unwrap(), i!(RO[0.0]))),
            (i!(RC[0.5]), |z| assert_eq!(z.unwrap(), i!(RO[0.0]))),
            (i!(RC[1.0]), |z| assert_eq!(z.unwrap(), i!(RO[0.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z.unwrap(), i!(RO[0.0])))
        ]
    );
}

#[test]
fn rc() {
    test_intersects!(
        i!(RC[0.0]);
        [
            (i!(RC[-2.0]), |z| assert_eq!(z.unwrap(), i!(RC[-2.0]))),
            (i!(RC[-1.0]), |z| assert_eq!(z.unwrap(), i!(RC[-1.0]))),
            (i!(RC[-0.5]), |z| assert_eq!(z.unwrap(), i!(RC[-0.5]))),
            (i!(RC[0.0]), |z| assert_eq!(z.unwrap(), i!(RC[0.0]))),
            (i!(RC[0.5]), |z| assert_eq!(z.unwrap(), i!(RC[0.0]))),
            (i!(RC[1.0]), |z| assert_eq!(z.unwrap(), i!(RC[0.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z.unwrap(), i!(RC[0.0])))
        ]
    );
}
