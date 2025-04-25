<script lang="ts">
  import {
    type AzikConfig,
    type TokSeqSet,
    gen_google_ime_table_runner,
  } from "$lib/wasm/Core";
  import init from "$lib/wasm/Core";

  let tokens = $state("");
  let sequences = $state("");
  let tokSeqSet = $derived.by<TokSeqSet[]>(() => {
    let tokensArray = tokens.split(" ");
    let sequencesArray = sequences.split(" ");
    let out: TokSeqSet[] = Array<TokSeqSet>(0);

    if (tokensArray.length === sequencesArray.length) {
      let sequenceCount = tokensArray.length;
      for (let i = 0; i < sequenceCount; i++) {
        out.push({ Token: tokensArray[i], Sequence: sequencesArray[i] });
      }
    }
    return out;
  });

  let hatsuon = $state("");
  let sokuon = $state("");
  let azikConfig = $derived<AzikConfig>({
    Sequence: tokSeqSet,
    Hatsuon: hatsuon,
    Sokuon: sokuon,
  });
  let out = $state("");
  let downloadGoogleIME = $derived(
    `data:text/plain;charset=utf-8,${encodeURIComponent(out)}`,
  );

  const googleImeHandler = async () => {
    await init();
    out = gen_google_ime_table_runner(azikConfig);
  };

  const resetAllHandler = () => {
    tokens = "";
    sequences = "";
    hatsuon = "";
    sokuon = "";
    out = "";
  };
</script>

<form class="space-y-4">
  <label class="label">
    <span class="label-text h5">割り当て先</span>
    <hr class="hr border-surface-500 pb-5" />
    <input class="input" type="text" bind:value={tokens} />
  </label>
  <label class="label">
    <span class="label-text h5">シーケンス</span>
    <hr class="hr border-surface-500 pb-5" />
    <input class="input" type="text" bind:value={sequences} />
  </label>

  <label class="label">
    <span class="label-text h5">撥音</span>
    <hr class="hr border-surface-500 pb-5" />
    <input class="input" type="text" bind:value={hatsuon} />
  </label>
  <label class="label">
    <span class="label-text h5">促音</span>
    <hr class="hr border-surface-500 pb-5" />
    <input class="input" type="text" bind:value={sokuon} />
  </label>
  <div class="flex flex-col items-center justify-center space-y-16">
    <div class="flex items-center justify-center space-x-4">
      {#if out === ""}
        <button
          class="btn preset-filled-primary-500 h-12"
          onclick={googleImeHandler}>Create4GoogleIME</button
        >
      {:else}
        <a
          class="btn preset-filled-primary-500 w-42 h-12"
          href={downloadGoogleIME}
          download="AzikForGoogleIme.txt">Download</a
        >
        <button
          class="btn preset-filled-primary-500 w-42 h-12"
          onclick={resetAllHandler}>ResetParams</button
        >
      {/if}
    </div>
  </div>
</form>
