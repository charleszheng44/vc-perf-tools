use rand:Rng;

pub fn gen_unfair_rand_nums() -> Vec<u32>{
    let v = vec![];
    for i in 0..100 {
        if i < 10 {
            v.push(900);
        }
        if i >= 10 && i < 20 {
            v.push(12)
        }
        v.push(11)
    }
    v
}
