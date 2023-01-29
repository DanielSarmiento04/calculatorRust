# Calculator

As you know rust have not a null datatype


## Concept 

In rust is so common that function return a option or result type.

- Option type:
    This is used to refer to result values, when function encapsulates a result in any type, include None type, vec <T>, Some etc
```
pub enum Option<T> {
    None,
    Some(T),
}
```

- Result
    This is used to refer the error, when function returns a mistake, it can define with Generic values

```
pub enum Result <T, E> {
    Ok(T),
    Err(E),
}
```

## Steps

1. clone the repo.

```bash
git clone https://github.com/DanielSarmiento04/calculatorRust

cd calculatorRust
```

2. Download dependencies

```bash

cargo build
```
3. Run the Project

```bash
cargo run
```



## Reference

[Fundamental Rust Course of Platzi](https://platzi.com/clases/3077-rust-basico/48992-creando-nuestra-calculadora/)

[Log Rocket](https://blog.logrocket.com/understanding-rust-option-results-enums/)