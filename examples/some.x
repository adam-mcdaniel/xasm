

true = 1
false = 0

class NoneType {}

none = NoneType()


class Option {
    fn some(self, object) {
        self.object = object
        self.is_something = true
        self
    }

    fn none(self) {
        self.object = none
        self.is_something = false
        self
    }

    fn and_then(self, lambda) {
        if self.is_something {
            result = lambda(self.object)
            if result {
                self.object = result
            } else { self.is_something = false }
        } else {}

        self
    }

    fn or_else(self, instead) {
        if self.is_something { self.object }
        else { instead }
    }
}

fn Some(val) {
    (Option()).some(val)
}

fn None() {
    (Option()).none()
}





divide = fn(b) { fn(a) { div(a, b) } }
multiply = fn(b) { fn(a) { mul(a, b) } }
power = fn(base) {
    fn(power) {
        total = 1
        while power {
            total = mul(total, base)
            power = sub(power, 1)
        }
        total
    }
}

square = fn(n) { (power(n))(2) }

n = Some(1.000000000000001)

counter = 59
while counter {
    counter = sub(counter, 1)
    n = n.and_then(square)
    println(
        n.or_else("bad calculation")
    )
}
