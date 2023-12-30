/// ```rust
/// macro_rules! name {
///     rule0 ;
///     rule1 ;
///     // ...
///     ruleN ;  
/// }
///
/// (matcher) => { expansion aka transcriber }
/// ```
#[macro_export]
macro_rules! hello {
    () => {
        println!("Hi there!");
    };
}
