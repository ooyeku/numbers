use crate::floats::complex_floats::Complex;


pub struct Equation {
    pub left: Complex,
    pub right: Complex,
    pub operation: char,
    pub sol: Complex,
}
// Complex Equation implementation solves the equation during initialization
impl Equation {
    pub fn new(left: Complex, right: Complex, operation: char) -> Equation {
        let sol = match operation {
            '+' => left.add(&right),
            '-' => left.subtract(&right),
            '*' => left.multiply(&right),
            '/' => left.divide(&right),
            _ => panic!("Invalid operation"),
        };
        Equation {
            left,
            right,
            operation,
            sol,
        }
    }

    pub fn get_sol(&mut self) -> &Complex {
        &self.sol
    }

    pub fn get_left(&mut self) -> &Complex {
        &self.left
    }

    pub fn get_right(&mut self) -> &Complex {
        &self.right
    }
}
#[cfg(test)]
mod test_equation {
    use super::*;

    #[test]
    fn test_new() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        let operation = '+';
        let equation = Equation::new(left, right, operation);
        assert_eq!(equation.left, Complex::new(1.0, 1.0));
        assert_eq!(equation.right, Complex::new(1.0, 1.0));
        assert_eq!(equation.sol, Complex::new(2.0, 2.0));
    }

    #[test]
    fn test_get_sol() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(2.0, 2.0));
    }

    #[test]
    fn test_get_left() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_left(), &Complex::new(1.0, 1.0));
    }

    #[test]
    fn test_get_right() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_right(), &Complex::new(1.0, 1.0));
    }

    #[test]
    #[should_panic]
    fn test_invalid_operation() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        let operation = 'a';
        Equation::new(left, right, operation);
    }

    #[test]
    fn test_add() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        let operation = '+';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(2.0, 2.0));
    }

    #[test]
    fn test_subtract() {
        let left = Complex::new(2.0, 2.0);
        let right = Complex::new(1.0, 1.0);
        let operation = '-';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(1.0, 1.0));
    }

    #[test]
    fn test_multiply() {
        let left = Complex::new(2.0, 2.0);
        let right = Complex::new(2.0, 2.0);
        let operation = '*';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(0.0, 8.0));
    }

    #[test]
    fn test_divide() {
        let left = Complex::new(2.0, 2.0);
        let right = Complex::new(2.0, 2.0);
        let operation = '/';
        let mut equation = Equation::new(left, right, operation);
        assert_eq!(equation.get_sol(), &Complex::new(1.0, 0.0));
    }

    #[test]
    fn test_add_complex() {
        let left = Complex::new(1.0, 1.0);
        let right = Complex::new(1.0, 1.0);
        assert_eq!(left.add(&right), Complex::new(2.0, 2.0));
    }

    #[test]
    fn test_subtract_complex() {
        let left = Complex::new(2.0, 2.0);
        let right = Complex::new(1.0, 1.0);
        assert_eq!(left.subtract(&right), Complex::new(1.0, 1.0));
    }

}
