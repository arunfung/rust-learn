fn main() {
   let numbers = vec![1, 2, 3, 4, 5, 6, 7];

   // 类型推导无法推断就是需要类型申明
   // 注意这里编译器只是无法推断出集合类型，但集合类型内部元素的类型，还是可以根据上 下文得出，所以我们可以简写成 Vec<_> 。
   let even_numbers: Vec<_> = numbers.into_iter().filter(|n| n % 2 == 0).collect();
   println!("{:?}", even_numbers);

   // 除了给变量一个显式的类型外，我们也可以让 collect 返回一个明确的类型
   let numbers = vec![1, 2, 3, 4, 5, 6, 7];
   let even_numbers_1 = numbers.into_iter().filter(|n| n % 2 == 0).collect::<Vec<_>>();
   println!("{:?}", even_numbers_1);
}
