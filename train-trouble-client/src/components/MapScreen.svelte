<script lang="ts">
  import { onMount } from "svelte";
  import { connect } from "train-trouble-engine-client";
  import type { Channel } from "../lib/channels";
  import type { MapView } from "../lib/views";
  import Map from "./Map.svelte";

  let view: MapView | null = null;
  let connected: boolean = false;

  onMount(() => {
    let connection = connect<Channel, MapView, void>(
      { type: "map" },
      (v) => (view = v),
      (c) => (connected = c)
    );

    return connection.stop;
  });
</script>

<main>
  {#if view != null}
    <Map occupied={view.occupied} />
  {/if}
</main>

<style>
  main {
    width: 100%;
    height: 100%;
    background-color: #000;
    display: flex;
  }
</style>
