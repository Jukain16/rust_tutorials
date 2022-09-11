fn main() {
   let v = vec![14, 20, 304, 45, 3, 3, 2, 14]; 
   for i in &v {
       v += v;
   }
   let mean = v / 8;
}
