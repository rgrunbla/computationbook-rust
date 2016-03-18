use syntax::Node;
use environment::Environment;

pub struct Machine {
    environment: Environment,
    expression: Box<Node>,
}

impl Machine {
    pub fn new(expression: Box<Node>, environment: Environment) -> Machine {
        Machine{
            expression: expression,
            environment: environment,
        }
    }

    pub fn new_with_empty_env(expression: Box<Node>) -> Machine {
        Machine {
            expression: expression,
            environment: Environment::new(),
        }
    }

    pub fn step(&mut self) {
        self.expression = self.expression.reduce(&mut self.environment);
    }

    pub fn run(&mut self) {
        while self.expression.reducible() {
            println!("{}", self.expression);
            self.step();
        }
        println!("{}", self.expression);
    }
}
