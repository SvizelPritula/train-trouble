<script lang="ts">
  import { teams, type Team } from "../lib/channels";

  export let select: (team: Team) => void;

  let team: Team | null;

  function submit() {
    select(team!);
  }
</script>

<svelte:head>
  <title>Výběr týmu | Train Trouble</title>
</svelte:head>

<main>
  <h1>Výběr týmu</h1>

  {#each teams as { id, name }}
    <label>
      <input type="radio" name="team" value={id} required bind:group={team} />
      {name}
    </label>
  {/each}

  <button disabled={team == null} on:click|preventDefault={submit}>
    Přihlásit
  </button>
</main>

<style>
  main {
    max-width: 15cm;
    margin: 0 auto;
    padding: 0 1rem;
  }

  h1 {
    margin: 2rem 0;
  }

  label {
    display: block;
    min-height: 2.5rem;
    font-size: 1.25rem;
  }

  button {
    width: 100%;
    padding: 0.25rem 0.5rem;

    background-color: #333;
    border: 0.15rem solid #666;
    border-radius: 0.25rem;

    color: inherit;
    font-size: 1rem;
  }

  button:hover:enabled {
    background-color: #282828;
  }

  button:disabled {
    color: #bbb;
    background-color: #444;
  }
</style>
