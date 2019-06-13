// run-pass

#![feature(associated_type_defaults)]

macro_rules! overload {
    ($a:expr, $b:expr) => {
        overload::overload2($a, $b)
    };
    ($a:expr, $b:expr, $c:expr) => {
        overload::overload3($a, $b, $c)
    }
}

fn main() {
    let r = overload!(42, true);
    println!("-> {:?}", r);

    let r = overload!("Hello world", 13.0);
    println!("-> {:?}", r);

    let r = overload!(42, true, 42.5);
    println!("-> {:?}", r);

    let r = overload!("Hello world", 13.0, 42);
    println!("-> {:?}", r);
}

mod overload {
    pub trait Overload {
        // type R;
        type R = ();
        fn overload(self) -> Self::R;
    }

    // overloads for 2 args
    impl Overload for (i32, bool) {
        // type R = ();
        fn overload(self) /*-> Self::R*/ {
            let (a, b) = self; // destructure args
            println!("i32 and bool {:?}", (a, b));
        }
    }
    impl<'a> Overload for (&'a str, f32) {
        type R = f32;
        fn overload(self) -> Self::R {
            let (a, b) = self; // destructure args
            println!("&str and f32 {:?}", (a, b));
            b
        }
    }

    // overloads for 3 args
    impl Overload for (i32, bool, f32) {
        // type R = ();
        fn overload(self) /*-> Self::R*/ {
            let (a, b, c) = self; // destructure args
            println!("i32 and bool and f32 {:?}", (a, b, c));
        }
    }
    impl<'a> Overload for (&'a str, f32, i32) {
        type R = i32;
        fn overload(self) -> Self::R {
            let (a, b, c) = self; // destructure args
            println!("&str and f32 and i32: {:?}", (a, b, c));
            c
        }
    }

    // overloads for more args
    // ...

    pub fn overload2<R, A, B>(a: A, b: B) -> R where (A, B): Overload<R = R> {
        (a, b).overload()
    }

    pub fn overload3<R, A, B, C>(a: A, b: B, c: C) -> R where (A, B, C): Overload<R = R> {
        (a, b, c).overload()
    }
}
