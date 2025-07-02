<script lang="ts">
  import SearchIcon from "./lib/SearchIcon.svelte";
  import keymap from "./keymap.svelte";
  import { app } from "./app.svelte";
  let input: HTMLInputElement;
  keymap.set("enter", () => app.print([input.value]));
  keymap.set("tab", () => app.print(["hi!", "wowowoowow"]));
  keymap.set("escape", () => app.close());
</script>

<main class="text-xl bg-transparent dark:text-white">
  <div
    class="max-h-screen w-lg rounded-xl flex flex-col items-center gap-5 bg-gray-200/80 dark:bg-gray-900/80"
  >
    <!-- svelte-ignore a11y_autofocus -->
    <label
      class="w-full p-0 h-14 border-b-1 border-b-gray-600 dark:border-b-gray-300 align-middle"
      for="search"
    >
      <SearchIcon />
      <input
        name="search"
        class="w-5/6 h-full p-0 text-3xl outline-none"
        type="text"
        bind:this={input}
        autocomplete="off"
        onfocusin={(e: FocusEvent) => (e.target as HTMLInputElement).select()}
        autofocus
      />
    </label>
    <div class="flex flex-col items-center p-1 gap-2 h-full overflow-y-scroll">
      <!-- {#each app.items as item, i} -->
      <!--   <Item -->
      <!--     index={i} -->
      <!--     data={item} -->
      <!--     selected={app.selectedIds.includes(item.id)} -->
      <!--     underCursor={i == app.cursorIndex} -->
      <!--   /> -->
      <!-- {/each} -->
    </div>
  </div>
</main>
