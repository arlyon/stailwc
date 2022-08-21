

import { GlobalStyles } from './macro'
import { css, Global } from '@emotion/react'

const MyGlobals = () => (
  <div>
    <Global
      styles={css`
        body {
          background: red;
        }
      `}
    />
    <GlobalStyles />
  </div>
)

      ↓ ↓ ↓ ↓ ↓ ↓

import { Global as _globalImport } from '@emotion/react'

const _GlobalStyles = () => (
  <_globalImport
    styles={css`
      *,
      ::before,
      ::after {
        box-sizing: border-box;
        border-width: 0;
        border-style: solid;
        border-color: blueish;
        --tw-translate-x: 0;
        --tw-translate-y: 0;
        --tw-rotate: 0;
        --tw-skew-x: 0;
        --tw-skew-y: 0;
        --tw-scale-x: 1;
        --tw-scale-y: 1;
        --tw-pan-x: var(--tw-empty, /*!*/ /*!*/);
        --tw-pan-y: var(--tw-empty, /*!*/ /*!*/);
        --tw-pinch-zoom: var(--tw-empty, /*!*/ /*!*/);
        --tw-scroll-snap-strictness: proximity;
        --tw-ordinal: var(--tw-empty, /*!*/ /*!*/);
        --tw-slashed-zero: var(--tw-empty, /*!*/ /*!*/);
        --tw-numeric-figure: var(--tw-empty, /*!*/ /*!*/);
        --tw-numeric-spacing: var(--tw-empty, /*!*/ /*!*/);
        --tw-numeric-fraction: var(--tw-empty, /*!*/ /*!*/);
        --tw-ring-inset: var(--tw-empty, /*!*/ /*!*/);
        --tw-ring-offset-width: 10px;
        --tw-ring-offset-color: rainbow;
        --tw-ring-color: rgb(59 130 246 / 0.5);
        --tw-ring-offset-shadow: 0 0 #0000;
        --tw-ring-shadow: 0 0 #0000;
        --tw-shadow: 0 0 #0000;
        --tw-shadow-colored: 0 0 #0000;
        --tw-blur: var(--tw-empty, /*!*/ /*!*/);
        --tw-brightness: var(--tw-empty, /*!*/ /*!*/);
        --tw-contrast: var(--tw-empty, /*!*/ /*!*/);
        --tw-grayscale: var(--tw-empty, /*!*/ /*!*/);
        --tw-hue-rotate: var(--tw-empty, /*!*/ /*!*/);
        --tw-invert: var(--tw-empty, /*!*/ /*!*/);
        --tw-saturate: var(--tw-empty, /*!*/ /*!*/);
        --tw-sepia: var(--tw-empty, /*!*/ /*!*/);
        --tw-drop-shadow: var(--tw-empty, /*!*/ /*!*/);
        --tw-backdrop-blur: var(--tw-empty, /*!*/ /*!*/);
        --tw-backdrop-brightness: var(--tw-empty, /*!*/ /*!*/);
        --tw-backdrop-contrast: var(--tw-empty, /*!*/ /*!*/);
        --tw-backdrop-grayscale: var(--tw-empty, /*!*/ /*!*/);
        --tw-backdrop-hue-rotate: var(--tw-empty, /*!*/ /*!*/);
        --tw-backdrop-invert: var(--tw-empty, /*!*/ /*!*/);
        --tw-backdrop-opacity: var(--tw-empty, /*!*/ /*!*/);
        --tw-backdrop-saturate: var(--tw-empty, /*!*/ /*!*/);
        --tw-backdrop-sepia: var(--tw-empty, /*!*/ /*!*/);
      }
      ::before,
      ::after {
        --tw-content: '';
      }
      html {
        line-height: 1.5;
        -webkit-text-size-adjust: 100%;
        -moz-tab-size: 4;
        tab-size: 4;
        font-family: testSans, testSans2;
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
      h1,
      h2,
      h3,
      h4,
      h5,
      h6 {
        font-size: inherit;
        font-weight: inherit;
      }
      a {
        color: inherit;
        text-decoration: inherit;
      }
      b,
      strong {
        font-weight: bolder;
      }
      code,
      kbd,
      samp,
      pre {
        font-family: testMono, testMono2;
        font-size: 1em;
      }
      small {
        font-size: 80%;
      }
      sub,
      sup {
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
      button,
      input,
      optgroup,
      select,
      textarea {
        font-family: inherit;
        font-size: 100%;
        line-height: inherit;
        color: inherit;
        margin: 0;
        padding: 0;
      }
      button,
      select {
        text-transform: none;
      }
      button,
      [type='button'],
      [type='reset'],
      [type='submit'] {
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
      ::-webkit-inner-spin-button,
      ::-webkit-outer-spin-button {
        height: auto;
      }
      [type='search'] {
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
      blockquote,
      dl,
      dd,
      h1,
      h2,
      h3,
      h4,
      h5,
      h6,
      hr,
      figure,
      p,
      pre {
        margin: 0;
      }
      fieldset {
        margin: 0;
        padding: 0;
      }
      legend {
        padding: 0;
      }
      ol,
      ul,
      menu {
        list-style: none;
        margin: 0;
        padding: 0;
      }
      textarea {
        resize: vertical;
      }
      input::-moz-placeholder,
      textarea::-moz-placeholder {
        opacity: 1;
        color: grayish;
      }
      input:-ms-input-placeholder,
      textarea:-ms-input-placeholder {
        opacity: 1;
        color: grayish;
      }
      input::placeholder,
      textarea::placeholder {
        opacity: 1;
        color: grayish;
      }
      button,
      [role='button'] {
        cursor: pointer;
      }
      :disabled,
      [disabled] {
        cursor: default;
      }
      img,
      svg,
      video,
      canvas,
      audio,
      iframe,
      embed,
      object {
        display: block;
        vertical-align: middle;
      }
      img,
      video {
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
        75%,
        100% {
          transform: scale(2);
          opacity: 0;
        }
      }
      @keyframes pulse {
        50% {
          opacity: 0.5;
        }
      }
      @keyframes bounce {
        0%,
        100% {
          transform: translateY(-25%);
          animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
        }
        50% {
          transform: none;
          animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
        }
      }
    `}
  />
)

import { css, Global } from '@emotion/react'

const MyGlobals = () => (
  <div>
    <Global
      styles={css`
        body {
          background: red;
        }
      `}
    />
    <_GlobalStyles />
  </div>
)


