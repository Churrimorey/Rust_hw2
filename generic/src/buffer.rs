pub struct Buffer<T>
where
    T: std::ops::Add + std::ops::AddAssign + Copy,
{
    member: Vec<T>,
}

impl<T> Buffer<T>
where
    T: std::ops::Add + std::ops::AddAssign + Copy,
{
    pub fn new(v: Vec<T>) -> Self {
        Self { member: v }
    }

    pub fn sum(&self) -> Option<T> {
        if self.member.is_empty() {
            return None;
        }
        let mut sum = self.member[0];
        for i in 1..self.member.len() {
            sum += self.member[i];
        }
        Some(sum)
    }
}
