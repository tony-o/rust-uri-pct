pub fn encode(s: &str) -> String {
  let mut e = String::with_capacity(s.len() * 3);
  for c in s.chars() {
    match c as u8 {
      45|46|95|126|65..=90|97..=122 => {
        e.push(c as char);
      }
      n => {
        e.push_str(&format!("%{:02X}", n));
      }
    }
  }
  e
}

pub fn decode(s: &str) -> String {
  s.chars().fold((String::with_capacity(s.len()), String::with_capacity(3)), |(mut a, mut b),c| {
    if c == '%' || b.len() > 0 {
      if c == '%' { b.clear(); }
      b.push(c);
      if b.len() == 3 {
        match u8::from_str_radix(b.get(1..=2).unwrap_or(""), 16) {
          Ok(n) => {
            a.push(n as char);
          }
          Err(_) => {}
        }
        b.clear();
      }
    } else {
      a.push(c);
    }
    (a,b)
  }).0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hello_world(){
    assert_eq!("hello%20world%21", encode("hello world!"));
    assert_eq!("hello world!", decode("hello%20world%21"));
  }

  #[test]
  fn long_str() {
    assert_eq!("you%20are%20now%20immortal", encode("you are now immortal"));
    assert_eq!("you are now immortal", decode("you%20are%20now%20immortal"));
    assert_eq!("you are now immortal", decode("%79%6F%75%20%61%72%65%20%6E%6F%77%20%69%6D%6D%6F%72%74%61%6C"));
  }

  #[test]
  fn bad_str() {
    assert_eq!("", decode("%2%GG"));
    assert_eq!("", decode("%2Z%GG"));
    assert_eq!("", decode("%2%%"));
  }

/*  #[bench]
  fn hello_world_encode(b: &mut Bencher) {
    b.iter(|| {
      encode("hello world!");
    });
  }

  #[bench]
  fn hello_world_decode(b: &mut Bencher) {
    b.iter(|| {
      encode("hello%20world%21");
    });
  }*/
}
