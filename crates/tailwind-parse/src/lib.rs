#![feature(let_chains)]

pub use plugin::*;

#[tailwind_parse_macro::parser]
mod plugin {
    use nom::{
        bytes::complete::{tag, take_while1},
        combinator::map_res,
        sequence::{terminated, preceded},
        IResult,
        Slice
    };
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

            let parse_cmd = || take_while1(|c| c != '-' && c != ' ' && c != '[');

            let mut parse_plugin = map_res(parse_cmd(), |s: NomSpan<'a>| {
                s.parse::<Plugin>()
            });
            
            let cmd = parse_plugin(s);
            
            if let Ok((rest, p)) = cmd && p.has_subcommand(){
                let max = preceded(tag("-"), parse_cmd())(rest)
                .map(|(rest, subcmd_span)|{
                    println!("{:?}, {:?}", s, subcmd_span);
                    (rest, s.slice(..subcmd_span.location_offset()+subcmd_span.len()-s.location_offset()))} );
                
                println!("{:?} SUBCOMMAND NOT SUPPORTED {:?}", p, max);

                if let Ok((rest, Ok(plugin))) = max.map(|(rest, sub)|(rest, sub.parse::<Plugin>())) {
                    return Ok((rest, plugin));
                };
            };

            cmd.map(|(rest, p)| (rest, p))
        }
    }
}
