

mod = dict
core = mod()
core["io"] = mod()


fn core["io"]["print"](value) {
    (@print)(value)
}

fn core["io"]["println"](value) {
    (core["io"]["print"])(value)
    (core["io"]["print"])("\n")
}

core["io"]["println"]("testing")