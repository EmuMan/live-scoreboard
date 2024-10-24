<script lang="ts">
  import { onMount } from "svelte";
  import type { Settings, Asset, Division, Match } from '$lib/models';
  import SideNav from "./SideNav.svelte";
  import TeamsPage from "./TeamsPage.svelte";
  import CurrentMatchPage from "./CurrentMatchPage.svelte";
  import BracketPage from "./BracketPage.svelte";
  import AssetsPage from "./AssetsPage.svelte";
  import SettingsPage from "./SettingsPage.svelte";
  import * as api from "$lib/api";
  
  let loadedConfig: string | null;
  let settings: Settings;
  let division: Division;
  let assets: Asset[] = [];
  let currentMatch: Match;

  let sideNav: SideNav;
  let visibilities: boolean[] = Array(5).fill(false);
  visibilities[0] = true;

  async function reloadAll() {
    loadedConfig = await api.getLoadedConfig();
    settings = await api.getSettings();
    division = await api.getDivision();
    assets = await api.getAssets();
    await api.correctRoundsToCount();
    currentMatch = await api.getCurrentMatch();
  }

  onMount(reloadAll);

  $: settings && api.setSettings(settings);
  $: division && api.setDivision(division);
  $: assets && api.setAssets(assets);
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
  bind:visibilities={visibilities}
>
  {#if visibilities[0] && division}
    <TeamsPage
      bind:division={division}
      bind:currentMatch={currentMatch}
      bind:settings={settings} />
  {/if}
  {#if visibilities[1] && division && currentMatch}
    <CurrentMatchPage
      bind:currentMatch={currentMatch}
      bind:division={division}
      bind:settings={settings} />
  {/if}
  {#if visibilities[2]}
    <BracketPage />
  {/if}
  {#if visibilities[3]}
    <AssetsPage bind:assets={assets} />
  {/if}
  {#if visibilities[4]}
    <SettingsPage
      bind:loadedConfig={loadedConfig}
      bind:settings={settings}
      bind:division={division}
      bind:currentMatch={currentMatch}
      reload={reloadAll} />
  {/if}
</SideNav>
