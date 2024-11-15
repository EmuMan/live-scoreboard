<script lang="ts">
    import { convertFileSrc, invoke } from "@tauri-apps/api/core";
    import { open, save } from "@tauri-apps/plugin-dialog";
    import type { Settings, Gamemode, Match, Map, Division, Role, Character } from '$lib/models';
    import EditableList from "$lib/EditableList.svelte";
    import Section from "$lib/Section.svelte";
    import Container from "$lib/Container.svelte";
    import { correctIndex, openDialog, saveDialog } from "$lib/util";
    import type { FilledModalFields } from "$lib/ModalForm.svelte";
    import { newFileField, newTextField } from "$lib/ModalForm.svelte";
    import { correctRoundsToCount, correctBracketToCount, getCurrentMatch, loadFromFilename, saveToFilename, startWebserver, stopWebserver } from "$lib/api";
    import { tick } from "svelte";
    import * as api from "$lib/api";

    export let loadedConfig: string | null;
    export let settings: Settings;
    export let division: Division;
    export let currentMatch: Match;
    export let reload: () => Promise<void>;

    let selectedGamemode: Gamemode | undefined;
    let selectedGamemodeIndex: number | undefined;

    let roundCount: number = settings.round_count;
    let bracketStageCount: number = settings.bracket_stage_count;
    let isWebserverRunning = false;

    $: {
        settings.round_count = roundCount;
        tick().then(() => {
            correctRoundsToCount().then(() => {
                getCurrentMatch().then((match) => {
                    currentMatch = match;
                });
            });
        });
    }

    $: {
        settings.bracket_stage_count = bracketStageCount;
        tick().then(() => {
            correctBracketToCount().then(() => {
                api.getDivision().then((div) => {
                    division = div;
                });
            });
        });
    }

    async function loadConfig() {
        const filename = await openDialog({
            name: "JSON",
            extensions: ["json"],
        });
        if (filename) {
            await loadFromFilename(filename);
            await reload();
        }
    }

    async function saveConfig() {
        const filename = await saveDialog([
            {
                name: "JSON",
                extensions: ["json"],
            }
        ], loadedConfig ?? undefined);
        if (filename) {
            await saveToFilename(filename);
            await reload();
        }
    }


    // so ts doesn't yell at me

    function gamemodeFromFilledFields(fields: FilledModalFields, oldItem?: Gamemode): Gamemode {
        return {
            name: fields["Name"] as string,
            icon: fields["Icon"] as string | null,
            maps: oldItem ? oldItem.maps : [],
        };
    }

    function mapFromFilledFields(fields: FilledModalFields, oldItem?: Map): Map {
        return {
            name: fields["Name"] as string,
            image: fields["Image"] as string | null,
        };
    }

    function roleFromFilledFields(fields: FilledModalFields, oldItem?: Role): Role {
        return {
            name: fields["Name"] as string,
            icon: fields["Icon"] as string | null,
        };
    }

    function characterFromFilledFields(fields: FilledModalFields, oldItem?: Character): Character {
        return {
            name: fields["Name"] as string,
            image: fields["Image"] as string | null,
        };
    }
</script>

<Container>
    <h1>Settings</h1>

    <Section>
        {#if loadedConfig}
            <p>Loaded config: <em>{loadedConfig}</em></p>
        {:else}
            <p>No config loaded</p>
        {/if}
    
        <button on:click={loadConfig}>Load</button>
        <button on:click={saveConfig}>Save</button>
    </Section>

    <Section>
        {#if isWebserverRunning}
            <p>Webserver is running!</p>
        {:else}
            <p>Webserver is stopped.</p>
        {/if}

        <button on:click={async () => {
            isWebserverRunning = true;
            await startWebserver();
        }} disabled={isWebserverRunning}>Start Webserver</button>
        <button on:click={async () => {
            isWebserverRunning = false;
            await stopWebserver();
        }} disabled={!isWebserverRunning}>Stop Webserver</button>
    </Section>

    <Section>
        <span>
            Event Name
            <input type="text" bind:value={settings.event_name}>
        </span>
        <br><br>
        <span>
            Round Count
            <input type="number" bind:value={roundCount}>
        </span>
        <br><br>
        <span>
            Bracket Stage Count
            <input type="number" bind:value={bracketStageCount}>
        </span>
    </Section>

    <Section>
        <p>Gamemodes</p>
        <EditableList
            items={settings.gamemodes}
            itemTemplate={async (gamemode) => gamemode.name}
            bind:selectedItem={selectedGamemode}
            bind:selectedIndex={selectedGamemodeIndex}
            onUpdate={(items, from, to) => {
                settings.gamemodes = items;
                if (from === undefined) return;
                currentMatch.rounds.forEach((round) => {
                    if (round.gamemode !== null) {
                        const newIndex = correctIndex(round.gamemode, from, to);
                        if (newIndex !== round.gamemode) {
                            if (newIndex === null) {
                                round.map = null;
                            }
                            round.gamemode = newIndex;
                        }
                    }
                });
            }}
            fields={[
                newTextField("Name", true),
                newFileField("Icon", false),
            ]}
            toFilledFields={(item) => {
                return {
                    "Name": item.name,
                    "Icon": item.icon,
                };
            }}
            fromFilledFields={gamemodeFromFilledFields}
            height="8rem" />
        <br><br>
        <p>Maps</p>
        <EditableList
            items={selectedGamemode?.maps ?? []}
            itemTemplate={async (map) => map.name}
            onUpdate={(items, from, to) => {
                if (selectedGamemode === undefined) return;
                selectedGamemode.maps = items;
                if (from === undefined) return;
                currentMatch.rounds.forEach((round) => {
                    if (round.gamemode === selectedGamemodeIndex && round.map !== null) {
                        round.map = correctIndex(round.map, from, to);
                    }
                });
            }}
            fields={[
                newTextField("Name", true),
                newFileField("Image", false),
            ]}
            toFilledFields={(item) => {
                return {
                    "Name": item.name,
                    "Image": item.image,
                };
            }}
            fromFilledFields={mapFromFilledFields}
            height="8rem" />
    </Section>

    <Section>
        <p>Roles</p>
        <EditableList
            items={settings.roles}
            itemTemplate={async (role) => role.name}
            onUpdate={(items, from, to) => {
                settings.roles = items;
                if (from === undefined) return;
                division.teams.forEach((team) => {
                    team.players.forEach((player) => {
                        if (player.role !== null) {
                            player.role = correctIndex(player.role, from, to);
                        }
                    });
                });
            }}
            fields={[
                newTextField("Name", true),
                newFileField("Icon", false),
            ]}
            toFilledFields={(item) => {
                return {
                    "Name": item.name,
                    "Icon": item.icon,
                };
            }}
            fromFilledFields={roleFromFilledFields}
            height="8rem" />
    </Section>

    <Section>
        <p>Characters</p>
        <EditableList
            items={settings.characters}
            itemTemplate={async (character) => {
                if (character.image === null) {
                    return character.name;
                }
                const absolutePath = await api.fromRelativePath(character.image);
                if (absolutePath === null) {
                    return `${character.name} <span style="color: red;">Image not found</span>`;
                }
                const path = convertFileSrc(absolutePath);
                return `${character.name} <img src=${path} alt=${character.name} style="max-width: 1rem; max-height: 1rem; padding-left: 1rem;" />`;
            }}
            onUpdate={(items, from, to) => {
                settings.characters = items;
                if (from === undefined) return;
                division.teams.forEach((team) => {
                    team.players.forEach((player) => {
                        if (player.character !== null) {
                            player.character = correctIndex(player.character, from, to);
                        }
                    });
                });
            }}
            fields={[
                newTextField("Name", true),
                newFileField("Image", false),
            ]}
            toFilledFields={(item) => {
                return {
                    "Name": item.name,
                    "Image": item.image,
                };
            }}
            fromFilledFields={characterFromFilledFields}
            height="8rem" />
    </Section>
</Container>

<style>
    p {
        margin-top: 0;
    }
</style>
