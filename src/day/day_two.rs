pub fn safe(v: &Vec<usize>) -> bool {
    let mut inc = true;
    let mut dec = true;
    let mut sorted_v = v.clone();
    for i in 1..v.len() {
        if v[i] <= v[i-1] {
            inc = false
        }
        if v[i] >= v[i-1] {
            dec = false
        }
    }
    if !inc && !dec {
        return false
    }
    if dec {
        sorted_v.reverse();
    }

    for i in 1..sorted_v.len() {
        if sorted_v[i] - sorted_v[i-1] > 3 {
            return false;
        }
    }
    true
}


pub fn safe_tolerate(l: &Vec<usize>) -> bool {
    if safe(&l) {
        return true;
    }
    for i in 0..l.len() {
        let mut modified = l.clone();
        modified.remove(i);
        if safe(&modified) {
            return true;
        }
    }
    false
}

pub fn reports_is_safe(p0: &str) -> i32 {
    let valid = p0.lines().map(|line| {
        let numbers = line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        safe(&numbers)
    }).collect::<Vec<bool>>().iter().filter(|x| **x == true).count() as i32;
    valid
}

pub fn reports_is_safe_with_one_erro(p0: &str) -> i32 {
    let valid = p0.lines().map(|line| {
        let numbers = line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        safe_tolerate(&numbers)
    }).collect::<Vec<bool>>().iter().filter(|x| **x == true).count() as i32;
    valid}