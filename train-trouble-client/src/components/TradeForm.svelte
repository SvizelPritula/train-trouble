<script lang="ts">
  import type { Submit, TradeActionType } from "../lib/actions";
  import type { TrainView } from "../lib/views";
  import type { RateView } from "../lib/views";

  export let rates: RateView[];
  export let train: TrainView;
  export let submit: Submit;

  let resource: string | null;
  let amount: number | null;

  $: load = train.load!;

  function submitForm(event: SubmitEvent) {
    event.preventDefault();

    if (event.submitter instanceof HTMLButtonElement) {
      let action = event.submitter.value as TradeActionType;

      if (resource != null && Number.isInteger(amount))
        submit({
          type: "trade",
          action,
          train: train.id,
          resource,
          amount: amount!,
        });
    }
  }
</script>

<form on:submit={submitForm}>
  <table>
    <thead>
      <tr>
        <th></th>
        <th>Komodita</th>
        <th>Množství</th>
      </tr>
    </thead>

    <tbody>
      {#each rates as { id }}
        <tr>
          <td>
            <input
              type="radio"
              name="resource"
              value={id}
              required
              id={`${train.id}-${id}`}
              bind:group={resource}
            />
          </td>
          <td><label for={`${train.id}-${id}`}>{id}</label></td>
          <td>{load[id]}</td>
        </tr>
      {/each}
    </tbody>
  </table>

  <label class="amount">
    <div>Množství</div>
    <input type="number" min="0" step="1" required bind:value={amount} />
  </label>

  <div class="buttons">
    <button type="submit" name="action" value="buy">Koupit</button>
    <button type="submit" name="action" value="sell">Prodat</button>
  </div>
</form>

<style>
  table {
    width: 100%;
    border-collapse: collapse;
    margin: 1rem 0;
  }

  td,
  th {
    text-align: center;
    padding: 0.2rem;
  }

  td label {
    display: inline-block;
    width: 100%;
  }

  tr > *:first-child {
    width: 0;
  }

  thead {
    border-bottom: 0.2rem #fff solid;
  }

  .amount div {
    margin-bottom: 0.25rem;
  }

  .amount input {
    width: 100%;
    padding: 0.25rem 0.5rem;

    background-color: #1a1a1a;
    border: 0.15rem solid #666;
    border-radius: 0.25rem;

    color: inherit;
    font-size: 1rem;
  }

  .buttons {
    display: flex;
    gap: 1rem;
    margin-top: 0.5rem;
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

  button:hover {
    background-color: #282828;
  }
</style>
