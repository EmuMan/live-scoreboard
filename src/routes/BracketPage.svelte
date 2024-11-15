<script lang="ts">
    import type { Division, Matchup, Settings } from '$lib/models';
    import Container from '$lib/Container.svelte';
    import Section from '$lib/Section.svelte';
    import BracketMatchup from './BracketMatchup.svelte';

    export let division: Division;
</script>

<Container>
    <h1>Bracket</h1>
    <div class="bracket">
        {#each division.bracket as stage, i}
            <Section>
                <div class="stage">
                    {#each stage as matchup}
                        <BracketMatchup bind:division bind:matchup />
                    {/each}
                </div>
                <div class="bottom-space"></div>
            </Section>
        {/each}
    </div>
</Container>

<style>
    .bracket {
        display: flex;
        flex-direction: row;
        gap: 1rem;
    }

    .stage {
        display: flex;
        flex-direction: column;
        justify-content: space-around;
        height: 100%;
    }

    div.bottom-space {
        padding-bottom: 2rem;
    }
</style>
