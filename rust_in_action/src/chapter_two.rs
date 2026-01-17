// Explicit lifetime annotation
pub fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

// Generic function
pub fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}