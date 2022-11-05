use stailwc_swc_utils::{merge_literals, named_literal, to_lit};
use swc_core::ecma::ast::ObjectLit;
use tailwind_config::TailwindTheme;

use crate::{Not, Prose, SubjectValue};

pub fn prose(
    p: Option<Prose>,
    _rest: &Option<SubjectValue>,
    _theme: &TailwindTheme,
) -> Option<ObjectLit> {
    match p {
        Some(Prose::Invert) => Some(to_lit(&[
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
                "var(--tw-prose-invert-quote-borders",
            ),
            ("--tw-prose-pre-code", "var(--tw-prose-invert-pre-code"),
            ("--tw-prose-pre-bg", "var(--tw-prose-invert-pre-bg"),
            ("--tw-prose-th-borders", "var(--tw-prose-invert-th-borders"),
            ("--tw-prose-td-borders", "var(--tw-prose-invert-td-borders"),
        ])),
        None => [
            to_lit(&[
                ("color", " var(--tw-prose-body)"),
                ("max-width", " 65ch"),
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
                ("font-size", " 1rem"),
                ("line-height", " 1.75"),
            ]),
            named_literal(
                "*:where([class~=\"lead\"]):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-lead)"),
                    ("font-size", "1.25em"),
                    ("line-height", "1.6"),
                    ("margin-top", "1.2em"),
                    ("margin-bottom", "1.2em"),
                ],
            ),
            named_literal(
                "*:where(a):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-links)"),
                    ("text-decoration", "underline"),
                    ("font-weight", "500"),
                ],
            ),
            named_literal(
                "*:where(strong):not(:where([class~=\"not-prose\"] *))",
                &[("color", "var(--tw-prose-bold)"), ("font-weight", "600")],
            ),
            named_literal(
                "*:where(ol):not(:where([class~=\"not-prose\"] *))",
                &[("list-style-type", "decimal"), ("padding-left", "1.625em")],
            ),
            named_literal(
                "*:where(ol[type=\"A\"]):not(:where([class~=\"not-prose\"] *))",
                &[("list-style-type", "upper-alpha")],
            ),
            named_literal(
                "*:where(ol[type=\"a\"]):not(:where([class~=\"not-prose\"] *))",
                &[("list-style-type", "lower-alpha")],
            ),
            named_literal(
                "*:where(ol[type=\"A\" s]):not(:where([class~=\"not-prose\"] *))",
                &[("list-style-type", "upper-alpha")],
            ),
            named_literal(
                "*:where(ol[type=\"a\" s]):not(:where([class~=\"not-prose\"] *))",
                &[("list-style-type", "lower-alpha")],
            ),
            named_literal(
                "*:where(ol[type=\"I\"]):not(:where([class~=\"not-prose\"] *))",
                &[("list-style-type", "upper-roman")],
            ),
            named_literal(
                "*:where(ol[type=\"i\"]):not(:where([class~=\"not-prose\"] *))",
                &[("list-style-type", "lower-roman")],
            ),
            named_literal(
                "*:where(ol[type=\"I\" s]):not(:where([class~=\"not-prose\"] *))",
                &[("list-style-type", "upper-roman")],
            ),
            named_literal(
                "*:where(ol[type=\"i\" s]):not(:where([class~=\"not-prose\"] *))",
                &[("list-style-type", "lower-roman")],
            ),
            named_literal(
                "*:where(ol[type=\"1\"]):not(:where([class~=\"not-prose\"] *))",
                &[("list-style-type", "decimal")],
            ),
            named_literal(
                "*:where(ul):not(:where([class~=\"not-prose\"] *))",
                &[("list-style-type", "disc"), ("padding-left", "1.625em")],
            ),
            named_literal(
                "*:where(ol > li):not(:where([class~=\"not-prose\"] *))::marker",
                &[
                    ("font-weight", "400"),
                    ("color", "var(--tw-prose-counters)"),
                ],
            ),
            named_literal(
                "*:where(ul > li):not(:where([class~=\"not-prose\"] *))::marker",
                &[("color", "var(--tw-prose-bullets)")],
            ),
            named_literal(
                "*:where(hr):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("border-color", "var(--tw-prose-hr)"),
                    ("border-top-width", "1px"),
                    ("margin-top", "3em"),
                    ("margin-bottom", "3em"),
                ],
            ),
            named_literal(
                "*:where(blockquote):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("font-weight", "500"),
                    ("font-style", "italic"),
                    ("color", "var(--tw-prose-quotes)"),
                    ("border-left-width", "0.25rem"),
                    ("border-left-color", "var(--tw-prose-quote-borders)"),
                    ("quotes", "\"\\201C\"\"\\201D\"\"\\2018\"\"\\2019\""),
                    ("margin-top", "1.6em"),
                    ("margin-bottom", "1.6em"),
                    ("padding-left", "1em"),
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
                    ("font-weight", "800"),
                    ("font-size", "2.25em"),
                    ("margin-top", "0"),
                    ("margin-bottom", "0.8888889em"),
                    ("line-height", "1.1111111"),
                ],
            ),
            named_literal(
                "*:where(h1 strong):not(:where([class~=\"not-prose\"] *))",
                &[("font-weight", "900")],
            ),
            named_literal(
                "*:where(h2):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-headings)"),
                    ("font-weight", "700"),
                    ("font-size", "1.5em"),
                    ("margin-top", "2em"),
                    ("margin-bottom", "1em"),
                    ("line-height", "1.3333333"),
                ],
            ),
            named_literal(
                "*:where(h2 strong):not(:where([class~=\"not-prose\"] *))",
                &[("font-weight", "800")],
            ),
            named_literal(
                "*:where(h3):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-headings)"),
                    ("font-weight", "600"),
                    ("font-size", "1.25em"),
                    ("margin-top", "1.6em"),
                    ("margin-bottom", "0.6em"),
                    ("line-height", "1.6"),
                ],
            ),
            named_literal(
                "*:where(h3 strong):not(:where([class~=\"not-prose\"] *))",
                &[("font-weight", "700")],
            ),
            named_literal(
                "*:where(h4):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-headings)"),
                    ("font-weight", "600"),
                    ("margin-top", "1.5em"),
                    ("margin-bottom", "0.5em"),
                    ("line-height", "1.5"),
                ],
            ),
            named_literal(
                "*:where(h4 strong):not(:where([class~=\"not-prose\"] *))",
                &[("font-weight", "700")],
            ),
            named_literal(
                "*:where(figure > *):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "0"), ("margin-bottom", "0")],
            ),
            named_literal(
                "*:where(figcaption):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-captions)"),
                    ("font-size", "0.875em"),
                    ("line-height", "1.4285714"),
                    ("margin-top", "0.8571429em"),
                ],
            ),
            named_literal(
                "*:where(code):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-code)"),
                    ("font-weight", "600"),
                    ("font-size", "0.875em"),
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
                    ("background-color", "var(--tw-prose-pre-bg)"),
                    ("overflow-x", "auto"),
                    ("font-weight", "400"),
                    ("font-size", "0.875em"),
                    ("line-height", "1.7142857"),
                    ("margin-top", "1.7142857em"),
                    ("margin-bottom", "1.7142857em"),
                    ("border-radius", "0.375rem"),
                    ("padding-top", "0.8571429em"),
                    ("padding-right", "1.1428571em"),
                    ("padding-bottom", "0.8571429em"),
                    ("padding-left", "1.1428571em"),
                ],
            ),
            named_literal(
                "*:where(pre code):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("background-color", "transparent"),
                    ("border-width", "0"),
                    ("border-radius", "0"),
                    ("padding", "0"),
                    ("font-weight", "inherit"),
                    ("color", "inherit"),
                    ("font-size", "inherit"),
                    ("font-family", "inherit"),
                    ("line-height", "inherit"),
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
                    ("table-layout", "auto"),
                    ("text-align", "left"),
                    ("margin-top", "2em"),
                    ("margin-bottom", "2em"),
                    ("font-size", "0.875em"),
                    ("line-height", "1.7142857"),
                ],
            ),
            named_literal(
                "*:where(thead):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("border-bottom-width", "1px"),
                    ("border-bottom-color", "var(--tw-prose-th-borders)"),
                ],
            ),
            named_literal(
                "*:where(thead th):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("color", "var(--tw-prose-headings)"),
                    ("font-weight", "600"),
                    ("vertical-align", "bottom"),
                    ("padding-right", "0.5714286em"),
                    ("padding-bottom", "0.5714286em"),
                    ("padding-left", "0.5714286em"),
                ],
            ),
            named_literal(
                "*:where(tbody tr):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("border-bottom-width", "1px"),
                    ("border-bottom-color", "var(--tw-prose-td-borders)"),
                ],
            ),
            named_literal(
                "*:where(tbody tr:last-child):not(:where([class~=\"not-prose\"] *))",
                &[("border-bottom-width", "0")],
            ),
            named_literal(
                "*:where(tbody td):not(:where([class~=\"not-prose\"] *))",
                &[
                    ("vertical-align", "baseline"),
                    ("padding-top", "0.5714286em"),
                    ("padding-right", "0.5714286em"),
                    ("padding-bottom", "0.5714286em"),
                    ("padding-left", "0.5714286em"),
                ],
            ),
            named_literal(
                "*:where(p):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "1.25em"), ("margin-bottom", "1.25em")],
            ),
            named_literal(
                "*:where(img):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "2em"), ("margin-bottom", "2em")],
            ),
            named_literal(
                "*:where(video):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "2em"), ("margin-bottom", "2em")],
            ),
            named_literal(
                "*:where(figure):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "2em"), ("margin-bottom", "2em")],
            ),
            named_literal(
                "*:where(h2 code):not(:where([class~=\"not-prose\"] *))",
                &[("font-size", "0.875em")],
            ),
            named_literal(
                "*:where(h3 code):not(:where([class~=\"not-prose\"] *))",
                &[("font-size", "0.9em")],
            ),
            named_literal(
                "*:where(li):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "0.5em"), ("margin-bottom", "0.5em")],
            ),
            named_literal(
                "*:where(ol > li):not(:where([class~=\"not-prose\"] *))",
                &[("padding-left", "0.375em")],
            ),
            named_literal(
                "*:where(ul > li):not(:where([class~=\"not-prose\"] *))",
                &[("padding-left", "0.375em")],
            ),
            named_literal(
                "& > :where(ul > li p):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "0.75em"), ("margin-bottom", "0.75em")],
            ),
            named_literal(
                "& > :where(ul > li > *:first-child):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "1.25em")],
            ),
            named_literal(
                "& > :where(ul > li > *:last-child):not(:where([class~=\"not-prose\"] *))",
                &[("margin-bottom", "1.25em")],
            ),
            named_literal(
                "& > :where(ol > li > *:first-child):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "1.25em")],
            ),
            named_literal(
                "& > :where(ol > li > *:last-child):not(:where([class~=\"not-prose\"] *))",
                &[("margin-bottom", "1.25em")],
            ),
            named_literal(
                "*:where(ul ul, ul ol, ol ul, ol ol):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "0.75em"), ("margin-bottom", "0.75em")],
            ),
            named_literal(
                "*:where(hr + *):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "0")],
            ),
            named_literal(
                "*:where(h2 + *):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "0")],
            ),
            named_literal(
                "*:where(h3 + *):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "0")],
            ),
            named_literal(
                "*:where(h4 + *):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "0")],
            ),
            named_literal(
                "*:where(thead th:first-child):not(:where([class~=\"not-prose\"] *))",
                &[("padding-left", "0")],
            ),
            named_literal(
                "*:where(thead th:last-child):not(:where([class~=\"not-prose\"] *))",
                &[("padding-right", "0")],
            ),
            named_literal(
                "*:where(tbody td:first-child):not(:where([class~=\"not-prose\"] *))",
                &[("padding-left", "0")],
            ),
            named_literal(
                "*:where(tbody td:last-child):not(:where([class~=\"not-prose\"] *))",
                &[("padding-right", "0")],
            ),
            named_literal(
                "& > :where(:first-child):not(:where([class~=\"not-prose\"] *))",
                &[("margin-top", "0")],
            ),
            named_literal(
                "& > :where(:last-child):not(:where([class~=\"not-prose\"] *))",
                &[("margin-bottom", "0")],
            ),
        ]
        .into_iter()
        .reduce(|a, b| merge_literals(a, b)),
    }
}
