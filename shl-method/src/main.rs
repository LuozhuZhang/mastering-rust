pub struct U256(pub [u64; 4]);
// 1 << 31 -> 2147483648
// 最后返回的数字会很大

fn main() {
   let n : u128 = 1 << 127;
   println!("{}", n );
}
