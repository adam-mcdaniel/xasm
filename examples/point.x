
// Represents a point on a plane
class Point {
    fn new(self, x, y) {
        self.goto(x, y)
        self
    }

    fn move(self, x, y) {
        self.goto(
            add(self.x, x),
            add(self.y, y)
        )
    }

    fn goto(self, x, y) {
        self.x = x
        self.y = y
    }
}


sp = new(Point, 3, 5)
println(sp)

sp.move(-1, -3)
println(sp)