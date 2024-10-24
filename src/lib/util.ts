import { open, save, type DialogFilter } from "@tauri-apps/plugin-dialog";

export async function openDialog(filter?: DialogFilter): Promise<string | null> {
    return await open({
        multiple: false,
        directory: false,
        filter,
    });
}

export async function saveDialog(
    filters: DialogFilter[],
    defaultPath: string | undefined = undefined
): Promise<string | null> {
    return await save({
        defaultPath,
        filters,
    });
}

export function correctIndex(index: number, from: number, to?: number): number | null {
    if (to === undefined) {
        if (index === from) {
            return null;
        } else if (index > from) {
            return index - 1;
        } else {
            return index;
        }
    } else {
        if (index === from) {
            return to;
        } else if (index > from && index <= to) {
            return index - 1;
        } else if (index < from && index >= to) {
            return index + 1;
        } else {
            return index;
        }
    }
}
