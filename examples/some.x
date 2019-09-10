

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

            if self.object {}
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




five = Some(0)


five = five.and_then(
    fn(a) {
        if a { div(2, a) }
    }
)


println(
    five.or_else("bad calculation")
)