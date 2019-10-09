println = @println;
print = @print;
push = @push;
list = @list;
pop = @pop;
len = @len;
add = @add;
sub = @sub;
mul = @mul;
div = @div;
new = @new;


class List {
    fn new(self) {
        self.list = list()
        self
    }

    fn pop(self) {
        if len(self.list) {
            self.list = pop(self.list)
        }
    }

    fn push(self, value) {
        self.list = push(self.list, value)
    }

    fn len(self) {
        len(self.list)
    }

    fn index(self, n) {
        (self.list)[n]
    }

    fn map(self, lambda) {
        copy = self
        copy = copy.new()

        counter = 0
        while sub(self.len(), copy.len()) {
            copy.push(
                lambda(self.index(counter))
            )

            counter = add(counter, 1)
        }

        copy
    }

    fn filter(self, lambda) {
        copy = self
        copy = copy.new()

        counter = 0
        while sub(self.len(), counter) {
            value = self.index(counter)
            if lambda(value) {
                print("kept ")
                copy.push(value)
            } else {
                print("filtered ")
                println(value)
            }

            counter = add(counter, 1)
        }

        copy
    }
}





list = new(List)
list.push(0)
list.push(1)
list.push(2)
list.push(3)
list.push(4)
list.push(5)



println(list.filter(fn(n) { n }))
println(list)


double = fn(a) {
    print("mapping over value ")
    println(a)
    mul(a, 2)
}


doubled = list.map(double)

println(list)
println(doubled)