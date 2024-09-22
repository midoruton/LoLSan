import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
//import "./App.css";
import {
  ChakraProvider,
  Heading,
  FormControl,
  Input,
  Button,
  Text,
  FormLabel,
  FormErrorMessage,
  FormHelperText,
  extendTheme,
  Box,
  Flex,
  Image,
  Link,
} from "@chakra-ui/react";

const theme = extendTheme({
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

function Contents() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    console.log("greet", { name });
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <Box className="container" p={4}>
      <Heading textAlign="center">Welcome to Tauri!</Heading>

      <Flex justify="center">
        <Link href="https://vitejs.dev" target="_blank">
          <Image
            src="/vite.svg"
            alt="Vite logo"
            height="6em"
            padding="1.5em"
            will-change="filter"
            transition="0.75s"
            _hover={{
              filter: "drop-shadow(0 0 2em #747bff)",
            }}
          />
        </Link>
        <Link href="https://tauri.app" target="_blank">
          <Image
            src="/tauri.svg"
            alt="Tauri logo"
            height="6em"
            padding="1.5em"
            will-change="filter"
            transition="0.75s"
            _hover={{
              filter: "drop-shadow(0 0 2em #24c8db)",
            }}
          />
        </Link>
        <Link href="https://reactjs.org" target="_blank">
          <Image
            src={reactLogo}
            alt="React logo"
            height="6em"
            padding="1.5em"
            will-change="filter"
            transition="0.75s"
            _hover={{
              filter: "drop-shadow(0 0 2em #61dafb)",
            }}
          />
        </Link>
      </Flex>
      <form
        onSubmit={(e) => {
          console.log("submit", { name });
          e.preventDefault();
          greet();
        }}
      >
        <FormControl>
          <Flex
            margin={0}
            paddingTop="10vh"
            display="flex"
            flexDirection="column"
            justifyContent={"center"}
            textAlign={"center"}
          >
            <FormLabel >
              Click on the Tauri, Vite, and React logos to learn more.
            </FormLabel>
            <Input
              id="greet-input"
              onChange={(e) => setName(e.currentTarget.value)}
              placeholder="Enter a name..."
            />
            <Button
              type="submit"
              borderRadius="8px"
              border="1px"
              padding="0.6em 1.2em"
              fontSize="1em"
              fontWeight={500}
              fontFamily="inherit"
              color="#0f0f0f"
              backgroundColor="#ffffff"
              transition="border-color 0.25s"
              boxShadow="0 2px 2px rgba(0, 0, 0, 0.2"
            >
              Greet
            </Button>
          </Flex>
        </FormControl>
      </form>
      <Text textAlign={"center"}>{greetMsg}</Text>
    </Box>
  );
}

function App() {
  return (
    <ChakraProvider theme={theme}>
      <Contents />
    </ChakraProvider>
  );
}

export default App;
