#[derive(Debug, Walkable)]
struct S {
    x: E,
    y: (E, E),
    z: O,
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
