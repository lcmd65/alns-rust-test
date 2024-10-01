use rand::seq::IteratorRandom;
use rand::thread_rng;

pub(crate) fn random_choice<T>(items: &[T]) -> &T {
    let mut rng = thread_rng();

    items.iter().choose(&mut rng).expect("Empty collection, cannot select a random element")
}

pub(crate) fn random_choice_from_range(start: usize, end: usize) -> usize {
    let mut rng = thread_rng();
    (start..=end)
        .collect::<Vec<_>>()
        .into_iter()
        .choose(&mut rng)
        .expect("Invalid range, cannot choose a random element")
}