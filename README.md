# Rust_hw2

总共有三个小练习，在此陈述实现的关键思路。

### Generic Data Type

对于`buffer`中的`vec`类型成员进行求和运算，规定了泛型需要满足`std::ops::Add + std::ops::AddAssign + Copy`的特性

### String Compare

- 通过`chars()`和`collect()`方法将`&str`转化为字符类型，方便比较
- 取两个`&str`变量的较小长度，对其中的字符一一进行比较，如果出现不一致，返回函数
- 如果一致，比较两者长度，得到返回结果

### Closure and Iterator

通过`map()`的闭包对`vec`类型变量进行操作

闭包中，通过`as`运算符将`char`转变为`u8`，加一后，再通过`as`将`u8`转换为`char`