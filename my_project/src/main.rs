struct FilterCondition {
    value: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.value
    }
}

fn custom_filter<T>(collection: &Vec<T>, condition: &FilterCondition) -> Vec<T>
where
    T: Clone + PartialEq,
{
    collection
        .iter()
        .cloned()
        .filter(|item| condition.is_match(item))
        .collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let condition = FilterCondition { value: 3 };

    let filtered_numbers = custom_filter(&numbers, &condition);

    println!("Filtered numbers: {:?}", filtered_numbers);
}

