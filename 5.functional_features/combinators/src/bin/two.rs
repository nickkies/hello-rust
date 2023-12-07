use std::collections::HashMap;

fn main() {}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

#[allow(dead_code)]
fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

#[allow(dead_code)]
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.values().filter(|p| *p == &value).count()
}

#[allow(dead_code)]
fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

#[allow(dead_code)]
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection.iter().map(|h| count_iterator(h, value)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_equals_for() {
        let map = get_map();
        assert_eq!(
            count_for(&map, Progress::Complete),
            count_iterator(&map, Progress::Complete)
        )
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = get_vec_map();
        assert_eq!(
            count_collection_for(&collection, Progress::Complete),
            count_collection_iterator(&collection, Progress::Complete)
        )
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        HashMap::from([
            ("variables1".to_string(), Complete),
            ("functions1".to_string(), Complete),
            ("hashmap1".to_string(), Complete),
            ("arc1".to_string(), Some),
            ("as_ref_mut".to_string(), None),
            ("from_str".to_string(), None),
        ])
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let other = HashMap::from([
            ("variables2".to_string(), Complete),
            ("functions2".to_string(), Complete),
            ("if1".to_string(), Complete),
            ("from_into".to_string(), Some),
            ("try_from_into".to_string(), None),
        ]);

        vec![get_map(), other]
    }
}
