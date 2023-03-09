mod inaccessible;
pub mod nested;

pub struct OpenBox<T> {
    pub contents: T,
}

pub struct ClosedBox<T> {
    contents: T,
}

impl<T> ClosedBox<T> {
    pub fn new(contents: T) -> ClosedBox<T> {
        ClosedBox { contents: contents }
    }
}

pub fn function() {
    println!("called `my::function()`");
}

mod cool {
    pub fn function() {
        println!("called `my::cool::function()`");
    }
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}

pub fn indirect_call() {
    print!("called `my::indirect_call()`, that\n> ");

    self::function();
    function();

    super::function();

    self::cool::function();

    {
        use crate::cool::function as root_function;
        root_function();
    }
}
