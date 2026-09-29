fn main() {
    let a: i32 = 2; 
    let b: i32 = 3; 
    let mut result: i32;

    result = a & b;  // AND: 10 & 11 = 10 (2)
    result = a | b;  // OR:  10 | 11 = 11 (3)
    result = a ^ b;  // XOR: 10 ^ 11 = 01 (1) (1 where bits differ)
    result = !b;     // NOT: Flips all bits (-4 in two's complement)
    result = a << b; // Left Shift: Shifts bit positions left (multiplies by 2^b)
    result = a >> b; // Right Shift: Shifts bit positions right (divides by 2^b)
}