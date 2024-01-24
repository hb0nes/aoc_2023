use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

// We don't need to generate the collection entirely, we can create it on-demand based on an index
// we know that the collection looks like: [2...idx * 2] with a length of idx
// If we want to get the third position in that index for an uneven number, for example 3:
// [ 2..6] length of 3
// [ 2, 4, 6 ]
// To sum up this entire collection without having to generate the whole thing, we can use a trick
// I learned back in highschool:
//
// [ 2, 4, 6 ]'s sum is:
//   [ 2, 4, 6 ]
//   [ 6, 4, 2 ] +
//   -----------
//   [ 8, 8, 8 ]
// Divide ^ by 2 to get the answer.
//    4 * 3 = 12
//
// In other words, adding the set onto itself, multiplying the resulting value of one of the
// elements by the length and dividing by 2 gives you the sum.
//
/// even: true,  idx: 3 -> (1, 5, 3)  -> 9
/// even: false, idx: 3 -> (2, 6, 3)  -> 12
fn idx_to_sum(even: bool, idx: usize) -> usize {
   let (min, max) = (2 - even as usize, idx * 2 - even as usize);
   (((min+max)*idx) as f64 / 2.0) as usize
}

// My approach here was determined after seeing a certain logic in the way things are calculated:
// For example with 15 duration and 40 distance
// 15 is an uneven number
// 15/2.ceil() == 8
// 8  * 7 == 56
// 9  * 6 == 54 # Diff from previous result: 2
// 10 * 5 == 50 # Diff: 4
// 11 * 4 == 44 # Diff: 6
// 12 * 3 == 36 # Diff: 8
// 13 * 2 == 26 # Diff: 10
// 14 * 1 == 14 # Diff: 12
//
// Factorial pattern above can be summed up as follows:
// diffs: 2  4  6  8 10 12
//     -> 12 10 8  6  4  2
//     -> 14 14 14 14
//     -> 7 * 6  = 42
//
// We can see in the above example that for an UNEVEN number like this, the score decreases more
// and more the further we move away from the middle.
// This happens at an increasing rate of 2. First, 56->54, then 54->40.
//
// Collecting those numbers would give us a linear collection of [2, 4, 6...]
//
// However, for EVEN numbers, this is different. You can check for yourself, but the collection
// would be [1, 3, 5...]
//
// So, if we start at the halfway point of a certain duration (which gives the highest score)
// and we check what the offset is with the distance,
// we can find out how many moves we can make to hit the distance exactly, or overshoot.
// In both scenario's, we shouldn't count that one, because we need to 'break the record'.
//
// In the example above, we see that we can go down 3 times.
// which would result in a cumulative diff of 2 + 4 + 6     == 12. 56 - 12 == 44
// If we go down one more time, we get        2 + 4 + 6 + 8 == 20. 56 - 20 == 36
// That would be too low to beat the distance.
//
// In other words, for an uneven number, we have those 3 possible moves, which are mirrored in the
// other direction as well (going up)
// That would give us 6 total moves. However, because we also have the middle part of an uneven
// number, which is mirrored (8*7 and 7*8), we add 2 to the score for uneven numbers
// 2 + 6 = 8
//
// For even numbers, (30/2==15 -> 15 * 15) there is only one multiplier pair giving the maximum
// score, so our starting amount of moves is always going to be 1.
// As I wrote before, the linear lookup collection is different for even numbers.
// Check out idx_to_sum()
fn ways_to_win(duration:f64, record: usize) -> usize {
    let even = (duration as usize).rem(2) == 0;
    let ways_to_win_start = !even as usize + 1;
    let halfway = duration.div(2_f64).ceil() as usize;
    let range = duration as usize - halfway;
    let max_score = match even {
        true => halfway * halfway,
        _ => halfway.mul(halfway.sub(1)),
    };
    let delta = max_score.sub(record) as usize;

    // Bisect
    let mut low_bound = 0;
    let mut up_bound = range;
    let mut idx = range/2;
    loop {
      let sum = idx_to_sum(even, idx);
      let sum_next = idx_to_sum(even, idx+1);
      if sum < delta && sum_next >= delta {
          break
      }
      if sum >= delta {
          up_bound = idx;
          idx = ((idx as f64 + low_bound as f64) / 2_f64).floor() as usize;
      } else {
          low_bound = idx;
          idx = ((idx as f64 + up_bound as f64)    / 2_f64).ceil()  as usize;
      }
    }

    ways_to_win_start + idx * 2
}

//Time:        44     89     96     91
//Distance:   277   1136   1890   1768
fn main() {
    let ab = ways_to_win(44_f64, 277);
    let ac = ways_to_win(89_f64, 1136);
    let ad = ways_to_win(96_f64, 1890);
    let ae = ways_to_win(91_f64, 1768);
    println!("a: {}", ab * ac * ad * ae);
    let b = ways_to_win(44899691_f64, 277113618901768);
    println!("b: {:?}", b);
}
