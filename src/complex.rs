use std::ops::{Mul, Add, Sub, Div};

pub trait Number<Rhs> 
{
}

impl<Rhs> Number<Rhs> for f64 
where Rhs: Add
{

}

#[derive(Debug)]
pub struct Complex<T: Add<Output = T> + Sub<Output=T> + Mul<Output = T> + Div<Output = T>>
{
    pub real:T,
    pub imaginary:T
}

impl<T: Add<Output = T> + Mul<Output = T> + Sub<Output = T>+ Div<Output = T> + Copy> Complex<T>
{
    pub fn new(real:T, imaginary:T) -> Complex<T>
    {
        Complex { real, imaginary }
    }
}

impl<T> Add<&Complex<T>> for &Complex<T>
where T:Add<Output = T> + Sub<Output=T> + Mul<Output = T> + Div<Output = T> + Copy
{
    type Output = Complex<T>;

    fn add(self, rhs: &Complex<T>) -> Self::Output {
        let real = self.real + rhs.real;
        let imaginary = self.imaginary + rhs.imaginary;

        Complex{real, imaginary}
    }
}


impl<T> Mul<&Complex<T>> for &Complex<T> 
where T:Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Copy
{
    type Output = Complex<T>;

    fn mul(self, rhs: &Complex<T>) -> Self::Output {
        let real = self.real*rhs.real - self.imaginary*rhs.imaginary;
        let imaginary = self.real*rhs.imaginary + self.imaginary * rhs.real;

        Complex{real, imaginary} 
    }
}

impl<T> std::fmt::Display for Complex<T>
where T: std::fmt::Display + Add<Output = T> + Sub<Output=T> + Mul<Output = T> + Div<Output = T> + Copy
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "C: {} + i{}",self.real,self.imaginary)
    }
}