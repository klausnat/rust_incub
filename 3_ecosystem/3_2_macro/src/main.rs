use std::collections::BTreeMap;

// Declarative Macros with macro_rules!
// User can create instances of BTreeMap, using this macros in different ways (using curly brackets, or using square brackets)
// using bold arrow: key => value, or using pairs: (key, value)
#[macro_export]
macro_rules! btreemap {
    // Empty map, curly brackets
    () => {
        BTreeMap::new()
    };

    // Empty map, square brackets []
    () => [
        BTreeMap::new()
    ];

    // With trailing comma: btreemap!{k1 => v1, k2 => v2,}
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map = BTreeMap::new();
            $(map.insert($key, $value);)+
            map
        }
    };

    // Without trailing comma: btreemap!{k1 => v1, k2 => v2}
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = BTreeMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };

    // From array, as pairs with trailing coma: btreemap![(k1,v1), (k2, v2),]
    ($(($key:expr, $value:expr)),+ $(,)?) => [
        {
            let mut map = BTreeMap::new();
            $(map.insert($key, $value);)+
            map
        }
    ];
}

fn main() {
    println!("implement macro")
}

// test Declarative macro 
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_empty_map_curly_brackets() {
        let map: BTreeMap<i32, i32> = btreemap! {};
        assert!(map.is_empty());
    }

    #[test]
    fn test_empty_map_square_brackets() {
        let map: BTreeMap<i32, i32> = btreemap![];
        assert!(map.is_empty());
    }

    #[test]
    fn test_single_entry_curly_brackets() {
        let map = btreemap! {1 => "one"};
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1), Some(&"one"));
    }

    #[test]
    fn test_single_entry_square_brackets_pairs() {
        let map = btreemap![(1, "first_entry"),];
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1), Some(&"first_entry"));
    }

    #[test]
    fn test_multiple_entries_curly_brackets() {
        let map = btreemap! {
            3 => "three",
            1 => "one",
            2 => "two"
        };

        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&3), Some(&"three"));
    }

    #[test]
    fn test_multiple_entries_square_brackets_pairs() {
        let map = btreemap![
            3 => "three",
            1 => "one",
            2 => "two"
        ];

        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&3), Some(&"three"));
    }

    #[test]
    fn test_char_int() {
        let map = btreemap! {
            "a" => 1,
            "b" => 2,
            "c" => 3
        };

        assert_eq!(map.len(), 3);
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), Some(&2));
        assert_eq!(map.get("c"), Some(&3));
    }

    #[test]
    fn test_expressions_as_keys_and_values() {
        let x = 5;
        let y = 10;

        let map = btreemap! {
            x + 1 => y * 2,
            x - 1 => y / 2
        };

        assert_eq!(map.get(&6), Some(&20));
        assert_eq!(map.get(&4), Some(&5));
    }

    #[test]
    fn test_duplicate_keys() {
        let map = btreemap! {
            1 => "first",
            1 => "second", // This should overwrite the previous value
            2 => "other"
        };

        assert_eq!(map.len(), 2); // Only 2 unique keys
        assert_eq!(map.get(&1), Some(&"second")); // Last value wins
        assert_eq!(map.get(&2), Some(&"other"));
    }

    #[test]
    fn test_sorted_order() {
        let map = btreemap! {
            5 => "e",
            1 => "a",
            3 => "c",
            4 => "d",
            2 => "b"
        };

        // Collect keys in iteration order (should be sorted)
        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys, vec![&1, &2, &3, &4, &5]);

        // Collect values in iteration order
        let values: Vec<_> = map.values().collect();
        assert_eq!(values, vec![&"a", &"b", &"c", &"d", &"e"]);
    }
}
