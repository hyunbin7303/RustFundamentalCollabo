# Function Declaration ownership
* fn func_name(*variable name*: String) takes a String and owns within a function, and the variable dies inside.
* fn func_name(*variable name*: &String) borrows a String.
* fn func_name(*variable name*: &mut String) borrows a String and can modify.


## How Ownership works
* ONLY single owner at a time.
* Ownership is transferred using assignment or return statements.
* Rust has a borrowing concept to allow temporary use of owned values.
* Will be dropped when owner goes out of scope.



## When do you need to use the owned string?
(Pass the ownership to the function)
* Returning string from the function
* Passing it to another thread
* Building at runtime.

```
fn return_capital_characters(input: &str) -> String {
    input.to_ascii_uppercase()
}
```
