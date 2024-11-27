

import { info, error, debug } from "@tauri-apps/plugin-log";
import {set_obsidian_vault_path} from "./command.ts";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { appConfigDir } from "@tauri-apps/api/path";

export async function configure_obsidian_valut_path() {
    debug("configure_obsidian_vault_path");
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
      set_obsidian_vault_path(selectedObsidianVaultPath);
      // user selected a single directory
    }else{
      error("Error selecting vault path");
    }
}

