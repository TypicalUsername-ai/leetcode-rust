pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut n = n;
    let mut state = 1;
    for place in flowerbed {
        if n == 0 {
            return true;
        }
        if place == 0 {
            state += 1;
        } else {
            state = 0;
        }
        if state == 3 {
            state = 1;
            n -= 1;
        }
    }
    if state == 2 {
        n -= 1
    }
    n == 0
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[should_panic]
    fn no_place() {
        let flowerbed = vec![1, 0, 1];
        assert!(can_place_flowers(flowerbed, 1))
    }

    #[test]
    fn place_start() {
        let flowerbed = vec![0, 0, 1];
        assert!(can_place_flowers(flowerbed, 1))
    }

    #[test]
    fn place_end() {
        let flowerbed = vec![1, 0, 0];
        assert!(can_place_flowers(flowerbed, 1))
    }

    #[test]
    #[should_panic]
    fn not_enough() {
        let flowerbed = vec![1, 0, 0];
        assert!(can_place_flowers(flowerbed, 2))
    }
}
