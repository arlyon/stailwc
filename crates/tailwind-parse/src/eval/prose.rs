use stailwc_swc_utils::{merge_literals, named_literal, to_lit};
use swc_core::ecma::ast::ObjectLit;
use tailwind_config::TailwindTheme;

use crate::{PluginResult, Prose, SubjectValue};

pub fn prose<'a>(
    p: Option<Prose>,
    _rest: &Option<SubjectValue>,
    _theme: &'a TailwindTheme,
) -> PluginResult<'a> {
    match p {
        Some(Prose::Invert) => Ok(to_lit(&[
            ("--tw-prose-body", "var(--tw-prose-invert-body)"),
            ("--tw-prose-headings", "var(--tw-prose-invert-headings)"),
            ("--tw-prose-lead", "var(--tw-prose-invert-lead)"),
            ("--tw-prose-links", "var(--tw-prose-invert-links)"),
            ("--tw-prose-bold", "var(--tw-prose-invert-bold)"),
            ("--tw-prose-counters", "var(--tw-prose-invert-counters)"),
            ("--tw-prose-bullets", "var(--tw-prose-invert-bullets)"),
            ("--tw-prose-hr", "var(--tw-prose-invert-hr)"),
            ("--tw-prose-quotes", "var(--tw-prose-invert-quotes)"),
            ("--tw-prose-captions", "var(--tw-prose-invert-captions)"),
            ("--tw-prose-code", "var(--tw-prose-invert-code)"),
            (
                "--tw-prose-quote-borders",
                "var(--tw-prose-invert-quote-borders)",
            ),
            ("--tw-prose-pre-code", "var(--tw-prose-invert-pre-code)"),
            ("--tw-prose-pre-bg", "var(--tw-prose-invert-pre-bg)"),
            ("--tw-prose-th-borders", "var(--tw-prose-invert-th-borders)"),
            ("--tw-prose-td-borders", "var(--tw-prose-invert-td-borders)"),
        ])),
        None => [
            to_lit(&[
                ("color", " var(--tw-prose-body)"),
                ("maxWidth", " 65ch"),
                ("--tw-prose-body", " #374151"),
                ("--tw-prose-headings", " #111827"),
                ("--tw-prose-lead", " #4b5563"),
                ("--tw-prose-links", " #111827"),
                ("--tw-prose-bold", " #111827"),
                ("--tw-prose-counters", " #6b7280"),
                ("--tw-prose-bullets", " #d1d5db"),
                ("--tw-prose-hr", " #e5e7eb"),
                ("--tw-prose-quotes", " #111827"),
                ("--tw-prose-quote-borders", " #e5e7eb"),
                ("--tw-prose-captions", " #6b7280"),
                ("--tw-prose-code", " #111827"),
                ("--tw-prose-pre-code", " #e5e7eb"),
                ("--tw-prose-pre-bg", " #1f2937"),
                ("--tw-prose-th-borders", " #d1d5db"),
                ("--tw-prose-td-borders", " #e5e7eb"),
                ("--tw-prose-invert-body", " #d1d5db"),
                ("--tw-prose-invert-headings", " #fff"),
                ("--tw-prose-invert-lead", " #9ca3af"),
                ("--tw-prose-invert-links", " #fff"),
                ("--tw-prose-invert-bold", " #fff"),
                ("--tw-prose-invert-counters", " #9ca3af"),
                ("--tw-prose-invert-bullets", " #4b5563"),
                ("--tw-prose-invert-hr", " #374151"),
                ("--tw-prose-invert-quotes", " #f3f4f6"),
                ("--tw-prose-invert-quote-borders", " #374151"),
                ("--tw-prose-invert-captions", " #9ca3af"),
                ("--tw-prose-invert-code", " #fff"),
                ("--tw-prose-invert-pre-code", " #d1d5db"),
                ("--tw-prose-invert-pre-bg", " rgb(0 0 0 / 50%)"),
                ("--tw-prose-invert-th-borders", " #4b5563"),
                ("--tw-prose-invert-td-borders", " #374151"),
                ("fontSize", " 1rem"),
                ("lineHeight", " 1.75"),
            ]),
            named_literal(
                "*:where([class~=\"lead\"]):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-lead)"),
                    ("fontSize", "1.25em"),
                    ("lineHeight", "1.6"),
                    ("marginTop", "1.2em"),
                    ("marginBottom", "1.2em"),
                ],
            ),
            named_literal(
                "*:where(a):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-links)"),
                    ("textDecoration", "underline"),
                    ("fontWeight", "500"),
                ],
            ),
            named_literal(
                "*:where(strong):not(:where([class~=\"not-prose\"] *))",
                &[("color", "var(--tw-prose-bold)"), ("fontWeight", "600")],
            ),
            named_literal(
                "*:where(ol):not(:where([class~=\"not-prose\"] *))",
                &[("listStyleType", "decimal"), ("paddingLeft", "1.625em")],
            ),
            named_literal(
                "*:where(ol[type=\"A\"]):not(:where([class~=\"not-prose\"] *))",
                &[("listStyleType", "upper-alpha")],
            ),
            named_literal(
                "*:where(ol[type=\"a\"]):not(:where([class~=\"not-prose\"] *))",
                &[("listStyleType", "lower-alpha")],
            ),
            named_literal(
                "*:where(ol[type=\"A\" s]):not(:where([class~=\"not-prose\"] *))",
                &[("listStyleType", "upper-alpha")],
            ),
            named_literal(
                "*:where(ol[type=\"a\" s]):not(:where([class~=\"not-prose\"] *))",
                &[("listStyleType", "lower-alpha")],
            ),
            named_literal(
                "*:where(ol[type=\"I\"]):not(:where([class~=\"not-prose\"] *))",
                &[("listStyleType", "upper-roman")],
            ),
            named_literal(
                "*:where(ol[type=\"i\"]):not(:where([class~=\"not-prose\"] *))",
                &[("listStyleType", "lower-roman")],
            ),
            named_literal(
                "*:where(ol[type=\"I\" s]):not(:where([class~=\"not-prose\"] *))",
                &[("listStyleType", "upper-roman")],
            ),
            named_literal(
                "*:where(ol[type=\"i\" s]):not(:where([class~=\"not-prose\"] *))",
                &[("listStyleType", "lower-roman")],
            ),
            named_literal(
                "*:where(ol[type=\"1\"]):not(:where([class~=\"not-prose\"] *))",
                &[("listStyleType", "decimal")],
            ),
            named_literal(
                "*:where(ul):not(:where([class~=\"not-prose\"] *))",
                &[("listStyleType", "disc"), ("paddingLeft", "1.625em")],
            ),
            named_literal(
                "*:where(ol > li):not(:where([class~=\"not-prose\"] *))::marker",
                &[("fontWeight", "400"), ("color", "var(--tw-prose-counters)")],
            ),
            named_literal(
                "*:where(ul > li):not(:where([class~=\"not-prose\"] *))::marker",
                &[("color", "var(--tw-prose-bullets)")],
            ),
            named_literal(
                "*:where(hr):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("borderColor", "var(--tw-prose-hr)"),
                    ("borderTopWidth", "1px"),
                    ("marginTop", "3em"),
                    ("marginBottom", "3em"),
                ],
            ),
            named_literal(
                "*:where(blockquote):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("fontWeight", "500"),
                    ("fontStyle", "italic"),
                    ("color", "var(--tw-prose-quotes)"),
                    ("borderLeftWidth", "0.25rem"),
                    ("borderLeftColor", "var(--tw-prose-quote-borders)"),
                    ("quotes", "\"\\201C\"\"\\201D\"\"\\2018\"\"\\2019\""),
                    ("marginTop", "1.6em"),
                    ("marginBottom", "1.6em"),
                    ("paddingLeft", "1em"),
                ],
            ),
            named_literal(
                "*:where(blockquote p:first-of-type):not(:where([class~=\"not-prose\"] *))::before",
                &[("content", "open-quote")],
            ),
            named_literal(
                "*:where(blockquote p:last-of-type):not(:where([class~=\"not-prose\"] *))::after",
                &[("content", "close-quote")],
            ),
            named_literal(
                "*:where(h1):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-headings)"),
                    ("fontWeight", "800"),
                    ("fontSize", "2.25em"),
                    ("marginTop", "0"),
                    ("marginBottom", "0.8888889em"),
                    ("lineHeight", "1.1111111"),
                ],
            ),
            named_literal(
                "*:where(h1 strong):not(:where([class~=\"not-prose\"] *))",
                &[("fontWeight", "900")],
            ),
            named_literal(
                "*:where(h2):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-headings)"),
                    ("fontWeight", "700"),
                    ("fontSize", "1.5em"),
                    ("marginTop", "2em"),
                    ("marginBottom", "1em"),
                    ("lineHeight", "1.3333333"),
                ],
            ),
            named_literal(
                "*:where(h2 strong):not(:where([class~=\"not-prose\"] *))",
                &[("fontWeight", "800")],
            ),
            named_literal(
                "*:where(h3):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-headings)"),
                    ("fontWeight", "600"),
                    ("fontSize", "1.25em"),
                    ("marginTop", "1.6em"),
                    ("marginBottom", "0.6em"),
                    ("lineHeight", "1.6"),
                ],
            ),
            named_literal(
                "*:where(h3 strong):not(:where([class~=\"not-prose\"] *))",
                &[("fontWeight", "700")],
            ),
            named_literal(
                "*:where(h4):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-headings)"),
                    ("fontWeight", "600"),
                    ("marginTop", "1.5em"),
                    ("marginBottom", "0.5em"),
                    ("lineHeight", "1.5"),
                ],
            ),
            named_literal(
                "*:where(h4 strong):not(:where([class~=\"not-prose\"] *))",
                &[("fontWeight", "700")],
            ),
            named_literal(
                "*:where(figure > *):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "0"), ("marginBottom", "0")],
            ),
            named_literal(
                "*:where(figcaption):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-captions)"),
                    ("fontSize", "0.875em"),
                    ("lineHeight", "1.4285714"),
                    ("marginTop", "0.8571429em"),
                ],
            ),
            named_literal(
                "*:where(code):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-code)"),
                    ("fontWeight", "600"),
                    ("fontSize", "0.875em"),
                ],
            ),
            named_literal(
                "*:where(code):not(:where([class~=\"not-prose\"] *))::before",
                &[("content", "\"`\"")],
            ),
            named_literal(
                "*:where(code):not(:where([class~=\"not-prose\"] *))::after",
                &[("content", "\"`\"")],
            ),
            named_literal(
                "*:where(a code):not(:where([class~=\"not-prose\"] *))",
                &[("color", "var(--tw-prose-links)")],
            ),
            named_literal(
                "*:where(pre):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-pre-code)"),
                    ("backgroundColor", "var(--tw-prose-pre-bg)"),
                    ("overflowX", "auto"),
                    ("fontWeight", "400"),
                    ("fontSize", "0.875em"),
                    ("lineHeight", "1.7142857"),
                    ("marginTop", "1.7142857em"),
                    ("marginBottom", "1.7142857em"),
                    ("borderRadius", "0.375rem"),
                    ("paddingTop", "0.8571429em"),
                    ("paddingRight", "1.1428571em"),
                    ("paddingBottom", "0.8571429em"),
                    ("paddingLeft", "1.1428571em"),
                ],
            ),
            named_literal(
                "*:where(pre code):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("backgroundColor", "transparent"),
                    ("borderWidth", "0"),
                    ("borderRadius", "0"),
                    ("padding", "0"),
                    ("fontWeight", "inherit"),
                    ("color", "inherit"),
                    ("fontSize", "inherit"),
                    ("fontFamily", "inherit"),
                    ("lineHeight", "inherit"),
                ],
            ),
            named_literal(
                "*:where(pre code):not(:where([class~=\"not-prose\"] *))::before",
                &[("content", "none")],
            ),
            named_literal(
                "*:where(pre code):not(:where([class~=\"not-prose\"] *))::after",
                &[("content", "none")],
            ),
            named_literal(
                "*:where(table):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("width", "100%"),
                    ("tableLayout", "auto"),
                    ("textAlign", "left"),
                    ("marginTop", "2em"),
                    ("marginBottom", "2em"),
                    ("fontSize", "0.875em"),
                    ("lineHeight", "1.7142857"),
                ],
            ),
            named_literal(
                "*:where(thead):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("borderBottomWidth", "1px"),
                    ("borderBottomColor", "var(--tw-prose-th-borders)"),
                ],
            ),
            named_literal(
                "*:where(thead th):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-headings)"),
                    ("fontWeight", "600"),
                    ("verticalAlign", "bottom"),
                    ("paddingRight", "0.5714286em"),
                    ("paddingBottom", "0.5714286em"),
                    ("paddingLeft", "0.5714286em"),
                ],
            ),
            named_literal(
                "*:where(tbody tr):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("borderBottomWidth", "1px"),
                    ("borderBottomColor", "var(--tw-prose-td-borders)"),
                ],
            ),
            named_literal(
                "*:where(tbody tr:last-child):not(:where([class~=\"not-prose\"] *))",
                &[("borderBottomWidth", "0")],
            ),
            named_literal(
                "*:where(tbody td):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("verticalAlign", "baseline"),
                    ("paddingTop", "0.5714286em"),
                    ("paddingRight", "0.5714286em"),
                    ("paddingBottom", "0.5714286em"),
                    ("paddingLeft", "0.5714286em"),
                ],
            ),
            named_literal(
                "*:where(p):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "1.25em"), ("marginBottom", "1.25em")],
            ),
            named_literal(
                "*:where(img):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "2em"), ("marginBottom", "2em")],
            ),
            named_literal(
                "*:where(video):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "2em"), ("marginBottom", "2em")],
            ),
            named_literal(
                "*:where(figure):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "2em"), ("marginBottom", "2em")],
            ),
            named_literal(
                "*:where(h2 code):not(:where([class~=\"not-prose\"] *))",
                &[("fontSize", "0.875em")],
            ),
            named_literal(
                "*:where(h3 code):not(:where([class~=\"not-prose\"] *))",
                &[("fontSize", "0.9em")],
            ),
            named_literal(
                "*:where(li):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "0.5em"), ("marginBottom", "0.5em")],
            ),
            named_literal(
                "*:where(ol > li):not(:where([class~=\"not-prose\"] *))",
                &[("paddingLeft", "0.375em")],
            ),
            named_literal(
                "*:where(ul > li):not(:where([class~=\"not-prose\"] *))",
                &[("paddingLeft", "0.375em")],
            ),
            named_literal(
                "& > :where(ul > li p):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "0.75em"), ("marginBottom", "0.75em")],
            ),
            named_literal(
                "& > :where(ul > li > *:first-child):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "1.25em")],
            ),
            named_literal(
                "& > :where(ul > li > *:last-child):not(:where([class~=\"not-prose\"] *))",
                &[("marginBottom", "1.25em")],
            ),
            named_literal(
                "& > :where(ol > li > *:first-child):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "1.25em")],
            ),
            named_literal(
                "& > :where(ol > li > *:last-child):not(:where([class~=\"not-prose\"] *))",
                &[("marginBottom", "1.25em")],
            ),
            named_literal(
                "*:where(ul ul, ul ol, ol ul, ol ol):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "0.75em"), ("marginBottom", "0.75em")],
            ),
            named_literal(
                "*:where(hr + *):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "0")],
            ),
            named_literal(
                "*:where(h2 + *):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "0")],
            ),
            named_literal(
                "*:where(h3 + *):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "0")],
            ),
            named_literal(
                "*:where(h4 + *):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "0")],
            ),
            named_literal(
                "*:where(thead th:first-child):not(:where([class~=\"not-prose\"] *))",
                &[("paddingLeft", "0")],
            ),
            named_literal(
                "*:where(thead th:last-child):not(:where([class~=\"not-prose\"] *))",
                &[("paddingRight", "0")],
            ),
            named_literal(
                "*:where(tbody td:first-child):not(:where([class~=\"not-prose\"] *))",
                &[("paddingLeft", "0")],
            ),
            named_literal(
                "*:where(tbody td:last-child):not(:where([class~=\"not-prose\"] *))",
                &[("paddingRight", "0")],
            ),
            named_literal(
                "& > :where(:first-child):not(:where([class~=\"not-prose\"] *))",
                &[("marginTop", "0")],
            ),
            named_literal(
                "& > :where(:last-child):not(:where([class~=\"not-prose\"] *))",
                &[("marginBottom", "0")],
            ),
        ]
        .into_iter()
        .reduce(merge_literals)
        .ok_or(vec![]),
    }
}
