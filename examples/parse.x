true = 1
false = 0

class Error {
    fn new(self, expected, actual) {
        self.expected = expected
        self.actual = actual
        
        self.IS_ERROR = true

        self
    }
}


fn is_error(e) { if e.IS_ERROR { true } else { false } }
fn not_error(val) { not(is_error(val)) }

class Parser {
    fn new(self, parser) {
        self.parser = parser
        
        self
    }

    fn parse(self, input) {
        (self["parser"])(input)
    }

    fn map(self, mapper) {
        copy = self
        copy.new(fn(input) {
            result = self.parse(input)
            if not_error(result) {
                mapper(result)
            } else {
                result
            }
        })
    }

    fn prefixes(self, operand) {
        copy = self
        copy.new(fn(input) {
            remaining = (self.parse(input))[1]
            operand.parse(remaining)
        })
    }

    fn suffix(self, operand) {
        copy = self
        copy.new(fn(input) {
            result = self.parse(input)
            consumed = result[0]
            remaining = result[1]
            remaining = operand.parse(remaining)

            push(
                push(list(), consumed),
                remaining
            )
        })
    }

    fn and(self, operand) {
        copy = self
        copy.new(fn(input) {
            first_result = self.parse(input)

            if not_error(first_result) {
                first_consumed = first_result[0]
                remaining = first_result[1]

                second_result = operand.parse(remaining)
                if not_error(second_result) {
                    second_consumed = second_result[0]
                    remaining = second_result[1]

                    push(
                        push(
                            list(),
                            push(
                                push(list(), first_consumed),
                                second_consumed
                            )
                        ),
                        remaining
                    )
                } else {second_result}
            } else {first_result}
        })
    }

    fn or(self, operand) {
        copy = self
        copy.new(fn(input) {
            result = self.parse(input)
            if not_error(result) { result }
            else { operand.parse(input) }
        })
    }
}


fn range(iter, a, b) {
    result = list()
    counter = b
    while sub(counter, sub(a, 1)) {
        result = push(result, iter[counter])
        counter = sub(counter, 1)
    }

    result
}


fn join(iter, sep) {
    result = ""
    counter = len(iter)
    while counter {
        counter = sub(counter, 1)
        result = add(result, iter[counter])
        if not(eq(counter, 0)) {
            result = add(result, sep)
        }
    }

    result
}


fn strncmp(a, b, n) {
    continuing = true
    result = true

    counter = n
    while continuing {
        if eq(counter, 0) {
            continuing = false
        } else {
            result = mul(result, eq(a[counter], b[counter]))
        }
        counter = sub(counter, 1)
    }

    result
}

fn strlen(str) {
    counter = 1
    while str[counter] {
        counter = add(counter, 1)
    }

    counter
}

fn take(num) {
    new(Parser, fn(input) {
        push(
            push(
                list(),
                join(range(input, 0, sub(num, 1)), "")
            ),
            join(range(input, num, sub(strlen(input), 1)), "")
        )
    })
}

fn seq(string) {
    new(Parser, fn(input) {
        if strncmp(string, input, sub(strlen(string), 1)) {
            (take(strlen(string))).parse(input)
        } else {
            new(Error, string, input)
        }
    })
}

sym = seq

fn any() {
    take(1)
}

fn main() {
    println(
        join(range("testing", 0, 3), ":")
    )

    println(
        strncmp("testw", "testing", 4)
    )

    println(
        ((seq("no way")).or((seq("hey")).and(seq(" there"))))
            .parse("hey there dude")
    )

    println(strlen("four"))
}


main()