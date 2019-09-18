pub fn always_true() -> bool {
    true
}

pub fn always_false() -> bool {
    false
}

pub fn add_five(x:i32) -> i32 {
    add_six(x)-1
}

fn add_six(x:i32) -> i32 {
    x+6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_true() {
        assert_eq!(crate::some_features::always_true(), true);
    }

    #[test]
    fn always_false() {
        assert_eq!(crate::some_features::always_false(), false);
    }

    #[test]
    fn add_five() {
        assert_eq!(crate::some_features::add_five(12), 17);
    }

    #[test]
    fn internal_add_six() {
        assert_eq!(add_six(6),12)
    }
}
