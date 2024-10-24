import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import type * as models from "$lib/models";

export async function startWebserver() {
    await invoke("start_webserver");
}

export async function stopWebserver() {
    await invoke("stop_webserver");
}

export async function getLoadedConfig(): Promise<string | null> {
    return await invoke("get_loaded_config");
}

export async function getSettings(): Promise<models.Settings> {
    return await invoke("get_settings");
}

export async function setSettings(settings: models.Settings) {
    await invoke("set_settings", { settings });
}

export async function getDivision(): Promise<models.Division> {
    return await invoke("get_division");
}

export async function setDivision(division: models.Division) {
    await invoke("set_division", { division });
}

export async function getAssets(): Promise<models.Asset[]> {
    return await invoke("get_assets");
}

export async function setAssets(assets: models.Asset[]) {
    await invoke("set_assets", { assets });
}

export async function getCurrentMatch(): Promise<models.Match> {
    return await invoke("get_current_match");
}

export async function setCurrentMatch(match: models.Match) {
    await invoke("set_current_match", { currentMatch: match });
}

export async function loadFromFilename(filename: string): Promise<boolean> {
    return await invoke("load_from_filename", { filename });
}

export async function saveToFilename(filename: string): Promise<boolean> {
    return await invoke("save_to_filename", { filename });
}

export async function fromRelativePath(path: string): Promise<string | null> {
    return await invoke("from_relative_path", { path });
}

export async function toRelativePath(path: string): Promise<string | null> {
    return await invoke("to_relative_path", { path });
}


export async function getUrlFromRelativePath(path: string): Promise<string | null> {
    const absolutePath = await fromRelativePath(path);
    if (absolutePath === null) {
        return null;
    }
    let url = convertFileSrc(absolutePath);
    return url;
}

export async function correctRoundsToCount(): Promise<void> {
    await invoke("correct_rounds_to_count");
}
