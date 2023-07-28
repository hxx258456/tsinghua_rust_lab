# 理解Rust中的str,&str,String的区别

很多rust的初学者都会对rust的字符串设计感到困惑，即使字符串不可变为什么不是str类型而是&str，大多数rust半吊子的解释就是直接搬运官方文档str是DST类型，编译器无法确定str类型的大小

然而给我的直观感受是"hello"字面量只有5个字节，怎么就不能确定大小了？

在这里纠结了好久，直到看了RFC关于dst的文章，才知道把值大小和类型大小混淆了

"hello"你看到的只是值大小，并不是类型大小

思考下面使用场景

```rust
let hello: str = "hello";
let other: str = "hello world!";
let str_attr = vec![hello, other];
```

我们会发现hello是5字节，但是other却有12字节，vec中要求每个元素的字节大小和对齐边界是一致的，那么str到底采用几个字节呢？所以官方的dst指的是类型大小不确定

由于变量在stack上分配时，必须确定类型大小，因此rust为了能够使得变量绑定字符串，引入了字符串切片类型&str,&str是胖指针，类似于golang的字符串实现，&str是一个struct，由两部分构成数据起始地址，长度,结构体大小是确定的因此&str可以存储在stack上

同时由于字符串字面量不可变(长度不可变)，因此Rust引入了可变字符串类型String，String相比于&str增加了一个容量字段，String存储在heap上，之所以设计String是因为实际业务开发中，你从用户输入端取到的一定是String而绝不是str/&str，当你拿到String，你可以把把他传给任意接受&str,String,AsRef<str>的函数/方法进行处理

需要注意的是&str指向的数据不一定分配在heap上，它可以存储在以下区域

- 静态区:&`static str和硬编码的字符串都是经过编译时计算直接存储在二进制文件中，程序启动时加载到静态区的
- stack:分配在栈上的字节数组，可以转换为&str类型
- heap:基于String生成的切片s[1..]



![](https://github.com/hxx258456/tsinghua_rust_lab/blob/main/images/rust_string_str.png?raw=true)