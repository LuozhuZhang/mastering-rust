fn main() {
    let mut slice = ['l', 'o', 'r', 'e', 'm'];
    // 得到index，和value的片段

    // chunks_mut(n)的n可以决定分成几段，在halo2的parallelize中传入的是chunk
    // 即每个thread处理的n数量，可以根据n确定每个thread需要处理的数据有哪些
    for (chunk_num, v) in slice.chunks_mut(2).enumerate() {
        println!("{}", chunk_num);
        println!("{:?}", v);
    }
}
