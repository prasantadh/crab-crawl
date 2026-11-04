#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($($element:expr),+ $(,)?) => {{
        let mut vs = Vec::new();
        $(vs.push($element);)+
        vs
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty())
}

#[test]
fn single() {
    let x: Vec<u32> = avec!(42);
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec!(42, 43);
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trailing() {
    let x: Vec<u32> = avec!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27
    );
    assert!(!x.is_empty());
    assert_eq!(x.len(), 27);
}

#[test]
fn clone_2() {
    let x: Vec<u32> = avec!(42; 5);
    assert!(!x.is_empty());
    assert_eq!(x.len(), 5);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}
