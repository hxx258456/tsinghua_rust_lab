# tsinghua_rust_lab
清华rust lab

[程序设计训练（Rust） (tsinghua.edu.cn)](https://lab.cs.tsinghua.edu.cn/rust/)

## syntax

### 变量绑定(不是赋值)

```rust
let x = 64; 
// 编译器支持类型推导,但是也存在编译器推导不出的情况,推荐用户显示指定变量类型

let x: i16 = 15;
```

### 变量可变性

```rust
let x = 5;
x += 1; // error: re-assigment of immutable variable x

let mut y = 5;
y += 1; // ok
```

### 变量shadowing

```rust
// 与go不同的地方rust支持声明相同变量
let x = 17;
let y = 54;
let x = "Shodowed!"; // x = 17失效
```

### const

```rust
// rust常量的值需要在编译期可以确定
// 常量在声明时必须指定类型
// 常量可以提前使用
const PI: f64 = 3.1415926; 
const MAGIC: i32 = 43;
```

### 面向表达式

```rust
// rust是一门面向表达式语言
// 表达式分为三种: 块表达式，位置表达式，值表达式
// arr[0] = x + 1;
// arr[0]为位置表达式
// x + 1是块表达式
// 块表达式就是被{}包裹的一系列表达式，返回最后一个表达式的值
// ;也是表达式返回()

// rust中的语句
// 当编译器遇到let pub fn use等关键字的时候，不会进行求值，会执行该语句
// ;跟在语句后面不会进行求值，当作分隔符使用
let a = (let b = 1);
```

### 基本类型

```rust
/// bool: true/false
/// char: 'R', '级' 公众的rune
/// number: i8~i128 u8~u128 isiz等于go中的int usize等于go中的uint f32,f64,10.0f32,10u8,10000_10000
/// fn函数
```

### never类型

```rust
// Rust 的 never 类型（ ! ）用于表示永远不可能有返回值的计算类型。
// 发散类型
let x: ! = loop {};
```

### 范围类型

```rust

```

### 数组

```rust
// [T;N]
// [i32; 10]
// N在编译期可以确定值，也就是说数组长度不可变 eg:这里和go的设计一样
let arr1 = [1,2,3]; // arr 3 ele
let arr2 = [2;32]; // arr 2 ele
```

### 切片

```rust
// &[T]
// &[i32]
// 切片表示引用数组中的一部分，切片不可直接创建，需要从别的变量借用borrow eg:go可以声明空切片
// 切片可以是可变的也可以是不可变的
let arr = [0,1,2,3,4,5,6,7,8,9];
let total_slice = &arr; // all
let total_slice = &arr[..]; // all
let partial_slice = &arr[2..5]; // [2,3,4]

// 可变的arr才能声明可变的切片
let mut arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; 
let mut slice = &mut arr;
```

## 字符串

```rust
/*
* rust中有两种字符串String和&str
* String是在堆上分配空间，可以增长的字符序列
* &str是rust原始字符出类型，是对String的切片，是对String的引用
* rust之所以这样设计字符串是为了契合自己的borrowing和move
*/

let s: &str = "galaxy";
let s2: String = "galaxy".to-string();
let s3: String = String::from("galaxy");
let s4: &str = &s3;
```

### 元组tuple

```rust
//! 固定大小、有序、异构的列表类型
//! 可以通过下标访问分量 foo.0
//! 可以用来变量的解构绑定

let foo: (i32, char, f64) = (72, 'H', 5.1);
let a = foo.0;
let (a, b, c) = foo; // a = 72 b = 'H' c = 5.1
let (x, y, z) = (72, 'H', 5.1);
let a = (1,); // 当元组只有一个值时需要加,号
```

### 向量Vec

```rust
// Vec是分配在堆上的、可增长的数组
// Vec<T> <T>表示泛型 ex:i32 Vec<i32>
// Vec::new()
// vec!

let v0: Vec<i32> = Vec::new();

let mut v1 = Vec::new();
v1.push(1);
v1.push(2);

let v2 = vec![1,2,3];

// v3 equal v4
let v3 = vec![0;4];
let v4 = vec![0,0,0,0];

// 在rust中必须使用usize类型作为下标访问Vec,数组,切片
let i: i8 = 2
let y = v3[i as usize];
```

### 引用

```rust
//! 在类型前面写&表示引用类型: &i32
/// 用&来取引用
//  用*来解引用
/*
*   rust中的引用和指针不是一个概念
*/

let x = 12;
let ref_x = &x;
println!("{}", *ref_x); // 12
```

### 条件语句

```rust
let a = if x > 0 {
10
} else if x == 0 {
0
} else {
println!("Not greater than zero!");
-10
}

// 整个条件语句是可以当作一个表达式来求值
// 也可以作为普通的条件语句来使用
if x < 0 {
    println!("Too small");
}
```

### while

```rust
// while循环
let mut x = 0;
while x < 100 {
    x += 1;
    if x > 50 {
        break;
    }
    if x < 10 {
        continue;
    }
    println!("{x}");
}
```

### loop

```rust
/// loop循环相当于while true
/// loop循环在rust中属于表达式，可以使用break返回一个值，作为整个循环的求值结果(while,for没有这个功能)

let mut x = 0;
let y = loop {
    x += 1;
    if x * x >= 100 {
        break x
    }
}
```

### for

```rust
// rust中的for用来处理迭代器
// n..m创建一个从n到m的半闭半开区间的迭代器
// n..=m创建一个从n到m闭区间的迭代器
// 很多数据结构可以用来做迭代器,数组,切片,Vec

// 0~9
for x in 0..10{
    println!("{x}")
}

// 0~10
for x in 0..=10{
    println!("{x}")
}

let xs = [0, 1, 2, 3, 4];
for x in xs {
    println!("{x}")
}

for x in &xs {
    println!("{x}")
}
```

### match

```rust
//! match
//! 匹配语句由一个表达式x和一组value => expression的分支语句组成
//! match匹配语句被视为一个表达式来求值
//! 每个分支返回结果必须相同
//! (_)用来捕获所有情况
//! 为了通过编译写穷尽的匹配模式
fn main() {
    let (x, y) = (3, -3);
    match (x, y) {
        (1, 1) => println!("one"),
        (2, j) => println!("two, {j}"),
        (_, 3) => println!("three"),
        (i, j) if i > 5 && j < 0 => println!("On gurad!"), // 可以使用if来限制匹配条件
        (a, b) => println!("{a},{b}"),                     // 匹配可以绑定变量,match内的临时变量名不可以重复这里不能再用i,j
    }
}
```

### 函数

```rust
fn add(x: u32, y: u32) -> u32 {
    x + y
}

```

### 宏

```rust
fn main() {
    let x = "foo";
    print!("{}, {}, {}", x, 3, true);
    println!("{:?}, {:?}", x, [1, 2, 3]);
    
   
    let fmted = format!("{}, {:x}, {:?}", 12, 155, Some("hello world"));
    println!("{fmted}"); // 12, 9b, Some("hello world")
    
    panic!("Kaboom!");
    assert!(1 == 2);
    assert_eq!(1, 2);
    unreachable!(); // 表示不会到达的分支
    unimplemented!(); // 标注还没有实现的功能
}

```



