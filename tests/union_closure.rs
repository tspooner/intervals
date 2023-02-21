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

macro_rules! test_ucs {
    ($x:expr; [$(($y:expr, |$z:ident| $test:expr)),+]) => {{
        let x = $x;
        $({
            let y = $y;
            let $z = x.union_closure(y);

            { $test }

            let $z = y.union_closure(x);

            { $test }
        })+
    }};
}

macro_rules! test_ucs_all {
    ($left:ident; $right:ident) => {
        test_ucs!(
            i!($left[0.0, 1.0]);
            [
                (i!($right[-2.0, -1.0]), |z| assert_eq!(z, i!(Closed[-2.0, 1.0]))),
                (i!($right[-1.0, 0.0]), |z| assert_eq!(z, i!(Closed[-1.0, 1.0]))),
                (i!($right[-0.5, 0.5]), |z| assert_eq!(z, i!(Closed[-0.5, 1.0]))),
                (i!($right[0.0, 1.0]), |z| assert_eq!(z, i!(Closed[0.0, 1.0]))),
                (i!($right[0.5, 1.5]), |z| assert_eq!(z, i!(Closed[0.0, 1.5]))),
                (i!($right[1.0, 2.0]), |z| assert_eq!(z, i!(Closed[0.0, 2.0]))),
                (i!($right[2.0, 3.0]), |z| assert_eq!(z, i!(Closed[0.0, 3.0])))
            ]
        );
    }
}

#[test]
fn unbounded() {
    test_ucs!(
        Interval::unbounded();
        [
            (i!(Closed[0.0, 1.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(Open[0.0, 1.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(LORC[0.0, 1.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(LCRO[0.0, 1.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(LO[0.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(LC[0.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[0.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[0.0]), |z| assert_eq!(z, Interval::unbounded()))
        ]
    );
}

#[test]
fn closed() {
    test_ucs_all!(Closed; Closed);
    test_ucs_all!(Closed; Open);
    test_ucs_all!(Closed; LORC);
    test_ucs_all!(Closed; LCRO);

    test_ucs!(
        i!(Closed[0.0, 1.0]);
        [
            (i!(LC[-2.0]), |z| assert_eq!(z, i!(LC[-2.0]))),
            (i!(LC[-1.0]), |z| assert_eq!(z, i!(LC[-1.0]))),
            (i!(LC[-0.5]), |z| assert_eq!(z, i!(LC[-0.5]))),
            (i!(LC[0.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[0.5]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[1.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[2.0]), |z| assert_eq!(z, i!(LC[0.0])))
        ]
    );

    test_ucs!(
        i!(Closed[0.0, 1.0]);
        [
            (i!(LO[-2.0]), |z| assert_eq!(z, i!(LC[-2.0]))),
            (i!(LO[-1.0]), |z| assert_eq!(z, i!(LC[-1.0]))),
            (i!(LO[-0.5]), |z| assert_eq!(z, i!(LC[-0.5]))),
            (i!(LO[0.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[0.5]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[1.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[2.0]), |z| assert_eq!(z, i!(LC[0.0])))
        ]
    );

    test_ucs!(
        i!(Closed[0.0, 1.0]);
        [
            (i!(RC[-2.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[-1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[-0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[0.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z, i!(RC[2.0])))
        ]
    );

    test_ucs!(
        i!(Closed[0.0, 1.0]);
        [
            (i!(RO[-2.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[-1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[-0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[0.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z, i!(RC[2.0])))
        ]
    );
}

#[test]
fn open() {
    test_ucs_all!(Open; Open);
    test_ucs_all!(Open; LORC);
    test_ucs_all!(Open; LCRO);

    test_ucs!(
        i!(Open[0.0, 1.0]);
        [
            (i!(LC[-2.0]), |z| assert_eq!(z, i!(LC[-2.0]))),
            (i!(LC[-1.0]), |z| assert_eq!(z, i!(LC[-1.0]))),
            (i!(LC[-0.5]), |z| assert_eq!(z, i!(LC[-0.5]))),
            (i!(LC[0.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[0.5]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[1.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[2.0]), |z| assert_eq!(z, i!(LC[0.0])))
        ]
    );

    test_ucs!(
        i!(Open[0.0, 1.0]);
        [
            (i!(LO[-2.0]), |z| assert_eq!(z, i!(LC[-2.0]))),
            (i!(LO[-1.0]), |z| assert_eq!(z, i!(LC[-1.0]))),
            (i!(LO[-0.5]), |z| assert_eq!(z, i!(LC[-0.5]))),
            (i!(LO[0.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[0.5]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[1.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[2.0]), |z| assert_eq!(z, i!(LC[0.0])))
        ]
    );

    test_ucs!(
        i!(Open[0.0, 1.0]);
        [
            (i!(RC[-2.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[-1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[-0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[0.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z, i!(RC[2.0])))
        ]
    );

    test_ucs!(
        i!(Open[0.0, 1.0]);
        [
            (i!(RO[-2.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[-1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[-0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[0.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z, i!(RC[2.0])))
        ]
    );
}

#[test]
fn lorc() {
    test_ucs_all!(LORC; LORC);
    test_ucs_all!(LORC; LCRO);

    test_ucs!(
        i!(LORC[0.0, 1.0]);
        [
            (i!(LC[-2.0]), |z| assert_eq!(z, i!(LC[-2.0]))),
            (i!(LC[-1.0]), |z| assert_eq!(z, i!(LC[-1.0]))),
            (i!(LC[-0.5]), |z| assert_eq!(z, i!(LC[-0.5]))),
            (i!(LC[0.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[0.5]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[1.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[2.0]), |z| assert_eq!(z, i!(LC[0.0])))
        ]
    );

    test_ucs!(
        i!(LORC[0.0, 1.0]);
        [
            (i!(LO[-2.0]), |z| assert_eq!(z, i!(LC[-2.0]))),
            (i!(LO[-1.0]), |z| assert_eq!(z, i!(LC[-1.0]))),
            (i!(LO[-0.5]), |z| assert_eq!(z, i!(LC[-0.5]))),
            (i!(LO[0.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[0.5]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[1.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[2.0]), |z| assert_eq!(z, i!(LC[0.0])))
        ]
    );

    test_ucs!(
        i!(LORC[0.0, 1.0]);
        [
            (i!(RC[-2.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[-1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[-0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[0.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z, i!(RC[2.0])))
        ]
    );

    test_ucs!(
        i!(LORC[0.0, 1.0]);
        [
            (i!(RO[-2.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[-1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[-0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[0.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z, i!(RC[2.0])))
        ]
    );
}

#[test]
fn lcro() {
    test_ucs_all!(LCRO; LCRO);

    test_ucs!(
        i!(LCRO[0.0, 1.0]);
        [
            (i!(LC[-2.0]), |z| assert_eq!(z, i!(LC[-2.0]))),
            (i!(LC[-1.0]), |z| assert_eq!(z, i!(LC[-1.0]))),
            (i!(LC[-0.5]), |z| assert_eq!(z, i!(LC[-0.5]))),
            (i!(LC[0.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[0.5]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[1.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[2.0]), |z| assert_eq!(z, i!(LC[0.0])))
        ]
    );

    test_ucs!(
        i!(LCRO[0.0, 1.0]);
        [
            (i!(LO[-2.0]), |z| assert_eq!(z, i!(LC[-2.0]))),
            (i!(LO[-1.0]), |z| assert_eq!(z, i!(LC[-1.0]))),
            (i!(LO[-0.5]), |z| assert_eq!(z, i!(LC[-0.5]))),
            (i!(LO[0.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[0.5]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[1.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[2.0]), |z| assert_eq!(z, i!(LC[0.0])))
        ]
    );

    test_ucs!(
        i!(LCRO[0.0, 1.0]);
        [
            (i!(RC[-2.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[-1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[-0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[0.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z, i!(RC[2.0])))
        ]
    );

    test_ucs!(
        i!(LCRO[0.0, 1.0]);
        [
            (i!(RO[-2.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[-1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[-0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[0.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[0.5]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z, i!(RC[2.0])))
        ]
    );
}

#[test]
fn lo() {
    test_ucs!(
        i!(LO[0.0]);
        [
            (i!(LC[-2.0]), |z| assert_eq!(z, i!(LC[-2.0]))),
            (i!(LC[-1.0]), |z| assert_eq!(z, i!(LC[-1.0]))),
            (i!(LC[-0.5]), |z| assert_eq!(z, i!(LC[-0.5]))),
            (i!(LC[0.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[0.5]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[1.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LC[2.0]), |z| assert_eq!(z, i!(LC[0.0])))
        ]
    );

    test_ucs!(
        i!(LO[0.0]);
        [
            (i!(LO[-2.0]), |z| assert_eq!(z, i!(LC[-2.0]))),
            (i!(LO[-1.0]), |z| assert_eq!(z, i!(LC[-1.0]))),
            (i!(LO[-0.5]), |z| assert_eq!(z, i!(LC[-0.5]))),
            (i!(LO[0.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[0.5]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[1.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[2.0]), |z| assert_eq!(z, i!(LC[0.0])))
        ]
    );

    test_ucs!(
        i!(LO[0.0]);
        [
            (i!(RC[-2.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[-1.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[-0.5]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[0.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[0.5]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[1.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[2.0]), |z| assert_eq!(z, Interval::unbounded()))
        ]
    );

    test_ucs!(
        i!(LO[0.0]);
        [
            (i!(RO[-2.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[-1.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[-0.5]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[0.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[0.5]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[1.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[2.0]), |z| assert_eq!(z, Interval::unbounded()))
        ]
    );
}

#[test]
fn lc() {
    test_ucs!(
        i!(LC[0.0]);
        [
            (i!(LO[-2.0]), |z| assert_eq!(z, i!(LC[-2.0]))),
            (i!(LO[-1.0]), |z| assert_eq!(z, i!(LC[-1.0]))),
            (i!(LO[-0.5]), |z| assert_eq!(z, i!(LC[-0.5]))),
            (i!(LO[0.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[0.5]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[1.0]), |z| assert_eq!(z, i!(LC[0.0]))),
            (i!(LO[2.0]), |z| assert_eq!(z, i!(LC[0.0])))
        ]
    );

    test_ucs!(
        i!(LC[0.0]);
        [
            (i!(RC[-2.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[-1.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[-0.5]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[0.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[0.5]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[1.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RC[2.0]), |z| assert_eq!(z, Interval::unbounded()))
        ]
    );

    test_ucs!(
        i!(LC[0.0]);
        [
            (i!(RO[-2.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[-1.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[-0.5]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[0.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[0.5]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[1.0]), |z| assert_eq!(z, Interval::unbounded())),
            (i!(RO[2.0]), |z| assert_eq!(z, Interval::unbounded()))
        ]
    );
}

#[test]
fn ro() {
    test_ucs!(
        i!(RO[0.0]);
        [
            (i!(RC[-2.0]), |z| assert_eq!(z, i!(RC[0.0]))),
            (i!(RC[-1.0]), |z| assert_eq!(z, i!(RC[0.0]))),
            (i!(RC[-0.5]), |z| assert_eq!(z, i!(RC[0.0]))),
            (i!(RC[0.0]), |z| assert_eq!(z, i!(RC[0.0]))),
            (i!(RC[0.5]), |z| assert_eq!(z, i!(RC[0.5]))),
            (i!(RC[1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RC[2.0]), |z| assert_eq!(z, i!(RC[2.0])))
        ]
    );

    test_ucs!(
        i!(RO[0.0]);
        [
            (i!(RO[-2.0]), |z| assert_eq!(z, i!(RC[0.0]))),
            (i!(RO[-1.0]), |z| assert_eq!(z, i!(RC[0.0]))),
            (i!(RO[-0.5]), |z| assert_eq!(z, i!(RC[0.0]))),
            (i!(RO[0.0]), |z| assert_eq!(z, i!(RC[0.0]))),
            (i!(RO[0.5]), |z| assert_eq!(z, i!(RC[0.5]))),
            (i!(RO[1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z, i!(RC[2.0])))
        ]
    );
}

#[test]
fn rc() {
    test_ucs!(
        i!(RC[0.0]);
        [
            (i!(RO[-2.0]), |z| assert_eq!(z, i!(RC[0.0]))),
            (i!(RO[-1.0]), |z| assert_eq!(z, i!(RC[0.0]))),
            (i!(RO[-0.5]), |z| assert_eq!(z, i!(RC[0.0]))),
            (i!(RO[0.0]), |z| assert_eq!(z, i!(RC[0.0]))),
            (i!(RO[0.5]), |z| assert_eq!(z, i!(RC[0.5]))),
            (i!(RO[1.0]), |z| assert_eq!(z, i!(RC[1.0]))),
            (i!(RO[2.0]), |z| assert_eq!(z, i!(RC[2.0])))
        ]
    );
}
