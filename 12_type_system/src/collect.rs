fn main() {
   let numbers = vec![1, 2, 3, 4, 5, 6, 7];

   // 类型推导无法推断就是需要类型申明
   let even_numbers: Vec<_> = numbers.into_iter().filter(|n| n % 2 == 0).collect();

   println!("{:?}", even_numbers);
}
