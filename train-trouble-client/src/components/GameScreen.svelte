<script lang="ts">
  import { onMount } from "svelte";
  import { connect } from "train-trouble-engine-client";
  import type { Channel } from "../lib/channels";
  import type { Action, Submit } from "../lib/actions";
  import type { View } from "../lib/views";
  import { createToastCollection, toastFromError } from "../lib/toasts";
  import ToastBanner from "./ToastBanner.svelte";
  import Zone from "./Zone.svelte";

  export let channel: Channel;

  let view: View | null = null;
  let connected: boolean = false;
  let submitRaw: Submit | null = null;

  let toasts = createToastCollection();
  $: toastList = toasts.toasts;

  onMount(() => {
    let connection = connect<Channel, View, Action>(
      channel,
      (v) => (view = v),
      (c) => (connected = c)
    );

    submitRaw = connection.submit;
    return connection.stop;
  });

  async function submit(action: Action) {
    try {
      await submitRaw!(action);
    } catch (error) {
      toasts.add(toastFromError(error));
    }
  }
</script>

<div class="game">
  <div class="toasts">
    {#if !connected}
      <ToastBanner toast={{ message: "Offline", type: "status" }} />
    {/if}

    {#each $toastList as toast}
      <ToastBanner {toast} dismiss={() => toasts.dismiss(toast)} />
    {/each}
  </div>

  {#if view == null || submit == null}
    <main class="loading">Načítám…</main>
  {:else if view.type == "zone"}
    <main>
      <Zone {view} {submit} />
    </main>
  {/if}
</div>

<style>
  .game {
    height: 100%;
  }

  .toasts {
    width: 100%;
    position: fixed;
    top: 0;
  }

  main {
    max-width: 20cm;
    margin: 0 auto;
    padding: 1rem 0;
  }

  .loading {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
