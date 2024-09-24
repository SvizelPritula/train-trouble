<script lang="ts">
  import { onMount } from "svelte";
  import { type Action, type Channel, type View } from "../lib/state";
  import { connect } from "train-trouble-engine-client";

  export let channel: Channel;

  let state: View | null = null;
  let submit: (action: Action) => Promise<void>;

  let action: string = "";

  onMount(() => {
    let connection = connect<Channel, View, Action>(channel, (v) => (state = v));
    submit = connection.submit;

    return connection.stop;
  });

  function submitAction() {
    submit(JSON.parse(action)).catch((e) => alert(e.message));
  }
</script>

<main>
  <pre>{JSON.stringify(state, null, 4)}</pre>

  <form on:submit|preventDefault={submitAction}>
    <textarea bind:value={action} />

    <div><button type="submit">Submit</button></div>
  </form>
</main>
