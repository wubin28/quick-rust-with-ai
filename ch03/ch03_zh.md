# 第3章 记忆力挑战游戏与变量

为了让小小白在学习Rust时不会因为教材枯燥而犯困，编写游戏是最好的方式。从本章开始，我会和小小白一起用Rust编写一系列游戏，在实现功能的过程中讲解相关的Rust知识点，让学习变得更有趣。

本章我们将编写一个名为"记忆力挑战"的游戏。在开始编写代码之前，让我们先来了解一下游戏规则。

# 3.1 如何玩记忆力挑战游戏

记忆力挑战游戏旨在锻炼记忆力和手眼协调能力。让我先简单介绍游戏的大致玩法。为什么不一开始就详细说明呢？这是因为对初学者和专业程序员来说，实现一个全新游戏时，每个环节都很陌生，很难在开始时就规划得清晰。开发路线往往是在经历挫折后才逐渐明朗起来。不过别担心，我们会采用**小步迭代**的方法，循序渐进地实现这个游戏。这种方式让我们能在开发过程中逐步理清思路，并及时改进。虽然接下来的 GIF 动图是在游戏完成后录制的（你会看到我按了最初没有考虑到的按钮 B），但我会在说明中保留最初探索时的思考过程，让你能**真实体会软件开发中的摸索历程**。

游戏开始前，LED点阵显示一个固定的笑脸来欢迎玩家。

准备就绪后，点阵会随机显示一幅目标图案供玩家记忆，然后玩家按左边的A按钮确认已记住图案。随后，点阵每隔1秒会随机显示一幅新图案。玩家需要判断当前显示的图案是否与最初的目标图案相同。如果认为图案相同，玩家按下A按钮选择。

如果玩家选择的图案与目标图案不一致，点阵会显示一个哭脸，如图3-1所示：

![f3-1.gif](f3-1.gif)

图3-1 当玩家选择的图案与目标图案不匹配时，点阵会显示一个哭脸

当图案匹配时，点阵会显示一个大大的笑脸作为祝贺，如图3-2所示：

![f3-2.gif](f3-2.gif)

图3-2 当图案匹配时，显示一个大大的笑脸

游戏结束后，显示屏会自动返回固定笑脸状态，准备开始新一轮游戏。

现在我们已经了解了游戏的大致玩法，让我们开始实现第一个功能：在游戏开始时显示固定的笑脸。

# 3.2 迭代1：显示固定笑脸

由于我们之前使用`cargo generate`生成的LED灯代码模板中已包含运行Rust嵌入式程序所需的依赖配置和初始代码，接下来让我们像第1章那样运行以下命令来创建并启动LED灯的Rust项目：

```bash
cargo generate wubin28/mb2-led-template
# 当工具提示输入Project Name时，可以输入项目名mem-match
# 运行下面的命令看到一个LED灯亮起就可以安心写新代码了
cargo run
```

接下来小小白该如何编写显示固定笑脸的程序？在过去，小小白只能通过搜索前人写过的代码、在讨论群里提问或翻阅书籍来解决问题，这些方法都很耗时费力。但现在有了AI助手，只要问它"请在我给你提供的点亮LED灯代码的基础上，实现显示固定笑脸的程序"，就能快速获得一份代码。

需要注意的是，AI也有其局限性。它第一次给出的代码大约有80%的概率会在运行时报错。这时你需要将错误信息和代码反馈给AI，请它分析并提供解决方案。即便按照AI的建议修改，往往还会遇到新的错误，可能需要反复几轮才能得到可运行的代码。即使代码能运行，也可能存在诸多问题：功能不完整、超出需求范围，或是代码可读性差。这些都需要你仔细审查和改进。这恰恰体现了学习Rust编程语言的价值——它让我们能够判断AI给出的代码是否合理。在AI的助推下，程序员以及入门Rust编程后的小小白，正在从"代码编写者"逐渐转变为"代码鉴赏家"。

让我们来鉴赏AI给出的迭代1代码（运行结果如图3-3所示）。为了简洁明了，本书只展示最终可运行的代码，省略了与AI多轮交互来修复代码错误的过程。

![f3-3.jpg](f3-3.jpg)

图3-3 迭代1：显示固定笑脸

与1.3.4中生成的代码相比，迭代1只修改了两个文件。第一处修改是在Cargo.toml文件中删除了以下这行代码，因为显示固定笑脸并不需要这个依赖：

```toml
# 删除了下面这个依赖
embedded-hal = "1.0.0"
```

第二处修改是main.rs文件，代码清单3-1展示了修改后的内容（带注释的部分一般是本次改过的代码，下同）：

代码清单3-1 d4f16b3, ch03/mem-match/src/main.rs

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
// 用于在LED点阵显示图案
use microbit::display::blocking::Display;
// 硬件定时器，作为调用display.show()函数显示图案的参数
use microbit::hal::Timer;
use panic_halt as _;

// 定义一个5x5的笑脸图案
// 1表示LED点亮,0表示LED熄灭
static SMILEY: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0], // 两个眼睛
    [0, 1, 0, 1, 0], // 两个眼睛
    [0, 0, 0, 0, 0], // 空行
    [1, 0, 0, 0, 1], // 嘴巴的两端
    [0, 1, 1, 1, 0], // 嘴巴的中间部分
];

#[entry]
fn main() -> ! {
    // 不再声明为mut可变变量，因为不需要
    let board = Board::take().unwrap();
    // 初始化LED显示模块
    let mut display = Display::new(board.display_pins);
    // 初始化硬件定时器
    let mut timer = Timer::new(board.TIMER0);
    // 创建显示缓冲区，用于存储要显示的图案
    let mut display_buffer = [[0u8; 5]; 5];

    // 将SMILEY图案转换为显示亮度值
    // 1转换为亮度值9(最亮),0转换为亮度值0(关闭)
    for row in 0..5 {
        for col in 0..5 {
            display_buffer[row][col] = if SMILEY[row][col] == 1 { 9 } else { 0 };
        }
    }

    // 无限循环显示笑脸图案，每100毫秒刷新一次显示
    loop {
        display.show(&mut timer, display_buffer, 100);
    }
}
```

注意，代码清单3-1的标题"d4f16b3, ch03/mem-match/src/main.rs"中，d4f16b3是git提交号，ch03/mem-match/src/main.rs是文件位置。由于文件位置会随着代码迭代不断变化，我们需要用提交号来准确定位特定版本的代码。要查看d4f16b3提交版本的代码，请运行以下git命令（后续不再重复说明）：

```bash
# 创建名为d4f16b3的临时分支，以查看那次提交的代码
git checkout -b d4f16b3 d4f16b3
# 看完代码后，回到main分支
git checkout main
```

🧠代码清单3-1中的变量`display_buffer`为何不命名为`displayBuffer`或`display-buffer`？

💡变量`display_buffer`使用了一种特别的命名风格：**用下划线连接多个英文单词**。这就是Rust代码中常用的snake_case命名法——变量名中的单词用下划线串联，像蛇一样蜿蜒前行。Rust的函数和方法也遵循这种命名方式。不过在crate.io上的代码库名称则采用kebab-case风格（像串在烤串上一样用连字符相连）。

## **3.2.1 重构代码**

在注释的帮助下，这份代码是否容易理解？对初学者来说，这段代码看起来已经相当清晰。然而，有经验的专业程序员会发现代码中存在**魔法数（magic number）**的问题。

什么是魔法数？就是那些在代码中直接使用但含义不明确的数字。比如代码清单3-1中的LED点阵大小5、LED最大亮度9、最低亮度0以及延时100毫秒。这些数字看起来似乎不需要解释。确实，当你刚写完代码时，这些数字的含义清清楚楚。但几周后当你需要修改功能或修复问题时再看这段代码，可能就要花时间重新理解这些数字的含义了。

解决这个问题很简单：我们只需要将这些魔法数字提取为常量，并给它们起一个能清晰表达用途的名字。这种提取常量的重构方法不仅提高了代码的可读性，而且不会改变程序的行为，是一个非常实用的改进方法。让我们看看代码清单3-2中重构后的代码。相信你也会发现，这个版本的可读性要好得多（为了节省篇幅，后面我们只展示重构后的代码）：

代码清单3-2 5ecb2e8f, ch03/mem-match/src/main.rs

```bash
// （其他行略）
onst MATRIX_DIMENSION: usize = 5;
const MAX_BRIGHTNESS: u8 = 9;
const MIN_BRIGHTNESS: u8 = 0;
const DURATION_100_MS: u32 = 100;

static SMILEY: [[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION] = [
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut display_buffer = [[MIN_BRIGHTNESS; MATRIX_DIMENSION]; MATRIX_DIMENSION];

    for row in 0..MATRIX_DIMENSION {
        for col in 0..MATRIX_DIMENSION {
            display_buffer[row][col] = if SMILEY[row][col] == 1 {
                MAX_BRIGHTNESS
            } else {
                MIN_BRIGHTNESS
            };
        }
    }

    loop {
        display.show(&mut timer, display_buffer, DURATION_100_MS);
    }
}
```

你可能会问，`SMILEY`那个二维数组中表示LED灯亮灭的1和0，是否也属于魔法数，为什么不提取呢？原因是保留这些数字反而更好——数字1和0能让我们直观地辨认出图案的样子。如果将它们提取为常量，反而会使图案变得难以识别，这就违背了我们提取魔法数的初衷：让代码更易懂。

每次重构后，记得运行`cargo run`来验证程序是否正常工作。以后不再重复说明这一点。

## 3.2.2 变量默认不可变

🧠 为什么在使用`let`声明变量时，有些变量需要加`mut`关键字，而有些不需要呢？

💡变量是否添加`mut`关键字决定了它的可变性。在代码清单3-2中，`let board`声明了一个不可变的变量`board`。这里的"不可变"是默认行为——当使用`let board`声明变量时，该变量自动成为不可变的。如果想创建可变变量，需要添加`mut`关键字，就像声明`display`变量那行代码所示。

如何验证`board`确实不可变，我们可以在声明`board`后添加一行代码，尝试给它重新赋值。如果运行`cargo build`，编译器会报错，因为不可变变量不能被重新赋值：

```rust
let board = Board::take().unwrap();
// 试图重新赋值
board = Board::take().unwrap();
// 运行cargo build时会报错：
// error[E0384]: cannot assign twice to immutable variable `board`
```

## 3.2.3 变量、常量与静态变量

🧠既然声明变量 `board` 时使用了 let，那么如果在声明变量 `SMILEY` 时用 let 会发生什么？

💡编译时会报下面的错误，并提示对于全局变量应该使用`const`或`static`来声明：

```rust
let SMILEY: [[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION] = [
// 运行cargo build时会报错：
error: expected item, found keyword `let`
  --> src/main.rs:16:1
   |
16 | let SMILEY: [[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION] = [
   | ^^^ consider using `const` or `static` instead of `let` for global variables
```

🧠在代码清单3-2中，声明`SMILEY`使用了`static`。如果将其换成`const`，程序也能正常运行。你知道这两者有什么区别吗？

💡两者的主要区别在于存储方式：`static`（静态变量）在程序运行期间始终存在于固定的内存地址，数据存储在`.rodata`段中，通过地址访问。而`const`（常量）在编译时就被求值，其值会被直接内联到代码中使用的地方，成为代码段的一部分。

## 3.2.4 基本数据类型和非基本数据类型

Rust有两种数据类型：基本（primitive）数据类型和非基本数据类型。

在代码清单3-2中暗藏以下数据类型：

- 静态变量`SMILEY`的声明中，`u8`是一个基本数据类型，代表8位无符号整数。
- SMILEY本身是一个非基本数据类型——一个二维数组。
- 在`if SMILEY[row][col] == 1 {`这行代码中，表达式`SMILEY[row][col] == 1`的结果是布尔型，这也是一个基本数据类型。
- 两个`for`循环中的`row`和`col`这两个数组下标变量的类型是`usize`（无符号整数，用于数组索引）或`isize`（有符号整数，可以表示负值）。

🧠你知道Rust中包含哪些基本数据类型和非基本数据类型吗？

💡Rust数据类型如下：

```bash
Rust数据类型
├── 基本数据类型
│   ├── 布尔型bool（Boolean）
│   ├── 数值型（Numeric）
│   │   ├── 整数型（i8/u8/../i128/u128/isize/usize） # 整型默认为i32
│   │   └── 浮点型（f32/f64） # 浮点型默认为f64
│   ├── 字符型char（Character） # 字符型字面量用单引号，如'a'
│   └── 字符串切片str（String Slice） # 字符串字面量（类型是&'static str，指向静态存储区域的字符串切片）用双引号，如"a"
├── 复合数据类型（Composite Types）
│   ├── 元组型（Tuple）
│   ├── 数组型（Array）
│   ├── 切片型（Slice）
│   ├── 指针类型（Pointer Types）
│   │   ├── 引用（&T / &mut T）
│   │   ├── 原始指针（*const T / *mut T）
│   │   ├── 智能指针（Smart Pointers）
│   │   │   ├── Box智能指针（Box）
│   │   │   └── 引用计数（Rc）
│   │   └── 函数指针（fn pointer）
│   ├── 函数类型（Function Types）
│   │   ├── 函数（fn）
│   │   └── 闭包（Closures）
│   └── trait类型（Trait Types）
│       ├── trait对象（dyn Trait）
│       └── trait实现（impl Trait）
├── 用户自定义数据类型（User-defined Types）
│   ├── 结构体（Struct）
│   ├── 枚举型（Enum）
│   ├── 联合体（Union）
│   └── 类型别名（Type Alias）
├── 特殊类型（Special Types）
│   ├── 永不返回型!（Never） # 表示用不完成的计算
│   └── 零大小类型（Zero Sized Type） # 不占用内存
│       ├── 单元类型unit（Unit Type） # 空元组类型
│       └── 幻象数据（PhantomData T）
└── 标准库类型（Standard Library Types）
    ├── 集合类型（Collections）
    │   ├── 向量Vec<T>（Vector）
    │   ├── 哈希映射HashMap<K,V>
    │   ├── B树映射BTreeMap<K,V>
    │   ├── 哈希集合HashSet<T>
    │   └── B树集合BTreeSet<T>
    ├── 字符串类型（String Types）
    │   ├── 字符串String（String）
    │   ├── C字符串（CString）
    │   └── 系统字符串（OsString）
    ├── 同步类型（Sync Types）
    │   ├── 原子引用计数（Arc<T>）
    │   ├── 互斥锁（Mutex<T>）
    │   └── 读写锁（RwLock<T>）
    ├── 原子类型（Atomic Types）
    │   ├── 原子布尔（AtomicBool）
    │   ├── 原子整数（AtomicI32/64/usize）
    │   └── 原子指针（AtomicPtr<T>）
    └── 内部可变性类型（Interior Mutability）
    │   ├── Cell<T>
    │   ├── RefCell<T>
    │   └── UnsafeCell<T>
    └── 标准库枚举（Standard Enums）
        ├── 可选值Option<T>
        └── 错误处理Result<T,E>
```

## 3.2.5 访问超出数组范围的下标会触发panic

让我们来聊聊Rust中一个有趣的设计决策。当程序试图访问数组范围之外的元素时（如代码清单3-3所示），Rust会立即触发一个叫做panic的机制。这个设计背后有着深思熟虑的考虑。

代码清单3-3 dabd227,  ch03/mem-match/src/main.rs

```rust
// （其他行略）
    for row in 0..MATRIX_DIMENSION {
        // col会访问超出数组范围的下标
        for col in 0..(MATRIX_DIMENSION + 1) {
            display_buffer[row][col] = if SMILEY[row][col] == 1 {
                MAX_BRIGHTNESS
            } else {
                MIN_BRIGHTNESS
            };
        }
    }
// （其他行略）
// 运行cargo run时会报以下panic错误并退出程序
// Frame 0: __bkpt @ 0x00005c3e inline
//        ./asm/lib.rs:48:1
// Frame 1: __bkpt @ 0x0000000000005c3e
//        ./asm/lib.rs:51:17
// Frame 2: panic @ 0x0000102a
//        /Users/<your_user_name>/.cargo/registry/src/index.crates.io-6f17d22bba15001f/panic-semihosting-0.5.6/src/lib.rs:92:15
// Frame 3: <unknown function @ 0x2001ff0c> @ 0x2001ff0c
// Error: CPU halted unexpectedly.
```

想象一下，如果在C或C++程序中不小心访问了数组外的内存，可能会引发各种严重问题：程序可能悄无声息地崩溃，数据可能遭到破坏，甚至可能被黑客利用这个漏洞发起攻击。而Rust采取了一种更谨慎的方式——它会立即停止程序运行，就像一位尽职的保安发现可疑行为时立即采取行动。

这种设计带来了三个显著优势：它像警报系统一样及时提醒程序员发现问题；像防护墙一样保护程序免受内存问题的困扰；更重要的是，它培养程序员编写更健壮代码的习惯。

那么，这个panic机制是如何工作的呢？它就像程序的应急系统。当遇到无法处理的错误时，panic会执行两个步骤：首先，像细心的管理员一样记录下错误的位置和原因；然后，有序地清理现场，确保所有资源得到妥善处理。

在日常编程中，我们最常在以下情况遇到panic：访问数组范围外的元素时、使用`unwrap()`或`expect()`处理可能失败的操作时，或主动调用`panic!()`宏时。这就像程序中的安全阀，在危险情况发生时及时启动保护机制。

要在终端查看我们正在开发的no_std嵌入式程序中的panic错误信息（如代码清单3-3所示），需要修改两处代码。

首先在Cargo.toml文件进行修改：

```toml
# （其他行略）
[dependencies]
# （其他行略）
# panic-halt = "0.2.0"
# 用下面一行替代原来的panic-halt = "0.2.0"
panic-semihosting = "0.5.0"
# （其他行略）
```

然后在main.rs文件的use导入依赖包处增加下面一行：

```rust
// （其他行略）
// 导入 panic_semihosting 包但不引入任何名称到当前作用域
// 下划线 '_' 表示导入这个包只是为了它的副作用（side effects）
// 在嵌入式开发中，panic_semihosting 用于在发生 panic 时通过半主机（semihosting）
// 功能输出错误信息，这对调试很有帮助
use panic_semihosting as _;
// （其他行略）
```

🧠你知道如何使用gdb调试工具在mb2开发板上进行单步调试，从而找出引发panic的具体代码位置吗？如图3-4所示：

![f3-4.png](f3-4.png)

图3-4 在gdb调试工具中单步调试以发现引发panic的那行代码

💡调试要点如下：

```bash
# 在终端1进入项目目录ch03/mem-match，准备有数组访问越界的程序
git checkout -b dabd227 dabd227
# 在终端1将程序烧录到mb2开发板并启动GDB stub准备调试
# 对于macOS或Ubuntu
cargo embed
# 对于Windows
cargo embed --probe <VID:PID:SN>

# 在终端2进入项目目录ch03/mem-match，并启动gdb调试工具
# 对于macOS或Windows
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/mem-match
# 对于Ubuntu
gdb-multiarch target/thumbv7em-none-eabihf/debug/mem-match

# 在终端2的gdb中
(gdb) target remote :1337
(gdb) break main
(gdb) continue
(gdb) quit

# 想要结束调试
# 在终端1按Ctrl+C退出GDB stub
# 返回main分支
git checkout main
```

🧠代码清单3-3中的for循环范围导致了数组下标越界访问。有什么更安全的写法可以避免这个问题呢？

💡我们可以使用 `iter()` 和 `enumerate()` 等迭代器方法来避免直接操作索引，从而确保安全访问。具体实现请看代码清单3-4：

代码清单3-4 1672446, ch03/mem-match/src/main.rs

```rust
// （其他行略）
    for (row_idx, row) in SMILEY.iter().enumerate().take(MATRIX_DIMENSION) {
        for (col_idx, &value) in row.iter().enumerate().take(MATRIX_DIMENSION) {
            display_buffer[row_idx][col_idx] = if value == 1 {
                MAX_BRIGHTNESS
            } else {
                MIN_BRIGHTNESS
            };
        }
    }
// （其他行略）
```

## 3.2.6 位于赋值语句右侧的if表达式

在编程的世界里，我们会遇到两个重要概念。第一个是语句，它像一个动作指令，告诉程序去执行某件事，但不返回结果。比如当我们写下 `let x = 5;` 或者 `return x;` 时，我们只是在告诉程序执行一个操作。

第二个概念是表达式，它像一个计算过程，总会产生一个结果值。就像计算 `5 + 6` 会得到 11，或者调用函数 `foo()` 时会返回一个值。

Rust语言在这方面有个独特的设计：它让几乎所有代码结构（如字面量、宏调用、函数调用、大括号围起来的代码块等）都成为表达式。比如在代码清单3-4中，对数组元素 `display_buffer[row_idx][col_idx]` 赋值时，右边用的就是`if`表达式（而不是语句）。

只有像`let`绑定、赋值、`use`、返回和函数定义这些少量的代码结构才是语句。

这种设计让代码更加优雅。我们可以直接使用表达式的结果，无需写很多中间变量，使代码更加流畅和简洁。

`if`表达式后面可以直接跟条件，不需要加括号（当然加括号也可以）。

`if`表达式的条件必须是 `bool` 类型。与C/C++不同，Rust不会自动将非 `bool` 类型转换为 `bool` 类型。

🧠在代码清单3-4中，`if`表达式的两个分支都返回`u8`类型的值。如果我们不小心**将其中一个分支的类型改为不同的类型**会怎样呢？比如把常量`MIN_BRIGHTNESS`的类型改为`u16`：

```rust
const MIN_BRIGHTNESS: u16 = 0;
```

此时运行`cargo build`命令，看看会出现什么情况。

# 3.3 迭代2：按下按钮A清除显示图案

到目前为止，我们编写的代码主要集中在控制mb2上的LED灯亮灭和闪烁。我们还没有涉及如何使用mb2正面的两个按钮（左边的按钮A和右边的按钮B）。

原本这个迭代要实现"按下按钮A确认记住图案"的功能。不过，由于我们还没有编写过任何按钮相关的代码，我们先简化为"按下按钮A清除显示图案，过1秒后重新显示固定笑脸"，如图3-5所示：

![f3-5.gif](f3-5.gif)

图3-5 迭代2：按下按钮A清除显示图案

一旦我们掌握了按钮的基本响应功能，后续的开发就会更加流畅。让我们先用AI实现功能，等确认代码能正常运行后，再以"代码鉴赏家"的视角，仔细闻闻代码中的腐臭，并进行必要的重构。

首先，我们要把代码中的魔法数提取为有意义的常量：

```rust
const DURATION_500_MS: u32 = 500;
const DURATION_1000_MS: u32 = 1000;
```

其次，我们需要把这个不太直观的表达式 `board.buttons.button_a.is_low().unwrap()` 提取为一个语义清晰的变量名 `is_button_a_pressed`。这里要注意一个专业程序员的命名习惯：**布尔类型（`bool`）的变量通常以 `is_` 开头，这样在与 if 表达式搭配使用时，读起来就像自然的英文句子一样流畅**。

重构后的代码如代码清单3-5和3-6所示：

代码清单3-5 34abf59, Cargo.toml

```toml
[dependencies]
# （其他行略）
microbit-v2 = "0.12.0" # 版本号降了
embedded-hal = "0.2.5" # 增加此行
```

代码清单3-6 34abf59, ch03/mem-match/src/main.rs

```rust
// （其他行略）
use cortex_m_rt::entry;
// 导入 embedded_hal crate 中的 DelayMs trait,
// 用于实现毫秒级延时功能
use embedded_hal::blocking::delay::DelayMs;
// 导入 embedded_hal crate 中的 InputPin trait,
// 用于读取输入引脚的状态
use embedded_hal::digital::v2::InputPin;
use microbit::board::Board;
// （其他行略）
#[entry]
fn main() -> ! {
// （其他行略）
    loop {
        display.show(&mut timer, display_buffer, DURATION_100_MS);

        // 变量名is_button_a_pressed更易懂
        let is_button_a_pressed = board.buttons.button_a.is_low().unwrap();
        // 下面的if表达式像自然的英语一样流畅
        // 检测按钮 A 是否被按下(低电平)
        if is_button_a_pressed {
            // 创建一个 5x5 的空白显示缓冲区(全部填充 0，即LED点阵全灭)
            let empty_buffer = [[MIN_BRIGHTNESS; MATRIX_DIMENSION]; MATRIX_DIMENSION];
            // 显示空白缓冲区内容 500ms
            display.show(&mut timer, empty_buffer, DURATION_500_MS);
            // 延时 1000ms(1 秒)
            timer.delay_ms(DURATION_1000_MS);
        }
    }
}
```

运行`cargo run`后，按下mb2正面左侧的按钮A，LED点阵确实按预期全部熄灭了一秒多。AI给出的代码运行得很好！

# 3.4 迭代3：循环显示随机图案

现在我们的记忆力挑战游戏已经实现了两个基本功能：显示固定笑脸和按下按钮A清除图案。下一步该做什么呢？游戏的精髓在于让玩家从循环显示的随机图案中，找出最初记住的目标图案。因此，这个迭代我们将专注于实现循环显示随机图案的功能，如图3-6所示：

![f3-6.gif](f3-6.gif)

图3-6 迭代3：循环显示随机图案

这个迭代的功能如下：当显示固定笑脸时，按下按钮A后，LED点阵会短暂熄灭，然后进入循环显示随机图案的模式。在这个模式下，每个随机图案会显示1秒，之后自动切换到下一个随机图案。

由于我们还没有编写过随机显示图案的代码，先让AI帮我们实现这个功能。在确认功能可以正常运行后，我们仔细鉴赏了AI给出的代码，发现了以下需要消除的代码腐臭：

- 直接使用数字的魔法数
- `ShowingPatterns`这个表现显示某个随机图案的枚举值的命名，不如`ShowingRandomPattern`更揭示意图
- `game_state`这个命名的准确性，不如`current_state`
- `button_a.is_low().unwrap()`这个`if`表达式的条件，不如`is_button_a_pressed`好懂

重构后的代码变更如代码清单3-7所示：

代码清单3-7 3f5a940c, ch03/mem-match/src/main.rs

```rust
// （其他行略）
use panic_semihosting as _;

// 定义一个XorShift随机数生成器结构体，包含一个32位无符号整数状态
struct XorShiftRng {
    state: u32,
}

// 为XorShiftRng实现方法
impl XorShiftRng {
    // new()是关联函数(associated function)而不是方法，因为它不接收self参数，
    // 而是接收一个种子值，如果种子为0则使用1作为初始状态
    fn new(seed: u32) -> Self {
        XorShiftRng {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    // next()是方法而不是函数，因为它接收&mut self作为第一个参数，
    // 生成下一个随机数，使用XorShift算法
    fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13; // 左移13位后与原值异或
        x ^= x >> 17; // 右移17位后与原值异或
        x ^= x << 5; // 左移5位后与原值异或
        self.state = x; // 更新状态
        x // 返回生成的随机数
    }

    // 生成指定范围内的随机数
    fn next_range(&mut self, range: usize) -> usize {
        (self.next() as usize) % range // 将随机数转换为范围内的值
    }
}

const PATTERN_NUM: usize = 10;
// （其他行略）

// 定义10个5x5的图案模式数组，每个元素是0或1
static SMILEY: [[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION] = [
    // 0 Heart shape 心形图案
    [
        [0, 1, 0, 1, 0],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
    ],
    // 1 Up arrow 上箭头图案
    [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 0, 1, 0, 1],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ],
// （其他行略）
    // 9 Wave 波浪图案
    [
        [0, 0, 0, 0, 0],
        [1, 0, 1, 0, 1],
        [1, 1, 1, 1, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0],
    ],
];

// 定义游戏状态枚举，可以比较相等，
// 没有满足自反性的PartialEq就够用了
#[derive(PartialEq)]
enum GameState {
    ShowingSmiley,
    ShowingRandomPattern,
}

#[entry]
fn main() -> ! {
// （其他行略）
    let mut timer = Timer::new(board.TIMER0);
    // 初始化游戏状态
    let mut current_state = GameState::ShowingSmiley;
    // 因为只读取按钮A的状态，所以button_a声明为默认不可变
    let button_a = board.buttons.button_a;

    // 使用定时器读数作为随机数种子
    let seed = timer.read();
    let mut rng = XorShiftRng::new(seed);

    let mut display_buffer = [[MIN_BRIGHTNESS; MATRIX_DIMENSION]; MATRIX_DIMENSION];
    // 将原先给图案缓冲区赋值的几行代码提取为函数copy_pattern_to_buffer()，使代码更易读
    copy_pattern_to_buffer(&SMILEY, &mut display_buffer);

    loop {
        match current_state {
            // // 固定笑脸显示状态的处理
            GameState::ShowingSmiley => {
                display.show(&mut timer, display_buffer, DURATION_100_MS);
                // 如果按下按钮A
                let is_button_a_pressed = button_a.is_low().unwrap();
                if is_button_a_pressed {
                    clear_buffer(&mut display_buffer);
                    display.show(&mut timer, display_buffer, DURATION_100_MS);
                    timer.delay_ms(DURATION_1000_MS);
                    // 切换到显示随机图案状态
                    game_state = GameState::ShowingRandomPattern;
                }
            }

            // 随机图案显示状态的处理
            GameState::ShowingRandomPattern => {
                // 随机选择一个图案
                let pattern_index = rng.next_range(PATTERN_NUM);

                copy_pattern_to_buffer(&PATTERNS[pattern_index], &mut display_buffer);
                display.show(&mut timer, display_buffer, DURATION_1000_MS);
                timer.delay_ms(DURATION_1000_MS);
            }
        }
    }
}

// 将图案赋值到显示缓冲区，1转换为亮度9（最亮），0转换为亮度0
fn copy_pattern_to_buffer(
    pattern: &[[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION],
    buffer: &mut [[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION],
) {
    for (row, pattern_row) in pattern.iter().enumerate() {
        for (col, &value) in pattern_row.iter().enumerate() {
            buffer[row][col] = if value == 1 {
                MAX_BRIGHTNESS
            } else {
                MIN_BRIGHTNESS
            };
        }
    }
}

// 清空显示缓冲区，将所有位置设置为0（最暗）
fn clear_buffer(buffer: &mut [[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION]) {
    for (row, buffer_row) in buffer.iter_mut().enumerate() {
        for (col, cell) in buffer_row.iter_mut().enumerate() {
            *cell = MIN_BRIGHTNESS;
        }
    }
}
```

## 3.4.1 函数与方法

🧠在代码清单3-7中，我们看到`XorShiftRng`结构体有关联函数`new()`和方法`next()`、`next_range()`，以及全局函数`main()`、`copy_pattern_to_buffer()`和`clear_buffer()`。它们都以`fn`开头声明。这些函数和方法之间有什么异同？

💡代码清单3-7展示了几种不同类型的函数和方法定义。这些函数或方法虽然都使用`fn`关键字声明，都由语句和可选的结束表达式组成，且在当前代码中都是私有的（没有`pub`修饰符），但它们有着明显的区别。

从位置和归属看，`XorShiftRng`结构体在`impl`块内定义了`new()`这个关联函数以及`next()`和`next_range()`这两个实例方法。相比之下，`main()`、`copy_pattern_to_buffer()`和`clear_buffer()`是定义在模块级别的全局函数。

这些函数的参数特征各不相同。实例方法`next()`和`next_range()`的第一个参数是`self`（可以是`&self`或`&mut self`）；关联函数`new()`没有`self`参数，但返回`Self`类型；全局函数则只接受与特定类型无关的普通参数。

在调用语法上，方法需要通过实例调用（如`rng.next()`），关联函数通过类型名调用（如`XorShiftRng::new()`），而全局函数可以直接调用（如`clear_buffer()`）。

🧠为什么`XorShiftRng`结构体的`next()`方法的最后一行`x`没有分号？加上分号会发生什么？

💡如果加上分号，编译器会报以下错误，并贴心地提示我们移除分号：

```bash
error[E0308]: mismatched types
  --> src/main.rs:24:27
   |
24 |     fn next(&mut self) -> u32 {
   |        ----               ^^^ expected `u32`, found `()`
   |        |
   |        implicitly returns `()` as its body has no tail or `return` expression
...
30 |         x;
   |          - help: remove this semicolon to return this value
```

为什么会这样？这是因为在Rust中，表达式和语句是两个截然不同的概念。表达式会产生一个值，而语句只执行一个动作。**当我们在表达式后面加上分号时，它就会变成一个语句**。这个区别在处理函数返回值时尤为重要。

让我用具体例子说明。在我们的`next()`方法中，最后一行的`x`是一个表达式，它计算并返回一个`u32`类型的值（**函数返回值的类型在函数签名中用`->`箭头声明**）。如果在它后面加上分号，它就会变成一个语句，返回一个空元组`()`。这就是为什么编译器会报错：方法声明要求返回`u32`，但实际返回的是`()`。

这种设计体现了Rust的优雅之处。在函数的最后一行，省略分号时，表达式的值会自动成为函数的返回值。这个特性是从函数式编程语言借鉴而来，让我们能写出更简洁的代码。比如，直接写`x`就可以，不需要写`return x;`。

编译器的提示也很贴心。当发现这个问题时，它会明确指出："expected u32, found ()"，并建议删除分号来修复问题。这不仅帮助我们写出更好的代码，还通过类型系统保证了代码的安全性。这就是我特别欣赏Rust设计理念的原因。

## 3.4.2 如果else if过多可以使用枚举

🧠代码清单3-7中的`enum`和`match`是什么？为什么要用这种代码结构来判断游戏状态，而不是使用if-else？

💡在这个LED矩阵显示游戏中，我们需要处理不同的显示状态。游戏目前设计了两个基本状态：显示欢迎笑脸和显示随机图案。

为了优雅地管理这些状态，我们选择使用Rust的枚举类型而非传统的if-else结构。这个决定带来了几个重要优势：

首先，在代码组织方面，枚举通过`match`语法将不同状态的处理逻辑清晰地分开，而if-else则会随着状态增多导致嵌套层级加深，降低代码可读性。

其次，枚举提供了编译时的安全保障。使用`match`处理枚举时，编译器会强制检查是否处理了所有可能的枚举值，避免遗漏。相比之下，if-else结构无法在编译时进行这种全面检查。

在类型安全方面，枚举通过限定所有可能的取值，确保不会出现预期之外的状态值。而if-else通常使用数字或字符串表示状态，这种做法容易引入非法值。

从维护性角度来看，当需要添加新状态时，枚举只需添加新的变体，编译器会自动提示所有需要更新的`match`语句。而使用if-else则需要在多处手动添加新的条件判断。

最后，枚举能够获得更好的IDE支持，包括代码补全和类型提示，在重构时也更容易定位所有相关代码。这些优势在代码规模增长时会更加明显，特别是在多人协作的项目中。

在理解枚举概念时，需要知道它本质是一种自定义数据类型，用来表示一组预定义的可能值，每个值称为变体。值得注意的是，Rust的枚举比C语言的更强大，因为它可以携带额外的数据。

在这段代码中，我们使用了特殊标注`#[derive(PartialEq)]`。这个派生宏自动为枚举类型实现了`PartialEq` trait，让我们能够比较两个状态是否相等。这个trait提供了`==`和`!=`操作符的功能。"Partial"一词很有意思，它暗示并非所有值都可以比较，比如浮点数中的`NaN`。在我们的游戏中，这个状态比较功能非常重要，因为我们要判断程序是否进入了特定状态。

🧠`GameState`标注了`#[derive(PartialEq)]`，代码清单3-7的注释说"没有满足自反性的`PartialEq`就够用了"。这是什么意思呢？如果把`#[derive(PartialEq)]`改为`#[derive(Eq)]`，编译时会出现什么情况？你能用例子解释一下什么是自反性（reflexivity），以及`PartialEq`和`Eq`有什么区别吗？

💡`Eq`是`PartialEq`的加强版。`Eq`相比`PartialEq`额外要求自反性。以浮点数为例：当使用分数表示时，分母为0会产生`NaN`（not a number）值。由于`NaN`不等于它自身，浮点数就不满足自反性。在代码清单3-7中，将`#[derive(PartialEq)]`改为`#[derive(PartialEq, Eq)]`是没问题的，能正常运行。

## 3.4.3 提取函数

🧠提取`copy_pattern_to_buffer()`和`clear_buffer()`这两个函数有什么好处？

💡提取这两个函数是不是让`loop`循环的代码结构更清晰、更易于理解了？同时，这些**函数的命名遵循了与变量名相同的snake_case命名风格**。

🧠在代码清单3-7中，`timer`变量声明时没有指定类型，编译器是通过它所绑定的值来推断类型的。如果我们把函数`copy_pattern_to_buffer()`中的某个参数（比如`pattern`）的类型声明去掉，像下面这样，编译器能通过类型推断判断出它的类型吗？

```rust
fn copy_pattern_to_buffer(
    pattern,
    buffer: &mut [[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION],
) {
```

💡编译会报错："error: expected one of `:`, `@`, or `|`, found `,`"。这是因为在Rust中，**函数签名必须明确声明每个参数的类型**。

# 3.5 迭代4：在终端打印玩家所选图案编号

记忆力挑战游戏需要判断玩家选择的图案是否与目标图案匹配，以此决定游戏胜负。由于LED点阵图案变化迅速，玩家在按下A按钮后往往难以记住自己选择了哪个图案。如果能在玩家按下A按钮时，在终端打印所选图案的编号，这将有助于后续迭代中的判断。这是一个符合小步迭代开发理念的好想法。

这是我们第一次在程序中添加终端打印功能，为此我们借助了AI的帮助。经过反复调试和优化，我们完成了迭代4的实现。当程序处于显示固定笑脸状态时，按下按钮A后会进入循环显示随机图案的状态。在这个状态下再次按下按钮A时（由于程序还未完善，按钮A的响应不够灵敏，可能需要多按几次或按住不放才能触发，后文会介绍如何修复），终端会显示当前图案的编号：

```bash
% cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `probe-rs run --chip nRF52833_xxAA target/thumbv7em-none-eabihf/debug/mem-match`
      Erasing ✔ [00:00:01] [#####] 40.00 KiB/40.00 KiB @ 36.38 KiB/s (eta 0s )
  Programming ✔ [00:00:02] [#####] 40.00 KiB/40.00 KiB @ 19.63 KiB/s (eta 0s )    Finished in 3.182s
Current pattern: 5
Current pattern: 3
Current pattern: 4
```

相关代码变更体现在三个文件中：依赖库配置文件Cargo.toml、嵌入式特定配置文件Embed.toml，以及源代码文件main.rs。这些变更分别在代码清单3-8、3-9和3-10中展示：

代码清单3-8 768fcdd, ch03/mem-match/Cargo.toml

```toml
# （其他行略）
[dependencies]
# （其他行略）

# panic-probe 是一个用于嵌入式设备的 panic 处理程序
# features = ["print-rtt"] 启用了通过 RTT(Real-Time Transfer)打印 panic 信息的功能
panic-probe = { version = "0.3", features = ["print-rtt"] }

# （其他行略）
# rtt-target 提供了 RTT 通信功能的实现
# features = ["cortex-m"] 表示这个库针对 Cortex-M 系列微控制器优化
rtt-target = { version = "0.3.1", features = ["cortex-m"] }

# critical-section 提供了一个跨平台的临界区抽象
# 用于处理并发和中断安全的代码段
critical-section = "1.2.0"

# cortex-m 提供了对 ARM Cortex-M 处理器的底层访问支持
# features = ["critical-section-single-core"] 启用了单核处理器的临界区实现
cortex-m = { version = "0.7.7", features = ["critical-section-single-core"] }
```

代码清单3-9 768fcdd, ch03/mem-match/Embed.toml

```toml
# （其他行略）
[default.reset]
# 启用设备复位功能
# 在调试会话开始时是否复位目标设备
enabled = true

# 复位后是否让设备保持在暂停状态
# false 表示复位后设备会直接运行，而不是停在复位向量
halt_afterwards = false

[default.rtt]
# 启用 RTT (Real-Time Transfer) 功能
# RTT 是一个高速调试通信协议
enabled = true

# 设置 RTT 上行(设备到主机)的传输模式
# NoBlockSkip: 当缓冲区满时，丢弃新数据而不阻塞
# 这对于调试输出很有用，因为我们通常不希望程序因为打印日志而卡住
up_mode = "NoBlockSkip"
# （其他行略）
```

代码清单3-10 768fcdd, ch03/mem-match/src/main.rs

```rust
// （其他行略）
// 导入格式化输出相关的 trait
use core::fmt::Write;
// （其他行略）
// 使用 panic_probe 作为 panic 处理程序
use panic_probe as _;
// 导入 RTT 初始化宏
use rtt_target::rtt_init;

// （其他行略）
#[entry]
fn main() -> ! {
    // 初始化 RTT 通道，设置上行通道
    let mut channels = rtt_init! {
        up: {
            // 通道 0
            0: { 
                // 缓冲区大小为 1024 字节
                size: 1024 
                // 缓冲区满时截断新数据
                mode: NoBlockTrim 
                // 通道名称
                name: "Terminal"
            }
        }
    };
    // 获取上行通道 0 的可变引用
    let channel = &mut channels.up.0;

// （其他行略）
    // 当前图案编号
    let mut current_pattern = usize::MAX;

// （其他行略）
    loop {
        match current_state {
            GameState::ShowingSmiley => {
                copy_pattern_to_buffer(&SMILEY, &mut display_buffer);
                display.show(&mut timer, display_buffer, DURATION_100_MS);
// （其他行略）
            }

            GameState::ShowingRandomPattern => {
                // 生成并显示随机图案
                current_pattern = rng.next_range(PATTERN_NUM);
                copy_pattern_to_buffer(&PATTERNS[current_pattern], &mut display_buffer);
                display.show(&mut timer, display_buffer, DURATION_1000_MS);
                // 如果按下按钮A
                let is_button_a_pressed = button_a.is_low().unwrap();
                if is_button_a_pressed {
                    // 通过 RTT 打印当前图案编号
                    writeln!(channel, "Current pattern: {}", current_pattern).ok();
                    // 切换到显示笑脸状态
                    current_state = GameState::ShowingSmiley;
                } else {
                    timer.delay_ms(DURATION_1000_MS);
                }
            }
        }
    }
}

// （其他行略）
```

🔎注意，在迭代1中，我们导入了`panic-semihosting`依赖包来在终端显示数组下标越界访问引发的panic错误信息。我们本想在这次迭代中继续使用它，不仅用来打印panic错误信息，还要用来打印程序状态字符串。但是，由于probe-rs（调试器工具）不支持某些semihosting操作，程序在实际运行时无法打印程序状态，并显示如下警告：

```bash
WARN probe_rs::cmd::run::normal_run_mode: Target wanted to run semihosting operation 0x1 with parameter 0x2001fdf4,but probe-rs does not support this operation yet.
```

从本迭代开始，我们将用RTT（Real-Time Transfer）替代semihosting来在终端打印程序状态，因为这两种方式无法同时使用。虽然RTT可以打印程序状态，但它不能打印panic错误信息。

# 3.6 迭代4a：提升按下按钮A的灵敏度

现在我们来解决按钮A响应不够灵敏的问题。虽然可以直接把问题描述和代码交给AI来解决，但如果我们先通过阅读代码找到问题根源，再寻求AI帮助，这样必然能获得更高质量的解决方案。

通过仔细阅读代码清单3-10中`GameState::ShowingRandomPattern`分支中处理按钮A和终端打印的代码，我们就能发现问题的根源。从代码来看，程序在`if is_button_a_pressed {`判断按钮A按下后会立即打印，这说明检测逻辑本身是及时的。按钮A应该会反应灵敏才对。但仔细看这段代码往上第2行，`display.show()`方法显示图案时使用了`DURATION_1000_MS`参数。这就是问题所在：在这1秒钟的显示延时期间按下按钮A时，程序不会有任何反应，因为它正在等待显示延时结束。等到延时结束时，你可能已经松开按钮了。这就解释了为什么按钮A看起来反应迟钝。

知道了问题的根源，就可以请AI帮着改进代码，让按钮更灵敏。经过反复几次与AI沟通和改进，最后提升按钮A灵明度的代码改动如代码清单3-11所示：

代码清单3-11 4580e78, ch03/mem-match/src/main.rs

```rust
#[entry]
fn main() -> ! {
// （其他行略）
    // 初始化按钮A是否之前曾被按下
    let mut was_button_a_pressed = button_a.is_low().unwrap();

    loop {
        match current_state {
            GameState::ShowingSmiley => {
                copy_pattern_to_buffer(&SMILEY, &mut display_buffer);
                display.show(&mut timer, display_buffer, DURATION_100_MS);
                // 获取按钮A当前时刻是否被按下
                let is_button_a_pressed = button_a.is_low().unwrap();
                // 如果按钮A当前刚被按下，且之前未曾按下过（表明确实是刚刚按下的）
                if is_button_a_pressed && !was_button_a_pressed {
                    clear_buffer(&mut display_buffer);
                    display.show(&mut timer, display_buffer, DURATION_100_MS);
                    timer.delay_ms(DURATION_1000_MS);
                    current_state = GameState::ShowingRandomPattern;
                }
                // 完事后用本分支内按钮按下的状态更新按钮之前曾被按下的状态
                was_button_a_pressed = is_button_a_pressed;
            }

            GameState::ShowingRandomPattern => {
                current_pattern = rng.next_range(PATTERN_NUM);
                copy_pattern_to_buffer(&PATTERNS[current_pattern], &mut display_buffer);

                // 将 1000ms 的延时分割成多个短延时,每次都检查按钮状态
                let mut elapsed = 0;
                while elapsed < 1000 {
                    // 图案只显示50ms
                    display.show(&mut timer, display_buffer, DURATION_50_MS);
                    // 获取按钮A当前时刻是否被按下
                    let is_button_a_pressed = button_a.is_low().unwrap();
                    // 如果按钮A当前刚被按下，且之前未曾按下过（表明确实是刚刚按下的）
                    if is_button_a_pressed && !was_button_a_pressed {
                        writeln!(channel, "Current pattern: {}", current_pattern).ok();
                        // 切换回显示笑脸状态
                        current_state = GameState::ShowingSmiley;
                        // 完事后用本分支内按钮按下的状态更新按钮之前曾被按下的状态
                        was_button_a_pressed = is_button_a_pressed;
                        // 跳出最内层while循环，进入外层loop循环的开始
                        break;
                    }
                    // 完事后用本分支内按钮按下的状态更新按钮之前曾被按下的状态
                    was_button_a_pressed = is_button_a_pressed;
                    // 累加已经过的时间
                    elapsed += DURATION_50_MS;
                }
            }
        }
    }
}
```

🧠你知道代码清单3-11用了什么方法来防止按钮在嵌入式程序中被重复触发吗？

💡代码清单3-11使用了边缘检测机制来防止按钮重复触发。这种机制通过比较按钮的当前状态和前一个状态来检测状态变化。在代码中，我们使用 `is_button_a_pressed` 和 `was_button_a_pressed` 这两个变量来实现边缘检测。

实现方式如下：

```rust
// 存储上一次的按钮状态
let mut was_button_a_pressed = button_a.is_low().unwrap();
// 获取当前按钮状态
let is_button_a_pressed = button_a.is_low().unwrap();
// 检测下降沿 (falling edge)
if is_button_a_pressed && !was_button_a_pressed {
    // 检测到按钮刚被按下，执行相应操作
}
// 更新上一次状态
was_button_a_pressed = is_button_a_pressed;

```

边缘检测机制是一种简单高效的按钮检测方法。它通过比较按钮的前后状态来识别状态变化的瞬间，能够精确捕捉按钮状态的变化，同时保持代码简洁和资源消耗低。

从技术实现角度看，这种机制优势明显。它无需复杂硬件支持，系统资源占用少，CPU负担轻，能实现近实时的响应速度。这些特点使它非常适合需要快速响应的交互场景。

不过，这种方法也有局限性。在处理机械按键时，特别是在高采样频率下，可能无法完全消除按键抖动，导致误触发。此外，这种方法不适合检测按键的长按状态。

尽管如此，边缘检测机制在许多场景下仍是最佳选择。它特别适合简单的按键控制场合，如游戏中的状态切换。在资源受限的嵌入式系统中，或只需检测按键按下、释放瞬间的应用中，这种方法表现出色。

在我们的游戏代码中，这种机制用于处理游戏状态的切换。通过检测按钮的下降沿（按钮从未按下变为按下的瞬间），我们能准确捕捉玩家输入并作出响应。如果需要进一步提高抗干扰能力，可以考虑将边缘检测与延时消抖技术结合使用。

🧠什么是上升沿和下降沿？

💡在电子世界中，信号的变化就像按钮的动作一样有趣。当按下或释放按钮时，会产生两种重要的信号变化时刻。

当松开一个已按下的按钮时，会产生"上升沿"。此时按钮电平检测点的信号从低电平变为高电平，就像按钮从压下状态弹回原位。在程序代码中，这种变化体现为数值从0变为1。

相反，按下按钮时会产生"下降沿"。此时信号从高电平降到低电平，就像按钮被按下去一样。在程序中，这个变化就是数值从1变为0的瞬间。

在我们的代码清单3-11中，有一行代码专门用来捕捉这个下降沿。它像一个敏锐的观察者，能在按钮刚被按下的那一刻立即发现并作出反应：

```rust
if is_button_a_pressed && !was_button_a_pressed {
```

检查这两个条件让我们能精确捕捉到按钮被按下的瞬间。首先，`!was_button_a_pressed`确认按钮在上一时刻处于松开状态。然后，`is_button_a_pressed`确认按钮在当前时刻已被按下。只有这两个条件同时满足，才说明按钮刚刚被按下——这就是我们成功捕捉到的按钮按下下降沿。

🧠在代码清单3-11中，同样的语句`let is_button_a_pressed = button_a.is_low().unwrap();`出现在两个不同的代码块中。这算变量遮蔽吗？

💡不算。

🧠在代码清单3-11中，我们看到了带有`break`的`while`语句。在多层循环嵌套时，你知道`break`和`continue`默认会作用于哪一层循环吗？如果想让它们作用于特定的循环层级，又该如何实现呢？

🧠你知道在Rust中，`loop`和`while`是表达式还是语句？

💡`loop`是表达式，`while`是语句。

# 3.7 迭代4b：引入按钮B来选择匹配图案

既然 mb2 上有两个按钮，我们何不充分利用右边的按钮 B 呢？我们可以将原本由按钮 A 负责的"选择匹配图案"功能转移给按钮 B。这样，按钮 A 就能承担一个新的功能——在循环显示随机图案时，玩家无需等待 1 秒就能按下按钮 A 来查看下一幅图案，如图 3-7 所示：

![f3-7.gif](f3-7.gif)

图3-7 玩家可以按下按钮A直接切换到下一幅图案

由于这个迭代功能的代码与我们之前写过的类似，这次让我们不依赖AI，而是自己动手实现。让我们来看看这个迭代的代码变更，如代码清单3-12所示：

代码清单3-12 5dfb1e88, ch03/mem-match/src/main.rs

```rust
#[entry]
fn main() -> ! {
// （其他行略）
    let button_a = board.buttons.button_a;
    let button_b = board.buttons.button_b;
// （其他行略）
    let mut was_button_a_pressed = button_a.is_low().unwrap();
    let mut was_button_b_pressed = button_b.is_low().unwrap();

    loop {
        match current_state {
            GameState::ShowingSmiley => {
// （其他行略）
            }

            GameState::ShowingRandomPattern => {
// （其他行略）
                while elapsed < 1000 {
                    display.show(&mut timer, display_buffer, DURATION_50_MS);
                    let is_button_a_pressed = button_a.is_low().unwrap();
                    if is_button_a_pressed && !was_button_a_pressed {
                        break;
                    }
                    let is_button_b_pressed = button_b.is_low().unwrap();
                    if is_button_b_pressed && !was_button_b_pressed {
                        // Print the current pattern number using RTT
                        writeln!(channel, "Current pattern: {}", current_pattern).ok();
                        current_state = GameState::ShowingSmiley;
                        was_button_a_pressed = is_button_a_pressed;
                        break;
                    }
                    was_button_a_pressed = is_button_a_pressed;
                    was_button_b_pressed = is_button_b_pressed;
                    elapsed += DURATION_50_MS;
                }
            }
        }
    }
}
```

# 3.8 迭代5：按下按钮B选择图案时发出提示音

目前玩家只能通过LED点阵的显示变化或终端打印的图案编号来确认是否按下了按钮B。如果让mb2在玩家按下按钮B时发出声音提示，那就更酷了。

由于这是我们第一次编写让mb2发出声音的代码，让我们借助AI的帮助。下面是经过验证的提示音功能代码变更，如代码清单3-13所示：

代码清单3-13 ec12eaaa, ch03/mem-match/src/main.rs

```rust
// （其他行略）
// 引入GPIO电平控制相关的Level枚举类型
use microbit::hal::gpio::Level;
// 引入常用的硬件抽象层(HAL)预定义特性
use microbit::hal::prelude::*;
// 引入PWM(脉冲宽度调制)相关的Channel和Pwm类型
use microbit::hal::pwm::{Channel, Pwm};
// 引入时间相关的Hertz(赫兹)类型
use microbit::hal::time::Hertz;
// 引入PWM0和TIMER0这两个外设访问控制(PAC)模块
use microbit::pac::{PWM0, TIMER0};
// （其他行略）

// 定义发出蜂鸣的函数,接收PWM和定时器的可变引用作为参数
fn make_beep(pwm: &mut Pwm<PWM0>, timer: &mut Timer<TIMER0>) {
    // 设置PWM频率为1000赫兹
    pwm.set_period(Hertz(1000));
    // 设置PWM占空比为最大值的一半
    pwm.set_duty_on_common(pwm.max_duty() / 2);
    // 启用PWM的C0通道
    pwm.enable(Channel::C0);
    // 延时100毫秒
    timer.delay_ms(DURATION_100_MS);
    // 禁用PWM的C0通道
    pwm.disable(Channel::C0);
}

#[entry]
fn main() -> ! {
// （其他行略）
    // 将P0_02引脚配置为推挽输出模式,高电平
    board.pins.p0_02.into_push_pull_output(Level::High);
    // 将扬声器引脚配置为推挽输出模式,低电平
    let speaker_pin = board.speaker_pin.into_push_pull_output(Level::Low);

    // 将P0_13引脚配置为推挽输出模式,低电平
    board
        .pins
        .p0_13
        .into_push_pull_output(microbit::hal::gpio::Level::Low);

    // 初始化PWM模块
    let mut pwm = Pwm::new(board.PWM0);
    // 将扬声器引脚降级为通用引脚并设置为PWM输出引脚
    pwm.set_output_pin(Channel::C0, speaker_pin.degrade());

    loop {
        match current_state {
            GameState::ShowingSmiley => {
// （其他行略）
            }

            GameState::ShowingRandomPattern => {
// （其他行略）
                while elapsed < 1000 {
// （其他行略）
                    if is_button_b_pressed && !was_button_b_pressed {
                        // 发出提示音
                        make_beep(&mut pwm, &mut timer);
// （其他行略）
                }
            }
        }
    }
}
```

现在每当按下按钮B时，都会发出一声清脆的提示音。但还是有一些问题需要解决。

## 3.8.1 恼人的警告

在编译代码时，会发现编译器报告下面的警告：

```bash
cargo build
   Compiling mem-match v0.1.0 (/Users/binwu/OOR-local/katas/learn-rust-by-games/ch03/mem-match)
warning: value assigned to `current_pattern` is never read
   --> src/main.rs:176:13
    |
176 |     let mut current_pattern = usize::MAX;
    |             ^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` on by default

warning: `mem-match` (bin "mem-match") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
```

查看当前代码（提交ec12eaaa）可以发现，变量`current_pattern`在主循环之前就被绑定为usize的最大值。这样设计是为了避免将其初始化为0，因为0表示第一幅图案。由于在绑定变量时还无法确定当前显示的是哪幅图案，所以先使用一个没有实际意义的`usize`最大值作为占位符。

Rust编译器细心地发现，这个变量的初始值在被读取之前就在`GameState::ShowingRandomPattern`分支中被覆盖了。为了防止这可能是一个意外的覆盖操作，编译器发出了警告。

🧠虽然有多种方法可以消除这个警告，但对于当前的代码来说，你知道最推荐的做法是什么吗？

💡推荐将变量`current_pattern`的绑定移至`GameState::ShowingRandomPattern`分支内（具体实现请参见提交记录92d8b57），因为这个变量只在这个分支里使用。另外，编译器还贴心地提示，这个变量在移动后，无需是`mut`。

## 3.8.2 难以阅读的main()函数

目前的`main()`函数已接近100行，其中包含了大量提示音的实现细节。这使得函数难以阅读，因为我们需要同时理解"做什么"(what)和"怎么做"(how)。更好的做法是让`main()`函数只关注"做什么"，将"怎么做"的细节提取到独立的函数中。这样重构后，`main()`函数会变得清晰易读。

🧠如何将提示音相关的实现细节提取到独立函数中，使main()函数更简洁清晰？

💡可以把`main()`函数中下面这段与提示音相关的代码，提取到名为`init_speaker()`的函数中（可以在个人免费版的Rust编辑器RustRover中选中下面的代码，然后用鼠标右击 → Refactor → Extract Method…）：

```rust
    // Enable the speaker
    board.pins.p0_02.into_push_pull_output(Level::High);
    // Get the speaker output pin
    let speaker_pin = board.speaker_pin.into_push_pull_output(Level::Low);

    // Get P0_13 from the board and convert it to push-pull output
    board
        .pins
        .p0_13
        .into_push_pull_output(microbit::hal::gpio::Level::Low);

    // Initialize the PWM (Pulse Width Modulation)
    let mut pwm = Pwm::new(board.PWM0);
    // Degrade the speaker pin to a general purpose pin and set it as a PWM output pin
    pwm.set_output_pin(Channel::C0, speaker_pin.degrade());
```

这样提取完函数后，main.rs文件如代码清单3-14所示：

代码清单3-14 d1f0dcfb, ch03/mem-match/src/main.rs

```rust
// （其他行略）
#[entry]
fn main() -> ! {
// （其他行略）
    let pwm = init_speaker(board);
// （其他行略）
}

// 提取出的init_speaker()函数
fn init_speaker(board: Board) -> Pwm<PWM0> {
    // Enable the speaker
    board.pins.p0_02.into_push_pull_output(Level::High);
    // Get the speaker output pin
    let speaker_pin = board.speaker_pin.into_push_pull_output(Level::Low);

    // Get P0_13 from the board and convert it to push-pull output
    board
        .pins
        .p0_13
        .into_push_pull_output(microbit::hal::gpio::Level::Low);

    // Initialize the PWM (Pulse Width Modulation)
    let mut pwm = Pwm::new(board.PWM0);
    // Degrade the speaker pin to a general purpose pin and set it as a PWM output pin
    pwm.set_output_pin(Channel::C0, speaker_pin.degrade());
    pwm
}
```

但这样做完后，编译会出错：

```bash
% cargo build
# （其他行略）
error[E0382]: use of partially moved value: `board`
   --> src/main.rs:185:28
    |
172 |     let mut timer = Timer::new(board.TIMER0);
    |                                ------------ value partially moved here
...
185 |     let pwm = init_speaker(board);
    |                            ^^^^^ value used here after partial move
    |
    = note: partial move occurs because `board.TIMER0` has type `microbit::nrf52833_pac::TIMER0`, which does not implement the `Copy` trait

error[E0596]: cannot borrow `pwm` as mutable, as it is not declared as mutable
   --> src/main.rs:218:35
    |
218 |                         make_beep(&mut pwm, &mut timer);
    |                                   ^^^^^^^^ cannot borrow as mutable
    |
help: consider changing this to be mutable
    |
185 |     let mut pwm = init_speaker(board);
    |         +++

warning: variable does not need to be mutable
   --> src/main.rs:247:9
    |
247 |     let mut pwm = Pwm::new(board.PWM0);
    |         ----^^^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default

Some errors have detailed explanations: E0382, E0596.
For more information about an error, try `rustc --explain E0382`.
warning: `mem-match` (bin "mem-match") generated 1 warning
error: could not compile `mem-match` (bin "mem-match") due to 2 previous errors; 1 warning emitted
```

🔎编译错误中的“partially moved value”是什么意思？

在Rust中，当我们从一个结构体中移走了某个字段的所有权，但后面又想使用整个结构体时，就会发生这种情况。让我们看看此时的代码：

```rust
let mut timer = Timer::new(board.TIMER0);  // board.TIMER0的所有权被移动给timer变量了
// （其他行略）
let pwm = init_speaker(board);  // 试图使用整个 board，但 TIMER0 已经不在那里了
```

问题的根源在于：当我们调用 `Timer::new(board.TIMER0)` 时，`board.TIMER0` 的所有权被转移给了 `timer` 变量。这是因为 `TIMER0` 没有实现 `Copy` trait，所以发生的是移动而不是复制操作（所有权机制是Rust最独特的特性，每个值都有一个所有者，且所有权会随着赋值、函数调用的传参和返回等方式进行转移，后面章节会详述）。这导致 `board` 结构体变得不完整——它的 `TIMER0` 字段已经不存在了。解决方案是让 `init_speaker` 函数不要那么“贪婪”：只接收它真正需要的 `board` 结构体字段，而不是整个 `board` 结构体。修复这些编译问题后的代码变更如代码清单3-15所示：

代码清单3-15 3bc914da, ch03/mem-match/src/main.rs

```rust
#[entry]
fn main() -> ! {
// （其他行略）
    let mut pwm = init_speaker(
        board.pins.p0_02.into(),
        board.speaker_pin.into(),
        board.pins.p0_13.into(),
        board.PWM0,
    );
// （其他行略）
}

// 只把board结构体中所需的字段当作参数
fn init_speaker(
    p0_02: microbit::hal::gpio::Pin<microbit::hal::gpio::Disconnected>,
    speaker_pin: microbit::hal::gpio::Pin<microbit::hal::gpio::Disconnected>,
    p0_13: microbit::hal::gpio::Pin<microbit::hal::gpio::Disconnected>,
    pwm0: PWM0,
) -> Pwm<PWM0> {
    // Enable the speaker
    p0_02.into_push_pull_output(Level::High);
    // Get the speaker output pin
    let speaker_pin = speaker_pin.into_push_pull_output(Level::Low);

    // Convert P0_13 to push-pull output
    p0_13.into_push_pull_output(microbit::hal::gpio::Level::Low);

    // Initialize the PWM
    let pwm = Pwm::new(pwm0);
    // Set speaker pin as PWM output
    pwm.set_output_pin(Channel::C0, speaker_pin);
    pwm
}
// （其他行略）

```

# 3.9 迭代6：按下按钮A随机显示目标图案1秒

游戏的实现已经接近尾声。现在还剩两个功能需要完成：显示目标图案，以及告诉玩家猜测是否正确。在这个迭代中，我们先来实现显示目标图案的功能。

为了专注于学习Rust，我们采用简单的游戏设计。具体做法是：当玩家看到初始笑脸并按下A按钮后，系统会发出两声滴响提示音，提醒玩家记住即将显示的目标图案。1秒后，游戏会自动进入循环显示随机图案的阶段。由于我们已经掌握了相关代码的编写，现在可以独立完成这个功能。让我们来看看这个迭代的具体代码变更，如代码清单3-16所示：

代码清单3-16 6439974, ch03/mem-match/src/main.rs

```rust
// （其他行略）
#[derive(PartialEq)]
enum GameState {
// （其他行略）
    ShowingTargetPattern,
}

#[entry]
fn main() -> ! {
// （其他行略）
loop {
        match current_state {
            GameState::ShowingSmiley => {
// （其他行略）            
                if is_button_a_pressed && !was_button_a_pressed {
                    clear_buffer(&mut display_buffer);
                    display.show(&mut timer, display_buffer, DURATION_100_MS);
                    make_beep(&mut pwm, &mut timer);
                    timer.delay_ms(DURATION_100_MS);
                    make_beep(&mut pwm, &mut timer);
                    current_state = GameState::ShowingTargetPattern;
                }
                was_button_a_pressed = is_button_a_pressed;
            }

            GameState::ShowingTargetPattern => {
                let current_pattern = rng.next_range(PATTERN_NUM);
                writeln!(channel, "Target pattern: {}", current_pattern).ok();
                copy_pattern_to_buffer(&PATTERNS[current_pattern], &mut display_buffer);
                display.show(&mut timer, display_buffer, DURATION_1000_MS);
                current_state = GameState::ShowingRandomPattern;
            }
// （其他行略）
       }
    }
}
```

# 3.10 迭代7：比较玩家选择与目标图案

现在我们来到了小小白实现游戏的最后一个迭代。这个迭代的任务很明确：比较玩家选择的图案是否与目标图案相同。如果匹配，就显示一个大大的笑脸；如果不匹配，则显示哭脸。之后游戏会重新开始。这个功能相对简单，相信小小白能独立完成。完成后的代码变更或许如代码清单3-17所示：

代码清单3-17 bb4677a, ch03/mem-match/src/main.rs

```rust
// （其他行略）
static BIG_SMILEY: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
    [0, 1, 1, 1, 0],
];

static CRYING_SMILEY: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
];

#[entry]
fn main() -> ! {
// （其他行略）
    // 将target_pattern绑定为0（但0是第一幅图，但其实此时并不确定目标图案是哪幅）
    let mut target_pattern = 0;

    loop {
        match current_state {
// （其他行略）
            GameState::ShowingTargetPattern => {
                let current_pattern = rng.next_range(PATTERN_NUM);
                target_pattern = current_pattern;
                writeln!(channel, "Target pattern: {}", target_pattern).ok();
// （其他行略）
            }

            GameState::ShowingRandomPattern => {
// （其他行略）
                while elapsed < 1000 {
// （其他行略）
                    if is_button_b_pressed && !was_button_b_pressed {
// （其他行略）
                        if current_pattern == target_pattern {
                            copy_pattern_to_buffer(&BIG_SMILEY, &mut display_buffer);
                            display.show(&mut timer, display_buffer, 1000);
                        } else {
                            copy_pattern_to_buffer(&CRYING_SMILEY, &mut display_buffer);
                            display.show(&mut timer, display_buffer, 1000);
                        }
// （其他行略）
                    }
// （其他行略）
                }
            }
        }
    }
}

```

🧠注意到代码清单3-17中有个注释，提到将`target_pattern`初始绑定为`0`时还不确定目标图案是哪一幅。那么该如何更好地处理这种情况呢？

💡此时可以利用Rust的Option。Option枚举类型非常适合表示"可能有值也可能没有值"的情况。我们可以将`target_pattern`声明为`Option<usize>`类型，初始值设为`None`，当显示目标图案时再将其设置为`Some(current_pattern)`。这样不仅能更准确地表达我们的意图，还能借助编译器确保我们正确处理了所有可能的情况，如代码清单3-18所示：

代码清单3-18 d40978e, ch03/mem-match/src/main.rs

```rust
// （其他行略）
#[entry]
fn main() -> ! {
// （其他行略）
    // 声明一个可变的Option类型变量target_pattern,内部类型是usize,初始值为None
    // Option用于处理可能存在也可能不存在的值
    let mut target_pattern: Option<usize> = None;

    loop {
        match current_state {
// （其他行略）
            GameState::ShowingTargetPattern => {
// （其他行略）
                // 将current_pattern包装成Some赋值给target_pattern
                target_pattern = Some(current_pattern);
                
                // 使用writeln!宏输出目标模式
                // unwrap()用于获取Option中的值,如果是None会panic
                // ok()将Result转换为Option,忽略可能的错误
                writeln!(channel, "Target pattern: {}", target_pattern.unwrap()).ok();
// （其他行略）
            }

            GameState::ShowingRandomPattern => {
// （其他行略）
                while elapsed < 1000 {
// （其他行略）
                    if is_button_b_pressed && !was_button_b_pressed {
// （其他行略）
                        // 比较当前模式是否与目标模式相同
                        // unwrap_or()在值为None时返回指定的默认值usize::MAX
                        if current_pattern == target_pattern.unwrap_or(usize::MAX) {
// （其他行略）                        
                        } else {
// （其他行略）
                        }
// （其他行略）
                    }
// （其他行略）
                }
            }
        }
    }
}
```

🧠为什么在代码清单3-18中，我们使用标准库的`Option`、`None`和`Some`等枚举类型时，无需用`use`语句将它们导入源代码？

💡这是因为`Option`、`None`和`Some`都被预导入（prelude）到了每个Rust程序中。prelude模块包含了Rust最常用的类型、trait等，让我们不必显式导入就能直接使用它们。这种设计既方便了编程，又避免了源代码中出现过多的use语句。