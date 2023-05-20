use crate::test::snapshot_inner;
use test_case::test_case;
#[test_case(r#####"tw`[section]:hover:block`"#####, r#####"({
  'section:hover': {
    display: "block",
  },
})
;"##### ; "0")]
#[test_case(r#####"tw`[section&]:hover:block`"#####, r#####"({
  'section:hover': {
    display: "block",
  },
})
;"##### ; "1")]
#[test_case(r#####"tw`[p]:hover:block`"#####, r#####"({
  'p:hover': {
    display: "block",
  },
})
;"##### ; "2")]
#[test_case(r#####"tw`hover:[p]:block`"#####, r#####"({
  ':hover p': {
    display: "block",
  },
})
;"##### ; "3")]
#[test_case(r#####"tw`[* + *]:block`"#####, r#####"({
  '& * + *': {
    display: "block",
  },
}) // Spaces

;"##### ; "4")]
#[test_case(r#####"tw`[.class1 .class2]:block`"#####, r#####"({
  '& .class1 .class2': {
    display: "block",
  },
}) // Classes

;"##### ; "5")]
#[test_case(r#####"tw`[.class1]:[.class2]:block`"#####, r#####"({
  '& .class1 .class2': {
    display: "block",
  },
}) // Multiple dynamic variants

;"##### ; "6")]
#[test_case(r#####"tw`[.class1 .class2]:[.class3]:block`"#####, r#####"({
  '& .class1 .class2 .class3': {
    display: "block",
  },
}) // Multiple dynamic variants

;"##### ; "7")]
#[test_case(r#####"tw`[p]:placeholder-red-500/[var(--myvar)]`"#####, r#####"({
  '& p::placeholder': {
    color: "rgb(239 68 68 / var(--myvar))",
  },
})
;"##### ; "8")]
#[test_case(r#####"tw`[p]:mt-[var(--myvar)]`"#####, r#####"({
  '& p': {
    marginTop: "var(--myvar)",
  },
})
;"##### ; "9")]
#[test_case(r#####"tw`[p]:marginTop[var(--myvar)]`"#####, r#####"({
  '& p': {
    marginTop: "var(--myvar)",
  },
})
;"##### ; "10")]
#[test_case(r#####"tw`[p]:[margin-top:var(--myvar)]`"#####, r#####"({
  '& p': {
    marginTop: "var(--myvar)",
  },
})
;"##### ; "11")]
#[test_case(r#####"tw`[p]:(mt-4 mb-4)`"#####, r#####"({
  '& p': {
    marginBottom: "1rem",
    marginTop: "1rem",
  },
})
;"##### ; "12")]
#[test_case(r#####"tw`[@media (min-width: 800px)]:block`"#####, r#####"({
  '@media (min-width: 800px)': {
    display: "block",
  },
})
;"##### ; "13")]
#[test_case(r#####"tw`[content\\!]:block`"#####, r#####"({
  '& content!': {
    display: "block",
  },
}) // Combinations

;"##### ; "14")]
#[test_case(r#####"tw`[&:nth-child(1)]:block`"#####, r#####"({
  ':nth-child(1)': {
    display: "block",
  },
})
;"##### ; "15")]
#[test_case(r#####"tw`[:nth-child(1)]:block`"#####, r#####"({
  ':nth-child(1)': {
    display: "block",
  },
})
;"##### ; "16")]
#[test_case(r#####"tw`[@media ...]:block`"#####, r#####"({
  '@media ...': {
    display: "block",
  },
})
;"##### ; "17")]
#[test_case(r#####"tw`[.selector]:block`"#####, r#####"({
  '& .selector': {
    display: "block",
  },
})
;"##### ; "18")]
#[test_case(r#####"tw`[section]:block`"#####, r#####"({
  '& section': {
    display: "block",
  },
})
;"##### ; "19")]
#[test_case(r#####"tw`[section &]:block`"#####, r#####"({
  'section &': {
    display: "block",
  },
})
;"##### ; "20")]
#[test_case(r#####"tw`md:[section]:block`"#####, r#####"({
  '@media (min-width: 768px)': {
    '& section': {
      display: "block",
    },
  },
})
;"##### ; "21")]
#[test_case(r#####"tw`[section]:[bla]:block`"#####, r#####"({
  '& section bla': {
    display: "block",
  },
})
;"##### ; "22")]
#[test_case(r#####"tw`[section &]:[pre &]:block`"#####, r#####"({
  '& pre section': {
    display: "block",
  },
})
;"##### ; "23")]
#[test_case(r#####"tw`[section &]:[& pre]:block`"#####, r#####"({
  '& section pre': {
    display: "block",
  },
})
;"##### ; "24")]
#[test_case(r#####"tw`[section &]:first:[pre &]:block`"#####, r#####"({
  'pre section &:first-child': {
    display: "block",
  },
})
;"##### ; "25")]
#[test_case(r#####"tw`[section &]:first:[& pre]:block`"#####, r#####"({
  'section &:first-child pre': {
    display: "block",
  },
})
;"##### ; "26")]
#[test_case(r#####"tw`first:[section &]:[pre &]:block`"#####, r#####"({
  ':first-child pre section': {
    display: "block",
  },
})
;"##### ; "27")]
#[test_case(r#####"tw`first:[section &]:[& pre]:block`"#####, r#####"({
  ':first-child section pre': {
    display: "block",
  },
})
;"##### ; "28")]
#[test_case(r#####"tw`first:[section &]:[& pre]:mt-[2px]`"#####, r#####"({
  ':first-child section pre': {
    marginTop: "2px",
  },
})
;"##### ; "29")]
#[test_case(r#####"tw`first:[section &]:[& pre]:[display:inline]`"#####, r#####"({
  ':first-child section pre': {
    display: "inline",
  },
})
;"##### ; "30")]
#[test_case(r#####"tw`[pre]:[display:inline]`"#####, r#####"({
  '& pre': {
    display: "inline",
  },
})
;"##### ; "31")]
#[test_case(r#####"tw`[& pre]:[display:inline]`"#####, r#####"({
  '& pre': {
    display: "inline",
  },
})
;"##### ; "32")]
#[test_case(r#####"tw`[:hover]:[display:inline]`"#####, r#####"({
  ':hover': {
    display: "inline",
  },
})
;"##### ; "33")]
#[test_case(r#####"tw`[.dropdown.dropdown-open &, .dropdown:focus &]:block`"#####, r#####"({
  '.dropdown.dropdown-open &,.dropdown:focus &': {
    display: "block",
  },
})
;"##### ; "34")]
#[test_case(r#####"tw`[path]:first:[stroke: #000] md:[path]:[stroke: #000]`"#####, r#####"({
  '@media (min-width: 768px)': {
    '& path': {
      stroke: "#000",
    },
  },
  'path:first-child': {
    stroke: "#000",
  },
})
;"##### ; "35")]
#[test_case(r#####"tw`first:block md:[path]:[stroke: #000]`"#####, r#####"({
  ':first-child': {
    display: "block",
  },
  '@media (min-width: 768px)': {
    '& path': {
      stroke: "#000",
    },
  },
})
;"##### ; "36")]
#[test_case(r#####"tw`[.sec section a[target="_blank"]]:block`"#####, r#####"({
  '& .sec section a[target=" blank"]': {
    display: "block",
  },
}) // < issue with _blank present in tailwindcss

;"##### ; "37")]
#[test_case(r#####"tw`[&.pre,& section,]:block`"#####, r#####"({
  '&.pre,& section': {
    display: "block",
  },
})"##### ; "38")]
fn test(input: &str, output: &str) {
    snapshot_inner(input, output)
}
