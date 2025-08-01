// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
      if let Some(x) = my_option {
          println!("{x:0?}");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    // println!("This Vec is empty, see? {my_empty_vec:?}");
    // println!("This Vec is empty, see? {{():?}}");
     let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
std::mem::swap(&mut value_a, &mut value_b);
    // value_a = value_b;
    // value_b = value_a;
    println!("value a: {value_a}; value b: {value_b}");
}
