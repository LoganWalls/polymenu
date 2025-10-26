<script lang="ts">
  import clsx from "clsx/lite";
  import type { ItemData } from "../common";
  import { submit } from "../common";
  import * as util from "polymenu/util";
  import FallbackIcon from "./FallbackIcon.svelte";

  let {
    index,
    data,
    selected,
    lastItem,
    selectedItems = $bindable(),
    cursorIndex = $bindable(),
  }: {
    index: number;
    data: ItemData;
    selected: boolean;
    lastItem: boolean;
    selectedItems: ItemData[];
    cursorIndex: number;
  } = $props();
  let imageMissing = $state(false);
  let underCursor = $derived(index == cursorIndex);
</script>

<!-- svelte-ignore a11y_mouse_events_have_key_events -->
<button
  class={clsx(
    "cursor-pointer p-2 w-full border-2",
    underCursor ? "bg-blue-500" : "bg-transparent",
    underCursor ? "text-white" : "text-inherit",
    selected ? "border-blue-500/50" : "border-transparent",
    underCursor && lastItem && "rounded-b-xl",
  )}
  onmouseover={() => {
    cursorIndex = index;
  }}
  onclick={(e) => {
    if (e.shiftKey) {
      util.toggleSet(data, selectedItems);
    } else {
      submit(data, selectedItems);
    }
  }}
>
  <div class="flex place-items-center gap-3">
    {#if data.icon !== null && data.icon !== undefined}
      {#if !imageMissing}
        <img
          class="h-10"
          src={`/files/icons/${data.icon}`}
          alt="Icon"
          onerror={() => (imageMissing = true)}
        />
      {:else}
        <FallbackIcon />
      {/if}
    {/if}
    <span>
      {#each data.key as char, i}
        <span
          class={clsx(
            data.matchIndices
              ?.map(([low, high]) => low <= i && i <= high)
              .reduce((a, b) => a || b) &&
              (underCursor ? "text-blue-300" : "text-blue-500"),
          )}
        >
          {char}
        </span>
      {/each}
    </span>
  </div>
</button>
