# String Type investigation

## String Object.
* Heap-allocated growable UTF-8 string buffer.
* Mutable.
* String is a pointer, with data on the heap.println!

## String literal
* Written as &str, also known as **string slice** pointing to that specific point of the binary.
* its type is &'static str. 
* The size of &str is fixed, which means that it cannot be resized(Immutable reference)
* It is a reference to a sequence of UTF-8 bytes.
* let my_var = "Hello" -> you create a &str. which is very fast.println!
