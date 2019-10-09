println = @println;

class Object {
    fn new(self, value) {
        self.value = value;
        self
    }
}

obj = (Object()).new(5);
println(obj.value)
