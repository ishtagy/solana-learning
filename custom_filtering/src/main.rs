struct FilterCondition {
    value: i32,
}

impl FilterCondition {
    fn is_match(&self, num: &i32) -> bool {
        if self.value == *num {
            true
        } else {
            false
        }
    }
}

fn custom_filter(list: Vec<i32>, my_value: &FilterCondition) -> Vec<i32> {
    list.into_iter().filter(|x| my_value.is_match(x)).collect()
}

fn main() {
    let nums: Vec<i32> = vec![4, 3, 4, 1, 2, 5, 4, 4];
    let filter = FilterCondition { value: 4 };

    let result = custom_filter(nums, &filter);

    println!("Result: {:?}", result);
}
