

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
}


l = new(List)

n = 10
while n {
    l.push(n)
    n = sub(n, 1)
}

println(l)