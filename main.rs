fn main() {
    let a = [76, 77, 78, 79, 80]; // Массив на 5 элементов типа i32
    let b = &a[1..4]; // Срез массива "a" включающий в себя элементы 1-3
    println!("{b:?}");
}
