

class Object {
    fn new(self, value) {
        self.value = value
        self
    }
}



obj = new(Object, 5)
println(obj.value)