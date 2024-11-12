#[must_use]
pub fn solution(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();

    seats
        .iter()
        .zip(students.iter())
        .map(|(seat, student)| (seat - student).abs())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode2037_case1() {
        let seats: Vec<i32> = vec![3, 1, 5];
        let students: Vec<i32> = vec![2, 7, 4];
        let desired: i32 = 4;

        assert_eq!(solution(seats, students), desired);
    }
}
