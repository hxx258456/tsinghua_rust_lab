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

  

- 







