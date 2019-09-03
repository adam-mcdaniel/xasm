
// Define print function
fn print(object) {
    // If `object` has a `to_str` attribute
    if object.to_str {
        // print the result of that method
        println(object.to_str())
    } else {
        // print the raw object
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
