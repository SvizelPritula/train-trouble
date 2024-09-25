<script lang="ts">
  import { type Channel } from "../lib/state";
  import Simulator from "./Simulator.svelte";

  let channel: Channel = { type: "map" };
  let channelName: string = JSON.stringify(channel);

  function submit() {
    channel = JSON.parse(channelName) as Channel;
  }
</script>

<main>
  <form on:submit|preventDefault={submit}>
    <input bind:value={channelName} />

    <div><button type="submit">Submit</button></div>
  </form>

  {#key channel}
    <Simulator {channel} />
  {/key}
</main>
