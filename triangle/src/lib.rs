use std::ops::Add;
pub struct Triangle {
    count: u8,
}
impl Triangle {
    pub fn build<T>(sides: [T; 3]) -> Option<Triangle>
    where
        T: Copy + Default + PartialEq + PartialOrd + Add<Output = T>,
    {
        let mut count = 0;
        for s in [0, 1, 2, 0, 1].windows(3) {
            if sides[s[0]] == T::default() || sides[s[0]] + sides[s[1]] < sides[s[2]] {
                return None;
            }
            if sides[s[0]] == sides[s[1]] {
                count += 1
            }
        }
        Some(Triangle { count })
    }
    pub fn is_equilateral(&self) -> bool { self.count == 3 }
    pub fn is_scalene(&self) -> bool { self.count == 0 }
    pub fn is_isosceles(&self) -> bool { self.count >= 1}
}
// use std::ops::Add;

// pub struct Triangle {
//     count: u8,
// }

// impl Triangle {
//     pub fn build<T>(sides: [T; 3]) -> Option<Triangle>
//     where
//         T: Copy + Default + PartialEq + PartialOrd + Add<Output = T>,
//     {
//         let mut count = 0;
        
//         for i in 0..3 {
//             if sides[i] == T::default() {
//                 return None; // 如果边长为默认值（如0），则不是有效的三角形。
//             }
//             for j in 0..3 {
//                 for k in 0..3 {
//                     if i != j && j != k && i != k {
//                         if sides[i] + sides[j] <= sides[k] {
//                             return None; // 任意两边之和必须大于第三边。
//                         }
//                         if sides[i] == sides[j] {
//                             count += 1;
//                         }
//                     }
//                 }
//             }
//         }

//         if count > 3 {
//             count = 3; // 由于我们在循环中多次检查了每对边，所以count可能会超过3。我们将其修正为3。
//         }

//         Some(Triangle { count })
//     }
//     pub fn is_equilateral(&self) -> bool {
//         self.count == 3
//     }
//     pub fn is_scalene(&self) -> bool {
//         self.count == 0
//     }
//     pub fn is_isosceles(&self) -> bool {
//         self.count >= 1
//     }
// }
