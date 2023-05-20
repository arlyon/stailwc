use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"import tw, { theme } from '../macro'"#####, r#####";"##### ; "0")]
#[test_case(r#####"theme`animation`"#####, r#####"({
  none: "none",
  spin: "spin 1s linear infinite",
  ping: "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite",
  pulse: "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite",
  bounce: "bounce 1s infinite",
})
;"##### ; "1")]
#[test_case(r#####"tw`animate-none`"#####, r#####"({
  animation: "none",
})
;"##### ; "2")]
#[test_case(r#####"tw`animate-spin`"#####, r#####"({
  '@keyframes spin': {
    to: {
      transform: "rotate(360deg)",
    },
  },
  animation: "spin 1s linear infinite",
})
;"##### ; "3")]
#[test_case(r#####"tw`animate-ping`"#####, r#####"({
  '@keyframes ping': {
    '75%, 100%': {
      transform: "scale(2)",
      opacity: "0",
    },
  },
  animation: "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite",
})
;"##### ; "4")]
#[test_case(r#####"tw`animate-pulse`"#####, r#####"({
  '@keyframes pulse': {
    '50%': {
      opacity: ".5",
    },
  },
  animation: "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite",
})
;"##### ; "5")]
#[test_case(r#####"tw`animate-bounce`"#####, r#####"({
  '@keyframes bounce': {
    '0%, 100%': {
      transform: "translateY(-25%)",
      animationTimingFunction: "cubic-bezier(0.8,0,1,1)",
    },
    '50%': {
      transform: "none",
      animationTimingFunction: "cubic-bezier(0,0,0.2,1)",
    },
  },
  animation: "bounce 1s infinite",
})
;"##### ; "6")]
#[test_case(r#####"tw`animate-[wiggle 1s ease-in-out infinite]`"#####, r#####"({
  animation: "wiggle 1s ease-in-out infinite",
})"##### ; "7")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
