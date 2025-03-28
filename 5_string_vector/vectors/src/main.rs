fn vector_sum(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in vec {
        sum += value;
    }
    sum
    
}

fn insert_value_beggining_end(vec: &Vec<i32>, value: i32) -> Vec<i32> {
    let mut new_vec = vec.clone();
    new_vec.insert(0, value);
    new_vec.push(value);
    new_vec
}

fn main() {
    let vec: Vec<i32> = vec![];

    match vec.first() {
        Some(first_value) => println!("The first value is {}", first_value),
        None => println!("The vector is empty!"),
        
    }

    let vec2 = vec![1, 2, 3, 4, 5];
    println!("The sum of the vector is {}", vector_sum(&vec2));

    let vec3 = insert_value_beggining_end(&vec2, 6);
    println!("The new vector is {:?}", vec3);

    println!("The old vector is {:?}", vec2);
}
