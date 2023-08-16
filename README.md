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
/// char: 'R', '级' go中的rune
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

let slice = &arr[0..=3]; // [0,1,2,3]

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

### rust所有权

- rust中每个值都有所有者owner

- 一个值同一时刻只有一个所有者

- 当所有者失效，值也将被丢弃

- 变量绑定获取数据的所有权

  ```rust
  // carete Vec object in heap
  // Give ownership of the Vec obj to v1
  let mut v1 = vec![1, 2, 3];
  v1.push(4);
  v1.pop();
  // 作用域结束,v1 仍然拥有对象，可以对其进行清理
  ```

- 移动语义

  ```rust
  let v1 = vec![1, 2, 3];
  
  // Ownership of the Vec object move to v2.
  let v2 = v1;
  // rust编译器认为拷贝数据的代价是高昂的，对于没有实现copy trait的类型，rust会进行所有权转移v1>v2
  // rust也可以通过std::mem::{replace, take, swap, drop}来实现高级所有权管理
  ```

- 所有权转移

  ```rust
  fn vector_length(v: Vec<i32>) -> Vec<i32> {
      // do something...
      // return ownership v back to the caller
      v
  }
  
  // 并不是所有时候都想移交所有权
  // 如果每次传递参数都要移交所有权代码将会变得非常繁琐
  ```

- 借用

  ```rust
  //! 与其移交所有权不如进行借用borrow
  /// 可以通过对变量取引用来借用变量中数据的所有权，此时所有权本身并没有发生变化
  let v = vec![1, 2, 3];
  let v_ref = &v;
  assert_eq!(v[1], v_ref[1]);
  
  // 借用存在的副作用
  let v_new = v; // 这里edition 2018之前会报错因为当时的rust使用的是词法作用域声明周期，当一个变量被借用时，不能进行所有权转移，因为这会导致引用失效
  // edition 2018 rfc提出了NLL非词法生命周期
  // 对象或引用的生命周期却决于控制流图，而不是词法作用域
  ```

- 可变借用

  ```rust
  fn push(vec_ref: &mut Vec<i32>, x: i32) {
      vec_ref.push(x);
  }
  
  fn main() {
      let mut v = vec![1, 2, 3, 4, 5];
      let v = &mut v;
      push(v, 1)
  }
  
  ```

- 借用与绑定

  ```rust
  fn push2(vec_ref: &mut Vec<i32>, x: i32) {
      let mut vector = *vec_ref; // error: connot move out of borrowed content
      vector.push(x);
  }
  // 不能通过解引用然后绑定给变量，这样做会引起数据的所有权转移（同时引用还没有失效）
  ```

- Copy类型

  ```rust
  // rust中定义了Copy类型trait，表示一种类型可以拷贝，而不是默认的移动语义
  //! 大多数基本类型都是Copy类型(i32,u64,f64,char,bool等等).
  /// 包含引用的类型不是Copy类型(例如,Vec String等)
  
  let x = 32;
  let y = x;
  println!("{x},{y}");
  ```

- 借用的规则

  - 不能在某个对象不存在后继续保留对他的引用
  - 一个对象可以同时存在多个不可变引用&T
  - 和一个可变引用&mut T
  - 以上两者不能同时存在

- 借用的作用

  ```rust
  // 以迭代器场景为例,在修改集合的同时进行迭代访问会引起迭代器失效
  let mut vs = vec![1, 2, 3, 4];
  for v in &vs {
      vs.pop();
      // ERROR: cannot borrow `vs` as mutable because
      // it is also borrowed as immutable
  }
  // 以上问题在go中如果通过for_range迭代时，会拷贝一份数据，在迭代时修改源数据不会影响迭代，但这造成了一些性能问题
  // 如果使用for ;;;{}这种方式迭代，会造成死循环
  
  
  // 下面代码在c++中可以通过编译，但是会在运行时报错
  let y: &i32;
  {
      let x = 5;
      y = &x; // rust编译器会检查出这里有错误 变量x的声明周期已经结束但是他正在被借用
  }
  println!("{}",*y);
  ```

### Vec迭代器

```rust
let mut vs = vec![1, 2, 3, 4, 5];
// vector三种迭代方式
// Borrow imutable
for v in &vs {
    println!("{v}");
}

// Borrow mutable
for v in &mut vs {
    *vs.push(3); // err: 编译器报错,同时存在多个可变借用
    println!("{v}");
}

// take ownership
for v in vs {
    println!("{v}");
}

// vs is valid
```

### 结构体

- 声明

  ```rust
  struct Point {
      x: i32,
      y: i32
  }
  ```

- 初始化

  ```rust
  let origin = Point { x: 0, y: 0 };
  ```

- 访问

  ```rust
  let mut p = Point{x: 19, y: 18};
  p.x += 1;
  p.y += 1;
  ```

- 结构体与可变性

  结构体没有域级的可变性内部可变性可以通过Cell类型实现

  可变性时变量绑定的属性，跟类型无关

  ```rust
  struct Point{
      x: i32,
      mut y: i32,// Illegal
  }
  ```

- 结构体的访问权限

  - 结构体在它所在的模块的名字空间里
  - 结构体的域默认是私有的
  - 私有域只能在结构体所在的模块内访问

  ```rust
  mod foo {
      pub struct Point {
          pub x: i32,
          y: i32,
      }
      
      pub fn new(x: i32,y: i32) -> Point {
          Point { x: x, y: y }
      }
  }
  
  fn main() {
      let b = foo::Point { x: 12, y: 12 }; // error: y is private
      let b = foo::new(12, 12);
  }
  ```

- 结构体的更新语法

  ``` rust
  // 结构体初始化时可以用..s从s中获取部分或者全部域.
  // 所有没有在初始化列表里指定的域都是从目标结构体里获取
  // 两个结构体类型必须一致
  struct Foo {
      a: i32,
      b: i32,
      c: i32,
      d: i32,
      e: i32,
  }
  
  let mut x = Foo {
      a: 6,
      b: 7,
      c: 8,
      d: 9,
      e: 10,
  };
  
  let x2 = Foo { e: 4, ..x };
  
  x = Foo { a: 2, b: 2, ..x }; // 结构体更新语法
  ```

### 元组结构体

元组结构体是结构体的一种形态，有结构体名字，但没有域的名字

可以像元组那样通过数字来访问域，例如x.0,x.1等等。

也可以通过match类进行匹配

```rust
fn main() {
    struct Color(i32, i32, i32);

    let mut c = Color(0, 255, 255);

    c.0 = 255;

    match c {
        Color(r, g, b) => println!("({}, {}, {})", r, g, b)
    }
}
```

元组结构体的用途

```rust
// 可以用来创建新的类型而不仅仅是别名
struct Meters(i32); // 新建类型 type Net.Addr string
struct Yards(i32);

type MetersAlias = i32; // 类型别名类似于go中的type Pointer = uint
```

单位元结构体

```rust
// 可以声明零大小的结构体(没有域的结构体)
// 这种结构体也是可以实例化的
// 通常被用来作为其他数据结构体的标记类型
strcut Unit;
let b = Unit;
```

### 枚举enum

- 枚举用来表示可以是多选一的数据
- 枚举的每种变体可以:
  - 没有数据(单位元变体)
  - 有命名的数据域(结构体变体)
  - 元组变体

```rust
enum Resultish {
    Ok,
    Err(String),
    Warning { Code: i32, message: String },
}// 枚举声明
```

- 枚举的变体存在于枚举本身的名字空间中 Resultish::Ok
- 可以使用user Resultish::*把所有变体引入到当前的名字空间
- 枚举可以进行模式匹配

```rust
enum Resultish {
    Ok,
    Err(String),
    Warning { Code: i32, message: String },
}
fn main() {
    match make_request() {
        Resultish::Ok => println!("Success!"),
        Resultish::Warning { Code, message } => println!("Warning: {}!", message),
        Resultish::Err(s) => println!("Failed with error: {s}"),
    }
}

fn make_request() -> Resultish {
    return Resultish::Ok;
}
```

### 递归枚举类型

```rust
enum List {
    Nil,
    Cons(i32, List),
}
// 使用上面方式定义在编译时会出现无穷大小的问题
// 结构体和枚举默认情况下时内联存储的，因此不能递归
// 它们的元素正常情况下不使用引用来存储，但可以显示指定
// Box<T>是指向堆上对象的指针，作为对象的唯一所有者，Box唯一拥有它的数据(T类型),不能创建别名
// Box在超过作用域时会自动销毁
let boxed_five = Box::new(4);

enum List {
    Nil,
    Cons(i32, Box<List>),
}
```

### 方法

```rust
impl Point {
    pub fn distance(&self, other: Point) -> f32 {
        let (dx, dy) = (self.x - other.x, self.y - other.y);
        ((dx.pow(2) = dy.pow(2)) as f32).sqrt()
    }
}

fn main() {
    let p = Point { x: 1, y: 2};
    p.distance();
}
// 结构体和枚举的方法可以实现在impl代码块里
// 方法通过.方法名的方式访问
// 可以使用pub将方法声明为公共访问的
// 对枚举和结构体是一样的
```

### 方法与所有权

```rust
// 方法的第一个参数(名字规定为self)决定这个方法的所有权种类
// &self: 方法引用对象的值
// &mut self: 方法借用对象的值
// self方法获得对象的所有权 方法会消耗掉对象，同时可以返回其他的值
impl Point {
    fn distance(&self, other: Point) -> f32 {
        let (dx, dy) = (self.x - other.x, self.y - other.y);
        ((dx.pow(2) = dy.pow(2)) as f32).sqrt()
    }
    
    fn translate(&mut self, x: i32, y: i32) {
		self.y += x;
        self.y += y;
    }
    
    fn mirror_y(self) -> Point {
        Point { x: -self.x, y: self.y }
    }
}
```

### 关联函数

```rust
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y}
    }
}

fn main() {
    let p = Point::new(1, 2);
}

// 关联函数与方法类似，但是没有self参数
// 调用时使用名字空间语法Point::new()
// 一般会创建名为New的关联函数起到构造函数的作用
// rust不支持函数重载
// rust中的方法继承用的是compose合成的办法
```

### 模式匹配

- 结构体的匹配

  ```rust
  pub struct Point {
      x: i32,
      y: i32,
  }
  
  match p {
      Point { x, y } => println!("({},{})", x, y)
  }
  
  match p {
      Point { y: y1, x: x1 } => println!("{x1}, {y1}")
  }
  
  match p {
      Point { y, .. } => println!("{y}")
  }
  ```

- 以引用方式匹配

  ```rust
  let mut x = 16;
  
  match x {
      ref r if x == 5 => println("{x}"),
      ref mut r => *r = 5
  }
  ```

- if let语句

  ```rust
  enum Resultish {
      Ok,
      Warning { code: i32, message: String },
      Err(String),
  }
  
  if let Reulstish::Err(s) = make_result() {
      println!("Total and utter failure: {}", s);
  } else {
      println!("ok.");
  }
  ```

- while let语句

  ```rust
  let mut v = vec![1, 2, 3];
  
  while let Some(x) = v.pop() {
      println!("{}", x);
  }
  ```

- 内部匹配

  ```rust
  // 模式匹配时可以把匹配的一部分绑定到一个变量，例如在匹配范围的时候，可以获取匹配的实际值；对于更复杂的数据结构，可以用 @ 创建内部元素的变量绑定。
  #[derive(Debug)]
  enum A { None, Some(B) }
  #[derive(Debug)]
  enum B { None, Some(i32) }
  
  fn foo (x: A) {
      match x {
          a @ A::None => println!("a is A::{:?}", a),
          ref a @ A::Some(B::None) => println!("a is A::{:?}", *a),
          A::Some(B::Some(b @ 1..=2)) => println!("b is {:?}", b),
          A::Some(b @ B::Some(_)) => println!("b is B::{:?}", b),
      }
  }
  
  foo(A::None); // a is A::None
  foo(A::Some(B::None)); // a is A::Some(None)
  foo(A::Some(B::Some(5))); // b is B::Some(5)
  ```

  

- 模式匹配的穷尽性

  模式匹配的所有分支对于模式来说必须是穷尽的

  ```rust
  fn main() {
      fn plus_one(x: Option<i32>) -> Option<i32> {
          match x {
              Some(i) => Some(i + 1),
              // error: not exhaustive
      	}
      }
      let five = Some(5);
      let six = plus_one(five);
      let none = plus_one(None);
  }
  ```

  

- for循环中的模式匹配

  ```rust
  let v = vec![1, 2, 3];
  // 在 for 循环的 for 和 in 之间描述循环变量时可以使用模式匹配
  for (i, x) in v.iter().enumerate() {
      println!("v[{i}] = {x}");
  }
  ```

- 函数参数中的模式匹配

  ```rust
  fn tuple_add((a, b): (i32, i32)) -> i32 {
  	a + b
  }
  fn main() {
  	tuple_add((1, 2));
  }
  
  ```


### 字符串连接操作

```rust
let a = String::from("hello");
let b = String::from(" world");
let c = a + &b; // `a` is moved to c

let a = String::from("hello");
let b = String::from(" world");
let c = a.clone() + &b; // `a` still alive

// 连接两个&str需要把第一个转成String
let a = "hello";
let b = " world";
let c = a.to_string() + b;
// 连接操作代码的实现
fn add(mut self, other: &str) -> String {
    self.push_str(other);
    self
}
```

### Option枚举类型

```rust
enum Option<T> {
    None,
    Some(T),
}
// rust使用Option枚举类来处理空值情况
match foo() {
    None => (),
    Some(value) => {
        bar(value)
        // ...
    },
}
// Option::unwrap 遇到None时发生panic
// Option::map 希望对一个map进行变换，也就是有值的时候作用一个函数，空值的时候继续保持空值
// Option::and_then 返回值变成Option(U)
// Option:;unwrap_or 对于空值的情况返回默认值
// Option::unwrap_or_else 默认值是由闭包函数计算而来
// Option::is_some bool
// Option::is_none bool
// Option::map_or
// Option::map_or_else
// Option::ok_or
// Option::ok_or_else
// Option::and
// Option::or
// Option::xor
```

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

let result = divide(2.0, 3.0);

result.unwrap();
```

Option的典型用途:

- 初始值
- 函数定义域不是全集
- 简单的错误情况
- 结构体的可选域或可拿走的域
- 可选的函数参数
- 空指针



### 错误处理

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Result与Option类似，除了正常结果外，还可以表示错误状态。

可以通过ok或err等方法转换成Option

对于返回结果是Result的函数，一定要显示进行处理，unwrap，expect，match模式匹配，不处理编译器会警告

```rust
// 一般常用的错误处理方法是在自己编写的库里使用自定义的错误类型，并定义Result的别名
use std::io:Error;

type Result<T> = Result<T, Error>;


// 使用的时候
use std::io;
fn foo() -> io::Result {
    // ....
}
```

?操作符用来提前传播错误

```rust
// 配合Result类型
// 如果是Err则提前返回，当前函数立即返回该错误
// 如果是Ok则取出值作为?操作符的结果
fn read_username_form_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// 配合Option类型
// 如果是None则提前返回，当前函数立即返回None
// 如果是Some则取出值作为?操作符的结果
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

#### 容器类型

- Vec<T>

  连续空间，末尾可以高效增删

- VecDeque<T>

  双端向量，两端可以高效增删，使用环形缓冲区实现

- LnkedList<T>

  双向链表，不能随机索引

- HashMap<K, V>

  ```rust
  use std::collections::HashMap;
  
  let mut scores = HashMap::new();
  
  scores.insert(String::from("Blue"), 10);
  
  let team_name = String::from("Blue");
  let score = scores.get(&team_name);
  
  for (key, value) in &scores {
      println!("{key}, {value}");
  }
  // hash表的所有权对于Copy类型，拷贝进hash表
  // 对于非Copy类型，所有权移动进哈希表，哈希表拥有所有权
  
  let field_name = String::from("Favorite color");
  let field_value = String::from("Blue");
  let mut map = HashMap::new();
  map.insert(field_name, field_value); // field_name and field_value ownership move to hashmap
  // field_name and field_value invalid
  
  scores.insert(String::from("Blue"), 10); // 改写
  scores.entry(String::from("Blue")).or_insert(50); // 不存在时添加
  
  // 基于旧值更新
  let text = "hell world wonderfule world";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
  }
  ```

  

- BTreeMap<K, V>

  有序HashMap

- HashSet<T>

- BTreeSet<T>

- BinaryHeap<T>

### 迭代器

```rust
pub trait Iterator {
    type Iterm, // 迭代器特型的关联类型
    fn next(&mut self) -> Option<Self::Item>,
    // ..
}
```

迭代器与所有权

三种迭代类型:

- into_inter(), 产生T类型
- iter()，产生&T
- iter_mut，产生&mut T

for in循环其实就是迭代器的语法糖

```rust
let values = vec!(1, 2, 3, 4, 5);


{
    let result = match values.into_iter() {
        mut iter => loop {
            match iter.next() {
                Some(x) => { println!("{x}", )},
                None => break,
            }
        }
    };
    result
}
```

迭代器可以进行类型转换操作

```rust
// collect 将迭代器转换为其他集合类型
let set = values.iter().collect::<HashSet<_>>();

// 把迭代器折叠成单一值
// fold有两个参数初始值，和折叠函数
let sum = values.iter().fold(0, |acc, v| acc + v);

let leicheng = values.iter().fold(1, |acc, v| acc * v);
```

迭代适配器

迭代适配器用来操作一个迭代器生成另一个迭代器，惰性求值，就是在调用的时候才进行求值

```rust
let twice_vs = values.iter().map(|x| x * 2).collect::<Vec<_>>();

let tale_vs = values.iter().take(5);
```

### 泛型

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point<T> {
    x: T，
    y: T,
}

enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

impl<T, E> Result<T, E> {
    fn is_ok(&self) -> bool {
        match *self {
            OK(_) => true,
            Err(_) => false,
        }
    }
}

fn foo<T, U> (x: T, y: U) {
    // ..
}
```

### 特型

为了抽象类型的共性机制，rust提出了trait的概念

1.抽象了多种类型的共同特性  (类似于go中有方法的interface)

2.大多数方法只需要列出方法的签名，不需要包含定义

```rust
trait PrettyPrint {
    fn format(&self) -> String;
}
```

为类型实现特型

```rust
impl PrettyPrint for Point {
    fn format(&self) -> String {
        format!("{}, {}", self.x, self.y)
    }
}
```

rust中使用trait在泛型编程中对类型参数进行约束

```rust
fn cloning_machine<T: Clone>(t: T) -> (T, T) {
    (t.clone(), t.clone())
}

fn cloning_machine<T>(t: T) -> (T, T) {
    where T: Clone {
        (t.clone(), t.clone())
    }
}

fn clone_and_compare<T: Clone + Ord>(t1: T, t2: T) -> bool {
    t1.clone() > t2.clone()
}	
```

结构化泛型数据中使用特型约束

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

trait PrettyPrint {
    fn format(&self) -> String;
}

impl<T: PrettyPrint, E: PrettyPrint> PrettyPrint for Result<T, E> {
    fn format(&self) -> String {
        match *self {
            Ok(t) => format!("Ok({})", t.format()),
            Err(e) => format!("Err({})", e.format()),
        }
    }
}

trait Equals {
    fn equals(&self, other: &Self) -> bool; //&Self指的是&Result
}

impl<T: Equals, E: Equals> Equals for Result<T, E> {
    fn equals(&self, other: &Self) -> bool {
        match (*self, *other) {
            Ok(t1), Ok(t2) => t1.equals(t2),
            Err(e1), Err(e2) => e1.euqals(e2),
            _ => false
        }
    }
}
```

rust中的继承

```rust
// rust特型之间存在先后关系
// Eq需要先实现PartialEq, Copy需要先实现Clone
trait Parent {
    fn foo(&self) {
        // ...
    }
}

trait Child: Parent {
    fn bar(&self) {
        self.foo();
        // ...
    }
}
```

特性中的默认方法

```rust
trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

trait Eq: PartialEq<Self> {}
```

实现特型的时候可以改写默认方法。

但是一定要想好充分的理由来这么做，例如重新定义了ne导致违反了eq和ne之间的逻辑关系

特型的自动获得

```rust
// 一些特型实现起来比较直观，编译器可以自动完成
// 使用#[derive(...)]属性让编译器完成相应特型的自动实现
// 这样做可以避免重复手动实现诸如Clone这样的类型
#[derive(Eq, PartialEq, Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

需要注意只能自动获得下列核心特型:

Hash, Ord, Clone, Copy, Default, Eq, PartialEq, PartialOrd

可以使用宏定义来完成自定义特型的自动获取

特型的自动获得需要满足下列条件:

类型的所有成员都能自动获得指定特型

例如: Eq不能在包含f32的结构体类型上自动获得，因为f32不是Eq的

rust中的核心特型

```rust
pub trait Clone: Sized {
    fn clone (&self) -> Self;
    
    fn clone_from(&mut self, source: &Self) {...}
} // 解决所有权问题的第三种方法

#[derive(Clone)]
struct Foo {
    x: i32,
}

#[derive(Clone)]
struct Bar{
    x: Foo,
}
```

```rust
pub trait Copy: Clone {
    
} // rust中的copy语义
// 包含引用的类型不能实现copy
// 标记特型: 没有实现任何方法，只是标记行为
// 一般来说一种类型可以clone就应该可以copy
```

```rust
pub trait Debug {
    fn fmt(&self, &mut Formatter) -> Result;
}
```

```rust
pub trait Default {
    fn default () -> Self;
}
// 为类型定义默认值
```

```rust
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    
    fn ne(&self, other: &Rhs) -> bool {...};
} // rhs 操作符重载，右表达式
// PartialEq表示部分等价关系a == b b == c

pub trait Eq: PartialEq<Self> {
    
}
// Eq表示等价关系a == a,也是一种标记行为
// f32为什么不是Eq因为在rust中f32有NaN的情况,NaN != NaN
```

```rust
pub trait Hash {
    fn hash<H: Hasher> (&self, state: &mut H);
    
    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H) {
        where Self: Sized{...}
    }
}
// 表示可哈希类型
// H类型参数是抽象的哈希状态，用于计算哈希值
// 如果同时实现了Eq特型，需要满足如下性质
// k1 == k2 -> hash(k1) == hash(k2)
```

```rust
pub trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> { // ?Sized指不确定大小的类型, = Self指为他指定了默认类型
    // Ordering is one of Less, Equal, Greater
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
    
    fn lt(&self, other: &Rhs) -> bool { ... };
    fn le(&self, other: &Rhs) -> bool { ... };
    fn gt(&self, other: &Rhs) -> bool { ... };
    fn ge(&self, other: &Rhs) -> bool { ... };
} // 注意PartialEq表示可能可以进行比较的特型

// 对所有a, b, c的比较操作必须满足
// 反对称性: 若a<b,则!(a>b).
// 传递性: 若a<b,b<c则a<c
// lt le gt ge eq ne具有基于partial_cmp的默认实现
```

```rust
pub trait Ord: Eq + PartialOrd<Self> {
    fm cmp(&self, other: &Self) -> Ordering;
}
// 表示实现该特性的类型形成全序
// 完全性:类型的所有值都能够比较,f32 NaN就不能比较，f32不是Eq
```

关联类型的需求

特型中有使用到的泛型类型，例如迭代器场景type Item，为集合类型实现迭代器的同时指定了特型关联的Item的类型

```rust
trait Graph {
    type N;
    type E;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}

impl Graph for MyGraph {
    type N = MyNode;
    type E = MyEdge;
    fn edges(&self, n: &MyNode) - > Vec<MyEdge> { /* .... */};
}
```

特型的作用域

```rust
// rust中可以为任意类型实现自定义特型 
trait Foo {
    fn bar(&self) -> bool;
}

impl Foo for i32 {
    fn bar(&self) -> bool {
        true
    }
}
// 不能为系统类型实现系统特型
// 要么自定义特型，要么自定义类型
```

特型作用域规则示例

```rust
use std::Display

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {}, {}", self.x, self.y)
    }
}
// 定义{}选项的美观打印
// 用于美观打印，不能自动获得
// 可以使用write!宏来做具体实现
```

Drop特型

```rust
pub trait Drop {
    fn drop(&mut self);
}
// 表示可以销毁的特型
// 用于对象销毁，由编译器自动生成，不能显示调用
```

一般情况下不需要实现Drop因为编译器会自动实现

什么时候需要手动实现?

rust的引用计数指针Rc<T>就有特殊的Drop规则，当引用技术大于1时，drop只是对计数器减1，当时当计数器降为0时，需要删除这个Rc对象

Sized特型  ?Sized特型

Sized指编译期知道类型大小，?Sized表示类型大小不固定例如[T],str

一般来说跟指针相关的泛型的类型参数的特型约束中会出现?Sized,例如Box<T>就有Box<T: ?Sized>

### 特型对象

rust中用来实现动态分发

```rust
trait Foo {
    fn bar(&self);
}

impl Foo for String {
    fn bar(&self) { /*....*/};
}

impl Foo for usize {
    fn bar(&self) { /* ... */};
}

// 通过静态分发的方式满足约束T: Foo的任意类型上调用bar的不同版本
// 代码编译时，编译器会给每个不同的bar生成对应的特化版本，对于实现Foo特型的每种类型，都会生成对应的函数
fn blah(x: T) where T: Foo {
    x.bar()
}

fn main() {
    let s = "Foo".to_string();
    let u = 12;
    
    blah(s);
    blah(u);
}

// rust也可以通过特型对象进行动态分发
// 特型对象要用Box<dyn Foo>或&dyn Foo的形式来使用
// 背后的数据类型要实现Foo
// 当使用动态分发时，特型背后的具体数据类型会被抹去，无法获得

fn use_foo(f: &dyn Foo) {
    match *f {
        198 => println!("i32"),
        // 这里会报错，因为编译器已经抹除掉了背后的类型数据，只能调用特型包含的方法
    }
}
// 动态分发会造成一定运行时开销，但是在处理一些情况时会有用(例如动态大小的类型)，实际上特型对象只能通过指针的方式来使用，会增加指向方法的v-table
```

不是所有的特型都可以以特型对象的形式安全的使用

例如，创建&dyn Clone会引起编译错误，因为Clone不是对象安全的

特型是对象安全的，需要满足一下条件:

- 所有超特型也是对象安全的
- 不能以Sized为超特型
- 所有关联函数能够从特型对象进行分发：
  - 不带任何类型参数
  - 方法除接收方(reciver)外，其他地方不能使用Self类型
  - 接收方是引用或者某种形式的指针类型,如&self,&mut Self,Box<Self>
  - 没有where Self: Sized字句

rust更鼓励大家用静态分发的方式使用特型

### 生命周期

一般情况下编译器可以自动推断生命周期，但是当使用到多个引用或者需要返回引用的时候需要显示指定生命周期

```rust
fn bar<'a>(x: &'a i32) { // 声明一个生命周期a
}
```

```rust
fn borrow_x_or_y<'a>(x: &'a str, y: &'a str) -> 'a str {
    /* 只要返回的引用还在,那么x和y的引用也必须还在 */
}

fn borrow_p<'a, 'b>(p: &'a str, q: &'b str) -> &'a str {
    /* q的生命周期是独立的 p的借用至少要跟返回的生命周期一致 */
}
```

```rust
struct Pizza(Vec<i32>);

    struct PizzaSlice<'a> {
        pizza: &'a Pizza,
        index: u32,
    }

    let p1 = Pizza(vec![1, 2, 3, 4, 5]);
    {
        let s1 = PizzaSlice {
            pizza: &p1,
            index: 2,
        };
    }
```

生命周期之间的关系

```rust
struct Foo<'b,'a> where 'b: 'a {
    v: &'a Vec<i32>,
    s: &'b str,
}
```

impl代码段中的生命周期

```rust
struct Foo<'b,'a> where 'b: 'a {
    v: &'a Vec<i32>,
    s: &'b str,
}

impl<'b, 'a> Foo<'b, 'a>  where 'b: 'a{
    fn new(v: &'a Vec<i32>, s: &'b str) -> Foo<'b, 'a> {
        Foo { v: v, s: s }
    }
}
```

'static生命周期,在整个程序运行过程中都有效

```rust
let s1: &str = "hello";
let s2: &'static str = "world"; 
```







