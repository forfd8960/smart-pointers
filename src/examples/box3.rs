struct LargeObject {
    data: [u8; 1000],
}

/*
size obj: 1000
size obj: 8
consume obj:
consume_bytes: 1000
*/
fn main() {
    let obj = LargeObject { data: [0; 1000] };
    println!("size obj: {}", size_of_val(&obj));

    let b_obj = Box::new(obj);
    println!("size obj: {}", size_of_val(&b_obj));

    println!("consume_bytes: {}", consume_bytes(b_obj));
}

fn consume_bytes(obj: Box<LargeObject>) -> usize {
    println!("consuming obj");
    obj.data.len()
}
