<script lang="ts">
    import type { Division, Matchup, Settings } from '$lib/models';
    import Container from '$lib/Container.svelte';
    import Section from '$lib/Section.svelte';

    export let division: Division;
    export let matchup: Matchup | null;

    function initializeMatchup() {
        matchup = {
            team1: null,
            team2: null,
            team1_score: 0,
            team2_score: 0,
            finished: false,
        };
    }
</script>

<Container>
    {#if matchup}
        <div class="matchup">
            <Section>
                <span>
                    <select size="1" bind:value={matchup.team1}>
                        <option value={null}>(none)</option>
                        {#each division.teams as team, i}
                            <option value={i}>{team.name}</option>
                        {/each}
                    </select>
                    <input type="number" bind:value={matchup.team1_score} />
                </span>
                <span>
                    <select size="1" bind:value={matchup.team2}>
                        <option value={null}>(none)</option>
                        {#each division.teams as team, i}
                            <option value={i}>{team.name}</option>
                        {/each}
                    </select>
                    <input type="number" bind:value={matchup.team2_score} />
                </span>
                <span>
                    <div>
                        <label for="finished">Finished?</label>
                        <input type="checkbox" id="finished" bind:checked={matchup.finished} />
                    </div>
                    <div class="h-space"></div>
                    <button on:click={() => matchup = null}>
                        <img src="icons/icons8-delete.svg" alt="Remove" class="remove-icon" />
                    </button>
                </span>
            </Section>
        </div>
    {:else}
        <button on:click={initializeMatchup}>
            <img src="icons/icons8-plus.svg" alt="Add" class="add-icon" />
        </button>
    {/if}
</Container>

<style>
    .matchup {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100%;
        gap: 1rem;
    }

    span {
        display: flex;
        gap: 1rem;
        align-items: center;
    }

    input {
        width: 2rem;
        margin-right: 2rem;
    }

    button {
        padding: 0.5rem;
        margin: 0;
        border: none;
        background-color: transparent;
        cursor: pointer;
        text-align: center;
    }
    
    div.h-space {
        margin-right: auto;
    }

    .add-icon {
        width: 3rem;
        height: 3rem;
        object-fit: contain;
    }

    .remove-icon {
        width: 1.5rem;
        height: 1.5rem;
        object-fit: contain;
    }
</style>
