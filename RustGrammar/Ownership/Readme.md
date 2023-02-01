# Function Declaration ownership
* fn func_name(*variable name*: String) takes a String and owns within a function, and the variable dies inside.
* fn func_name(*variable name*: &String) borrows a String.
* fn func_name(*variable name*: &mut String) borrows a String and can modify.



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
