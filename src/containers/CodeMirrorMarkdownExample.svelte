<script lang="ts">
  // NOTE Somewhere mentioned that dynamic imports might not be good?
  // import { onMount } from "svelte";
  // let c;

  // onMount(async () => {
  //   c = await (
  //     await import("$components/CodeMirror/CodeMirror.svelte")
  //   ).default;
  // });

  import CodeMirror from "$components/CodeMirror";
  import { markdown } from "@codemirror/lang-markdown";
  import { languages } from "@codemirror/language-data";
  import { oneDark } from "@codemirror/theme-one-dark";

  let value = "";
</script>

<!--
{#if c}

  <svelte:component this="{c}" bind:value />
{/if} -->

<!-- TODO #4 Remove dangerousDisableAssetCspModification=true in conf! -->
<CodeMirror
  bind:value
  theme="{oneDark}"
  lang="{markdown({ codeLanguages: languages })}"
/>
