# My Lang Exercises

This mini problem set is designed for someone learning `my_lang`. Each exercise focuses on a different feature of the language and includes a short solution program.

These exercises are intentionally different from the example programs in `examples/`.

## Exercise 1 - Minimum of a pair of integers

### Problem

Write a function `min_pair : (Int, Int) -> Int` that returns the smaller of the two integers in a pair.

### Handout

- Use `fst` and `snd`.
- Use an `if then else` expression.
- Compare integers with `<`.

### Solution

```text
let min_pair : ((Int, Int) -> Int) = fun p : (Int, Int) -> if fst p < snd p then fst p else snd p
min_pair (8, 3)
```

## Exercise 2 - Safe division using sums

### Problem

Write a function `safe_div : ((Int, Int) -> (Int + Int))` that takes a pair `(a, b)` and returns `inr 0 (Int)` if the divisor is zero, and `inl (a / b) (Int)` otherwise.

### Handout

- Use a sum type `(Int + Int)` as the result.
- Use `fst`, `snd`, `if then else`, and `==`.
- Put the quotient in the left injection and the fallback value `0` in the right injection.

### Solution

```text
let safe_div : ((Int, Int) -> (Int + Int)) = fun p : (Int, Int) -> if snd p == 0 then inr 0 (Int) else inl (fst p / snd p) (Int)
safe_div (10, 2)
safe_div (10, 0)
```

## Exercise 3 - Recursive factorial

### Problem

Write a factorial function `fact` that computes $n!$ for a non-negative integer `n`.

### Handout

- Use `fix`.

### Solution

```text
let fact : (Int -> Int) = fix (fun self : (Int -> Int) -> fun n : Int -> if n == 0 then 1 else n * (self (n - 1)))
fact 5
```

## Exercise 4 - Maximum of a non-empty list

### Problem

Write a function max_list : [Int] -> Int that returns the maximum element of a list of integers. For the empty list, return 0.

### Handout

- Use caselist to obtain the first element and the tail.
- Use reclist to scan the tail.
- Use integer comparisons with >.

### Solution

```text
let max_list : ([Int] -> Int) = fun xs : [Int] -> caselist xs 0 (fun h : Int -> fun t : [Int] -> reclist t h (fun x : Int -> fun xs : [Int] -> fun acc : Int -> if x > acc then x else acc))
max_list (cons 3 (cons 9 (cons 4 (cons 7 (nil [Int])))))
```

## Exercise 5 - Sorted list check

### Problem

Write a function `is_sorted : [Int] -> Bool` that returns `true` if a list of integers is sorted in non-decreasing order.

### Handout

- Use `caselist` to handle lists of length 0 or 1.
- Compare adjacent elements with `<`.
- Recursion is required.

### Solution

```text
let is_sorted : ([Int] -> Bool) = fix (fun self : ([Int] -> Bool) -> fun xs : [Int] -> caselist xs true (fun h : Int -> fun t : [Int] -> caselist t true (fun h2 : Int -> fun t2 : [Int] -> if h < h2 then self (cons h2 t2) else false)))
is_sorted (cons 1 (cons 2 (cons 3 (cons 5 (nil [Int])))))
```

## Notes

- These exercises are meant as learning material, not as test cases.
- They are a bit harder than the examples in `examples/` and focus on more advanced combinations of features.
