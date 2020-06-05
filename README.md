# Sha-1

## 1. Base Information

* `String` consist of `Char`
* peer `Char` has 8 bit

So in this program, the `Char` is the smallest unit to calculate. Otherwise, the bit of `u8` in *Rust* just is `8`, so we use `u8` to calculate.

## 2. Precondition

* The length of input is less than `2**64`.

## 3. Algorithm 

There are 3 main steps in *Sha-1* algorithm.

### 3.1. Format `String` to a standard `Blocks`

* What is `Block`

A group of bit which's length is stable 512.

* What is `Blocks`

A group of `Block`. The count of `Block` is up to the length of input.

This step has 4 small steps.

1. `String` -> `[u8]`
1. `[u8]` -> `Vec<Vec<u8>>`
1. `Vec<Vec<u8>>` -> `Vec<U8Block>`(format `Vec<u8>` to `U8Block`, `U8Block` at here is `Vec<u8>`, each `Block` has 64 `u8`)
1. `Vec<U8Block>` -> `Vec<U32Block>`(format `U8Block` to `U32Block`, `U32Block` at here is `Vec<u32>`, each `Block` has 16 `u32`)

The more important steps is 2 and 3.

The small step 2 and 3 are the processes to transform one-dimensional array to two-dimensional array by 512. And it required that the length of latest `Block` must less than 447(512 - 1(1 bit `1`) - 64(length part)). Because it must add 1 bit `1` and 64 bit the part of the length of input(bit length, not the chat length). If the length of latest `Block` is more than 447, we should append the 1 bit `1` at the end of this `Block`, fill the rest of this `Block`(512 - (the current length + 1)) with `0` and create a new `Block` after this `Block` to store the length part.

* transform 4 `u8` to 1 `u32`

```rust
let u8_block = vec![1, 1, 1, 1];
let u32_letter = u32::from(u8_block[0]) << 24
    | u32::from(u8_block[1]) << 16
    | u32::from(u8_block[2]) << 8
    | u32::from(u8_block[3]);
```

### 3.2. Transform `State` by `Blocks`

* What is `State`

The `State` is a tuple consists of 5 `u32`.

In the process, It has two important constants.

* Initial `State`: `(0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0)`.

* 4 constants: `0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xCA62C1D6`

We will transform the initial `State` by `Blocks` to result `State`. 

#### 3.2.1. Expand standard `Block`

Before the transition, we need to expand each `Block` to length(unit is `u32`) 80. We mention that the length of standard `Block` is 16(unit is `u32`, 512 / 32 = 16). We need apply a function to calculate the element which's index is between 17 and 80. The function is: 

```
W t = M t                                             ( 0 ≤ t ≤ 15)

W t = ( W t-3 AND W t-8 AND W t-14 AND W t-16 ) <<< 1 (16 ≤ t ≤ 79)
```

The `W` is the `Block` after expanding. The `M` is the standard `Block`. The `t` is the index. `AND` is bitwise and. `<<<` is circular left shift.

* `AND`

```rust
let foo = 10 ^ 20;
```

* `<<<`

```rust
// `input` is the num you want to shift, the `n` is the length of `input`.
fn circular_left_shift(input: u32, n: u32) -> u32 {
    (input << n) | (input >> (32 - n))
}
```

#### 3.2.2 Transform `State` by one `Block`

The transform start with a initial `State` and a `Block`. It has 80 steps, and these steps has been divided input four part. All steps are similar, and there are some difference between the four part steps.

##### Preprocess

```rust
let (mut A,mut  B,mut  C,mut D, mut E) = initial_state;
```

All steps progress the same way.

* `temp = (A<<<5) + ft(B,C,D) + E + Wt + Kt`
* `E = D`
* `D = C`
* `C = B <<< 30`
* `B = A`
* `A = temp`

`t` is the step which between 0 and 79. `W` is the `Block` after expanding.

`ft(B, C, D)` is the function under here.

```
ft(B,C,D) = (B AND C) OR ((NOT B) AND D)        ( 0 <= t <= 19)

ft(B,C,D) = B XOR C XOR D                       (20 <= t <= 39)

ft(B,C,D) = (B AND C) OR (B AND D) OR (C AND D) (40 <= t <= 59)

ft(B,C,D) = B XOR C XOR D                       (60 <= t <= 79).
```

`Kt` is the constant under here.

```
Kt = 0x5A827999 ( 0 <= t <= 19)

Kt = 0x6ED9EBA1 (20 <= t <= 39)

Kt = 0x8F1BBCDC (40 <= t <= 59)

Kt = 0xCA62C1D6 (60 <= t <= 79)
```

the result `State` of step 80 is the result `State` of this transform.

#### 3.2.3 Transform `State` between `Blocks`

Apply the Initial `State` mention before as the `initial_state` of the first transform of the first `Block`. Add the `initial_state` and the result `State` as the `initial_state` of the next transform of the next `Block`.

The result `State` of latest transform of the latest `Block` is the final `State`.

### 3.3 Transform `State` to hash code

Format the five `u32` to `String` by hex and concat them together. The result `String` is the hash.
 