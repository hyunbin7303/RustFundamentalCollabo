# String or &str Type investigation
All string types in Rust are always guaranteed to be valid UTF-8 encoded bit of text.

## String Object.
* Mutable, Heap-allocated growable UTF-8 string buffer.
* The memory must be requested from the memory allocator at runtime, so it is unknown at compile time.
* String is made up of three parts : A Pointer, a length, and a capacity.
```
pub struct String {
	vec: Vec<u8>
}
```
So When we do `let s1 = String::from("Hello");`
On the left side, we keep above data(pointer, length and capacity) on the stack.
On the right side, we keep the contents on the heap.
* String is an owned type that needs to be allocated.
* When String object goes out of scope, Rust calls a function drop automatically at the closing curly bracket.

### Usage of String
* Input string from the user.

## String literal ( &'static str)
* Immutable sequence of UTF-8 bytes of dynamic Length(so size is unknown)
* Known as **string slice** pointing to that specific point of the binary(or we can say, pointer to a block of memory). this means that str most commonly appears as &str: It is a reference to a sequence of UTF-8 bytes.
* It is a data type that doesn't have an owner.
* The size of &str is fixed, which means that it cannot be resized(Immutable reference)
* The text is hardcoded directly into the final executable(read-only memory of the binary), so contents at compile time -> Very fast and efficient, and that's why we put `'static` reference to it.




### Question
```
fn return_str() -> &'static str {
    let name = String::from("Testing With Kevin");
    let name_ref = &country;
    name_ref
}
```
What is going to be an output for the above code(or possibly what kind of errors)?






