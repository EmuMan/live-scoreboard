<script lang="ts">
    import { convertFileSrc } from "@tauri-apps/api/core";
    import type { Division, Match, Player, Settings, Team } from '$lib/models';
    import EditableList from '$lib/EditableList.svelte';
    import Section from '$lib/Section.svelte';
    import Container from '$lib/Container.svelte';
    import * as api from '$lib/api';
    import { correctIndex } from "$lib/util";
    import { newDropdownField, newFileField, newTextField, type FilledModalFields } from "$lib/ModalForm.svelte";

    export let division: Division;
    export let currentMatch: Match;
    export let settings: Settings;

    let selectedTeam: Team | undefined;

    function teamFromFilledFields(fields: FilledModalFields, oldItem?: Team): Team {
        return {
            name: fields["Name"] as string,
            icon: fields["Icon"] as string | null,
            players: oldItem?.players ?? [],
        };
    }

    function playerFromFilledFields(fields: FilledModalFields, oldItem?: Player): Player {
        return {
            name: fields["Name"] as string,
            role: fields["Role"] as number | null,
            character: fields["Character"] as number | null,
        };
    }
</script>

<Container>
    <h1>Teams</h1>
    <Section>
        <p>Teams</p>
        <EditableList
            items={division.teams}
            itemTemplate={async (team) => {
                if (team.icon === null) {
                    return team.name;
                }
                const absolutePath = await api.fromRelativePath(team.icon);
                if (absolutePath === null) {
                    return `${team.name} <span style="color: red;">Icon not found</span>`;
                }
                const path = convertFileSrc(absolutePath);
                return `${team.name} <img src=${path} alt=${team.name} style="max-width: 1rem; max-height: 1rem;" />`;
            }}
            bind:selectedItem={selectedTeam}
            onUpdate={(items, from, to) => {
                division.teams = items;
                if (from === undefined) return;
                if (currentMatch.team1 !== null) {
                    currentMatch.team1 = correctIndex(currentMatch.team1, from, to);
                }
                if (currentMatch.team2 !== null) {
                    currentMatch.team2 = correctIndex(currentMatch.team2, from, to);
                }
                division.bracket.forEach((round) => {
                    round.forEach((matchup) => {
                        if (matchup === null) return;
                        if (matchup.team1 !== null) {
                            matchup.team1 = correctIndex(matchup.team1, from, to);
                        }
                        if (matchup.team2 !== null) {
                            matchup.team2 = correctIndex(matchup.team2, from, to);
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
            fromFilledFields={teamFromFilledFields}
            height={"10rem"} />
        <br><br>
        <p>Players</p>
        <EditableList
            items={selectedTeam?.players ?? []}
            itemTemplate={async (player) => {
                const role = player.role === null ? "(none)" : settings.roles[player.role].name;
                const character = player.character === null ? "(none)" : settings.characters[player.character].name;
                return `${player.name} | ${role} | ${character}`;
            }}
            onUpdate={(items) => {
                if (selectedTeam) {
                    selectedTeam.players = items;
                    division.teams = division.teams;
                }
            }}
            fields={[
                newTextField("Name", true),
                newDropdownField("Role", false, settings.roles.map((role) => role.name)),
                newDropdownField("Character", false, settings.characters.map((character) => character.name)),
            ]}
            toFilledFields={(item) => {
                return {
                    "Name": item.name,
                    "Role": item.role,
                    "Character": item.character,
                };
            }}
            fromFilledFields={playerFromFilledFields}
            height={"10rem"} />
    </Section>
</Container>

<style>
    p {
        margin-top: 0;
    }
</style>
