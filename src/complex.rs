use std::ops::{Mul, Add, Sub, Div};


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

    pub fn conjugate(&self) -> Complex<T>
    {
        Complex { real: self.real, imaginary: self.imaginary-self.imaginary - self.imaginary }
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

impl<T> Mul<T> for &Complex<T> 
where T:Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Copy
{
    type Output=Complex<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Complex{real: self.real * rhs, imaginary: self.imaginary * rhs}
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

impl<T> Div<&Complex<T>> for &Complex<T> 
where T:Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T> + Copy + Default + PartialEq
{
    type Output = Complex<T>;

    fn div(self, rhs: &Complex<T>) -> Self::Output {

     
        let conjugate = self * &rhs.conjugate();


        let mut unit = T::default(); 

        if self.real != T::default()
        {
            unit = self.real / self.real;
        }
        else if self.imaginary != T::default()
        {
            unit = self.imaginary / self.imaginary;
        }

        let unit = unit / (rhs.real*rhs.real + rhs.imaginary*rhs.imaginary); 
         
        return &conjugate * unit;
        
    }
}

impl<T> std::fmt::Display for Complex<T>
where T: std::fmt::Display + Add<Output = T> + Sub<Output=T> + Mul<Output = T> + Div<Output = T> + Copy
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "C: {} + i{}",self.real,self.imaginary)
    }
}