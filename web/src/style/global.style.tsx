import { colorVariablesCSS } from "~/style/colorVariableCSS";
import {
  color,
  heading1Text,
  primaryFontBold,
  primaryFontBoldItalic,
  primaryFontItalic,
  text,
} from "~/style/commonStyle";
import "~/style/fontPreamble.style";
import "~/style/layerPreamble.style";
import "~/style/reset.style";
import "~/style/tw.style";
import { css } from "@linaria/core";
import { baseText } from "~/style/textStylesTS";

export const globals = css`
  html {
    ${colorVariablesCSS}
  }
  @layer root {
    html {
      ${baseText}
    }

    a,
    span {
      // display: inline-block;
    }

    a,
    a:visited,
    a:hover,
    a:active {
      color: ${color("colors/primary-800")};
    }

    a:hover {
      text-decoration: underline;
    }

    button {
      font-size: inherit;
    }

    * {
      box-sizing: border-box;
    }

    body {
      min-height: 100vh;
      background: ${color("colors/special-bg")};
      overflow-x: hidden;

      line-height: 1.5;
      -webkit-font-smoothing: antialiased;
      -moz-osx-font-smoothing: grayscale;
      text-rendering: optimizeLegibility;

      ${text("primary", "body", "colors/text-600a")}
    }

    em {
      ${primaryFontItalic}
      strong {
        ${primaryFontBoldItalic}
      }
    }

    strong {
      color: ${color("colors/text-900a")};
      ${primaryFontBold}
    }

    #root {
      min-height: 100vh;
      display: flex;
      flex-direction: column;
    }

    h1 {
      ${heading1Text};
    }

    html {
      // firefox-only for now (https://developer.mozilla.org/en-US/docs/Web/CSS/scrollbar-color)
      scrollbar-color: ${color("colors/primary-700")}
        ${color("colors/special-bg")};
      overflow-y: overlay;
    }

    /* hide arrows on number inputs */
    input::-webkit-outer-spin-button,
    input::-webkit-inner-spin-button {
      -webkit-appearance: none;
      margin: 0;
    }
    input[type="number"] {
      -moz-appearance: textfield;
    }

    ::-webkit-scrollbar {
      width: 12px;
    }
    ::-webkit-scrollbar-track {
      border-radius: 4px;
      background: none;
    }
    ::-webkit-scrollbar-thumb {
      width: 4px;
      background: ${color("colors/primary-700")};
      border-left: solid transparent;
      border-left-width: 8px;
      background-clip: padding-box;
      transition: all 0.5s ease-in-out;
      &:hover {
        width: 12px;
        border-left-width: 0;
      }
    }
  }
`;
