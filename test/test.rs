#[derive(Debug, Walkable)]
struct S<T> {
    x: E,
    y: (E, E),
    z: O,
    t: T,
}

#[derive(Debug)]
struct O {
    i: i32,
}

#[derive(Debug, Walkable)]
enum E {
    Sa(S),
    St(O, Box<S>, O),
    Se(Box<E>),
}

type Foo = S<i32>;

#[derive(Debug, Walkable)]
struct A {
    a: Foo,
}
