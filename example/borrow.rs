pub fn foo() {
    let a = 15;
    let b = &a;

    println!("{a}, {b}");
}

// move error
// pub fn bar() {
//     struct NonCopy;

//     let a = NonCopy;

//     let b = a; // OK
//     let c = a; // KO
// }
