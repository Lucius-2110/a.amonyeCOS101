fn main() {
    let A: i32 = 10;
    let B: i32 = 20;

    let mut res = A > B;  // false
    res = A < B;           // true
    res = A >= B;          // false
    res = A <= B;          // true
    res = A == B;          // false (checks if equal)
    res = A != B;          // true  (checks if NOT equal)
}