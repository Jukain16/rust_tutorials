fn main() {
   let v: Vec<i32> = Vec::new(); 
   let v = vec![14, 20, 304, 45, 3, 3, 2, 14];
   for mut i in &mut v {
       *i += i;
   }
}
