#[allow(unused_variables)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 这里不需要调用 unwrap
        // my_option.unwrap();
    }

    let my_arr = &[
        -1, -2, -3, // 添加逗号
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear(); // 在原 Vec 上进行修改
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 交换这两个值
    std::mem::swap(&mut value_a, &mut value_b); // 使用 std::mem::swap
    println!("value a: {}; value b: {}", value_a, value_b);
}
