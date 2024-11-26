import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { appConfigDir } from "@tauri-apps/api/path";
import {
  ChakraProvider,
  Heading,
  Button,
  Text,
  Box,
  Flex,
  Image,
  Link,
} from "@chakra-ui/react";
import { theme } from "./Config.tsx";
import schema from "./schema/AllGameData.json";
import { info, error, debug } from "@tauri-apps/plugin-log";
import {configure_obsidian_valut_path} from "./logic.ts";
import  {start_get_liveclient_data_loop} from "./command.ts";
function Contents() {

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
      <Button onClick={configure_obsidian_valut_path}>Dialog</Button>
      <Button
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
        onClick={start_get_liveclient_data_loop}
      >
        Start Getting LiveData Loop
      </Button>
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
