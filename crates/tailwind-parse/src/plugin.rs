//!The plugin represents the core command the tailwind parser is
//! looking for. For example, the `text` plugin is represented by
//! the `Text` variant.
//!
//! The `Plugin` is a part of a `Literal` which is a part of an
//! `Expression`.

pub use plugin::*;

#[tailwind_parse_macro::parser]
mod plugin {
    use crate::NomSpan;
    use nom::{
        bytes::complete::{tag, take_while1},
        combinator::map_res,
        sequence::preceded,
        IResult, Slice,
    };

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    #[root]
    pub enum Plugin {
        #[transparent]
        Position(Position),
        #[transparent]
        Visibility(Visibility),
        #[transparent]
        Display(Display),
        #[transparent]
        TextTransform(TextTransform),
        #[transparent]
        TextDecoration(TextDecoration),

        Border(Option<Border>),
        Rounded(Option<Rounded>),
        Min(Min),
        Max(Max),
        H,
        W,
        P,
        Px,
        Pl,
        Pr,
        Py,
        Pt,
        #[rename("align")]
        VerticalAlign,
        Pb,
        M,
        Mx,
        My,
        Ml,
        Mr,
        Mt,
        #[rename("origin")]
        TransformOrigin,
        Mb,
        Z,
        Text,
        Bg,
        Font,
        Fill,
        Shadow,
        Transition,
        Placeholder,
        Inset(Option<Inset>),
        Delay,
        Duration,
        Divide(Option<Divide>),
        Rotate,
        Appearance,
        Truncate,
        Animate,
        Pointer,
        Ease,
        Order,
        Whitespace(Whitespace),
        From,
        To,
        Outline,
        Mix,
        Flex(Option<Flex>),
        Grid(Option<Grid>),
        Col,
        Grow,
        Shrink,
        Basis,
        Object(Object),
        Justify,
        Items,
        Leading,
        Gap(Option<Gap>),
        Cursor,
        Scale,
        Box,
        Select,
        Overflow,
        Top,
        Bottom,
        Left,
        Right,
        #[rename("self")]
        AlignSelf(AlignSelf),
        Translate,
        Tracking,
        Invert,
        Space,
        Transform,
        Opacity,
        Blur,
        Ring,
        Sr,

        Prose(Option<Prose>),
        Not(Not),
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Prose {
        Invert,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Not {
        Prose,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum AlignSelf {
        Auto,
        Start,
        End,
        Center,
        Stretch,
        Baseline,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Whitespace {
        Normal,
        Nowrap,
        Pre,
        PreLine,
        PreWrap,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Divide {
        X,
        Y,
        Solid,
        Dashed,
        Dotted,
        Double,
        None,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Gap {
        X,
        Y,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Object {
        Contain,
        Cover,
        Fill,
        None,
        ScaleDown,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Inset {
        X,
        Y,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Grid {
        Cols,
        Rows,
        FlowRow,
        FlowCol,
        FlowDense,
        FlowRowDense,
        FlowColDense,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Flex {
        Row,
        Col,
        Grow,
        Shrink,
        RowReverse,
        ColReverse,
        Wrap,
        WrapReverse,
        NoWrap,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Position {
        Static,
        Fixed,
        Absolute,
        Relative,
        Sticky,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Visibility {
        Visible,
        Invisible,
    }

    /// The options for the `display` property.
    ///
    /// Note: flex and grid are handled by their own plugins.
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Display {
        Block,
        InlineBlock,
        Inline,
        InlineFlex,
        Table,
        InlineTable,
        TableCaption,
        TableCell,
        TableColumn,
        TableColumnGroup,
        TableFooterGroup,
        TableHeaderGroup,
        TableRowGroup,
        TableRow,
        FlowRoot,
        InlineGrid,
        Contents,
        ListItem,
        Hidden,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum TextTransform {
        Uppercase,
        Lowercase,
        Capitalize,
        NormalCase,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum TextDecoration {
        Underline,
        Overline,
        LineThrough,
        NoUnderline,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Rounded {
        T,
        B,
        L,
        R,
        Tl,
        Tr,
        Bl,
        Br,
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Border {
        B,
        T,
        L,
        R,
        X,
        Y,
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Min {
        W,
        H,
    }
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Max {
        W,
        H,
    }

    impl<'a> Plugin {
        /// At a hight level, this algorithm:
        ///
        /// 1. attempts to parse a plugin from the first segment in the text
        /// 2. if it fails, it then checks the rootless map to determine whether it should keep searching
        /// 3. if there is a subcommand, it attempts to parse the subcommand
        /// 4. if the subcommand fails to parse, it falls back to the first discovered error
        ///
        pub fn parse(s: NomSpan<'a>) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
            let parse_cmd = || take_while1(|c| c != '-' && c != ' ' && c != '[' && c != '!');
            let mut parse_plugin = map_res(parse_cmd(), |s: NomSpan<'a>| s.parse::<Plugin>());

            // step 1
            let cmd = parse_plugin(s);

            // step 2
            let rest = if let Ok((rest, p)) = cmd && p.has_subcommand() {
                rest
            } else if cmd.is_err() && let Ok((rest, cmd)) = parse_cmd()(s) && Plugin::has_subsegments(&cmd) {
                rest
            } else  {
                // this is not a subcommand, return early
                return cmd;
            };

            // step 3
            let max = preceded(tag("-"), parse_cmd())(rest).map(|(rest, subcmd_span)| {
                (
                    rest,
                    s.slice(
                        ..subcmd_span.location_offset() + subcmd_span.len() - s.location_offset(),
                    ),
                )
            });

            // step 4
            match max.map(|(rest, sub)| (rest, sub.parse::<Plugin>())) {
                Ok((rest, Ok(plugin))) => Ok((rest, plugin)),
                _ => cmd,
            }
        }
    }
}
