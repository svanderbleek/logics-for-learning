trait Clone {}
impl Clone for usize {}
impl<T> Clone for Vec<T> where T: Clone {}

trait Eq<T> {}
impl Eq<usize> for usize {}
impl<T: Eq<U>> Eq<Vec<U>> for Vec<T> {}

fn main() {
    println!("yo")
}
