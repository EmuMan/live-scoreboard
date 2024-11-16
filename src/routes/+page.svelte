<script lang="ts">
  import { onMount } from "svelte";
  import type { Settings, Resources, Division, Match } from '$lib/models';
  import SideNav from "./SideNav.svelte";
  import TeamsPage from "./TeamsPage.svelte";
  import CurrentMatchPage from "./CurrentMatchPage.svelte";
  import BracketPage from "./BracketPage.svelte";
  import ResourcesPage from "./ResourcesPage.svelte";
  import SettingsPage from "./SettingsPage.svelte";
  import * as api from "$lib/api";
  
  let loadedConfig: string | null;
  let settings: Settings;
  let division: Division;
  let resources: Resources;
  let currentMatch: Match;

  let sideNav: SideNav;
  let visibilities: boolean[] = Array(5).fill(false);
  visibilities[0] = true;

  async function reloadAll() {
    loadedConfig = await api.getLoadedConfig();
    settings = await api.getSettings();
    division = await api.getDivision();
    resources = await api.getResources();
    await api.correctRoundsToCount();
    currentMatch = await api.getCurrentMatch();
  }

  onMount(reloadAll);

  $: settings && api.setSettings(settings);
  $: division && api.setDivision(division);
  $: resources && api.setResources(resources);
  $: currentMatch && api.setCurrentMatch(currentMatch);
</script>

<SideNav
  bind:this={sideNav}
  buttons={[
    { name: "Teams", icon: "/icons/icons8-group.svg" },
    { name: "Current Match", icon: "/icons/icons8-battle.svg" },
    { name: "Bracket", icon: "/icons/icons8-tournament.svg" },
    { name: "Assets", icon: "/icons/icons8-image.svg" },
    { name: "Settings", icon: "/icons/icons8-settings.svg" },
  ]}
  bind:visibilities
>
  {#if visibilities[0] && division}
    <TeamsPage
      bind:division
      bind:currentMatch
      bind:settings />
  {/if}
  {#if visibilities[1] && division && currentMatch}
    <CurrentMatchPage
      bind:currentMatch
      bind:division
      bind:settings />
  {/if}
  {#if visibilities[2]}
    <BracketPage bind:division />
  {/if}
  {#if visibilities[3]}
    <ResourcesPage bind:resources />
  {/if}
  {#if visibilities[4]}
    <SettingsPage
      bind:loadedConfig
      bind:settings
      bind:division
      bind:currentMatch
      reload={reloadAll} />
  {/if}
</SideNav>
