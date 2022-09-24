use std::collections::HashMap;
fn mean(v: Vec<u32>) {
   let n = v[0] + v[1] + v[2] + v[3] + v[4] + v[5] + v[6] + v[7];
   let mean = n / 8;
   println!("{}", mean);
}

fn median(v: Vec<u32>) {
    let o = v[7] + v[0];
    let median = o / 2;
    println!("{}", median);
}

fn mode(v: Vec<u32>) {
  let mut mode = HashMap::new();
  mode.insert(3, 14);
  println!("{:?}", mode);
}

fn main() {
    let v = vec![14, 20, 304, 45, 3, 3, 2, 14];
    mean(v);
    median(v);
    mode(v);
}
