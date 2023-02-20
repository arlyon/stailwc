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
        error::Error,
        sequence::preceded,
        IResult, Slice,
    };

    /// The plugin represents the core command the tailwind parser is
    /// looking for. For example, the `text` plugin is represented by
    /// the `Text` variant.
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    #[root]
    pub enum Plugin {
        /// Transparent plugins do not add to the class name.
        ///
        /// Example: 'sticky', 'static', 'fixed'
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

        /// Border has an optional sub-item
        ///
        /// Example: 'border', 'border-t', 'border-b', 'border-l', 'border-r'
        Border(Option<Border>),
        Rounded(Option<Rounded>),
        Min(Min),
        Max(Max),
        List(List),
        H,
        W,
        P,
        Px,
        Pl,
        Auto(Auto),
        Pr,
        LineClamp,
        Py,
        Pt,
        #[rename("align")]
        VerticalAlign,
        Pb,
        Content,
        M,
        Mx,
        My,
        Ml,
        Backdrop(Backdrop),
        Stroke,
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
        Snap(Snap),
        Duration,
        Divide(Option<Divide>),
        Rotate,
        Appearance,
        Truncate,
        Animate,
        Pointer,
        Aspect,
        Ease,
        Order,
        Whitespace(Whitespace),
        From,
        To,
        Outline,
        Mix,
        Flex(Option<Flex>),
        Grid(Option<Grid>),
        Col(Option<Col>),
        Row(Option<Row>),
        Float,
        Grow,
        Shrink,
        Basis,
        Object(Object),
        Justify,
        Items,
        Leading,
        Gap(Option<Gap>),
        Cursor,
        Antialiased,
        Scroll(Scroll),
        Scale,
        Box,
        Select,
        Overflow(Option<Overflow>),
        Top,
        Bottom,
        Left,
        Right,
        #[rename("self")]
        AlignSelf(AlignSelf),
        Translate(Translate),
        Tracking,
        Invert,
        Space,
        Transform,
        Opacity,
        Italic,
        Blur,
        Ring,
        Sr,

        Prose(Option<Prose>),
        Not(Not),
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Scroll {
        Auto,
        Smooth,

        M,
        Mx,
        My,
        Ml,
        Mr,
        Mt,
        Mb,

        P,
        Px,
        Py,
        Pt,
        Pl,
        Pr,
        Pb,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Snap {
        Start,
        End,
        Center,

        Normal,
        Always,

        None,
        X,
        Y,
        Both,
        Mandatory,
        Proximity,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Backdrop {
        Blur,
        Brightness,
        Contrast,
        Grayscale,
        HueRotate,
        Invert,
        Opacity,
        Saturate,
        Sepia,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Auto {
        Cols,
        Rows,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Col {
        Start,
        End,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Row {
        Start,
        End,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Overflow {
        X,
        Y,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum List {
        None,
        Disc,
        Decimal,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Prose {
        Invert,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Not {
        Prose,
        Italic,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Translate {
        X,
        Y,
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
        /// 1. take a segment of the input
        /// 2. if it has a sub-segment, attempt to parse a plugin from the two
        /// 3. if it doesn't have a sub-segment, attempt to parse a plugin from the segment
        /// 4. if that plugin has subcommands, attempt to parse a subcommand from the sub-segment
        ///
        /// this code is ugly
        pub fn parse(s: NomSpan<'a>) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
            let parse_segment = || take_while1(|c| c != '-' && c != ' ' && c != '[' && c != '!');
            let next_segment = || preceded(tag("-"), parse_segment());

            let (mut rest, mut segment) = parse_segment()(s)?;
            let (mut prev_rest, mut prev_segment) = (rest, segment);

            while Plugin::has_subsegments(&segment) {
                if let Ok((rest2, subsegment)) = next_segment()(rest) {
                    prev_segment = segment;
                    prev_rest = rest;

                    rest = rest2;
                    segment = s.slice(
                        ..subsegment.location_offset() + subsegment.len()
                            - segment.location_offset(),
                    );
                } else {
                    break;
                }
            }

            // attempt to parse a plugin, instead parsing the previous segment if it fails
            // this is to allow for plugins with subcommands to be parsed
            let plugin_parse = segment
                .parse::<Plugin>()
                .map(|p| (rest, p))
                .or_else(|_| prev_segment.parse::<Plugin>().map(|p| (prev_rest, p)))
                .map_err(|_| nom::Err::Error(Error::new(segment, nom::error::ErrorKind::Tag)));

            if !plugin_parse
                .as_ref()
                .map(|(_, p)| p.has_subcommand())
                .unwrap_or(false)
            {
                return plugin_parse;
            }

            // attempt to parse a subcommand
            let subcommand =
                preceded(tag("-"), parse_segment())(rest).map(|(rest, subcmd_span)| {
                    (
                        rest,
                        s.slice(
                            ..subcmd_span.location_offset() + subcmd_span.len()
                                - s.location_offset(),
                        ),
                    )
                });

            // upon failing to parse, then return the first error
            match subcommand.map(|(rest, sub)| (rest, sub.parse::<Plugin>())) {
                Ok((rest, Ok(plugin))) => Ok((rest, plugin)),
                _ => plugin_parse,
            }
        }
    }
}
