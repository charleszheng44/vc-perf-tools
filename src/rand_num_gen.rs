use rand::Rng;

pub fn gen_rand_nums(total_nums: u32, 
    min: u32, 
    max: u32, 
    nums_sum: u32) -> Vec<u32> {
    let mut rand_nums: Vec<u32> = vec![];
    let mut rng = rand::thread_rng();
    for _ in 0..total_nums {
        let cur_rand = rng.gen_range(min..max); 
        rand_nums.push(cur_rand);
    }
    let old_rand_sum = rand_nums.iter().sum::<u32>();
    let mut new_rand_nums = rand_nums.iter().
        map(|x| x * nums_sum/old_rand_sum).collect::<Vec<u32>>();
    let new_rand_sum = new_rand_nums.iter().sum::<u32>();
    
    let delta: i64 = nums_sum as i64 - new_rand_sum as i64;

    if delta > 0 {
        let mut reminder = delta;
        for i in 0..total_nums {
            new_rand_nums[i as usize ] = new_rand_nums[i as usize] + 1;
            reminder -= 1;
            if reminder == 0 {
                break
            }
        }
    } 

    if delta < 0 {
        let mut reminder = delta;
        for i in 0..total_nums {
            new_rand_nums[i as usize ] = new_rand_nums[i as usize] - 1;
            reminder -= 1;
            if reminder == 0 {
                break
            }
        }
    } 

    return new_rand_nums
}
