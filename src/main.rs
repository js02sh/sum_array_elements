fn main() {
    let numbers: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];

    let mut sum = 0;
    let sum2:i32 = numbers.iter().sum();
    //let sum2 = numbers.iter().sum()::<i32>;

    let mut i = 0;
    loop {
        sum += numbers[i];
        i += 1;

        if i == 10 {
            break;
        }
    }
    println!("Sum of all elements: {}", sum);
    println!("Sume with iter: {}", sum2);
}
