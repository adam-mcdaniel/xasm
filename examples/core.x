print = @print;
new = @new;
eq = @eq;
not = @not;
add = @add;
sub = @sub;
mul = @mul;
div = @div;
rem = @rem;
len = @len;
not = @not;


true = 1
false = 0

module = dict


core = module()

fn print(value) {
    if value.display {
        print(value.display())
    } else {
        print(value)
    }
}


fn println(value) {
    print(value)
    print("\n")
}

fn eq(a, b) {
    if a.eq { a.eq(b) }
    else { eq(a, b) }
}

fn neq(a, b) {
    not(eq(a, b))
}

fn discard(val) {
    if val { discard() }
}

class core["String"] {
    fn new(self, value) {
        self.value = value
        self
    }

    fn len(self) {
        counter = 1
        while (self.value)[counter] {
            counter = add(counter, 1)
        }

        counter
    }

    fn eq(self, string) {
        eq(self.value, string.value)
    }

    fn push(self, ch) {
        self.value = add(self.value, ch)
    }

    fn index(self, n) {
        (self.value)[n]
    }

    fn clear(self) {
        self.value = ""
    }

    fn pop(self) {
        result = self
        result.clear()

        counter = 0
        while neq(counter, sub(self.len(), 1)) {
            result.push(self.index(counter))
            counter = add(counter, 1)
        }

        self.index(sub(self.len(), 1))
        self.value = result.value
    }

    fn display(self) { self.value }
}


class core["Bool"] {
    fn new(self, value) {
        self.value = value
        self
    }

    fn and(self, rhs) {
        result = self
        result.value = mul(self.value, rhs.value)
        result
    }

    fn or(self, rhs) {
        result = self
        result.value = add(self.value, rhs.value)
        if eq(result.value, 2) {
            result.value = 1
        }
        result
    }

    fn not(self) {
        result = self
        result.value = not(result.value)
        result
    }

    fn bool(self) {
        self.value
    }

    fn display(self) {
        if self.value { "true" }
        else { "false" }
    }
}


class core["Number"] {
    fn new(self, value) {
        self.value = value
        self
    }

    fn dec(self) { self.value = sub(self.value, 1) }
    fn inc(self) { self.value = add(self.value, 1) }

    fn add_eq(self, number) {
        self.value = add(self.value, number.value)
    }

    fn add(self, number) {
        result = self
        result.add_eq(number)
        result
    }

    fn sub_eq(self, number) {
        self.value = sub(self.value, number.value)
    }

    fn sub(self, number) {
        result = self
        result.sub_eq(number)
        result
    }

    fn mul_eq(self, number) {
        self.value = mul(self.value, number.value)
    }

    fn mul(self, number) {
        result = self
        result.mul_eq(number)
        result
    }

    fn div_eq(self, number) {
        self.value = div(self.value, number.value)
    }

    fn div(self, number) {
        result = self
        result.div_eq(number)
        result
    }

    fn mod_eq(self, number) {
        self.value = rem(self.value, number.value)
    }

    fn mod(self, number) {
        result = self
        result.mod_eq(number)
        result
    }

    fn pow_eq(self, number) {
        base = self
        while sub(number.value, 1) {
            self.mul_eq(base)
            number.dec()
        }
    }

    fn pow(self, number) {
        result = self
        result.pow_eq(number)
        result
    }

    fn square_eq(self) {
        arg = self
        arg.value = 2
        self.pow_eq(arg)
    }

    fn square(self, number) {
        result = self
        result.square_eq()
        result
    }

    fn display(self) { self.value }
}



test = new(core["String"], "testing!")
copy = test

println(test)
println(eq(copy, test))

test.push("abc")
discard(test.pop())
println(test)

println(eq(copy, test))



a = new(core["Bool"], true)
b = new(core["Bool"], false)


println(a.and(b.not()))




n = new(core["Number"], 2)
m = new(core["Number"], 4)

println(m.square())