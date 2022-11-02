pub use plugin::*;

#[tailwind_parse_macro::parser]
mod plugin {
    use nom::{bytes::complete::take_while, combinator::map_res, IResult};
    use nom_locate::LocatedSpan;
    use swc_core::common::Span;

    type NomSpan<'a> = LocatedSpan<&'a str, Span>;

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
        H,
        W,
        Min(Min),
        Max(Max),
        P,
        Px,
        Pl,
        Pr,
        Py,
        Pt,
        Pb,
        M,
        Mx,
        My,
        Ml,
        Mr,
        Mt,
        Mb,
        Z,
        Text,
        Bg,
        Font,
        Shadow,
        Transition,
        Placeholder,
        Delay,
        Duration,
        Divide,
        Rotate,
        Appearance,
        Pointer,
        Ease,
        Order,
        Rounded(Option<Rounded>),
        From,
        To,
        Outline,
        Mix,
        Flex,
        Grid,
        Col,
        Grow,
        Shrink,
        Basis,
        Justify,
        Items,
        Gap,
        Cursor,
        Scale,
        Box,
        Select,
        Overflow,
        Top,
        Bottom,
        Left,
        Right,
        Translate,
        Tracking,
        Invert,
        Space,
        Transform,
        Opacity,
        Blur,
        Ring,
        Sr,
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
        pub fn parse(s: NomSpan<'a>) -> IResult<NomSpan<'a>, Self, nom::error::Error<NomSpan<'a>>> {
            let _max = Plugin::max_dashes();

            let cmd = map_res(take_while(|c| c != '-'), |s: NomSpan<'a>| {
                s.parse::<Plugin>()
            })(s);

            // if the command has sub-commands, attempt to parse
            // if that fails, and the subcommand is optional, keep it,
            // otherwise return an error

            if cmd
                .as_ref()
                .map(|(_, c)| c.has_subcommand())
                .unwrap_or(false)
            {
                println!("NOT SUPPORTED");
            }

            cmd
        }
    }
}
