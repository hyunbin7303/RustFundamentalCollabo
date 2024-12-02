# Traits
* Trait is very similar to interface in Java or Dotnet.
* `A trait defines the functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way.` 
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
## Traits as Parameters






## Trait bound Syntax



## Specifying multiple trait bounds with the + Syntax
Sepcify more than one trait bound.
`pub fn notify(item: &(impl Summary + Display))` or `pub fn notify<T: Summary + Display>(item: &T)`
With the two trait bound specified, the body of notify can call summarize and use {} to format item.


## Clearer trait bounds with where Clauses

Replace this
`fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {` 
to this,
```
fn some_function<T,U>(t: &T, u: &U) -> i32 
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

## Returning types that implement traits
Using `impl Trait` syntax for the return. 
```
fn returns_summarizable() -> impl Summary{
    Tweet {
        ~~~
    }
}

```

## using Trait bounds to conditionally implement methods.
* By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.

