use std::cell::LazyCell;

use swc_core::ecma::{
    parser::{EsConfig, Syntax},
    transforms::testing::test_transform,
    visit::as_folder,
};
use tailwind_config::TailwindConfig;

use crate::{AppConfig, Engine, TransformVisitor};

// fn test_visitor(engine: Engine) -> TransformVisitor<'static> {
//     let mut ac: AppConfig = Default::default();
//     ac.config.theme.height.insert("4", "1rem");
//     ac.config.theme.spacing.insert("4", "1rem");
//     ac.config.theme.colors.insert("black", "black");
//     ac.config.theme.colors.insert("red-500", "red");
//     ac.config.theme.margin.insert("4", "1rem");
//     ac.config.theme.screens.insert("lg", Screens::Min("1024px"));

//     let mut t = TransformVisitor::new(&mut ac);

//     if let Err(_) = HANDLER.inner.set(Handler::with_tty_emitter(
//         ColorConfig::Auto,
//         true,
//         false,
//         None,
//     )) {
//         // set on a previous run
//     };

//     t.engine = engine;

//     t
// }

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::Emotion)),
//     basic,
//     // Input codes
//     r#"<Test tw="h-4" />"#,
//     // Output codes after transformed with plugin
//     r#"<Test css={{height: "1rem"}} />"#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::Emotion)),
//     string,
//     // Input codes
//     r#"<Test tw={"h-4"} />"#,
//     // Output codes after transformed with plugin
//     r#"<Test css={{height: "1rem"}} />"#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::Emotion)),
//     template,
//     r#"const x = tw`h-4`"#,
//     r#"const x = {height: "1rem"}"#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::Emotion)),
//     template_jsx,
//     r#"<Test css={tw`h-4`} />"#,
//     r#"<Test css={{height: "1rem"}} />"#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::Emotion)),
//     existing_css_object,
//     // Input codes
//     r#"<Test tw="h-4" css={{width: "1rem"}} />"#,
//     // Output codes after transformed with plugin
//     r#"<Test css={[{width: "1rem"}, {height: "1rem"}]} />"#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::Emotion)),
//     existing_css_array,
//     // Input codes
//     r#"<Test tw="h-4" css={[]} />"#,
//     // Output codes after transformed with plugin
//     r#"<Test css={[{height: "1rem"}]} />"#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::Emotion)),
//     multiple_mod,
//     // Input codes
//     r#"<Test tw="hover:h-4 hover:text-black" />"#,
//     // Output codes after transformed with plugin
//     r#"<Test css={{":hover": {height: "1rem", color: "black"}}} />"#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::Emotion)),
//     minus_values,
//     // Input codes
//     r#"<Test tw="-m-4" />"#,
//     // Output codes after transformed with plugin
//     r#"<Test css={{margin: "-1rem"}} />"#
// );

// // this currently doesn't work if the tw attr is BEFORE the subcomponent
// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::Emotion)),
//     multiple_values,
//     r#"<Test tw="m-4" render={<button tw="m-4" />} />"#,
//     r#"<Test render={<button css={{margin: "1rem"}}/>} css={{margin: "1rem"}} />"#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::Emotion)),
//     emotion,
//     // Input codes
//     r#"
// import { TailwindStyle } from "stailwc";

// export default function App({ Component, pageProps }) {
//     return <>
//         <TailwindStyle />
//         <Component {...pageProps} />
//     </>;
// }
// "#,
//     // Output codes after transformed with plugin
//     r#"
// import { css, Global } from "@emotion/react";
// export default function App({ Component, pageProps }) {
//     return <>
//         <Global styles={css`a,hr{color:inherit}progress,sub,sup{vertical-align:baseline}blockquote,body,dd,dl,fieldset,figure,h1,h2,h3,h4,h5,h6,hr,menu,ol,p,pre,ul{margin:0}fieldset,legend,menu,ol,ul{padding:0}*,::after,::before{box-sizing:border-box;border:0 solid #e5e7eb;--tw-border-spacing-x:0;--tw-border-spacing-y:0;--tw-translate-x:0;--tw-translate-y:0;--tw-rotate:0;--tw-skew-x:0;--tw-skew-y:0;--tw-scale-x:1;--tw-scale-y:1;--tw-pan-x: ;--tw-pan-y: ;--tw-pinch-zoom: ;--tw-scroll-snap-strictness:proximity;--tw-ordinal: ;--tw-slashed-zero: ;--tw-numeric-figure: ;--tw-numeric-spacing: ;--tw-numeric-fraction: ;--tw-ring-inset: ;--tw-ring-offset-width:0px;--tw-ring-offset-color:#fff;--tw-ring-color:rgb(59 130 246 / 0.5);--tw-ring-offset-shadow:0 0 #0000;--tw-ring-shadow:0 0 #0000;--tw-shadow:0 0 #0000;--tw-shadow-colored:0 0 #0000;--tw-blur: ;--tw-brightness: ;--tw-contrast: ;--tw-grayscale: ;--tw-hue-rotate: ;--tw-invert: ;--tw-saturate: ;--tw-sepia: ;--tw-drop-shadow: ;--tw-backdrop-blur: ;--tw-backdrop-brightness: ;--tw-backdrop-contrast: ;--tw-backdrop-grayscale: ;--tw-backdrop-hue-rotate: ;--tw-backdrop-invert: ;--tw-backdrop-opacity: ;--tw-backdrop-saturate: ;--tw-backdrop-sepia: }::after,::before{--tw-content:\"\"}html{line-height:1.5;-webkit-text-size-adjust:100%;-moz-tab-size:4;tab-size:4;font-family:}body{line-height:inherit}hr{height:0;border-top-width:1px}abbr:where([title]){-webkit-text-decoration:underline dotted;text-decoration:underline dotted}h1,h2,h3,h4,h5,h6{font-size:inherit;font-weight:inherit}a{text-decoration:inherit}b,strong{font-weight:bolder}code,kbd,pre,samp{font-family:;font-size:1em}small{font-size:80%}sub,sup{font-size:75%;line-height:0;position:relative}sub{bottom:-.25em}sup{top:-.5em}table{text-indent:0;border-color:inherit;border-collapse:collapse}button,input,optgroup,select,textarea{font-family:inherit;font-size:100%;font-weight:inherit;line-height:inherit;color:inherit;margin:0;padding:0}button,select{text-transform:none}[type=button],[type=reset],[type=submit],button{-webkit-appearance:button;background-color:transparent;background-image:none}:-moz-focusring{outline:auto}:-moz-ui-invalid{box-shadow:none}::-webkit-inner-spin-button,::-webkit-outer-spin-button{height:auto}[type=search]{-webkit-appearance:textfield;outline-offset:-2px}::-webkit-search-decoration{-webkit-appearance:none}::-webkit-file-upload-button{-webkit-appearance:button;font:inherit}summary{display:list-item}menu,ol,ul{list-style:none}textarea{resize:vertical}input::placeholder,textarea::placeholder{opacity:1;color:#9ca3af}[role=button],button{cursor:pointer}:disabled{cursor:default}audio,canvas,embed,iframe,img,object,svg,video{display:block;vertical-align:middle}img,video{max-width:100%;height:auto}[hidden]{display:none}::-webkit-backdrop{--tw-border-spacing-x:0;--tw-border-spacing-y:0;--tw-translate-x:0;--tw-translate-y:0;--tw-rotate:0;--tw-skew-x:0;--tw-skew-y:0;--tw-scale-x:1;--tw-scale-y:1;--tw-pan-x: ;--tw-pan-y: ;--tw-pinch-zoom: ;--tw-scroll-snap-strictness:proximity;--tw-ordinal: ;--tw-slashed-zero: ;--tw-numeric-figure: ;--tw-numeric-spacing: ;--tw-numeric-fraction: ;--tw-ring-inset: ;--tw-ring-offset-width:0px;--tw-ring-offset-color:#fff;--tw-ring-color:rgb(59 130 246 / 0.5);--tw-ring-offset-shadow:0 0 #0000;--tw-ring-shadow:0 0 #0000;--tw-shadow:0 0 #0000;--tw-shadow-colored:0 0 #0000;--tw-blur: ;--tw-brightness: ;--tw-contrast: ;--tw-grayscale: ;--tw-hue-rotate: ;--tw-invert: ;--tw-saturate: ;--tw-sepia: ;--tw-drop-shadow: ;--tw-backdrop-blur: ;--tw-backdrop-brightness: ;--tw-backdrop-contrast: ;--tw-backdrop-grayscale: ;--tw-backdrop-hue-rotate: ;--tw-backdrop-invert: ;--tw-backdrop-opacity: ;--tw-backdrop-saturate: ;--tw-backdrop-sepia: }::backdrop{--tw-border-spacing-x:0;--tw-border-spacing-y:0;--tw-translate-x:0;--tw-translate-y:0;--tw-rotate:0;--tw-skew-x:0;--tw-skew-y:0;--tw-scale-x:1;--tw-scale-y:1;--tw-pan-x: ;--tw-pan-y: ;--tw-pinch-zoom: ;--tw-scroll-snap-strictness:proximity;--tw-ordinal: ;--tw-slashed-zero: ;--tw-numeric-figure: ;--tw-numeric-spacing: ;--tw-numeric-fraction: ;--tw-ring-inset: ;--tw-ring-offset-width:0px;--tw-ring-offset-color:#fff;--tw-ring-color:rgb(59 130 246 / 0.5);--tw-ring-offset-shadow:0 0 #0000;--tw-ring-shadow:0 0 #0000;--tw-shadow:0 0 #0000;--tw-shadow-colored:0 0 #0000;--tw-blur: ;--tw-brightness: ;--tw-contrast: ;--tw-grayscale: ;--tw-hue-rotate: ;--tw-invert: ;--tw-saturate: ;--tw-sepia: ;--tw-drop-shadow: ;--tw-backdrop-blur: ;--tw-backdrop-brightness: ;--tw-backdrop-contrast: ;--tw-backdrop-grayscale: ;--tw-backdrop-hue-rotate: ;--tw-backdrop-invert: ;--tw-backdrop-opacity: ;--tw-backdrop-saturate: ;--tw-backdrop-sepia: }[type=checkbox]:checked,[type=checkbox]:checked:focus,[type=checkbox]:checked:hover,[type=radio]:checked,[type=radio]:checked:focus,[type=radio]:checked:hover{border-color:transparent;background-color:currentColor}[type=checkbox],[type=file]{border-radius:0}[multiple],[type=date],[type=datetime-local],[type=email],[type=month],[type=number],[type=password],[type=search],[type=tel],[type=text],[type=time],[type=url],[type=week],select,textarea{-webkit-appearance:none;appearance:none;background-color:#fff;border-color:#6b7280;border-width:1px;border-radius:0;padding:.5rem .75rem;font-size:1rem;line-height:1.5rem;--tw-shadow:0 0 #0000}[multiple]:focus,[type=date]:focus,[type=datetime-local]:focus,[type=email]:focus,[type=month]:focus,[type=number]:focus,[type=password]:focus,[type=search]:focus,[type=tel]:focus,[type=text]:focus,[type=time]:focus,[type=url]:focus,[type=week]:focus,select:focus,textarea:focus{outline:transparent solid 2px;outline-offset:2px;--tw-ring-inset:var(--tw-empty, /*!*/ /*!*/);--tw-ring-offset-width:0px;--tw-ring-offset-color:#fff;--tw-ring-color:#2563eb;--tw-ring-offset-shadow:var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color);--tw-ring-shadow:var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color);box-shadow:var(--tw-ring-offset-shadow),var(--tw-ring-shadow),var(--tw-shadow);border-color:#2563eb}input::placeholder,textarea::placeholder{color:#6b7280;opacity:1}::-webkit-datetime-edit-fields-wrapper{padding:0}::-webkit-date-and-time-value{min-height:1.5em}::-webkit-datetime-edit,::-webkit-datetime-edit-day-field,::-webkit-datetime-edit-hour-field,::-webkit-datetime-edit-meridiem-field,::-webkit-datetime-edit-millisecond-field,::-webkit-datetime-edit-minute-field,::-webkit-datetime-edit-month-field,::-webkit-datetime-edit-second-field,::-webkit-datetime-edit-year-field{padding-top:0;padding-bottom:0}select{background-image:url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e");background-position:right .5rem center;background-repeat:no-repeat;background-size:1.5em 1.5em;padding-right:2.5rem;print-color-adjust:exact}[multiple]{background-image:initial;background-position:initial;background-repeat:unset;background-size:initial;padding-right:.75rem;print-color-adjust:unset}[type=checkbox],[type=radio]{-webkit-appearance:none;appearance:none;padding:0;print-color-adjust:exact;display:inline-block;vertical-align:middle;background-origin:border-box;-webkit-user-select:none;user-select:none;flex-shrink:0;height:1rem;width:1rem;color:#2563eb;background-color:#fff;border-color:#6b7280;border-width:1px;--tw-shadow:0 0 #0000}[type=radio]{border-radius:100%}[type=checkbox]:focus,[type=radio]:focus{outline:transparent solid 2px;outline-offset:2px;--tw-ring-inset:var(--tw-empty, /*!*/ /*!*/);--tw-ring-offset-width:2px;--tw-ring-offset-color:#fff;--tw-ring-color:#2563eb;--tw-ring-offset-shadow:var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color);--tw-ring-shadow:var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);box-shadow:var(--tw-ring-offset-shadow),var(--tw-ring-shadow),var(--tw-shadow)}[type=checkbox]:checked,[type=radio]:checked{background-size:100% 100%;background-position:center;background-repeat:no-repeat}[type=checkbox]:checked{background-image:url("data:image/svg+xml,%3csvg viewBox='0 0 16 16' fill='white' xmlns='http://www.w3.org/2000/svg'%3e%3cpath d='M12.207 4.793a1 1 0 010 1.414l-5 5a1 1 0 01-1.414 0l-2-2a1 1 0 011.414-1.414L6.5 9.086l4.293-4.293a1 1 0 011.414 0z'/%3e%3c/svg%3e")}[type=radio]:checked{background-image:url("data:image/svg+xml,%3csvg viewBox='0 0 16 16' fill='white' xmlns='http://www.w3.org/2000/svg'%3e%3ccircle cx='8' cy='8' r='3'/%3e%3c/svg%3e")}[type=checkbox]:indeterminate{background-image:url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 16 16'%3e%3cpath stroke='white' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M4 8h8'/%3e%3c/svg%3e");border-color:transparent;background-color:currentColor;background-size:100% 100%;background-position:center;background-repeat:no-repeat}[type=checkbox]:indeterminate:focus,[type=checkbox]:indeterminate:hover{border-color:transparent;background-color:currentColor}[type=file]{background:unset;border-color:inherit;border-width:0;padding:0;font-size:unset;line-height:inherit}[type=file]:focus{outline:ButtonText solid 1px;outline:-webkit-focus-ring-color auto 1px}`}/>
//         <Component {...pageProps} />
//     </>;
// }
// "#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::StyledComponents)),
//     styled_components,
//     // Input codes
//     r#"
// import { TailwindStyle } from "stailwc";

// export default function App({ Component, pageProps }) {
//     return <>
//         <TailwindStyle />
//         <Component {...pageProps} />
//     </>;
// }
// "#,
//     // Output codes after transformed with plugin
//     r#"
// import { createGlobalStyle } from "styled-components";

// export default function App({ Component, pageProps }) {
//     return <>
//         <Global />
//         <Component {...pageProps} />
//     </>;
// }
// const Global = createGlobalStyle(`a,hr{color:inherit}progress,sub,sup{vertical-align:baseline}blockquote,body,dd,dl,fieldset,figure,h1,h2,h3,h4,h5,h6,hr,menu,ol,p,pre,ul{margin:0}fieldset,legend,menu,ol,ul{padding:0}*,::after,::before{box-sizing:border-box;border:0 solid #e5e7eb;--tw-border-spacing-x:0;--tw-border-spacing-y:0;--tw-translate-x:0;--tw-translate-y:0;--tw-rotate:0;--tw-skew-x:0;--tw-skew-y:0;--tw-scale-x:1;--tw-scale-y:1;--tw-pan-x: ;--tw-pan-y: ;--tw-pinch-zoom: ;--tw-scroll-snap-strictness:proximity;--tw-ordinal: ;--tw-slashed-zero: ;--tw-numeric-figure: ;--tw-numeric-spacing: ;--tw-numeric-fraction: ;--tw-ring-inset: ;--tw-ring-offset-width:0px;--tw-ring-offset-color:#fff;--tw-ring-color:rgb(59 130 246 / 0.5);--tw-ring-offset-shadow:0 0 #0000;--tw-ring-shadow:0 0 #0000;--tw-shadow:0 0 #0000;--tw-shadow-colored:0 0 #0000;--tw-blur: ;--tw-brightness: ;--tw-contrast: ;--tw-grayscale: ;--tw-hue-rotate: ;--tw-invert: ;--tw-saturate: ;--tw-sepia: ;--tw-drop-shadow: ;--tw-backdrop-blur: ;--tw-backdrop-brightness: ;--tw-backdrop-contrast: ;--tw-backdrop-grayscale: ;--tw-backdrop-hue-rotate: ;--tw-backdrop-invert: ;--tw-backdrop-opacity: ;--tw-backdrop-saturate: ;--tw-backdrop-sepia: }::after,::before{--tw-content:\"\"}html{line-height:1.5;-webkit-text-size-adjust:100%;-moz-tab-size:4;tab-size:4;font-family:}body{line-height:inherit}hr{height:0;border-top-width:1px}abbr:where([title]){-webkit-text-decoration:underline dotted;text-decoration:underline dotted}h1,h2,h3,h4,h5,h6{font-size:inherit;font-weight:inherit}a{text-decoration:inherit}b,strong{font-weight:bolder}code,kbd,pre,samp{font-family:;font-size:1em}small{font-size:80%}sub,sup{font-size:75%;line-height:0;position:relative}sub{bottom:-.25em}sup{top:-.5em}table{text-indent:0;border-color:inherit;border-collapse:collapse}button,input,optgroup,select,textarea{font-family:inherit;font-size:100%;font-weight:inherit;line-height:inherit;color:inherit;margin:0;padding:0}button,select{text-transform:none}[type=button],[type=reset],[type=submit],button{-webkit-appearance:button;background-color:transparent;background-image:none}:-moz-focusring{outline:auto}:-moz-ui-invalid{box-shadow:none}::-webkit-inner-spin-button,::-webkit-outer-spin-button{height:auto}[type=search]{-webkit-appearance:textfield;outline-offset:-2px}::-webkit-search-decoration{-webkit-appearance:none}::-webkit-file-upload-button{-webkit-appearance:button;font:inherit}summary{display:list-item}menu,ol,ul{list-style:none}textarea{resize:vertical}input::placeholder,textarea::placeholder{opacity:1;color:#9ca3af}[role=button],button{cursor:pointer}:disabled{cursor:default}audio,canvas,embed,iframe,img,object,svg,video{display:block;vertical-align:middle}img,video{max-width:100%;height:auto}[hidden]{display:none}::-webkit-backdrop{--tw-border-spacing-x:0;--tw-border-spacing-y:0;--tw-translate-x:0;--tw-translate-y:0;--tw-rotate:0;--tw-skew-x:0;--tw-skew-y:0;--tw-scale-x:1;--tw-scale-y:1;--tw-pan-x: ;--tw-pan-y: ;--tw-pinch-zoom: ;--tw-scroll-snap-strictness:proximity;--tw-ordinal: ;--tw-slashed-zero: ;--tw-numeric-figure: ;--tw-numeric-spacing: ;--tw-numeric-fraction: ;--tw-ring-inset: ;--tw-ring-offset-width:0px;--tw-ring-offset-color:#fff;--tw-ring-color:rgb(59 130 246 / 0.5);--tw-ring-offset-shadow:0 0 #0000;--tw-ring-shadow:0 0 #0000;--tw-shadow:0 0 #0000;--tw-shadow-colored:0 0 #0000;--tw-blur: ;--tw-brightness: ;--tw-contrast: ;--tw-grayscale: ;--tw-hue-rotate: ;--tw-invert: ;--tw-saturate: ;--tw-sepia: ;--tw-drop-shadow: ;--tw-backdrop-blur: ;--tw-backdrop-brightness: ;--tw-backdrop-contrast: ;--tw-backdrop-grayscale: ;--tw-backdrop-hue-rotate: ;--tw-backdrop-invert: ;--tw-backdrop-opacity: ;--tw-backdrop-saturate: ;--tw-backdrop-sepia: }::backdrop{--tw-border-spacing-x:0;--tw-border-spacing-y:0;--tw-translate-x:0;--tw-translate-y:0;--tw-rotate:0;--tw-skew-x:0;--tw-skew-y:0;--tw-scale-x:1;--tw-scale-y:1;--tw-pan-x: ;--tw-pan-y: ;--tw-pinch-zoom: ;--tw-scroll-snap-strictness:proximity;--tw-ordinal: ;--tw-slashed-zero: ;--tw-numeric-figure: ;--tw-numeric-spacing: ;--tw-numeric-fraction: ;--tw-ring-inset: ;--tw-ring-offset-width:0px;--tw-ring-offset-color:#fff;--tw-ring-color:rgb(59 130 246 / 0.5);--tw-ring-offset-shadow:0 0 #0000;--tw-ring-shadow:0 0 #0000;--tw-shadow:0 0 #0000;--tw-shadow-colored:0 0 #0000;--tw-blur: ;--tw-brightness: ;--tw-contrast: ;--tw-grayscale: ;--tw-hue-rotate: ;--tw-invert: ;--tw-saturate: ;--tw-sepia: ;--tw-drop-shadow: ;--tw-backdrop-blur: ;--tw-backdrop-brightness: ;--tw-backdrop-contrast: ;--tw-backdrop-grayscale: ;--tw-backdrop-hue-rotate: ;--tw-backdrop-invert: ;--tw-backdrop-opacity: ;--tw-backdrop-saturate: ;--tw-backdrop-sepia: }[type=checkbox]:checked,[type=checkbox]:checked:focus,[type=checkbox]:checked:hover,[type=radio]:checked,[type=radio]:checked:focus,[type=radio]:checked:hover{border-color:transparent;background-color:currentColor}[type=checkbox],[type=file]{border-radius:0}[multiple],[type=date],[type=datetime-local],[type=email],[type=month],[type=number],[type=password],[type=search],[type=tel],[type=text],[type=time],[type=url],[type=week],select,textarea{-webkit-appearance:none;appearance:none;background-color:#fff;border-color:#6b7280;border-width:1px;border-radius:0;padding:.5rem .75rem;font-size:1rem;line-height:1.5rem;--tw-shadow:0 0 #0000}[multiple]:focus,[type=date]:focus,[type=datetime-local]:focus,[type=email]:focus,[type=month]:focus,[type=number]:focus,[type=password]:focus,[type=search]:focus,[type=tel]:focus,[type=text]:focus,[type=time]:focus,[type=url]:focus,[type=week]:focus,select:focus,textarea:focus{outline:transparent solid 2px;outline-offset:2px;--tw-ring-inset:var(--tw-empty, /*!*/ /*!*/);--tw-ring-offset-width:0px;--tw-ring-offset-color:#fff;--tw-ring-color:#2563eb;--tw-ring-offset-shadow:var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color);--tw-ring-shadow:var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color);box-shadow:var(--tw-ring-offset-shadow),var(--tw-ring-shadow),var(--tw-shadow);border-color:#2563eb}input::placeholder,textarea::placeholder{color:#6b7280;opacity:1}::-webkit-datetime-edit-fields-wrapper{padding:0}::-webkit-date-and-time-value{min-height:1.5em}::-webkit-datetime-edit,::-webkit-datetime-edit-day-field,::-webkit-datetime-edit-hour-field,::-webkit-datetime-edit-meridiem-field,::-webkit-datetime-edit-millisecond-field,::-webkit-datetime-edit-minute-field,::-webkit-datetime-edit-month-field,::-webkit-datetime-edit-second-field,::-webkit-datetime-edit-year-field{padding-top:0;padding-bottom:0}select{background-image:url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e");background-position:right .5rem center;background-repeat:no-repeat;background-size:1.5em 1.5em;padding-right:2.5rem;print-color-adjust:exact}[multiple]{background-image:initial;background-position:initial;background-repeat:unset;background-size:initial;padding-right:.75rem;print-color-adjust:unset}[type=checkbox],[type=radio]{-webkit-appearance:none;appearance:none;padding:0;print-color-adjust:exact;display:inline-block;vertical-align:middle;background-origin:border-box;-webkit-user-select:none;user-select:none;flex-shrink:0;height:1rem;width:1rem;color:#2563eb;background-color:#fff;border-color:#6b7280;border-width:1px;--tw-shadow:0 0 #0000}[type=radio]{border-radius:100%}[type=checkbox]:focus,[type=radio]:focus{outline:transparent solid 2px;outline-offset:2px;--tw-ring-inset:var(--tw-empty, /*!*/ /*!*/);--tw-ring-offset-width:2px;--tw-ring-offset-color:#fff;--tw-ring-color:#2563eb;--tw-ring-offset-shadow:var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color);--tw-ring-shadow:var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);box-shadow:var(--tw-ring-offset-shadow),var(--tw-ring-shadow),var(--tw-shadow)}[type=checkbox]:checked,[type=radio]:checked{background-size:100% 100%;background-position:center;background-repeat:no-repeat}[type=checkbox]:checked{background-image:url("data:image/svg+xml,%3csvg viewBox='0 0 16 16' fill='white' xmlns='http://www.w3.org/2000/svg'%3e%3cpath d='M12.207 4.793a1 1 0 010 1.414l-5 5a1 1 0 01-1.414 0l-2-2a1 1 0 011.414-1.414L6.5 9.086l4.293-4.293a1 1 0 011.414 0z'/%3e%3c/svg%3e")}[type=radio]:checked{background-image:url("data:image/svg+xml,%3csvg viewBox='0 0 16 16' fill='white' xmlns='http://www.w3.org/2000/svg'%3e%3ccircle cx='8' cy='8' r='3'/%3e%3c/svg%3e")}[type=checkbox]:indeterminate{background-image:url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 16 16'%3e%3cpath stroke='white' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M4 8h8'/%3e%3c/svg%3e");border-color:transparent;background-color:currentColor;background-size:100% 100%;background-position:center;background-repeat:no-repeat}[type=checkbox]:indeterminate:focus,[type=checkbox]:indeterminate:hover{border-color:transparent;background-color:currentColor}[type=file]{background:unset;border-color:inherit;border-width:0;padding:0;font-size:unset;line-height:inherit}[type=file]:focus{outline:ButtonText solid 1px;outline:-webkit-focus-ring-color auto 1px}`);
// "#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::Emotion)),
//     emotion_component_native,
//     // Input codes
//     r#"
// tw.button`text-red-500`
// "#,
//     // Output codes after transformed with plugin
//     r#"
// _styled.button({
//     color: "red"
// });
// "#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::Emotion)),
//     emotion_component_extend,
//     // Input codes
//     r#"
// const Red = tw.button`text-red-500`
// const Button = tw(Red)`bg-black`
// "#,
//     // Output codes after transformed with plugin
//     r#"
// const Red = _styled.button({
//     color: "red"
// });
// const Button = _styled(Red)({
//     backgroundColor: "black"
// });
// "#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::StyledComponents)),
//     styled_components_component_native,
//     // Input codes
//     r#"
// tw.button`text-red-500`
// "#,
//     // Output codes after transformed with plugin
//     r#"
// _styled.button({
//     color: "red"
// });
// "#
// );

// test!(
//     Syntax::Typescript(TsConfig {
//         tsx: true,
//         ..Default::default()
//     }),
//     |_| as_folder(test_visitor(Engine::StyledComponents)),
//     styled_components_component_extend,
//     // Input codes
//     r#"
// const Red = tw.button`text-red-500`
// const Button = tw(Red)`bg-black`
// "#,
//     // Output codes after transformed with plugin
//     r#"
// const Red = _styled.button({
//     color: "red"
// });
// const Button = _styled(Red)({
//     backgroundColor: "black"
// });
// "#
// );

// include!(concat!(env!("OUT_DIR"), "/test_cases.rs"));

// fn snapshots_inner(path: &str) {
//     let input_path = Path::new(path);
//     let snapshot_path = Path::new("snapshots/output").join(input_path.file_name().unwrap());
//     let snapshot = read_to_string(snapshot_path).unwrap();

//     if let Err(_) = HANDLER.inner.set(Handler::with_tty_emitter(
//         ColorConfig::Auto,
//         true,
//         false,
//         None,
//     )) {
//         // set on a previous run
//     }

//     let tailwind_path = input_path.with_file_name("tailwind.config.js");
//     let tailwind_path = if tailwind_path.exists() {
//         format!("'./{}'", tailwind_path.to_str().unwrap())
//     } else {
//         "undefined".to_string()
//     };

//     let (input, expected) = snapshot.split_once("\n      ↓ ↓ ↓ ↓ ↓ ↓\n").unwrap();

//     let app_config =
//         run_fun!(node -e "console.log(JSON.stringify(require('./install.js')({silent: true, tailwindPath: ${tailwind_path}})[1]))")
//             .unwrap();

//     let deser = &mut serde_json::Deserializer::from_str(&app_config);
//     let app_config: Result<AppConfig, _> = serde_path_to_error::deserialize(deser);

//     test_transform(
//         Syntax::Typescript(TsConfig {
//             tsx: true,
//             ..Default::default()
//         }),
//         |_| {
//             as_folder(TransformVisitor {
//                 config: &mut app_config.unwrap().config,
//                 strict: true,
//                 engine: Engine::Emotion,
//                 tw_attr_stack: Default::default(),
//                 tw_style_imported: false,
//                 tw_tpl: None,
//             })
//         },
//         input,
//         expected,
//         false,
//     );
// }

const CONFIG: LazyCell<TailwindConfig> = std::cell::LazyCell::new(|| {
    let app_config = include_str!("../tw.json");
    let deser = &mut serde_json::Deserializer::from_str(app_config);
    let app_config: Result<AppConfig, _> = serde_path_to_error::deserialize(deser);
    app_config.expect("ok").config
});

pub fn snapshot_inner(input: &str, expected: &str) {
    let config = CONFIG;

    test_transform(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        |_| {
            as_folder(TransformVisitor {
                config: &config,
                strict: true,
                engine: Engine::Emotion,
                tw_attr_stack: Default::default(),
                tw_style_imported: false,
                tw_tpl: None,
            })
        },
        input,
        expected,
        false,
    );
}
