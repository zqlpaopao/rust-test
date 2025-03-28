

# 1、创建和初始化

- `new()`: 创建一个空的 `Vec<T>`。
- `with_capacity(capacity)`: 创建一个具有指定容量的 `Vec<T>`。
- try_with_capacity

在 Rust 中，`Vec::try_with_capacity` 是一个实验性的方法，用于尝试创建一个具有指定容量的 `Vec`。如果内存分配失败，它会返回一个错误，而不是引发恐慌。这对于需要处理内存分配失败的场景特别有用。

**使用示例**

```rust
use std::collections::TryReserveError;

fn main() -> Result<(), TryReserveError> {
    let capacity = 10;
    let vec = Vec::try_with_capacity(capacity)?;

    // 使用 vec 进行其他操作
    Ok(())
}
```

**特点**

- **安全性**：通过返回 `Result`，允许调用者处理内存分配失败的情况。
- **适用场景**：适用于需要在内存分配失败时进行错误处理的应用程序。

**注意事项**

- `try_with_capacity` 可能需要启用特定的 Rust 版本或特性，因为它可能不是在所有版本中都稳定。

这种方法对于需要高可靠性和错误处理的系统来说非常有用。

## from_raw_parts

`Vec::from_raw_parts` 是一个在 Rust 中用于从原始指针构建 `Vec` 的方法。它允许你将已经分配的内存块转化为一个 `Vec`，但需要小心使用，因为它涉及到不安全代码。

### 使用场景

`from_raw_parts` 通常用于与其他语言或库交互时，或者在你需要手动管理内存的情况下。

方法签名

```rust
unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Vec<T>
```

参数

- **`ptr`**：指向内存块的指针。
- **`length`**：`Vec` 中的元素数量。
- **`capacity`**：`Vec` 的容量。

使用示例

```rust
use std::mem;
use std::ptr;

fn main() {
    // 假设我们有一个已经分配的内存块
    let mut numbers = vec![1, 2, 3];
    let ptr = numbers.as_mut_ptr();
    let length = numbers.len();
    let capacity = numbers.capacity();

    // 防止原有的 Vec 释放内存
    mem::forget(numbers);

    // 从原始指针重建 Vec
    let unsafe_vec = unsafe { Vec::from_raw_parts(ptr, length, capacity) };

    // 使用重建的 Vec
    println!("{:?}", unsafe_vec);
}
```

注意事项

- **不安全性**：由于涉及裸指针操作，使用 `from_raw_parts` 是不安全的，需要在 `unsafe` 块中调用。
- **内存管理**：确保指针指向的内存块是有效的，并且在 `Vec` 的生命周期内不会被释放或修改。
- **所有权转移**：调用 `from_raw_parts` 后，`Vec` 接管内存管理，原有的内存块不应再被使用。

使用 `from_raw_parts` 时，务必确保对内存管理有充分的理解，以避免出现未定义行为或内存安全问题。

## from_nonnull_parts



`Vec::from_nonnull_parts` 是一个用于从 `NonNull` 指针构建 `Vec` 的方法。与 `from_raw_parts` 类似，它允许你将已分配的内存块转化为一个 `Vec`，但使用的是 `NonNull` 指针类型，这是一种不允许为空的指针。

方法签名

```rust
unsafe fn from_nonnull_parts(ptr: NonNull<T>, length: usize, capacity: usize) -> Vec<T>
```

参数

- **`ptr`**：指向内存块的 `NonNull` 指针。
- **`length`**：`Vec` 中的元素数量。
- **`capacity`**：`Vec` 的容量。

使用示例

```rust
use std::ptr::NonNull;
use std::mem;

fn main() {
    // 假设我们有一个已经分配的内存块
    let mut numbers = vec![1, 2, 3];
    let ptr = NonNull::new(numbers.as_mut_ptr()).expect("Pointer should not be null");
    let length = numbers.len();
    let capacity = numbers.capacity();

    // 防止原有的 Vec 释放内存
    mem::forget(numbers);

    // 从 NonNull 指针重建 Vec
    let unsafe_vec = unsafe { Vec::from_nonnull_parts(ptr, length, capacity) };

    // 使用重建的 Vec
    println!("{:?}", unsafe_vec);
}
```

注意事项

- **不安全性**：与 `from_raw_parts` 一样，`from_nonnull_parts` 是不安全的，需要在 `unsafe` 块中调用。
- **内存管理**：确保指针指向的内存块是有效的，并且在 `Vec` 的生命周期内不会被释放或修改。
- **所有权转移**：调用 `from_nonnull_parts` 后，`Vec` 接管内存管理，原有的内存块不应再被使用。

使用 `from_nonnull_parts` 时，确保你对内存管理和所有权规则有深入的理解，以避免内存安全问题。

## new_in 自定义内存分配策略

`Vec::new_in` 是一种用于在自定义分配器中创建 `Vec` 的方法。这在需要控制内存分配策略的场景中非常有用。

使用场景

`new_in` 允许你为 `Vec` 指定一个自定义分配器，适用于需要特殊内存管理的应用场景。

方法签名

```rust
fn new_in(alloc: A) -> Vec<T, A>
```

参数

- **`alloc`**：一个实现了 `Allocator` trait 的自定义分配器。

使用示例

假设你有一个自定义分配器：

```rust
use std::alloc::{Global, System};
use std::vec::Vec;

fn main() {
    // 使用全局分配器创建一个 Vec
    let vec_global: Vec<i32, Global> = Vec::new_in(Global);

    // 使用系统分配器创建一个 Vec
    let vec_system: Vec<i32, System> = Vec::new_in(System);

    // 进行其他操作
    println!("{:?} {:?}", vec_global, vec_system);
}
```

注意事项

- **分配器支持**：确保使用的分配器实现了 `Allocator` trait。
- **稳定性**：在使用自定义分配器之前，确认所用的 Rust 版本对该功能的支持。

通过 `new_in`，你可以更灵活地管理内存分配，尤其是在需要优化性能或资源使用的场景中。

## with_capacity_in

`Vec::with_capacity_in` 和 `Vec::try_with_capacity_in` 是用于在自定义分配器中创建具有指定容量的 `Vec` 的方法。

`Vec::with_capacity_in`

该方法创建一个具有指定初始容量的 `Vec`，并使用提供的分配器。

方法签名

```rust
fn with_capacity_in(capacity: usize, alloc: A) -> Vec<T, A>
```

参数

- **`capacity`**：`Vec` 的初始容量。
- **`alloc`**：实现了 `Allocator` trait 的自定义分配器。

使用示例

```rust
use std::alloc::{Global, System};
use std::vec::Vec;

fn main() {
    // 使用全局分配器创建一个具有指定容量的 Vec
    let vec_global: Vec<i32, Global> = Vec::with_capacity_in(10, Global);

    // 使用系统分配器创建一个具有指定容量的 Vec
    let vec_system: Vec<i32, System> = Vec::with_capacity_in(10, System);

    println!("{:?} {:?}", vec_global, vec_system);
}
```

## `Vec::try_with_capacity_in`



该方法尝试创建一个具有指定容量的 `Vec`，并使用提供的分配器。如果内存分配失败，则返回一个错误。

方法签名

```rust
fn try_with_capacity_in(capacity: usize, alloc: A) -> Result<Vec<T, A>, AllocError>
```

参数

- **`capacity`**：`Vec` 的初始容量。
- **`alloc`**：实现了 `Allocator` trait 的自定义分配器。

使用示例

```rust
use std::alloc::{Global, System, AllocError};
use std::vec::Vec;

fn main() -> Result<(), AllocError> {
    // 使用全局分配器尝试创建一个具有指定容量的 Vec
    let vec_global: Vec<i32, Global> = Vec::try_with_capacity_in(10, Global)?;

    // 使用系统分配器尝试创建一个具有指定容量的 Vec
    let vec_system: Vec<i32, System> = Vec::try_with_capacity_in(10, System)?;

    println!("{:?} {:?}", vec_global, vec_system);
    Ok(())
}
```

注意事项

- **自定义分配器**：确保分配器实现了 `Allocator` trait。
- **错误处理**：`try_with_capacity_in` 可以处理内存分配失败的情况，适合需要高可靠性的场景。

这些方法提供了更大的灵活性和控制，特别是在需要特定内存管理策略的情况下。



from_nonnull_in into_raw_parts 和上面类似 需要自定义分配器





`Vec::into_raw_parts` 和 `Vec::into_raw_parts_with_alloc` 是用于将 `Vec` 分解为其基本组成部分的方法。

## `Vec::into_raw_parts`

此方法将 `Vec` 分解为一个原始指针、长度和容量。

#### 方法签名

```rust
fn into_raw_parts(self) -> (*mut T, usize, usize)
```

#### 返回值

- **指针**：指向 `Vec` 数据的原始指针。
- **长度**：`Vec` 中元素的数量。
- **容量**：`Vec` 的容量。

#### 使用示例

```rust
fn main() {
    let vec = vec![1, 2, 3];
    let (ptr, len, capacity) = vec.into_raw_parts();

    // 使用原始指针时需要小心，确保安全性
    unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        println!("{:?}", slice);

        // 需要手动管理内存释放
        let _vec = Vec::from_raw_parts(ptr, len, capacity);
    }
}
```

## `Vec::into_raw_parts_with_alloc`

此方法不仅分解 `Vec`，还返回其使用的分配器。

#### 方法签名

```rust
fn into_raw_parts_with_alloc(self) -> (*mut T, usize, usize, A)
```

#### 返回值

- **指针**：指向 `Vec` 数据的原始指针。
- **长度**：`Vec` 中元素的数量。
- **容量**：`Vec` 的容量。
- **分配器**：`Vec` 使用的分配器。

#### 使用示例

```rust
use std::alloc::Global;

fn main() {
    let vec: Vec<i32, Global> = Vec::with_capacity_in(10, Global);
    let (ptr, len, capacity, alloc) = vec.into_raw_parts_with_alloc();

    // 使用原始指针和分配器时需要小心
    unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        println!("{:?}", slice);

        // 重新构建 Vec 时需要提供分配器
        let _vec = Vec::from_raw_parts_in(ptr, len, capacity, alloc);
    }
}
```

### 注意事项

- **安全性**：使用这些方法后，内存管理变成手动，需要特别小心以避免内存泄漏或未定义行为。
- **内存释放**：确保在不再需要时正确释放内存。
- **分配器**：`into_raw_parts_with_alloc` 在使用自定义分配器时特别有用，因为它保留了分配器信息。



# 2、容量相关

在 Rust 的 `Vec<T>` 中，这些方法用于管理向量的容量和内存分配。

### `capacity`

- **功能**：返回 `Vec` 的当前容量，即在不重新分配内存的情况下可以存储的元素数量。
- **使用示例**：

  ```rust
  fn main() {
      let vec = vec![1, 2, 3];
      println!("Capacity: {}", vec.capacity());
  }
  ```

### `reserve`

- **功能**：确保 `Vec` 至少有足够的容量来容纳指定数量的额外元素。
- **使用示例**：

  ```rust
  fn main() {
      let mut vec = vec![1, 2, 3];
      vec.reserve(10);
      println!("Capacity after reserve: {}", vec.capacity());
  }
  //Capacity after reserve: 13
  
  ```

### `reserve_exact`

- **功能**：确保 `Vec` 有足够的容量来容纳指定数量的额外元素，但不留多余的空间。
- **使用示例**：

  ```rust
  fn main() {
      let mut vec = vec![1, 2, 3];
      vec.reserve_exact(10);
      println!("Capacity after reserve_exact: {}", vec.capacity());
  }
  //Capacity after reserve_exact: 13
  
  ```

### `try_reserve`

- **功能**：尝试增加 `Vec` 的容量以容纳指定数量的额外元素。如果内存分配失败，会返回一个错误。
- **使用示例**：

  ```rust
  fn main() {
      let mut vec = vec![1, 2, 3];
      if vec.try_reserve(10).is_err() {
          println!("Failed to reserve memory");
      }
  }
  ```

### `try_reserve_exact`

- **功能**：尝试精确增加 `Vec` 的容量以容纳指定数量的额外元素。如果内存分配失败，会返回一个错误。
- **使用示例**：

  ```rust
  fn main() {
      let mut vec = vec![1, 2, 3];
      if vec.try_reserve_exact(10).is_err() {
          println!("Failed to reserve memory");
      }
  }
  ```

### `shrink_to_fit`

- **功能**：将 `Vec` 的容量缩小到等于其长度，以释放多余的内存。
- **使用示例**：

  ```rust
  fn main() {
      let mut vec = vec![1, 2, 3];
      vec.reserve(10); // 增加容量
      println!("Capacity before shrink: {}", vec.capacity());
      vec.shrink_to_fit();
      println!("Capacity after shrink: {}", vec.capacity());
  }
  
  Capacity before shrink: 13
  Capacity after shrink: 3
  ```

​	注意事项

- **性能**：`reserve` 和 `reserve_exact` 的区别在于前者可能会申请多于所需的内存以优化未来的增长。
- **错误处理**：`try_reserve` 和 `try_reserve_exact` 提供了失败时的错误处理机制，适用于内存紧张的环境。



在 Rust 的 `Vec<T>` 中，`shrink_to` 和 `into_boxed_slice` 是用于优化内存和转换的方法。

### `shrink_to`

- **功能**：将 `Vec` 的容量缩小到不小于指定的容量，如果当前长度大于指定容量，则缩小到当前长度。
- **使用示例**：

  ```rust
  fn main() {
      let mut vec = vec![1, 2, 3, 4, 5];
      vec.reserve(10); // 增加容量
      println!("Capacity before shrink: {}", vec.capacity());
      vec.shrink_to(3); // 缩小容量
      println!("Capacity after shrink: {}", vec.capacity());
  }
  ```
  
  你是对的，我在之前的解释中犯了一个错误。实际上，`shrink_to` 方法的行为是：
  
  - 如果当前容量小于或等于指定的容量（在这个例子中是 3），则不会发生任何变化。
  - 如果当前容量大于指定的容量，但小于或等于当前长度加上指定的容量（在这个例子中是 5+3=8），则容量将被设置为当前长度加上指定的容量。
  - 如果当前容量大于当前长度加上指定的容量（在这个例子中是 15 > 5+3），则容量将被设置为当前长度加上指定的容量。
  
  在你的示例中，`vec.shrink_to(3)` 并没有将容量缩小到 3，因为当前长度是 5，大于 3。实际上，容量保持不变，仍然是 15。然后，`vec.shrink_to_fit()` 将容量缩小到等于当前长度，即 5。
  
  正确的示例代码应该是：
  
  ```rust
  let mut vec = vec![1, 2, 3, 4, 5];
  vec.reserve(10); // 增加容量
  println!("Capacity before shrink: {}", vec.capacity());
  vec.shrink_to_fit(); // 缩小容量到等于当前长度
  println!("Capacity after shrink: {}", vec.capacity());
  println!("Capacity after shrink: vec{:?}", vec);
  ```
  
  这将输出：
  
  ```
  Capacity before shrink: 15
  Capacity after shrink: 5
  Capacity after shrink: vec[1, 2, 3, 4, 5]
  ```
  
  我为之前的错误解释道歉，希望这个修正后的解释能够帮助你更好地理解 `shrink_to` 和 `shrink_to_fit` 的行为。



### `shrink_to`

- **功能**：将 `Vec` 的容量缩小到不小于指定的容量。如果当前长度大于指定容量，则缩小到当前长度。
- **使用场景**：当你想将容量减少到某个特定值或当前长度时使用。

### `shrink_to_fit`

- **功能**：将 `Vec` 的容量缩小到等于其当前长度。
- **使用场景**：当你想要释放所有多余的内存，仅保留存储当前元素所需的容量时使用。

# 3、vec转换

### `into_boxed_slice`

- **功能**：将 `Vec` 转换为一个 `Box<[T]>`，这会将 `Vec` 的所有权转移到堆上分配的固定大小的切片。
- **使用示例**：

  ```rust
  fn main() {
      let vec = vec![1, 2, 3];
      let boxed_slice: Box<[i32]> = vec.into_boxed_slice();
      println!("Boxed slice: {:?}", boxed_slice);
  }
  ```

注意事项

- **`shrink_to`**：可以用于减少 `Vec` 的内存占用，但要注意最低容量限制。
- **`into_boxed_slice`**：转换后，`Vec` 不再可变，适合需要固定大小内存的场景。



在 Rust 中，`Vec<T>` 提供了两个方法来将其转换为其他类型的数据结构：`into_boxed_slice()` 和 `truncate()`。这两个方法的用途和效果是不同的。

### `into_boxed_slice()`

- **功能**：将 `Vec<T>` 转换为 `Box<[T]>`，即一个在堆上分配的、不可变的、动态大小的数组。
- **使用场景**：当你需要将一个 `Vec` 转换为一个堆分配的数组时使用，通常是因为你想要将这个数组传递给一个函数或存储在一个结构体中。

### `truncate()` 裁剪

- **功能**：将 `Vec<T>` 的长度截断为指定的大小。如果当前长度小于指定的大小，则不会发生任何变化。
- **使用场景**：当你需要从 `Vec` 的末尾删除元素时使用，例如在处理文件或网络数据时。

示例

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 into_boxed_slice()
    let boxed_slice = vec.into_boxed_slice();
    println!("Boxed slice: {:?}", boxed_slice);

    // 使用 truncate()
    let mut vec2 = vec![1, 2, 3, 4, 5];
    vec2.truncate(3);
    println!("Truncated vector: {:?}", vec2);
}

Boxed slice: [1, 2, 3, 4, 5]
Truncated vector: [1, 2, 3]
```

在这个示例中，我们首先将一个 `Vec` 转换为一个 `Box<[T]>`，然后打印出结果。接着，我们创建了另一个 `Vec`，并使用 `truncate()` 方法将其长度截断到 3。最后，我们打印出截断后的 `Vec` 的内容。

这两个方法的主要区别在于它们的目的和结果类型。`into_boxed_slice()` 用于将 `Vec` 转换为一个堆分配的数组，而 `truncate()` 用于从 `Vec` 的末尾删除元素。



在 Rust 中，`Vec<T>` 提供了两个方法来获取其元素的引用：`as_slice()` 和 `as_mut_slice()`。这两个方法都返回一个 `&[T]` 类型的引用，表示向量的所有元素。

### `as_slice()`

- **功能**：返回一个不可变的引用到向量的所有元素。
- **使用场景**：当你需要对向量的元素进行只读操作时使用，例如在遍历或查找元素时。

### `as_mut_slice()`

- **功能**：返回一个可变的引用到向量的所有元素。
- **使用场景**：当你需要对向量的元素进行修改操作时使用，例如在排序、反转或填充元素时。

示例

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 as_slice()
    let slice :&[i32] = vec.as_slice();
    println!("Slice: {:?}", slice);

    // 使用 as_mut_slice()
    let mut slice_mut:&mut[i32] = vec.as_mut_slice();
    slice_mut[0] = 10;
    println!("Mutated slice: {:?}", slice_mut);
}

Slice: [1, 2, 3, 4, 5]
Mutated slice: [10, 2, 3, 4, 5]
```

在这个示例中，我们首先使用 `as_slice()` 方法获取一个不可变的引用到向量的所有元素，并打印出结果。然后，我们使用 `as_mut_slice()` 方法获取一个可变的引用，并修改了第一个元素的值。最后，我们再次打印出结果，展示了元素的值已经被修改。

这两个方法的主要区别在于它们返回的引用类型：`as_slice()` 返回一个不可变的引用，而 `as_mut_slice()` 返回一个可变的引用。选择哪个方法取决于你是否需要修改向量的元素。



在 Rust 中，`Vec<T>` 提供了两个方法来获取其底层数据的原始指针：`as_ptr()` 和 `as_mut_ptr()`。这两个方法都返回一个指向向量第一个元素的裸指针。

### `as_ptr()`

- **功能**：返回一个不可变的裸指针到向量的第一个元素。
- **使用场景**：当你需要将向量的数据传递给外部库或系统调用时使用，例如在与 C 或 C++ 接口交互时。

### `as_mut_ptr()`

- **功能**：返回一个可变的裸指针到向量的第一个元素。
- **使用场景**：当你需要在不使用 Rust 的借用检查的情况下修改向量的内容时使用，通常是因为你需要在底层操作向量的内存。

示例

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 as_ptr()
    let ptr : *const i32 = vec.as_ptr();
    println!("Raw pointer: {:p}", ptr);

    // 使用 as_mut_ptr()
    let mut ptr_mut:*mut i32 = vec.as_mut_ptr();
    unsafe {
        *ptr_mut = 10;
    }
    println!("Modified vector: {:?}", vec);
}
Raw pointer: 0x6000030f4140
Modified vector: [10, 2, 3, 4, 5]

```

在这个示例中，我们首先使用 `as_ptr()` 方法获取一个不可变的裸指针到向量的第一个元素，并打印出结果。然后，我们使用 `as_mut_ptr()` 方法获取一个可变的裸指针，并在 `unsafe` 块中修改了第一个元素的值。最后，我们打印出修改后的向量。

这两个方法的主要区别在于它们返回的指针类型：`as_ptr()` 返回一个不可变的指针，而 `as_mut_ptr()` 返回一个可变的指针。选择哪个方法取决于你是否需要修改向量的内容。请注意，在使用裸指针时，需要小心避免数据竞争和内存安全问题。



在 Rust 中，`Vec<T>` 提供了两个与内存管理相关的方法：`allocator()` 和 `set_len()`。

### `allocator()`

- **功能**：返回一个指向 `Vec` 使用的分配器的引用。
- **使用场景**：当你需要了解或操作 `Vec` 的内存分配时使用，例如在进行高级优化或与特定的内存管理策略集成时。

### `set_len()`

- **功能**：将 `Vec` 的长度设置为指定的值，并且在必要时填充或截断元素。
- **使用场景**：当你需要手动控制 `Vec` 的长度时使用，例如在处理网络数据包或其他需要精确控制缓冲区大小的情况下。

示例

```rust
use std::alloc::System;

fn main() {
    let mut vec = Vec::<i32>::with_capacity(5);
    vec.push(1);
    vec.push(2);

    // 使用 allocator()
    let allocator = vec.allocator();
    println!("Allocator: {:?}", allocator);

    // 使用 set_len()
    vec.set_len(3);
    vec[2] = 3;
    println!("Modified vector: {:?}", vec);
}
```

在这个示例中，我们首先创建了一个初始容量为 5 的 `Vec<i32>`，并添加了两个元素。然后，我们使用 `allocator()` 方法获取了 `Vec` 使用的分配器的引用，并打印出结果。接着，我们使用 `set_len()` 方法将向量的长度设置为 3，并手动添加了第三个元素。最后，我们打印出修改后的向量。

这两个方法的主要区别在于它们的功能和使用场景：`allocator()` 用于获取 `Vec` 的分配器信息，而 `set_len()` 用于手动控制 `Vec` 的长度。请注意，在使用 `set_len()` 时，需要小心避免越界访问和未初始化的数据。



# 4、元素交换与删除

在 Rust 中，`Vec<T>` 提供了两个与元素交换和移除相关的方法：`swap_remove()` 和 `remove()`。

### `swap_remove(index: usize) -> T`

- **功能**：将指定索引处的元素与最后一个元素交换，然后从向量中移除最后一个元素并返回它。
- **使用场景**：当你需要在不保留顺序的情况下快速移除元素时使用，例如在实现某些算法或数据结构时。

### `remove(index: usize) -> T`

- **功能**：移除指定索引处的元素，并返回被移除的元素。
- **使用场景**：当你需要按顺序移除元素时使用，例如在处理用户输入或其他需要维持元素顺序的场景中。

示例

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 swap_remove()
    let removed_element = vec.swap_remove(2);
    println!("Removed element: {}", removed_element);
    println!("Modified vector: {:?}", vec);

    // 使用 remove()
    let removed_element = vec.remove(1);
    println!("Removed element: {}", removed_element);
    println!("Modified vector: {:?}", vec);
}

Removed element: 3
Modified vector: [1, 2, 5, 4]
Removed element: 2
Modified vector: [1, 5, 4]

```

在这个示例中，我们首先创建了一个包含五个元素的 `Vec<i32>`。然后，我们使用 `swap_remove()` 方法将索引为 2 的元素与最后一个元素交换，并从向量中移除最后一个元素。接着，我们使用 `remove()` 方法移除索引为 1 的元素。最后，我们打印出被移除的元素和修改后的向量。

这两个方法的主要区别在于它们的行为和使用场景：`swap_remove()` 在移除元素时会交换元素的位置，而 `remove()` 会按顺序移除元素。选择哪个方法取决于你是否需要保留元素的顺序。



# 5、插入

在 Rust 中，`Vec<T>` 提供了两个与元素插入相关的方法：`insert()` 和 `insert_strided()`。

### `insert(index: usize, element: T)`

- **功能**：在指定索引处插入一个新元素，所有后续元素向后移动。
- **使用场景**：当你需要在向量的特定位置插入一个元素时使用，例如在处理用户输入或其他需要精确控制元素位置的场景中。

### `insert_strided(index: usize, value: T, stride: usize)`

- **功能**：在指定索引处插入一个新元素，并将所有后续元素向后移动指定的步长（stride）。
- **使用场景**：当你需要在向量中插入一个元素，并且同时需要移动其他元素以保持某种间隔或排列方式时使用，例如在处理数据集或其他需要复杂插入模式的场景中。

示例

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 insert()
    vec.insert(2, 10);
    println!("Modified vector: {:?}", vec);

    // 使用 insert_strided()
    vec.insert_strided(1, 20, 2);
    println!("Modified vector: {:?}", vec);
}
Modified vector: [1, 2, 10, 3, 4, 5]

```

在这个示例中，我们首先创建了一个包含五个元素的 `Vec<i32>`。然后，我们使用 `insert()` 方法在索引为 2 的位置插入一个新元素 10。接着，我们使用 `insert_strided()` 方法在索引为 1 的位置插入一个新元素 20，并将所有后续元素向后移动 2 个步长。最后，我们打印出修改后的向量。

这两个方法的主要区别在于它们的行为和使用场景：`insert()` 简单地在指定位置插入一个新元素，而 `insert_strided()` 允许你同时移动其他元素以保持某种间隔或排列方式。选择哪个方法取决于你需要的插入模式和元素排列方式。

在 Rust 中，`Vec<T>` 提供了两个与向量元素添加相关的方法：`push` 和 `push_within_capacity`。

### `push(element: T)`

- **功能**：将一个元素添加到向量的末尾。如果向量的容量不足以容纳新元素，会自动扩展向量的容量。
- **使用场景**：在大多数情况下，`push` 是添加元素到向量的首选方法。它简单易用，并且在需要时自动调整容量。

### `push_within_capacity不稳定` 

- **功能**：尝试将一个元素添加到向量的末尾，只有在向量有足够的剩余容量时才会成功。返回一个布尔值，指示是否成功添加了元素。
- **使用场景**：当你需要精细控制向量的容量和内存分配时使用。例如，在高性能或内存受限的环境中，你可能希望避免不必要的内存重新分配。

示例

```rust
fn main() {
    let mut vec = Vec::with_capacity(3);

    // 使用 push()
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("After push(): {:?}", vec);

    // 使用 push_within_capacity()
    let success = vec.push_within_capacity(4);
    println!("push_within_capacity() success: {}, vec: {:?}", success, vec);

    // 尝试再次使用 push_within_capacity()
    let success = vec.push_within_capacity(5);
    println!("push_within_capacity() success: {}, vec: {:?}", success, vec);
}
```

在这个示例中，我们首先创建了一个初始容量为 3 的向量。然后，我们使用 `push()` 方法添加三个元素到向量中。接下来，我们尝试使用 `push_within_capacity()` 方法添加第四个元素，因为向量已经没有足够的剩余容量，所以这个操作失败。最后，我们再次尝试使用 `push_within_capacity()` 方法添加第五个元素，同样失败。

这两个方法的主要区别在于它们对向量容量的处理方式：`push()` 会在需要时自动扩展容量，而 `push_within_capacity()` 只在向量有足够的剩余容量时才会成功添加元素。选择哪个方法取决于你对内存管理和性能的具体需求。



在 Rust 中，`Vec<T>` 提供了两种将元素添加到向量末尾的方法：`append` 和 `append_elements`。

### `append(&mut self, other: &mut Vec<T>)`

- **功能**：将 `other` 向量中的所有元素添加到当前向量的末尾。注意，这个操作会消耗 `other` 向量，并将其留为空。
- **使用场景**：当你需要合并两个向量，并且不再需要源向量时使用。这个方法可以避免创建中间向量，提高性能。

### `extend_elements<I>(&mut self, elements: I) where I: IntoIterator<Item = T>`

- **功能**：将 `elements` 迭代器中的所有元素添加到当前向量的末尾。这个方法接受任何实现了 `IntoIterator` 特性的类型，例如数组、向量、字符串等。
- **使用场景**：当你需要将多个元素或其他可迭代对象的内容添加到向量中时使用。这个方法提供了更多的灵活性，因为你可以传入不同类型的可迭代对象。

示例

```rust
fn main() {
    let mut vec1 = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];

    // 使用 append()
    vec1.append(&mut vec2);
    println!("After append(): vec1: {:?}, vec2: {:?}", vec1, vec2);

    let arr = [7, 8, 9];

    // 使用 extend_elements()
    vec1.extend_elements(arr.iter());
    println!("After extend_elements(): vec1: {:?}", vec1);
}
After append(): vec1: [1, 2, 3, 4, 5, 6], vec2: []

```

在这个示例中，我们首先创建了两个向量 `vec1` 和 `vec2`。然后，我们使用 `append()` 方法将 `vec2` 的元素添加到 `vec1` 的末尾，并注意到 `vec2` 现在是空的。接下来，我们创建了一个数组 `arr`，并使用 `extend_elements()` 方法将其内容添加到 `vec1` 的末尾。

这两个方法的主要区别在于它们如何处理要添加的元素：`append()` 方法消耗源向量，而 `extend_elements()` 方法接受可迭代对象并将其内容添加到目标向量中。选择哪个方法取决于你对源数据的需求和对性能的考虑。

# 6、过滤

在 Rust 中，`Vec<T>` 提供了两个与元素过滤相关的方法：`retain()` 和 `retain_mut()`。

### `retain<F>(&mut self, f: F) where F: FnMut(&T) -> bool`

- **功能**：移除所有不满足给定谓词 `f` 的元素。
- **使用场景**：当你需要根据某个条件过滤向量中的元素时使用，例如在数据清洗或处理用户输入时。

### `retain_mut<F>(&mut self, f: F) where F: FnMut(&mut T) -> bool`

- **功能**：移除所有不满足给定谓词 `f` 的元素，并且允许修改被检查的元素。
- **使用场景**：当你需要根据某个条件过滤向量中的元素，并且同时需要修改这些元素时使用，例如在更新或转换数据时。

示例

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 retain()
    vec.retain(|&x| x % 2 == 0);
    println!("After retain(): {:?}", vec);

    // 使用 retain_mut()
    vec.retain_mut(|x| {
        *x *= 2;
        *x!= 8
    });
    println!("After retain_mut(): {:?}", vec);
}

After retain(): [2, 4]
After retain_mut(): [4]

```

在这个示例中，我们首先创建了一个包含五个元素的 `Vec<i32>`。然后，我们使用 `retain()` 方法移除所有不是偶数的元素。接着，我们使用 `retain_mut()` 方法将所有元素乘以 2，并移除值为 8 的元素。最后，我们打印出修改后的向量。

这两个方法的主要区别在于它们的行为和使用场景：`retain()` 只移除不满足条件的元素，而 `retain_mut()` 允许你在过滤元素的同时修改它们。选择哪个方法取决于你是否需要对被检查的元素进行修改。



# 7、相同元素或者指定key删除元素

在 Rust 中，`Vec<T>` 提供了两个与元素去重相关的方法：`dedup()` 和 `dedup_by_key()`。

### `dedup()`

- **功能**：移除向量中相邻的重复元素。
- **使用场景**：当你需要从向量中移除相邻的重复元素时使用，例如在处理用户输入或其他需要简化数据的场景中。

### `dedup_by_key<F>(&mut self, key: F) where F: FnMut(&T) -> K, K: PartialEq<K>`

- **功能**：移除向量中具有相同键的相邻元素。
- **使用场景**：当你需要根据某个键函数来去重向量中的元素时使用，例如在处理结构化数据或其他需要按特定字段去重的场景中。

示例

```rust
fn main() {
     let mut vec1 = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let mut vec2 = vec![Person { name: "Alice".to_string(), age: 30 }, Person { name: "Bob".to_string(), age: 25 }, Person { name: "Alice".to_string(), age: 31 }];

    // 使用 dedup()
    vec1.dedup();
    println!("After dedup(): {:?}", vec1);

    // 使用 dedup_by_key()
    vec2.dedup_by_key(|p| p.name.clone());
    println!("After dedup_by_key(): {:?}", vec2);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

After dedup(): [1, 2, 3, 4]
After dedup_by_key(): [Person { name: "Alice", age: 30 }, Person { name: "Bob", age: 25 }, Person { name: "Alice", age: 31 }]

保留的是最后一次
```

在这个示例中，我们首先创建了两个向量：`vec1` 包含重复的整数，`vec2` 包含重复的人名。然后，我们使用 `dedup()` 方法移除 `vec1` 中相邻的重复整数。接着，我们使用 `dedup_by_key()` 方法移除 `vec2` 中具有相同名字的人。最后，我们打印出修改后的向量。

这两个方法的主要区别在于它们的行为和使用场景：`dedup()` 只考虑相邻元素的值是否相等，而 `dedup_by_key()` 允许你根据自定义的键函数来去重元素。选择哪个方法取决于你需要去重的元素的特点和逻辑。



在 Rust 中，`dedup_by` 是一个方法，它允许你根据一个自定义的比较函数来移除向量中相邻的重复元素。这个方法是 `dedup` 的一个变体，提供了更多的灵活性。

### `dedup_by<F>(&mut self, same_bucket: F) where F: FnMut(&T, &T) -> bool`

两两比较 不相邻的 不行

- **功能**：移除向量中相邻的元素，这些元素在使用 `same_bucket` 函数进行比较时被认为是相等的。
- **使用场景**：当你需要根据某个自定义的比较逻辑来去重向量中的元素时使用，例如在处理结构化数据或其他需要按特定字段去重的场景中。

### 示例

```rust
fn main() {
    let mut vec = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];

    // 使用 dedup_by()
    vec.dedup_by(|a, b| a.abs() == b.abs());
    println!("After dedup_by(): {:?}", vec);
}

After dedup_by(): [1, 2, 3, 4]


let mut vec = vec![1, 2, 2, 3, 3, 3,2, 4, 4,8,34, 4];

    // 使用 dedup_by()
    vec.dedup_by(|a, b| a.abs() == b.abs());
    println!("After dedup_by(): {:?}", vec);
After dedup_by(): [1, 2, 3, 2, 4, 8, 34, 4]
```

在这个示例中，我们首先创建了一个向量 `vec`，它包含了重复的整数。然后，我们使用 `dedup_by()` 方法移除相邻的元素，这些元素在使用 `same_bucket` 函数进行比较时被认为是相等的。在这个例子中，我们的比较函数是 `|a, b| a.abs() == b.abs()`，这意味着我们将移除所有绝对值相等的相邻元素。最后，我们打印出修改后的向量。

这个方法的主要优点是它提供了比 `dedup()` 更多的灵活性，因为你可以根据自己的需要定义比较逻辑。例如，在处理字符串时，你可能希望忽略大小写差异；在处理复杂数据结构时，你可能希望根据某个特定的字段进行比较。



# 8、末尾移除元素

在 Rust 中，`Vec<T>` 提供了两种从向量末尾移除元素的方法：`pop` 和 `pop_if`。

### `pop() `

- **功能**：从向量的末尾移除并返回一个元素。如果向量为空，则返回 `None`。
- **使用场景**：在大多数情况下，`pop` 是移除向量末尾元素的首选方法。它简单易用，并且在需要时自动调整向量的长度。

### `pop_if` 不稳定

- **功能**：从向量的末尾移除并返回一个元素，只有当该元素满足由 `predicate` 函数指定的条件时才会被移除。如果没有满足条件的元素或向量为空，则返回 `None`。
- **使用场景**：当你需要根据某种条件来选择性地移除向量末尾的元素时使用。例如，在处理过滤器或其他需要按特定规则操作的数据结构时。

### 示例

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 pop()
    let popped = vec.pop();
    println!("After pop(): {:?}, vec: {:?}", popped, vec);

    // 使用 pop_if()
    let popped = vec.pop_if(|&x| x % 2 == 0);
    println!("After pop_if(): {:?}, vec: {:?}", popped, vec);

    // 再次尝试使用 pop_if()
    let popped = vec.pop_if(|&x| x > 5);
    println!("After pop_if(): {:?}, vec: {:?}", popped, vec);
}
//After pop(): Some(5), vec: [1, 2, 3, 4]
```

在这个示例中，我们首先创建了一个包含五个元素的向量。然后，我们使用 `pop()` 方法移除并返回向量的最后一个元素。接下来，我们使用 `pop_if()` 方法移除并返回一个满足条件（即是偶数）的元素。最后，我们再次尝试使用 `pop_if()` 方法移除一个大于 5 的元素，因为向量中没有这样的元素，所以这个操作失败。

这两个方法的主要区别在于它们移除元素的方式：`pop()` 总是移除向量末尾的元素，而 `pop_if()` 只在满足特定条件时才会移除元素。选择哪个方法取决于你对元素移除的具体需求。



# 8、清空或部分清空向量

在 Rust 中，`Vec<T>` 提供了三种方法来清空或部分清空向量：`drain`、`clear` 和 `retain`。

### `drain`

- **功能**：从向量中移除并返回一个包含指定范围内元素的新向量。移除的元素将从原始向量中删除。
- **使用场景**：当你需要移除向量中的一部分元素，并且需要保留那些被移除的元素时使用。例如，在处理队列或缓存时。

### `clear()`

- **功能**：清空向量中的所有元素。
- **使用场景**：当你需要完全清空向量时使用。例如，在循环中重复使用向量时。

### `retain`

- **功能**：移除向量中所有不满足给定谓词函数 `f` 的元素。返回值为移除的元素数量。
- **使用场景**：当你需要根据某种条件过滤向量中的元素时使用。例如，在处理过滤器或其他需要按特定规则操作的数据结构时。

主要区别

1. **目的**：`drain` 用于移除并保留一部分元素，`clear` 用于清空所有元素，`retain` 用于根据条件过滤元素。
2. **返回值**：`drain` 返回被移除的元素，`clear` 不返回任何值，`retain` 返回被移除的元素数量。
3. **操作范围**：`drain` 可以指定移除的元素范围，`clear` 清空整个向量，`retain` 根据条件过滤整个向量。

示例

```rust
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    // 使用 drain()
    let drained = vec.drain(1..3);
    println!("After drain(): vec: {:?}, drained: {:?}", vec, drained);

    // 使用 clear()
    vec.clear();
    println!("After clear(): vec: {:?}", vec);

    // 使用 retain()
    let mut vec = vec![1, 2, 3, 4, 5];
    let removed_count = vec.retain(|&x| x % 2 == 0);
    println!("After retain(): vec: {:?}, removed_count: {}", vec, removed_count);
}
```

```
pub struct User{
    pub username: String,
    pub age: u8,
}
fn  main(){
let mut vec = vec![User{username: String::from("John"), age: 18}, User{username: String::from("John1"), age: 19},];
   for val in vec.drain(..) {
    tidy(val)
   }
    println!("{:#?}", vec);
}

fn tidy(usee :User){
    println!("TIDY {:?}",usee);
}

TIDY User { username: "John", age: 18 }
TIDY User { username: "John1", age: 19 }
[]

```



==可以获取vec的所有权==

在这个示例中，我们首先使用 `drain()` 方法移除并保留了向量中索引 1 到 2 的元素。然后，我们使用 `clear()` 方法清空了整个向量。最后，我们使用 `retain()` 方法根据条件过滤了向量中的元素，并记录了被移除的元素数量。

# 9、分割调整长度

在 Rust 中，`Vec<T>` 提供了多种方法来操作和管理向量的大小和内容。以下是四种方法的详细解释：

### `split_off` 

- **功能**：将向量分裂成两个部分，从指定的索引 `at` 开始，返回一个新的向量包含原向量中从 `at` 到结尾的所有元素。原向量将只保留从开始到 `at - 1` 的元素。
- **使用场景**：当你需要将向量分成两部分并处理其中一部分时使用。例如，在实现某种算法或数据结构时。

### `resize_with`

- **功能**：将向量的长度调整为 `len`，并使用给定的函数 `f` 来填充新添加的元素。如果向量的长度小于 `len`，则会添加元素；如果向量的长度大于 `len`，则会截断向量。
- **使用场景**：当你需要调整向量的大小并以某种方式初始化新添加的元素时使用。例如，在创建一个预分配大小的向量时。

### `leak(self) `

- **功能**：将向量的所有权转移给调用者，返回向量的第一个元素。注意，这个方法会使得向量失效，不能再使用。
- **使用场景**：当你需要将向量的所有权转移给其他部分的代码，并且不再需要这个向量时使用。例如，在将向量转换为其他数据结构时。

### `resize`

- **功能**：将向量的长度调整为 `len`，并使用给定的值 `value` 来填充新添加的元素。如果向量的长度小于 `len`，则会添加元素；如果向量的长度大于 `len`，则会截断向量。
- **使用场景**：当你需要调整向量的大小并用特定值初始化新添加的元素时使用。例如，在创建一个固定大小的缓冲区时。

示例

```rust
fn main() {
    let mut vec = vec![1, 2, 3];

    // 使用 split_off()
    let split_vec = vec.split_off(1);
    println!("After split_off(): vec: {:?}, split_vec: {:?}", vec, split_vec);

    // 使用 resize_with()
    vec.resize_with(5, || 0);
    println!("After resize_with(): vec: {:?}", vec);

    // 使用 leak()
    let leaked_element = vec.leak();
    println!("After leak(): leaked_element: {:?}, vec: {:?}", leaked_element, vec); // vec is now invalid

    // 使用 resize()
    let mut vec = vec![1, 2, 3];
    vec.resize(5, 0);
    println!("After resize(): vec: {:?}", vec);
}

After split_off(): vec: [1], split_vec: [2, 3]
After resize_with(): vec: [1, 0, 0, 0, 0]
After leak(): leaked_element: [1, 0, 0, 0, 0], vec: [1, 0, 0, 0, 0]
After resize(): vec: [1, 2, 3, 0, 0]

```

在这个示例中，我们首先使用 `split_off()` 方法将向量分成两部分。然后，我们使用 `resize_with()` 方法调整向量的大小并用零填充新添加的元素。接下来，我们使用 `leak()` 方法将向量的所有权转移给调用者，并返回第一个元素。最后，我们使用 `resize()` 方法调整向量的大小并用零填充新添加的元素。







































