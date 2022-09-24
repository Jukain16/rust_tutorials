fn mean() {
   let n = v[0] + v[1] + v[2] + v[3] + v[4] + v[5] + v[6] + v[7];
   let mean = n / 8;
   println!("{}", mean);
}

fn median() {
    let o = v[4] + v[5];
    let median = o / 2;
    println!("{}", median);
}

fn mode() {
}

fn main() {
    let v = vec![14, 20, 304, 45, 3, 3, 2, 14];
    mean();
    median();
}
