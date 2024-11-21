import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
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
import Ajv from "ajv";
import schema from "./schema/AllGameData.json";
import { info, error, debug } from "@tauri-apps/plugin-log";
const ajv = new Ajv();
function Contents() {

  async function create_lol_champions_obsidian_file() {
    debug("create_lol_champions_obsidian_file");

    debug(`schema:${schema}`);
    const validate = ajv.compile(schema);
    debug(`validate:${validate}`);
    await invoke<string>("get_liveclient_data")
      .then((response) => {
        if (validate(response)) {
          info(response);
        } else {
          error(ajv.errorsText(validate.errors));
        }
      })
      .catch((e) => {
        error("Error getting live client data");
        error(e);
      });

    debug("create_lol_champions_obsidian_file_end");
  }

  async function set_obsidian_vault_path() {
    debug("set_obsidian_vault_path");
    const appConfigDirPath = await appConfigDir();
    debug(`appConfigDirPath:${appConfigDirPath}`);
    const selectedObsidianVaultPath = await openDialog({
      directory: true,
      multiple: false,
      defaultPath: appConfigDirPath,
    });
    debug(`selectedObsidianVaultPath:${selectedObsidianVaultPath}`);
    if (selectedObsidianVaultPath === null) {
      // user cancelled the selection
      info("No directory selected");
    } else if (typeof selectedObsidianVaultPath === "string") {
      // user selected a single directory
      await invoke("set_obsidian_vault_path", {
        vaultPath: selectedObsidianVaultPath,
      })
      .then((_) => {
        info("Vault path set successfully");
      })
      .catch((e) => {
        error("Error setting vault path");
        error(e);
      });
      // user selected a single directory
    }else{
      error("Error selecting vault path");
    }
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
      <Button onClick={set_obsidian_vault_path}>Dialog</Button>
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
        onClick={create_lol_champions_obsidian_file}
      >
        Greet
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
