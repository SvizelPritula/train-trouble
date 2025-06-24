<script lang="ts">
  import { hashToRoute, type Route, type Team } from "../lib/channels";
  import GameScreen from "./GameScreen.svelte";
  import MapScreen from "./MapScreen.svelte";
  import TeamSelect from "./TeamSelect.svelte";

  let hash = window.location.hash;
  let route: Route | null;
  $: route = hashToRoute[hash] ?? null;

  let team: Team | null = null;
</script>

<svelte:window on:hashchange={() => (hash = window.location.hash)} />

{#key route}
  {#if route == null}
    <main>Tato lokace neexistuje.</main>
  {:else if route.type == "map"}
    <MapScreen />
  {:else if route.type == "zone"}
    {#if team != null}
      <GameScreen zone={route.zone} {team} />
    {:else}
      <TeamSelect select={t => team = t} />
    {/if}
  {/if}
{/key}

<style>
  main {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
