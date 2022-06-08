use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;

pub fn rand_prob_(number: i32, to: i32) -> bool
{
    let num = thread_rng().gen_range(0..to);
    if number > num {
        true
    }
    else {
        false
    }
}

pub fn rand_prob(number: i32) -> bool
{
    rand_prob_(number, 100)
}
pub fn rand_range(start: i32, stop: i32) -> i32
{
    thread_rng().gen_range(start..stop)
}

pub fn vec_shuffle<T>(vec: &mut Vec<T>)
{
    vec.shuffle(&mut thread_rng());
}

pub fn rand_number_increase_prob(mut start_prob: i32, minus_per_iteration: i32) -> i32
{
    let mut res = rand_prob(start_prob);
    let mut num = 0;
    if res {
        num += 1;
    }
    while res {
        start_prob -= minus_per_iteration;
        res = rand_prob(start_prob);
        if res {
            num += 1;
        }
    }
    num
}
