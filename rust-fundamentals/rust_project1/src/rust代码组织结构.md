---

# 🧩 Rust 模块系统

---

## 🖥️ 二进制 Crate
- **生成一个可执行程序**。
- 通常包含一个 `main` 函数，并在编译时生成一个可运行的文件。

## 📚 库 Crate
- **生成一个库文件**，供其他程序或库引用。
- 库没有 `main` 函数，通常用来提供公共功能供其他项目使用。

---

## 📦 **Package（包）**
在 Rust 中，**Package** 是一个 Cargo 单位，它描述了如何构建、测试和共享 Crates。通过 Package，Rust 项目可以更好地进行组织和管理。

### **Package 包含的内容**
- **1 个 `Cargo.toml` 文件**：该文件是 Package 的核心，包含项目的元数据、依赖项、构建脚本等。Cargo 通过 `Cargo.toml` 来指导如何构建项目中的 crate。
- **0 或 1 个 Library Crate（库 crate）**：一个 Package 可以包含最多一个库 crate，库 crate 用于定义共享的功能或 API，通常被其他项目引用。
- **任意数量的 Binary Crate（二进制 crate）**：一个 Package 可以包含多个二进制 crate，每个二进制 crate 生成一个可执行文件。二进制 crate 通常有 `main.rs` 作为入口文件。
- **至少包含一个 Crate**：每个 Package 必须包含至少一个 crate（库 crate 或二进制 crate）。即使只有一个库 crate，它也需要在构建时有一个入口文件。

### **Package 的职责**
- **构建**：定义如何构建多个 crate。
- **测试**：可以在 Package 内部进行测试，Cargo 会自动识别并执行这些测试。
- **共享 crate**：Package 还可以作为一个库或二进制文件发布和共享，方便其他开发者使用。

---

## 📄 **Crate（单元包）**
**Crate** 是 Rust 中最小的构建单元，每个 Crate 是一个模块树，可以产生一个库或可执行文件。Crate 提供了 Rust 代码的逻辑单元，并且是通过 `cargo build` 或 `cargo run` 来构建的。

### **Crate 的类型**
- **🖥️ Binary Crate（二进制 crate）**：二进制 crate 是生成可执行文件的 crate。一个二进制 crate 通常包含一个 `main.rs` 文件，该文件定义了程序的入口点（`fn main()`）。二进制 crate 通常用于构建应用程序。
- **📚 Library Crate（库 crate）**：库 crate 提供一个可以被其他 crate 引用和使用的功能集合。它没有入口函数（`main`），并且可以通过 `use` 关键字将其功能暴露给其他模块或项目。

### **Crate Root（crate 根）**
- **Crate Root** 是指 Rust 编译器在编译 Crate 时的起点文件。它是 Rust 中代码的顶层模块，也是构建该 Crate 的起始点。
  - 对于 **binary crate**，Crate Root 是 `src/main.rs`。
  - 对于 **library crate**，Crate Root 是 `src/lib.rs`。

Crate Root 的作用是告诉 Rust 编译器从哪里开始分析并构建模块树。

---

## 🗂️ **Module（模块）**
在 Rust 中，**Module（模块）** 是组织代码的基本单元。模块帮助你将代码分成多个文件和子模块，提供更好的组织结构。

### **模块的作用**
- **代码组织**：模块使代码更加模块化和可管理。你可以将功能相关的代码组织到同一个模块下，**提高代码的可读性**和**可维护性**。
- **作用域控制**：模块通过 `mod` 关键字定义，可以控制哪些函数、结构体、枚举等在外部可见。模块的内部成员默认为私有（private），只有通过 `pub` 关键字显式声明为公共（public）的成员才能被外部访问。
- **私有路径和公有路径**：Rust 通过模块的可见性来控制访问权限。私有模块的成员无法直接从外部访问，而公共模块的成员则可以通过 `use` 导入和访问。

### **模块的结构**
- **定义模块**：使用 `mod` 关键字来定义模块。例如，`mod mymodule;` 会包含一个名为 `mymodule.rs` 的文件，或在子目录 `mymodule/mod.rs` 中定义该模块。
- **嵌套模块**：模块可以嵌套，形成层级结构。每个模块都可以有自己的子模块，从而实现复杂的模块组织结构。
- **公开模块**：通过 `pub` 关键字可以公开一个模块或其成员，允许外部访问。

#### **`use` 关键字**
`use` 关键字用于将模块或模块成员引入当前作用域。通过 `use`，你可以方便地访问其他模块中的功能，而不需要使用完全限定的路径。例如：

```rust
use std::fs::File; // 使用 std::fs 模块中的 File 类型
```

---

## 📂 **文件结构与 Crate Root**
### **`src/main.rs`**
- **Binary Crate 的 Crate Root**：这是指 Rust 项目中的二进制 crate（即可执行文件）入口点文件。在一个 Rust 项目中，`src/main.rs` 是二进制 crate 的根文件，也就是程序执行时的入口点（`main` 函数所在的地方）。一个二进制 crate 代表的是一个可执行程序。
- **Crate 名和 Package 名相同**：在 Cargo 项目中，crate 和 package 是有区别的。crate 是代码单元，而 package 是分发单元。通常，一个 package 中可以包含多个 crate。但在默认情况下，Rust 的约定是 crate 名（在 `Cargo.toml` 中指定的 crate 名）与 package 名是相同的。也就是说，Cargo 会自动将项目的 crate 名和 package 名设为一致，除非你特别修改。

### **`src/lib.rs`**
- **Package 包含一个 Library Crate**：`src/lib.rs` 是定义库 crate 的根文件。一个库 crate 是用来提供功能库的，不是直接执行的。它通常被其他项目作为依赖来使用。`src/lib.rs` 中可以包含库的公共 API。
- **Library Crate 的 Crate Root**：这意味着 `src/lib.rs` 是库 crate 的起点或根文件（crate root）。它定义了库的主要内容，供其他 crate 或应用程序引用。
- **Crate 名与 Package 名相同**：这一点跟 `src/main.rs` 中的内容一致，说明库 crate 和 package 名也是相同的。

---

## 🛠️ **Crate 的作用**
- **将相关功能组合到一个作用域内**，便于在项目间进行共享，防止冲突。
- Cargo 把 Crate Root 文件交给 `rustc` 来构建 library 或 binary。

---

## 🧠 **总结**
- **模块系统** 是 Rust 中组织代码的核心机制，通过 **Package**、**Crate** 和 **Module** 的协作，Rust 项目得以高效组织和管理。
- **Package** 是 Cargo 的构建单位，包含一个或多个 Crate。
- **Crate** 是最小的编译单元，分为二进制 Crate 和库 Crate。
- **Module** 用于组织代码，控制作用域和可见性，提高代码的可读性和复用性。

---

## 🧑‍💻 **Rust 编程：模块（Module）**

在 Rust 中，**模块（Module）** 是组织代码的基本单元。模块帮助你将代码分成多个文件和子模块，提供更好的组织结构。

---

### 🚀 **模块的作用**
模块在 Rust 中的作用主要体现在以下几个方面：

- **代码组织**：模块使代码更加模块化和可管理。你可以将功能相关的代码组织到同一个模块下，**提高代码的可读性**和**可维护性**。
- **作用域控制**：模块通过 `mod` 关键字定义，可以控制哪些函数、结构体、枚举等在外部可见。模块的内部成员默认为私有（private），只有通过 `pub` 关键字显式声明为公共（public）的成员才能被外部访问。
- **私有路径和公有路径**：Rust 通过模块的可见性来控制访问权限。私有模块的成员无法直接从外部访问，而公共模块的成员则可以通过 `use` 导入和访问。

---

### 📁 **模块的结构**

#### 1. **定义模块**
模块通过 `mod` 关键字定义。例如：

```rust
mod mymodule; // 引入 mymodule.rs 文件或目录中的 mod.rs 文件
```

#### 2. **嵌套模块**
模块可以嵌套，形成层级结构。每个模块都可以有自己的子模块，从而实现复杂的模块组织结构。

```rust
mod outer

 {
    pub mod inner {  // 嵌套的模块
        pub fn greet() {
            println!("Hello from the inner module!");
        }
    }
}
```

#### 3. **公开模块**
通过 `pub` 关键字可以公开一个模块或其成员，允许外部访问：

```rust
pub fn my_function() {
    println!("This function is public.");
}
```

---

### 🧩 **模块与文件系统的关系**
在 Rust 中，模块的结构通常会反映在文件系统中。

#### 单一文件模块
```plaintext
src/
  ├── main.rs // 引用其他模块
  ├── mymodule.rs
```

#### 子模块和目录
当一个模块变得复杂时，可以将其拆分到不同的文件中。例如：

```plaintext
src/
  ├── outer/
  │   └── mod.rs  // 外层模块
  │   └── inner.rs  // 内层模块
```

---

### 🔑 **控制模块项的可见性**
模块的项默认是 **私有** 的，除非通过 `pub` 显式声明为公有。通过 `pub` 可以控制项的可见性，以下是一些例子：

#### 公有结构体与私有结构体
```rust
mod my_module {
    pub struct PublicStruct {
        pub field: i32,  // 公有字段
    }

    struct PrivateStruct {
        field: i32,  // 私有结构体
    }
}
```

#### 公有与私有函数
```rust
mod my_module {
    pub fn public_function() {
        println!("This is a public function.");
    }

    fn private_function() {
        println!("This is a private function.");
    }
}
```

---

### 🛠 **使用 `use` 关键字导入模块**
`use` 关键字可以将模块或模块成员引入当前作用域，方便访问其他模块中的功能。

```rust
use std::fs::File;  // 引入 std::fs 模块中的 File 类型
```

---

### 📑 **示例：创建模块**

#### **定义一个基本的 Module**
```rust
mod math {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

fn main() {
    let result = math::add(5, 7); // 调用模块中的函数
    println!("Result: {}", result);
}
```

#### **模块的嵌套**
```rust
mod outer {
    pub mod inner {
        pub fn greet() {
            println!("Hello from the inner module!");
        }
    }
}

fn main() {
    outer::inner::greet(); // 访问嵌套模块中的函数
}
```

#### **模块包含其他项**
```rust
mod shapes {
    pub struct Circle {
        pub radius: f64,
    }

    pub enum Shape {
        Circle(Circle),
        Square(f64),
    }

    pub fn area(shape: &Shape) -> f64 {
        match shape {
            Shape::Circle(c) => std::f64::consts::PI * c.radius * c.radius,
            Shape::Square(s) => s * s,
        }
    }
}

fn main() {
    let circle = shapes::Circle { radius: 2.0 };
    let shape = shapes::Shape::Circle(circle);

    println!("Area: {}", shapes::area(&shape));
}
```

---

