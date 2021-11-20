use crate::*;
use crate::decorators;

#[test]
fn test_bold() {
  assert_eq!(
    decorators::bold("hello"),
    String::from("\x1B[1mhello\x1B[0m")
  )
}

#[test]
fn test_underline() {
  assert_eq!(
    decorators::underline("hello"),
    String::from("\x1B[4mhello\x1B[0m")
  )
}

#[test]
fn test_dimmed() {
    println!("hello");
  assert_eq!(
    decorators::dimmed("hello"),
    String::from("\x1B[2mhello\x1B[0m")
  )
}

#[test]
fn test_italic() {
  assert_eq!(
    decorators::italic("hello"),
    String::from("\x1B[3mhello\x1B[0m")
  )
}

#[test]
fn test_blink() {
  assert_eq!(
    decorators::blink("hello"),
    String::from("\x1B[5mhello\x1B[0m")
  )
}

#[test]
fn test_reverse() {
  assert_eq!(
    decorators::reverse("hello"),
    String::from("\x1B[7mhello\x1B[0m")
  )
}

#[test]
fn test_hidden() {
  assert_eq!(
    decorators::hidden("hello"),
    String::from("\x1B[8mhello\x1B[0m")
  )
}

#[test]
fn test_stricken() {
  assert_eq!(
    decorators::stricken("hello"),
    String::from("\x1B[9mhello\x1B[0m")
  )
}