

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
            self.object = lambda(self.object)
            println(self.object)
            if not(is_error(self.object)) {}
            else { self.is_something = false }
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


n = Some(0)

n = n.and_then(fn(a) {
    if not(eq(a, 0)) {
        div(2, a)
    }
})

println(
    n.or_else("bad calculation")
)
