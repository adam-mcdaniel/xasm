
// Random number generator class
class Rand {
    fn new(self, seed) {
        self.seed = seed
        self.output = seed
        self.seed = self.generate()

        // Return initialized self
        self
    }

    fn generate(self) {
        // Store old output
        old_output = self.output
        // Get new output
        self.output = rem(mul(self.output, self.seed), 1918279)
        // Make old output new seed
        self.seed = rem(old_output, 10981823)

        // Return output
        self.output
    }
}


r = new(Rand, 9)

// Get random number between [min, max)
fn randint(min, max) {
    add(
        min,
        rem(r.generate(), sub(max, min))
    )
}


n = 10000
while n {
    println(randint(0, 4))
    n = sub(n, 1)
}