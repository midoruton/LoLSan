import { info, error, debug } from "@tauri-apps/plugin-log";
import { invoke } from "@tauri-apps/api/core";

export  async function set_obsidian_vault_path(selectedObsidianVaultPath: string) {
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
}

export async function start_get_liveclient_data_loop(){
    debug("start_get_liveclient_data_loop");
    invoke("start_get_liveclient_data_loop",{});
}