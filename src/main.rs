mod buffer;
mod compare;
use buffer::Buffer;
use compare::compare_string;
fn main() {
    let mut buffer: Buffer<u64> = Buffer::new();
    buffer.push(1);
    buffer.push(2);
    buffer.push(3);
    buffer.pop();
    println!("{:?}", buffer.sum());
    println!("{}", compare_string("abc", "abcc"));

    let v1 = vec!['a', 'b', 'c', 'd', 'e'];
    let v2: Vec<_> = v1
        .iter()
        .map(|x| char::from_u32(*x as u32 + 1).unwrap())
        .collect();
    println!("{:?}", v2);
}
