# Traits
// Explanation of trait.
* Trait is very similar to interface in Java or Dotnet.
* 
## Trait Basic use

```
pub trait A {
    fn DisplayName(&self) -> String;
    fn DisplayNameWithSentense(&self) -> String{
        return format!("My name is  {}.)", self.DisplayName());
    }
}

#[derive(Debug)]
struct Person { FirstName: String, LastName: String }

impl A for Person {
    
}
```

