

import { GlobalStyles } from './macro'
;<GlobalStyles />

      ↓ ↓ ↓ ↓ ↓ ↓

import { css as _css } from '@emotion/react'
import { Global as _globalImport } from '@emotion/react'

const _GlobalStyles = () => (
  <_globalImport
    styles={_css`*, ::before, ::after {
box-sizing: border-box;
border-width: 0;
border-style: solid;
border-color: #e5e7eb;
--tw-translate-x: 0;
--tw-translate-y: 0;
--tw-rotate: 0;
--tw-skew-x: 0;
--tw-skew-y: 0;
--tw-scale-x: 1;
--tw-scale-y: 1;
--tw-pan-x: var(--tw-empty,/*!*/ /*!*/);
--tw-pan-y: var(--tw-empty,/*!*/ /*!*/);
--tw-pinch-zoom: var(--tw-empty,/*!*/ /*!*/);
--tw-scroll-snap-strictness: proximity;
--tw-ordinal: var(--tw-empty,/*!*/ /*!*/);
--tw-slashed-zero: var(--tw-empty,/*!*/ /*!*/);
--tw-numeric-figure: var(--tw-empty,/*!*/ /*!*/);
--tw-numeric-spacing: var(--tw-empty,/*!*/ /*!*/);
--tw-numeric-fraction: var(--tw-empty,/*!*/ /*!*/);
--tw-ring-inset: var(--tw-empty,/*!*/ /*!*/);
--tw-ring-offset-width: 0px;
--tw-ring-offset-color: #fff;
--tw-ring-color: rgb(59 130 246 / 0.5);
--tw-ring-offset-shadow: 0 0 #0000;
--tw-ring-shadow: 0 0 #0000;
--tw-shadow: 0 0 #0000;
--tw-shadow-colored: 0 0 #0000;
--tw-blur: var(--tw-empty,/*!*/ /*!*/);
--tw-brightness: var(--tw-empty,/*!*/ /*!*/);
--tw-contrast: var(--tw-empty,/*!*/ /*!*/);
--tw-grayscale: var(--tw-empty,/*!*/ /*!*/);
--tw-hue-rotate: var(--tw-empty,/*!*/ /*!*/);
--tw-invert: var(--tw-empty,/*!*/ /*!*/);
--tw-saturate: var(--tw-empty,/*!*/ /*!*/);
--tw-sepia: var(--tw-empty,/*!*/ /*!*/);
--tw-drop-shadow: var(--tw-empty,/*!*/ /*!*/);
--tw-backdrop-blur: var(--tw-empty,/*!*/ /*!*/);
--tw-backdrop-brightness: var(--tw-empty,/*!*/ /*!*/);
--tw-backdrop-contrast: var(--tw-empty,/*!*/ /*!*/);
--tw-backdrop-grayscale: var(--tw-empty,/*!*/ /*!*/);
--tw-backdrop-hue-rotate: var(--tw-empty,/*!*/ /*!*/);
--tw-backdrop-invert: var(--tw-empty,/*!*/ /*!*/);
--tw-backdrop-opacity: var(--tw-empty,/*!*/ /*!*/);
--tw-backdrop-saturate: var(--tw-empty,/*!*/ /*!*/);
--tw-backdrop-sepia: var(--tw-empty,/*!*/ /*!*/);
        }
::before, ::after {
--tw-content: '';
        }
html {
line-height: 1.5;
-webkit-text-size-adjust: 100%;
-moz-tab-size: 4;
tab-size: 4;
font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
        }
body {
margin: 0;
line-height: inherit;
        }
hr {
height: 0;
color: inherit;
border-top-width: 1px;
        }
abbr:where([title]) {
text-decoration: underline dotted;
        }
h1, h2, h3, h4, h5, h6 {
font-size: inherit;
font-weight: inherit;
        }
a {
color: inherit;
text-decoration: inherit;
        }
b, strong {
font-weight: bolder;
        }
code, kbd, samp, pre {
font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
font-size: 1em;
        }
small {
font-size: 80%;
        }
sub, sup {
font-size: 75%;
line-height: 0;
position: relative;
vertical-align: baseline;
        }
sub {
bottom: -0.25em;
        }
sup {
top: -0.5em;
        }
table {
text-indent: 0;
border-color: inherit;
border-collapse: collapse;
        }
button, input, optgroup, select, textarea {
font-family: inherit;
font-size: 100%;
line-height: inherit;
color: inherit;
margin: 0;
padding: 0;
        }
button, select {
text-transform: none;
        }
button, [type="button"], [type="reset"], [type="submit"] {
-webkit-appearance: button;
background-color: transparent;
background-image: none;
        }
:-moz-focusring {
outline: auto;
        }
:-moz-ui-invalid {
box-shadow: none;
        }
progress {
vertical-align: baseline;
        }
::-webkit-inner-spin-button, ::-webkit-outer-spin-button {
height: auto;
        }
[type="search"] {
-webkit-appearance: textfield;
outline-offset: -2px;
        }
::-webkit-search-decoration {
-webkit-appearance: none;
        }
::-webkit-file-upload-button {
-webkit-appearance: button;
font: inherit;
        }
summary {
display: list-item;
        }
blockquote, dl, dd, h1, h2, h3, h4, h5, h6, hr, figure, p, pre {
margin: 0;
        }
fieldset {
margin: 0;
padding: 0;
        }
legend {
padding: 0;
        }
ol, ul, menu {
list-style: none;
margin: 0;
padding: 0;
        }
textarea {
resize: vertical;
        }
input::-moz-placeholder, textarea::-moz-placeholder {
opacity: 1;
color: #9ca3af;
        }
input:-ms-input-placeholder, textarea:-ms-input-placeholder {
opacity: 1;
color: #9ca3af;
        }
input::placeholder, textarea::placeholder {
opacity: 1;
color: #6b7280;
        }
button, [role="button"] {
cursor: pointer;
        }
:disabled, [disabled] {
cursor: default;
        }
img, svg, video, canvas, audio, iframe, embed, object {
display: block;
vertical-align: middle;
        }
img, video {
max-width: 100%;
height: auto;
        }
[hidden] {
display: none;
        }
@keyframes spin {
to {
transform: rotate(360deg);
        }
        }
@keyframes ping {
75%, 100% {
transform: scale(2);
opacity: 0;
        }
        }
@keyframes pulse {
50% {
opacity: .5;
        }
        }
@keyframes bounce {
0%, 100% {
transform: translateY(-25%);
animation-timing-function: cubic-bezier(0.8,0,1,1);
        }
50% {
transform: none;
animation-timing-function: cubic-bezier(0,0,0.2,1);
        }
        }
[type='text'], [type='email'], [type='url'], [type='password'], [type='number'], [type='date'], [type='datetime-local'], [type='month'], [type='search'], [type='tel'], [type='time'], [type='week'], [multiple], textarea, select {
appearance: none;
background-color: #fff;
border-color: #6b7280;
border-width: 1px;
border-radius: 0px;
padding-top: 0.5rem;
padding-right: 0.75rem;
padding-bottom: 0.5rem;
padding-left: 0.75rem;
font-size: 1rem;
line-height: 1.5rem;
--tw-shadow: 0 0 #0000;
        }
[type='text']:focus, [type='email']:focus, [type='url']:focus, [type='password']:focus, [type='number']:focus, [type='date']:focus, [type='datetime-local']:focus, [type='month']:focus, [type='search']:focus, [type='tel']:focus, [type='time']:focus, [type='week']:focus, [multiple]:focus, textarea:focus, select:focus {
outline: 2px solid transparent;
outline-offset: 2px;
--tw-ring-inset: var(--tw-empty,/*!*/ /*!*/);
--tw-ring-offset-width: 0px;
--tw-ring-offset-color: #fff;
--tw-ring-color: #2563eb;
--tw-ring-offset-shadow: var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color);
--tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color);
box-shadow: var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow);
border-color: #2563eb;
        }
::-webkit-datetime-edit-fields-wrapper {
padding: 0;
        }
::-webkit-date-and-time-value {
min-height: 1.5em;
        }
::-webkit-datetime-edit, ::-webkit-datetime-edit-year-field, ::-webkit-datetime-edit-month-field, ::-webkit-datetime-edit-day-field, ::-webkit-datetime-edit-hour-field, ::-webkit-datetime-edit-minute-field, ::-webkit-datetime-edit-second-field, ::-webkit-datetime-edit-millisecond-field, ::-webkit-datetime-edit-meridiem-field {
padding-top: 0;
padding-bottom: 0;
        }
select {
background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e");
background-position: right 0.5rem center;
background-repeat: no-repeat;
background-size: 1.5em 1.5em;
padding-right: 2.5rem;
print-color-adjust: exact;
        }
[multiple] {
background-image: initial;
background-position: initial;
background-repeat: unset;
background-size: initial;
padding-right: 0.75rem;
print-color-adjust: unset;
        }
[type='checkbox'], [type='radio'] {
appearance: none;
padding: 0;
print-color-adjust: exact;
display: inline-block;
vertical-align: middle;
background-origin: border-box;
user-select: none;
flex-shrink: 0;
height: 1rem;
width: 1rem;
color: #2563eb;
background-color: #fff;
border-color: #6b7280;
border-width: 1px;
--tw-shadow: 0 0 #0000;
        }
[type='checkbox'] {
border-radius: 0px;
        }
[type='radio'] {
border-radius: 100%;
        }
[type='checkbox']:focus, [type='radio']:focus {
outline: 2px solid transparent;
outline-offset: 2px;
--tw-ring-inset: var(--tw-empty,/*!*/ /*!*/);
--tw-ring-offset-width: 2px;
--tw-ring-offset-color: #fff;
--tw-ring-color: #2563eb;
--tw-ring-offset-shadow: var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color);
--tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);
box-shadow: var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow);
        }
[type='checkbox']:checked, [type='radio']:checked {
border-color: transparent;
background-color: currentColor;
background-size: 100% 100%;
background-position: center;
background-repeat: no-repeat;
        }
[type='checkbox']:checked {
background-image: url("data:image/svg+xml,%3csvg viewBox='0 0 16 16' fill='white' xmlns='http://www.w3.org/2000/svg'%3e%3cpath d='M12.207 4.793a1 1 0 010 1.414l-5 5a1 1 0 01-1.414 0l-2-2a1 1 0 011.414-1.414L6.5 9.086l4.293-4.293a1 1 0 011.414 0z'/%3e%3c/svg%3e");
        }
[type='radio']:checked {
background-image: url("data:image/svg+xml,%3csvg viewBox='0 0 16 16' fill='white' xmlns='http://www.w3.org/2000/svg'%3e%3ccircle cx='8' cy='8' r='3'/%3e%3c/svg%3e");
        }
[type='checkbox']:checked:hover, [type='checkbox']:checked:focus, [type='radio']:checked:hover, [type='radio']:checked:focus {
border-color: transparent;
background-color: currentColor;
        }
[type='checkbox']:indeterminate {
background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 16 16'%3e%3cpath stroke='white' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M4 8h8'/%3e%3c/svg%3e");
border-color: transparent;
background-color: currentColor;
background-size: 100% 100%;
background-position: center;
background-repeat: no-repeat;
        }
[type='checkbox']:indeterminate:hover, [type='checkbox']:indeterminate:focus {
border-color: transparent;
background-color: currentColor;
        }
[type='file'] {
background: unset;
border-color: inherit;
border-width: 0;
border-radius: 0;
padding: 0;
font-size: unset;
line-height: inherit;
        }
[type='file']:focus {
outline: 1px auto -webkit-focus-ring-color;
        }`}
  />
)

;<_GlobalStyles />


