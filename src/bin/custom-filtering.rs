// Using rand module for generating a vector with random numbers
use rand::Rng;

// Define comparison operations
#[derive(Debug)]
enum Comparison {
    Equal,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

// Modify FilterCondition to include comparison type
struct FilterCondition<T> {
    value: T,
    comparison: Comparison,
}

// Implement methods for FilterCondition
// PartialOrd trait is needed for comparison between values
impl<T: PartialOrd> FilterCondition<T> {
    // Constructor for FilterCondition
    fn new(value: T, comparison: Comparison) -> Self {
        FilterCondition { value, comparison }
    }

    // Enhanced is_match method to handle different comparisons
    fn is_match(&self, item: &T) -> bool {
        match self.comparison {
            Comparison::Equal => item == &self.value,
            Comparison::GreaterThan => item > &self.value,
            Comparison::LessThan => item < &self.value,
            Comparison::GreaterThanOrEqual => item >= &self.value,
            Comparison::LessThanOrEqual => item <= &self.value,
        }
    }
}

// Custom filter function
fn custom_filter<T>(collection: &Vec<T>, condition: &FilterCondition<T>) -> Vec<T> 
    where 
    T: PartialOrd + Clone,
{
    let mut result = Vec::new();
    
    for item in collection {
        if condition.is_match(item) {
            result.push(item.clone());
        }
    }
    
    result
}

fn main() {
    let mut rng = rand::thread_rng();

    // Create a vector of random integers for testing
    let numbers: Vec<i32> = (0..10)
        .map(|_| rng.gen_range(1..=100))  // Generate a random number between 1 and 100
        .collect();

    // Test different comparisons
    let greater_than_five = FilterCondition::new(5, Comparison::GreaterThan);
    let less_than_twenty_three = FilterCondition::new(23, Comparison::LessThan);
    let equal_to_twelve = FilterCondition::new(12, Comparison::Equal);
    let greater_or_equal_seventy_eight = FilterCondition::new(78, Comparison::GreaterThanOrEqual);
    let less_then_or_equal_ten = FilterCondition::new(60, Comparison::LessThanOrEqual);
    
    // Apply filters and print results
    println!("Original numbers: {:?}", numbers);
    println!(
        "Numbers greater than 5: {:?}", 
        custom_filter(&numbers, &greater_than_five)
    );
    println!(
        "Numbers less than 23: {:?}", 
        custom_filter(&numbers, &less_than_twenty_three)
    );
    println!(
        "Numbers equal to 12: {:?}", 
        custom_filter(&numbers, &equal_to_twelve)
    );
    println!(
        "Numbers greater than or equal to 78: {:?}", 
        custom_filter(&numbers, &greater_or_equal_seventy_eight)
    );
    println!(
        "Numbers less than or equal to 60: {:?}", 
        custom_filter(&numbers, &less_then_or_equal_ten)
    )
}