```rust
pub trait ConvertT0<T> {
    fn convert(&self) -> T;
}

pub trait ConverTo {
    type DEST;
    fn convert(&self) -> Self::DEST;
}

impl ConverTo for i32 {
    type DEST = f32;
    fn convert(&self) -> Self::DEST {
        *self as Self::DEST
    }
}

impl ConvertT0<f32> for i32 {
    fn convert(&self) -> f32 {
        *self as f32
    }
}

impl ConvertT0<f64> for i32{
    fn convert(&self) -> f64 {
        *self as f64
    }
}

impl ConverTo for i32 {
    type DEST = f64;
    fn convert(&self) -> Self::DEST {
        *self as Self::DEST
    }
}
```

![](https://res.weread.qq.com/wrepub/epub_22987515_21)

思考trait的实现方式trait本身也是一种strcut对象,那么关联类型DEST相当于struct中的一个字段，而不同的泛型参数是不同的类型例如，ConvertT0<i32>和ConvertT0<f64>这是两种不同的类型，所以在编辑器的视角，如果trait有类型参数，那么不同的类型参数就是不同的trait因为生成的mir对象是不同的，可以同时针对同一个类型实现impl，但是如果没有类型参数，只有关联类型，给定关联类型是不能针对同一个类型impl的，因为他就是一种类型只是字段值不同.