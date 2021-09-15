#[link(name = "badmath", kind = "static")]
extern "C" {
    fn bad_add(v1: f32, v2: f32) -> f32;
}

fn main() {
    println!("Hello, from RUST!");
    let res = unsafe { bad_add(12., 7.) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_extern_bad_add() {
        assert_eq!(unsafe { super::bad_add(12., 8.) }, 12. + 8.);
    }
}
