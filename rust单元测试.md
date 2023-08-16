在rust中用函数来实现单元测试

```rust
#[test]
fn it_works() {
    let result = 2 + 2;
    asserteq!(result, 4);
}
```

cargo test会执行所有的测试

测试子模块

```rust
fn vector_length(data: &Vec<i32>) -> usize {
    data.len()
}

#[cfg(test)]
mod tests {
    use super::vector_length;

    #[test]
    fn test_vector_length() {
        assert_eq!(vector_length(&vec![1, 2, 3]), 3)
    }
}

```

```shell
cargo test -- --test-threads=1
cargo test -- test_vector_length
```

