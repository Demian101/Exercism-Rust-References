use std::cmp::Ordering;

pub fn find<T: Ord, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    let (mut sp, mut ep) = (0_usize, array.as_ref().len());

    while sp < ep {
        let mid = (sp + ep) >> 1;
        match key.cmp(unsafe { array.as_ref().get_unchecked(mid) }) {
            Ordering::Equal => return Some(mid),
            Ordering::Greater => sp = mid + 1,
            Ordering::Less => ep = mid,
        }
    }
    None
}

// why AsRef as a param?
// why array.as_ref()
// why unsafe? get_unchecked? 