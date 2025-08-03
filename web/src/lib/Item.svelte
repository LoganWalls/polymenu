<script lang="ts">
  import clsx from "clsx/lite";

  export interface ItemData {
    key: string;
  }

  const {
    index,
    data,
    selected,
    underCursor,
  }: {
    index: number;
    data: ItemData;
    selected: boolean;
    underCursor: boolean;
  } = $props();
</script>

<!-- svelte-ignore a11y_mouse_events_have_key_events -->
<button
  class={clsx(
    "cursor-pointer p-2 rounded-lg w-full border-2",
    underCursor ? "bg-blue-500" : "bg-transparent",
    underCursor ? "text-white" : "text-inherit",
    selected ? "border-blue-500/50" : "border-transparent",
  )}
  onclick={() => console.log($state.snapshot(index), $state.snapshot(data))}
>
  {#each data.key as char}
    <span
      class={clsx(
        data.matchIndices?.includes(i) &&
          (underCursor ? "text-blue-300" : "text-blue-500"),
      )}
    >
      {char}
    </span>
  {/each}
</button>
