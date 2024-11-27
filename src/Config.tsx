import { extendTheme } from "@chakra-ui/react";
export const theme = extendTheme({
    styles: {
      global: {
        "html, body": {
          fontFamily: "Inter, Avenir, Helvetica, Arial, sans-serif",
          fontSize: "16px",
          lineHeight: "24px",
          fontWeight: "400",
          color: "#0f0f0f",
          backgroundColor: "#f6f6f6",
          WebkitFontSmoothing: "antialiased",
          MozOsxFontSmoothing: "grayscale",
          textRendering: "optimizeLegibility",
          fontSynthesis: "none",
          WebkitTextSizeAdjust: "100%",
        },
      },
    },
  });
  ////https://v2.tauri.app/plugin/logging/
  //function forwardConsole(
  //  fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
  //  logger: (message: string) => Promise<void>
  //) {
  //  const original = console[fnName];
  //  console[fnName] = (message) => {
  //    original(message);
  //    logger(message);
  //  };
  //}
  //
  //forwardConsole('log', trace);
  //forwardConsole('debug', debug);
  //forwardConsole('info', info);
  //forwardConsole('warn', warn);
  //forwardConsole('error', error);