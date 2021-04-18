use rand::seq::SliceRandom;

pub fn take_2<T>(v: &Vec<T>) -> (T, T)
where
    T: Copy,
{
    let ab = v
        .choose_multiple(&mut rand::thread_rng(), 2)
        .collect::<Vec<_>>();
    (*ab[0], *ab[1])
}
