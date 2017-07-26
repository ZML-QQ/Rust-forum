fn function<'a>(x: &'a Type) { }

struct Struct<'a> { 
    x: &'a Type 
}

enum Enum<'a> { 
    Variant(&'a Type) 
}

impl<'a> Struct<'a> { 
    fn x<'a>(&self) -> &'a Type { 
        self.x 
    } 
}

impl<'a> Trait<'a> for Type { }
impl<'a> Trait for Type<'a> { }

fn function<F>(f: F) where for<'a> F: FnOnce(&'a Type) { }
struct Struct<F> where for<'a> F: FnOnce(&'a Type) { x: F }
enum Enum<F> where for<'a> F: FnOnce(&'a Type) { Variant(F) }
impl<F> Struct<F> where for<'a> F: FnOnce(&'a Type) { fn x(&self) -> &F { &self.x } }