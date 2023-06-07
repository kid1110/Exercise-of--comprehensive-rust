pub trait Iterator{
    type Item;
    fn next(&mut self)-> Option<Self::Item>;
}
fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = v.into_iter();

    let v0 = iter.next();
    println!("v0: {v0:?}");
}
