fn main() {
    assert!(!compare_string("s1", "s2"));
    assert!(compare_string("滴滴叭叭呜", "CloudWeGo"));
    assert!(!compare_string(
        "我能吞下玻璃而不伤身体",
        "我能吞下玻璃而不👆身体"
    ));
    assert!(!compare_string(" abc", "abc"));
    assert!(!compare_string("abc", "abc "));
    assert!(!compare_string("Rust", "Rust"));
}

///if s1>s2, return true
fn compare_string(s1: &str, s2: &str) -> bool {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    // println!("{:?}", chars1);
    // println!("{:?}", chars2);

    let len1 = chars1.len();
    let len2 = chars2.len();
    let len = if len1 > len2 { len2 } else { len1 };

    for i in 0..len {
        if chars1[i] > chars2[i] {
            return true;
        } else if chars1[i] < chars2[i] {
            return false;
        }
    }

    if len1 > len2 {
        return true;
    } else {
        return false;
    }
}
