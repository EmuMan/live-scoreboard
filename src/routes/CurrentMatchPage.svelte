<script lang="ts">
    import type { Division, Match, Settings } from '$lib/models';
    import Container from '$lib/Container.svelte';
    import Section from '$lib/Section.svelte';
    import MaybeIcon from '$lib/MaybeIcon.svelte';

    export let settings: Settings;
    export let division: Division;
    export let currentMatch: Match;

    function clearRounds() {
        currentMatch.rounds = currentMatch.rounds.map((round) => ({
            gamemode: null,
            map: null,
            team1_score: 0,
            team2_score: 0,
            completed: false,
        }));
    }
</script>

<Container>
    <h1>Current Match</h1>
    <Section>
        <p>Teams</p>
        <div class="teams">
            <Section>
                <div class="team">
                    <p>Team 1</p>
                    <select size="1" bind:value={currentMatch.team1}>
                        {#each division.teams as team, i}
                            <option value={i}>{team.name}</option>
                        {/each}
                    </select>
                    {#if currentMatch.team1 !== null}
                        <MaybeIcon
                            path={division.teams[currentMatch.team1].icon}
                            size={"5rem"} />
                    {/if}
                </div>
            </Section>
            <Section>
                <div class="team">
                    <p>Team 2</p>
                    <select size="1" bind:value={currentMatch.team2}>
                        {#each division.teams as team, i}
                            <option value={i}>{team.name}</option>
                        {/each}
                    </select>
                    {#if currentMatch.team2 !== null}
                        <MaybeIcon
                            path={division.teams[currentMatch.team2].icon}
                            size={"5rem"} />
                    {/if}
                </div>
            </Section>
        </div>
        <span>
            <label for="swapScoreboard">Swap Scoreboard?</label>
            <input type="checkbox" id="swapScoreboard" bind:checked={currentMatch.swap_scoreboard} />
        </span>
    </Section>
    <Section>
        <p>Rounds</p>
        <button on:click={clearRounds}>Clear Rounds</button>
        <div class="rounds">
            {#each currentMatch.rounds as round, i}
                {@const gamemode = round.gamemode === null ? undefined : settings.gamemodes.at(round.gamemode)}
                <Section>
                    <div class="round">
                        <p>Round {i + 1}</p>
                        <select size="1" bind:value={round.gamemode} on:change={() => round.map = null}>
                            <option value={null}>(none)</option>
                            {#each settings.gamemodes as gamemode, i}
                                <option value={i}>{gamemode.name}</option>
                            {/each}
                        </select>
                        <select size="1" bind:value={round.map}>
                            <option value={null}>(none)</option>
                            {#if gamemode !== undefined}
                                {#each gamemode.maps as map, i}
                                    <option value={i}>{map.name}</option>
                                {/each}
                            {/if}
                        </select>
                        <input type="number" bind:value={round.team1_score} />
                        <input type="number" bind:value={round.team2_score} />
                        <span>
                            <label for="completed-{i}">Completed?</label>
                            <input type="checkbox" id="completed-{i}" bind:checked={round.completed} />
                        </span>
                    </div>
                </Section>
            {/each}
        </div>
    </Section>
</Container>

<style>
    p {
        margin: 0;
    }

    .teams {
        display: flex;
        gap: 1rem;
    }
    
    .team {
        display: flex;
        flex-direction: column;
        gap: 0.7rem;
    }

    .rounds {
        display: flex;
        overflow-x: auto;
    }

    .round {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
</style>
