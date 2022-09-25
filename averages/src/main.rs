use std::collections::HashMap;
fn main() {
    let mut v = vec![14, 20, 304, 45, 3, 3, 2, 14];
    let mean: f32;
    let median: i32;
    let mode: i32;
    {
        let mut sum = 0;
        for i in &v {
            sum += i;
        }
        mean = sum as f32 / v.len() as f32;
    }
    {
        v.sort();
        let mid = v.len() / 2;
        median = v[mid];
    }
    {
        let mut times = HashMap::new();
        for i in &v {
            let cnt = times.entry(*i as usize).or_insert(0);
            *cnt += 1;
        }
        let mut frequent: (i32, i32) = (*times.iter().nth(0)
            .expect("Fatal.").0 as i32, *times.iter().nth(0)
            .expect("Fatal.").1 as i32);
            for i in times.iter() {
                if *i.1 > frequent.1 {
                    frequent = (*i.0 as i32, *i.1);
                }
            }
        mode = frequent.0;
    }
    println!("The mean is {}, The median is {}, and the mode is {}", mean, median, mode);
}
