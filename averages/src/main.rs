fn main() {
   let v = vec![14, 20, 304, 45, 3, 3, 2, 14];
   for mut i in v {
       i += i;
       let mean = i / 8;
      println!("The mean is {}", mean);
   }
}
