// Copyright 2015 Joe Neeman.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate memmem;
use memmem::{Searcher, TwoWaySearcher};

macro_rules! search(
    ($name:ident, $needle:expr, $hay:expr, $result:expr) => (
        #[test]
        fn $name() {
            let searcher = TwoWaySearcher::new($needle.as_bytes());
            println!("searching for {:?} in {:?}", $needle, $hay);
            assert_eq!(searcher.search_in($hay.as_bytes()), $result);
        }
    );
);

search!(periodic_1, "aaaaaaaa", "aaaaaaabaaaaaaabaaaaaaabaaaaaaaa", Some(24));
search!(periodic_2, "aaaaaaaa", "aaaaaaabaaaaaaabaaaaaaabaaaaaaab", None);
search!(periodic_3, "abcabc", "abcabdabcabdabcabdabcabc", Some(18));
search!(periodic_4, "abcabc", "abcabdabcabdabcabdabcabd", None);

search!(aperiodic_1, "dog", "The quick brown fox jumped over the lazy dog.", Some(41));
search!(aperiodic_2, "doggy", "The quick brown fox jumped over the lazy dog.", None);