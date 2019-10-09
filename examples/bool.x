eq = @eq;
new = @new;
not = @not;
println = @println;


fn neq(a, b) {
    not(eq(a, b))
}


println(eq(5, 5))
println(eq(5, 4))

println(neq(5, 4))
println(neq(5, 5))

println(eq("test", "test"))
println(eq("test", "testing"))

println(neq("test", "testing"))
println(neq("test", "test"))


class Object {
    fn new(self, a) {
        self.value = a
        self
    }
}

println(
    eq(
        new(Object, 1),
        new(Object, 1)
    )
)

println(
    eq(
        new(Object, 1),
        new(Object, 2)
    )
)