

fn print(object) {
    if (object.to_str) {
        println(object.to_str())
    } else {
        println(object)
    }
}

print("string")

class Test {
    fn new(self) {
        self
    }

    fn to_str(self) {
        "TESTING!!!"
    }
}


print(new(Test))
