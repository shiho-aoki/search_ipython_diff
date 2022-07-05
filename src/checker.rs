
pub fn run (ans: &Vec<char>, test: &Vec<char>) {
    let ans_len = ans.len();
    let test_len = test.len();
    
    let mut length = 1f32;
    if ans_len > test_len {
        length = ans_len as f32;
    }else{
        length = test_len as f32;
    }

    let mut cont = 1f32;
    let len_ = *&length as i32;
    for i in 0..len_ {
        let i_ = *&i as usize;
        let ans_ = *&ans.get(i_);
        let test_ = *&test.get(i_);
        if ans_ != test_ {
            cont += 1.0;
        }
    }
    println!("> len: {}", length);
    println!("> cout: {}", cont);
}