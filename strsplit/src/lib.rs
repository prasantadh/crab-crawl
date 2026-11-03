#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        // the cool thing here is
        // Option<&str> is Copy so self.remainder is copied out
        //      -> &self is released immediately
        // .and_then takes by value not by reference so remainder is not holding &self
        //      but just consuming the copy made earlier
        self.remainder.and_then(|remainder| {
            if let Some(position) = remainder.find(self.delimiter) {
                self.remainder = Some(&remainder[(position + self.delimiter.len())..]);
                Some(&remainder[..position])
            } else {
                self.remainder.take()
            }
        })
    }
}

/* other notes:
* non-lexical lifetimes: a borrow ends at its last use, not at the end of the enclosing scope.
* borrow splitting: &mut self.a and &self.b can co-exist
* precise/disjoint closure captures: a closure captures (*self).delimiter and
*       (*self).remainder individually instead of *self.
*/

// this function does not currently compile because StrSplit has same lifetime
// for both remainder and delimiter. and format! returns a string in this scope
// that cannot be returned as a reference.
// One solution is to split the lifetime of the haystack from needle.
// another is to implement the trait for delimiter (where the video ends)
// fn until_char(s: &str, c: char) -> &str {
//     StrSplit::new(s, &format!("{}", c))
//         .next()
//         .expect("StrSplit always returns at least one item")
// }
//
#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn empty_tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
//
// #[test]
// fn until_char_test() {
//     assert_eq!(until_char("helloworld!", 'o'), "hell");
// }
