# Rust入门

## 一、基本语法

### 1.1 变量

如果要声明变量要使用let关键字

~~~rust
let a=10;
~~~

使用mut关键自声明可变变量 

~~~rust
let mut a=10;
a=100;
~~~

### 1.2 常量

使用const关键字声明常量

~~~rust
const a:i32=10;
~~~

不可变变量和常量的区别：

既然不可变变量是不可变的，那不就是常量吗？为什么叫变量？

变量和常量还是有区别的。在 Rust 中，以下程序是合法的：

~~~rust
let a = 123;
let a = 456;
~~~

但是如果 a 是常量就不合法：

~~~rust
const a:i32=10;
let a=100;

~~~

变量的值可以"重新绑定"，但在"重新绑定"以前不能私自被改变，这样可以确保在每一次"绑定"之后的区域里编译器可以充分的推理程序逻辑。

### 1.3 重影（Shadowing）

重影就是所谓的"重新绑定"

重影就是指变量的名称可以被重新使用的机制：

~~~rust
let a=10;
let a=a+100;
let a="";
~~~

## 二、数据类型

### 2.1 标量类型

rust有4种标量类型：

- 整数类型
- 浮点类型
- 字符类型
- bool类型

| 8-bit   | i8    | u8    |
| ------- | ----- | ----- |
| 16-bit  | i16   | u16   |
| 32-bit  | i32   | u32   |
| 64-bit  | i64   | u64   |
| 128-bit | i128  | u128  |
| arch    | isize | usize |

isize 和 usize 两种整数类型是用来衡量数据大小的，它们的位长度取决于所运行的目标平台，如果是 32 位架构的处理器将使用 32 位位长度整型。

整数的表述方法有以下几种：

| 进制                 | 例          |
| :------------------- | :---------- |
| 十进制               | 98_222      |
| 十六进制             | 0xff        |
| 八进制               | 0o77        |
| 二进制               | 0b1111_0000 |
| 字节(只能表示 u8 型) | b'A'        |

浮点数型（Floating-Point）

Rust 与其它语言一样支持 32 位浮点数（f32）和 64 位浮点数（f64）。默认情况下，64.0 将表示 64 位浮点数，因为现代计算机处理器对两种浮点数计算的速度几乎相同，但 64 位浮点数精度更高。

~~~rust
fn main() {
  let x = 2.0; *// f64*
  let y: f32 = 3.0; *// f32*
}
~~~

布尔型

布尔型用 bool 表示，值只能为 true 或 false。

字符类型

字符型用 char 表示。

Rust的 char 类型大小为 4 个字节，代表 Unicode标量值，这意味着它可以支持中文，日文和韩文字符等非英文字符甚至表情符号和零宽度空格在 Rust 中都是有效的 char 值。

Unicode 值的范围从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF （包括两端）。 但是，"字符"这个概念并不存在于 Unicode 中，因此您对"字符"是什么的直觉可能与Rust中的字符概念不匹配。所以一般推荐使用字符串储存 UTF-8 文字（非英文字符尽可能地出现在字符串中）。

**注意：**由于中文文字编码有两种（GBK 和 UTF-8），所以编程中使用中文字符串有可能导致乱码的出现，这是因为源程序与命令行的文字编码不一致，所以在 Rust 中字符串和字符都必须使用 UTF-8 编码，否则编译器会报错。

### 2.2 复合类型

rust提供两种原始的复合类型tuple元组和array数组

~~~rust
//声明tuple tuple里面的元素类型不必一致
let tup: (i32, f64, bool) = (10, 1.9, false);
println!("{}-{}-{}", tup.0, tup.1, tup.2);
//解构tuple
let (x,y,z)=tup;
//声明数组 数组表示同种类型数据的集合，长度固定
let number=[1,2,3,4,5];
let a=[3:5]//等价于 let a=[3,3,3,3,3]
//访问数组元素 使用数组索引

~~~

### 2.3 数学运算

~~~RUST
fn main() {
    let sum = 5 + 10; // 加
    let difference = 95.5 - 4.3; // 减
    let product = 4 * 30; // 乘
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余
}
~~~

## 三、函数和注释

## 四、控制流

### 4.1 if表达式 

如果使用了多个else if最好使用match表达式重构

~~~rust
fn main() {
    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    }else if a < 0 {
        b = -1;
    }else {
        b = 0;
    }
    println!("b is {}", b);
}
~~~

### 4.2 循环

#### 4.2.1 loop循环

~~~RUST
loop {
  
}
~~~

#### 4.2.2 while 循环

~~~rust
while true {
}
~~~

#### 4.2.3 for循环

~~~rust
let a=[10,20,30,40,50];
for elemet in a.iter(){
   println!("{}", elemet);
}
let a = [10, 20, 30, 40, 50];
for i in 0..5 {
   println!("a[{}] = {}", i, a[i]);
}
~~~

## 五、所有权

### 5.1所有权规则：

- 每个值都有一个变量，这个变量是该值的所有者
- 每个值同时只能有一个所有者
- 当值超过作用域时，该值将被删除

### 5.2 移动

变量与数据交互方式主要有移动（Move）和克隆（Clone）两种：

~~~rust
let x = 5;
let y = x;
~~~

这个程序将值 5 绑定到变量 x，然后将 x 的值复制并赋值给变量 y。现在栈中将有两个值 5。此情况中的数据是"基本数据"类型的数据，不需要存储到堆中，仅在栈中的数据的"移动"方式是直接复制，这不会花费更长的时间或更多的存储空间。"基本数据"类型有这些：

- 所有整数类型，例如 i32 、 u32 、 i64 等。
- 布尔类型 bool，值为 true 或 false 。
- 所有浮点类型，f32 和 f64。
- 字符类型 char。
- 仅包含以上类型数据的元组（Tuples）。

但如果发生交互的数据在堆中就是另外一种情况：

~~~rust
let s1 = String::from("hello");
let s2 = s1; 
println!("{}, world!", s1); // 错误！s1 已经失效
~~~

第一步产生一个 String 对象，值为 "hello"。其中 "hello" 可以认为是类似于长度不确定的数据，需要在堆中存储。

第二步的情况略有不同（**这不是完全真的，仅用来对比参考**）：

### 5.3 克隆

Rust会尽可能地降低程序的运行成本，所以默认情况下，长度较大的数据存放在堆中，且采用移动的方式进行数据交互。但如果需要将数据单纯的复制一份以供他用，可以使用数据的第二种交互方式——克隆。

~~~rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
~~~

运行结果：

~~~
s1 = hello, s2 = hello
~~~

### 5.4 函数返回值的所有权机制

实例

~~~rust
fn main() {
  let s1 = gives_ownership();
  *// gives_ownership 移动它的返回值到 s1*

  let s2 = String::from("hello");
  *// s2 被声明有效*

  let s3 = takes_and_gives_back(s2);
  *// s2 被当作参数移动, s3 获得返回值所有权*
} *// s3 无效被释放, s2 被移动, s1 无效被释放.*

fn gives_ownership() -> String {
  let some_string = String::from("hello");
  *// some_string 被声明有效*

  return some_string;
  *// some_string 被当作返回值移动出函数*
}

fn takes_and_gives_back(a_string: String) -> String { 
  *// a_string 被声明有效*

  a_string *// a_string 被当作返回值移出函数*
}
~~~

被当作函数返回值的变量所有权将会被移动出函数并返回到调用函数的地方，而不会直接被无效释放。

### 5.5 引用与租借

引用（Reference）是 C++ 开发者较为熟悉的概念。

如果你熟悉指针的概念，你可以把它看作一种指针。

实质上"引用"是变量的间接访问方式。

实例

~~~rust
fn main() {
  let s1 = String::from("hello");
  let s2 = &s1;
  println!("s1 is {}, s2 is {}", s1, s2);
}
~~~

运行结果：

```
s1 is hello, s2 is hello
```

**&** 运算符可以取变量的"引用"。

当一个变量的值被引用时，变量本身不会被认定无效。因为"引用"并没有在栈中复制变量的值：

![img](https://www.runoob.com/wp-content/uploads/2020/04/F25111E7-C5D3-464A-805D-D2186A30C8A0.jpg)

函数参数传递的道理一样：

实例

~~~rust
fn main() {
  let s1 = String::from("hello");

  let len = calculate_length(&s1);

  println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}
~~~

运行结果：

```
The length of 'hello' is 5.
```

引用不会获得值的所有权。

引用只能租借（Borrow）值的所有权。

引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权：

实例

~~~rust
fn main() {
  let s1 = String::from("hello");
  let s2 = &s1;
  let s3 = s1;
  println!("{}", s2);
}
~~~

这段程序不正确：因为 s2 租借的 s1 已经将所有权移动到 s3，所以 s2 将无法继续租借使用 s1 的所有权。如果需要使用 s2 使用该值，必须重新租借：

实例

~~~rust
fn main() {
  let s1 = String::from("hello");
  let mut s2 = &s1;
  let s3 = s2;
  s2 = &s3; *// 重新从 s3 租借所有权*
  println!("{}", s2);
}
~~~

这段程序是正确的。

既然引用不具有所有权，即使它租借了所有权，它也只享有使用权（这跟租房子是一个道理）。

如果尝试利用租借来的权利来修改数据会被阻止：

实例

~~~rust
fn main() {
  let s1 = String::from("run");
  let s2 = &s1;
  println!("{}", s2);
  s2.push_str("oob"); *// 错误，禁止修改租借的值*
  println!("{}", s2);
}
~~~

这段程序中 s2 尝试修改 s1 的值被阻止，租借的所有权不能修改所有者的值。

当然，也存在一种可变的租借方式，就像你租一个房子，如果物业规定房主可以修改房子结构，房主在租借时也在合同中声明赋予你这种权利，你是可以重新装修房子的：

实例

~~~rust
fn main() {
  let mut s1 = String::from("run");
  *// s1 是可变的*

  let s2 = &mut s1;
  *// s2 是可变的引用*

  s2.push_str("oob");
  println!("{}", s2);
}
~~~

这段程序就没有问题了。我们用 &mut 修饰可变的引用类型。

可变引用与不可变引用相比除了权限不同以外，可变引用不允许多重引用，但不可变引用可以：

实例

~~~rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
~~~

这段程序不正确，因为多重可变引用了 s。

Rust 对可变引用的这种设计主要出于对并发状态下发生数据访问碰撞的考虑，在编译阶段就避免了这种事情的发生。

由于发生数据访问碰撞的必要条件之一是数据被至少一个使用者写且同时被至少一个其他使用者读或写，所以在一个值被可变引用时不允许再次被任何引用。

### 5.6 垂悬引用（Dangling References）

这是一个换了个名字的概念，如果放在有指针概念的编程语言里它就指的是那种没有实际指向一个真正能访问的数据的指针（注意，不一定是空指针，还有可能是已经释放的资源）。它们就像失去悬挂物体的绳子，所以叫"垂悬引用"。

"垂悬引用"在 Rust 语言里不允许出现，如果有，编译器会发现它。

下面是一个垂悬的典型案例：

实例

~~~rust
fn main() {
  let reference_to_nothing = dangle();
}

fn dangle() -> &String {
  let s = String::from("hello");

  &s
}
~~~

很显然，伴随着 dangle 函数的结束，其局部变量的值本身没有被当作返回值，被释放了。但它的引用却被返回，这个引用所指向的值已经不能确定的存在，故不允许其出现。

### 5.7 切片

切片（Slice）是对数据值的部分引用。

字符串切片

最简单、最常用的数据切片类型是字符串切片（String Slice）。

```rust
fn main() {
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);
}
```

使用 **..** 表示范围的语法在循环章节中出现过。**x..y** 表示 **[x, y)** 的数学含义。**..** 两边可以没有运算数：

```
..y 等价于 0..y
x.. 等价于位置 x 到数据结束
.. 等价于位置 0 到结束
```

**注意：**到目前为止，尽量不要在字符串中使用非英文字符，因为编码的问题。具体原因会在"字符串"章节叙述。

被切片引用的字符串禁止更改其值：

```rust
fn main() {
    let mut s = String::from("runoob");
    let slice = &s[0..3];
    s.push_str("yes!"); // 错误
    println!("slice = {}", slice);
}
```

非字符串切片

除了字符串以外，其他一些线性数据结构也支持切片操作，例如数组：

```rust
fn main() {
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }
}

```

## 六、结构体struct

### 6.1 struct的定义

```rust
struct User {
    username: String,
    email: String,
    password: String,
    active: bool,
}
```

### 6.2 实例化struct

```rust
let user = User {
        username: String::from("zhangshan"),
        password: String::from("123456"),
        email: String::from("example@test.com"),
        active: false,
    };
```

### 6.3 元组结构体

有一种更简单的定义和使用结构体的方式：**元组结构体**。

元组结构体是一种形式是元组的结构体。

与元组的区别是它有名字和固定的类型格式。它存在的意义是为了处理那些需要定义类型（经常使用）又不想太复杂的简单数据：

```rust
struct Color(u8, u8, u8);
struct Point(f64, f64);

let black = Color(0, 0, 0);
let origin = Point(0.0, 0.0);
```

"颜色"和"点坐标"是常用的两种数据类型，但如果实例化时写个大括号再写上两个名字就为了可读性牺牲了便捷性，Rust 不会遗留这个问题。元组结构体对象的使用方式和元组一样，通过 . 和下标来进行访问：

```rust
fn main() {
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);
}
```

运行结构：

```rust
black = (0, 0, 0)
origin = (0, 0)
```

### 6.4 结构体方法

方法（Method）和函数（Function）类似，只不过它是用来操作结构体实例的。

如果你学习过一些面向对象的语言，那你一定很清楚函数一般放在类定义里并在函数中用 this 表示所操作的实例。

Rust 语言不是面向对象的，从它所有权机制的创新可以看出这一点。但是面向对象的珍贵思想可以在 Rust 实现。

结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字。

计算一个矩形的面积：

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
   
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1's area is {}", rect1.area());
}
```

输出结果：

```
rect1's area is 1500
```

### 6.5 结构体关联函数

之所以"结构体方法"不叫"结构体函数"是因为"函数"这个名字留给了这种函数：它在 impl 块中却没有 &self 参数。

这种函数不依赖实例，但是使用它需要声明是在哪个 impl 块中的。

一直使用的 **String::from** 函数就是一个"关联函数"。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect = Rectangle::create(30, 50);
    println!("{:?}", rect);
}
```

运行结果：

```
Rectangle { width: 30, height: 50 }
```

> **贴士：**结构体 impl 块可以写几次，效果相当于它们内容的拼接！

## 七、枚举与模式匹配

### 7.1 枚举的定义

枚举允许我们列举所有可能的值来定义一个类型

```rust
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}
let v4 = IpAddrKind::V4(192, 168, 0, 1);
let v6 = IpAddrKind::V6(String::from("::1"));
```

### 7.2 Option 枚举类

Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白。

许多语言支持 null 的存在（C/C++、Java），这样很方便，但也制造了极大的问题，null 的发明者也承认这一点，"一个方便的想法造成累计 10 亿美元的损失"。

null 经常在开发者把一切都当作不是 null 的时候给予程序致命一击：毕竟只要出现一个这样的错误，程序的运行就要彻底终止。

为了解决这个问题，很多语言默认不允许 null，但在语言层面支持 null 的出现（常在类型前面用 ? 符号修饰）。

Java 默认支持 null，但可以通过 @NotNull 注解限制出现 null，这是一种应付的办法。

Rust 在语言层面彻底不允许空值 null 的存在，但无奈null 可以高效地解决少量的问题，所以 Rust 引入了 Option 枚举类：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

如果你想定义一个可以为空值的类，你可以这样：

```rust
let opt = Option::Some("Hello");
```

如果你想针对 opt 执行某些操作，你必须先判断它是否是 **Option::None**：

```rust
fn main() {
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}
```

运行结果：

```rust
Hello
```

如果你的变量刚开始是空值，你体谅一下编译器，它怎么知道值不为空的时候变量是什么类型的呢？

所以初始值为空的 Option 必须明确类型：

```rust
fn main() {
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}
```

运行结果：

```
opt is nothing
```

这种设计会让空值编程变得不容易，但这正是构建一个稳定高效的系统所需要的。由于 Option 是 Rust 编译器默认引入的，在使用时可以省略 Option:: 直接写 None 或者 Some()。

Option 是一种特殊的枚举类，它可以含值分支选择：

```rust
fn main() {
        let t = Some(64);
        match t {
                Some(64) => println!("Yes"),
                _ => println!("No"),
        }
}
```

### 7.3 if let 语法

```rust
let i = 0;
match i {
    0 => println!("zero"),
    _ => {},
}
```

使用if let

```rust
let i = 0;
if let 0 = i {
    println!("zero");
}
```

if let 语法格式如下：

```rust
if let 匹配值 = 源变量 {
    语句块
}
```

可以在之后添加一个 else 块来处理例外情况。

if let 语法可以认为是只区分两种情况的 match 语句的"语法糖"（语法糖指的是某种语法的原理相同的便捷替代品）。

对于枚举类依然适用

```rust
fn main() {
    enum Book {
        Papery(u32),
        Electronic(String)
    }
    let book = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}
```

## 八、组织管理

### 8.1 Package和crate

 crate的类型

- binary
- library

Crate Root

> 是源代码文件
>
> rust编译器从这里开始 ，组成你的crate的根module

一个package

>- 包含一个Cargo.toml 他描述如何构建这些Crates
>
>- 只能包含0-1个library crate
>
>- 可以有任意多的binary crate
>
>- 但必须至少包含一个crate（library或binary）

示例：如下目录结构

```
src
|-common
|--mod.rs
|--uks.rs
|-model
|--mod.rs
|--user.rs
|-main.rs
```

common/mod.rs

```rust
pub mod uks;//使用pub关键字暴露对外访问
pub fn test() {
    println!("hello world");
}
```

common/uks.rs

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

model/mod.rs

```rust
pub mod user;
```

model/user.rs

```rust
pub fn get_test() {
    println!("hello test");
}
```

main.rs

```rust
mod model;//mod划分代码作用域
use model::user;//use引入当前作用域
mod common;
use common::uks;
fn main() {
  	user::get_test();
    let total = uks::add(10, 90);
    println!("total:{}", total);
    common::test();
}
```

运行结果：

```
hello test
total:100
hello world
```

## 九、常用的集合

常用容器

- Vector
- String
- HashMap

### 9.1 Vector

使用vector存储多个相同类型的值

Vec<T>

#### 9.1.1 创建vector

使用Vec::new函数

```rust
let v: Vec<i32> = Vec::new();
```

通常使用初始值的方式创建vector,使用vec!宏

```rust
let v = vec![1, 2, 3];
```

#### 9.1.2 添加元素 push

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
```

#### 9.1.3 读取元素

使用两种方式读取vector种的元素

1. 使用索引的方式 

2. 使用get

   >注意：使用索引访问时，当索引超过vector的范围时会引起恐慌

```rust
let v = vec![1, 2, 3, 4, 5];
let third = &v[2];
println!("the third element is: {}", third);
match v.get(2) {
    Some(third) => println!("the third element is: {}", third),
    None => println!("this is no third element"),
};
```

所有权规则：

> 在同一作用域中不能同时使用不可变和可变的应用

```rust
let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0];
v.push(6);
println!("the first element is:{}", first);
```

输出结果：

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:7:5
  |
6 |     let first = &v[0];
  |                  - immutable borrow occurs here
7 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
8 |     println!("the first element is:{}", first);
  |                                         ----- immutable borrow later used here
```

遍历vector

```rust
let v = vec![1, 2, 3, 4, 5];
for element in &v {
    println!("element is:{}", element);
}
```

修改vector

```rust
let mut v = vec![1, 2, 3, 4, 5];
for element in &mut v {
    *element += 1;
    println!("element is:{}", element);
}
```

#### 9.1.4 vector+enum

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main{
  let row = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(88.8),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}
```

### 9.2 String

rust中通常的字符串指的是String和&str字符串切片

#### 9.2.1 创建String

使用new关键字

```rust
let mut s = String::new();
```

使用初始值创建 to_string或String::from

```rust
let data = "init content";
let s = data.to_string();
let s1 = String::from("init content");
```

#### 9.2.2 更新String

```rust
let mut s = String::from("hello");
s.push_str("world");
s.push('!');
println!("{}", s);
```

#### 9.2.3 字符串拼接

使用+号拼接:

> 使用+号拼接字符串会获得第一个字符串的所有权

```rust
let s1 = String::from("hello");
let s2 = String::from(" world");
let s3 = s1 + &s2;//在这里s1的所有权被移动了
println!("s1:{}", s1);//这里会发生错误 因为s1的所有权已经被移动到add里面去了
println!("s2:{}", s2);
println!("s3:{}", s3);
```

输出结果：

```
let s1 = String::from("hello");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
12 |     let s2 = String::from(" world");
13 |     let s3 = s1 + &s2;
   |              -- value moved here
14 |     println!("s1:{}", s1);
   |                       ^^ value borrowed here after move
```

使用format!拼接

> 使用format!不会获取参数的所有权

```rust
let s1 = String::from("hello");
let s2 = String::from(" world");
let s3 = format!("{}-{}", s1, s2);
println!("s1:{}", s1);
println!("s2:{}", s2);
println!("s3:{}", s3);
```

输出结果：

```
s1:hello
s2: world
s3:hello- world
```

#### 9.2.4 切割字符串

> 注意：String类型不支持按照索引进行访问

可以使用[]和一个范围来创建字符串切片

```rust
let s = String::from("hello world");
let s1 = &s[0..4];
println!("{}", s1);
```

> 注意：使用字符串切片的时，索引必须沿着字符边界切割，否则会引发恐慌。

### 9.3 HashMap

HashMap定义 : 使用键值对存储数据 一个键（Key）对应一个值（Value）

HashMap<K,V>

#### 9.3.1 创建HashMap

使用new创建

```rust
let mut scores: HashMap<String, i32> = HashMap::new();
score.insert(String::from("math"), 90);
```

使用collect创建

```rust
let teams = vec![String::from("blue"), String::from("yellow")];
let intial_score = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(intial_score.iter()).collect();
//zip 接受一个参数，将调用者的元素和参数中的元素 组合成一个tuple 若数量不匹配多余的元素会被丢弃
//collect 方法返回不同类型的集合数据结构 所以需要手动指定HashMap类型告诉编译器
```

#### 9.3.2 HashMap 和所有权

- 对于实现了Copy trait的类型（如：i32），值会被复制到HashMap中
- 对于拥有所有权的值（如：String），值会被移动，所有权转移到HashMap中
- 如果将值的引用插入到HashMap中，值本身不会移动

#### 9.3.3 读取HashMap的值

使用get方法获取HashMap的值 返回一个Option

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
let team_name = String::from("Blue");
let score = scores.get(&team_name);
match score {
   Some(s) => println!("got score:{}", s),
   None => println!("team not exist"),
}
```

运行结果：

```
got score:10
```

使用for 遍历：

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
for (k, v) in &scores {
   println!("{}-{}", k, v);
}
```

输出结果：

```
Blue-10
Yellow-50
```

#### 9.3.4 更新HashMap

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

scores.insert(String::from("Blue"), 25); //如果存在覆盖原来的值 不存在则插入
scores.entry(String::from("Blue")).or_insert(60); //先判断key是否存在 不存在则插入
```

```rust
fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
```

输出结果：

```
{
    "hello": 1,
    "wonderful": 1,
    "world": 2,
}
```

## 十、错误处理

错误的分类

- 可恢复的错误 如：文件未找到，可再次尝试
- 不可恢复的错误 如：访问数组超出边界

### 10.1 不可恢复的错误panic！

```rust
fn main() {
    panic!("error occured");
    println!("Hello, Rust");
}
```

运行结果：

```
thread 'main' panicked at 'error occured', src\main.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

### 10.2 可恢复的错误

#### 10.2.1 Result枚举

定义：

```rust
enum Result <T,E>{
  Ok(T),
  Err(E),
}
```

T:操作成功的情况下，Ok变体里返回的数据类型

E:操作失败的情况，Err变体里返回的错误类型

处理Result的一种方式：match表达式

```rust
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("error create file:{:?}", e),
            },
            oe => panic!("Error opening file error:{:?}", oe),
        },
    };
}
```

> 上面使用了很多match，match很有用但是原始。

简化代码：

```rust
let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("Error create file:{:?}", e);
            })
        } else {
            panic!("Error opening file:{:?}", error);
        }
    });
```

unwrap:match表达式的一个快捷方法：

> 如果Result是Ok，返回Ok里面的值
>
> 如果Result是Err，调用panic!宏

上述代码：

```rust
let f = File::open("hello.txt");
let f = match f {
    Ok(file) => file,
    Err(err) => panic!("open file err:{:?}", err),
};
```

等价于：

```rust
let f = File::open("hello.txt").unwrap();
```

expect:和unwrap类似，但可指定错误信息

```rust
let f = File::open("hello.txt").expect("无法打开文件");
```

#### 10.2.2 传播错误

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

?运算符：传播错误的一种快捷方式

如果Reuslt是Ok，Ok的值就是表达式的结果，然后继续执行程序

如果Result是Err，Err就是整个函数的返回值，相当于return

使用?号简化：

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}
```

链式调用简化：

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```



?与from函数

![image-20211011164917083](/Users/haozuoyang/Library/Application Support/typora-user-images/image-20211011164917083.png)

> ?号运算符只能用于返回结果为Result的函数

?与main函数：

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}

```

### 10.3 错误处理原则

总体原则：

- 在定义一个可能失败的函数时，优先考虑返回Result

- 否则panic!

  - 演示某些概念：unwrap

  - 原型代码：unwrap, expect

  - 测试代码：unwrap, expect

  - 可以确定Result就是Ok：unwrap

    ```rust
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    ```

当代码最终可能处于损害状态时，最好使用panic！

1. 场景1：调用你的代码传入了无意义的参数：panic!
2. 场景2:  调用外部不可控代码，返回非法值，无法修复时 panic！
3. 如果失败可以预期 Result
4. 当你的代码对值进行操作，首先应该先验证这些值，panic！

## 十一、范型、trait、声明周期

### 11.1 范型

泛型是具体类型或其他属性的抽象替代。我们可以表达泛型的属性，比如他们的行为或如何与其他泛型相关联，而不需要在编写和编译代码时知道他们在这里实际上代表什么。

#### 11.1.1 在函数定义中使用范型

当使用泛型定义函数时，本来在函数签名中指定参数和返回值的类型的地方，会改用泛型来表示。采用这种技术，使得代码适应性更强，从而为函数的调用者提供更多的功能，同时也避免了代码的重复。

示例：查找最大的数

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut larger = list[0];
    for item in list.iter() {
        if *item > larger {
            larger = *item
        }
    }
    larger
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

```

输出结果：

```
The largest number is 100
The largest char is y
```

#### 11.1.2 在结构体中使用范型

示例1:

```rust
struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let integer = Point { x: 12, y: 23 };
    let float = Point { x: 1.2, y: 2.3 };
}
```

示例2:

```rust
struct Point<T,V> {
    x: T,
    y: V,
}
fn main() {
    let point1 = Point { x: 12, y: 2.3 };
    let point2 = Point { x: 1.2, y: 2.3 };
}
```

#### 11.1.3 在枚举中使用范型

示例：

```rust
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

#### 11.1.4 在方法中使用范型

> 注意：
>
> - 把T放在impl关键字后，表示在类型T上实现方法
>
>   例如：impl<T> Point<T>
>
> - 只针对具体类型实现方法 
>
>   例如：impl  Point<f32>

```rust
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    
}

```

struct的范型类型参数可以和方法的范型类型参数不同

```rust

struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    let p1 = Point { x: 5, y: 4 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x={},p3.y={}", p3.x, p3.y);
}

```

输出结果：

```
p3.x=5,p3.y=c
```

#### 11.1.5 范型代码的性能

在阅读本部分内容的同时，你可能会好奇使用泛型类型参数是否会有运行时消耗。好消息是：Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失。

Rust 通过在编译时进行泛型代码的 **单态化**（*monomorphization*）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

### 11.2 trait:定义共享行为

