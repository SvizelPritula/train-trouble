<script lang="ts">
  import { onMount } from "svelte";
  import { type Channel, type View } from "../lib/state";
  import { connect } from "../engine";

  let state: View | null = null;

  onMount(() => {
    let { stop } = connect<Channel, View>("view", (v) => (state = v));

    return stop;
  });
</script>

<main><pre>{JSON.stringify(state, null, 4)}</pre></main>
