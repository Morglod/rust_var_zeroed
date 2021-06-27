# rust_var_zeroed

Create structs with rust without Default trait

Similar to C/C++ (except var_stack_zeroed is zeroed):
```cpp
struct MyStruct {
    int a: i32;
    MyStruct* ptr;
};

int main() {
    MyStruct my_var;
    memset(&my_var, 0, sizeof MyStruct);

    MyStruct* my_var2 = (MyStruct*)malloc(sizeof MyStruct);
    memset(&my_var2, 0, sizeof MyStruct);

    return 0;
}
```

```rs
struct MyStruct {
    a: i32,
    ptr: *mut MyStruct,
}

fn main() {
    // let mut my_var: &mut MyStruct
    var_stack_zeroed!(my_var, MyStruct);
    println!("{:?}", my_var.ptr); // -> 0x0

    // let mut my_var2: *mut MyStruct
    var_heap_zeroed!(my_var2, MyStruct);
    println!("{}", unsafe { (*my_var2).a }); // -> 0
}
```

Waiting for unstable Box features:  
https://doc.rust-lang.org/beta/std/boxed/struct.Box.html#method.new_uninit

## Install

in Cargo.toml):
```
[dependencies]
rust_var_zeroed = "0.1.0"
```