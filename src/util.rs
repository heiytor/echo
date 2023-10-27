/// Subtracts `a - b` and returns the result, or `0` if the subtraction overflows.
pub fn usize_sub(a: usize, b: usize) -> usize {
    let sub = a.checked_sub(b);
    match sub {
        None => 0,
        Some(sub) => sub,
    }
}

// pub mod macros {
//     macro_rules! usize_sub {
//         () => {
//             0
//         };
//         ($first:expr $(, $rest:expr)*) => {
//             $first.checked_sub(usize_sub!($($rest),*))
//                 .unwrap_or(0)
//         };
//     }
//
//     pub(crate) use usize_sub;
// }
