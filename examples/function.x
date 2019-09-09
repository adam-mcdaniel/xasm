
multiply = fn(n) {
    fn(m) {
        mul(n, m)
    }
}

double = multiply(2)
triple = multiply(3)


fn True(a) {
    fn(b) { a }
}

fn False(a) {
    fn(b) { b }
}

fn If(c) {
    fn(a) {
        fn(b) {
            (c(a))(b)
        }
    }
}

println(
    ((If(False))(1))(2)
)


println(double(5))
println(triple(5))