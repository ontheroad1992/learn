fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    test();

    test1();

    test2();
}

// 允许变量覆盖（遮蔽）
fn test2() {
    let x = 5;

    let x = x + 1;

    {
        // 作用域
        let x = x * 2;

        println!("the value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x)
}

fn test1() {
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

fn test() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

struct Struct {
    e: i32,
}
