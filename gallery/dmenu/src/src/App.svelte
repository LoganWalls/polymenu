<script lang="ts">
  import { app, keymap } from "./globalState";
  import * as util from "polymenu/util";

  import Fuse from "fuse.js";
  import SearchIcon from "./lib/SearchIcon.svelte";
  import Item from "./lib/Item.svelte";
  import { submit, type ItemData } from "./common";

  let selectedItems: ItemData[] = $state([]);
  let cursorIndex = $state(0);
  let items: ItemData[] = $state([]);
  let allItems: ItemData[] = $state([]);
  const itemsVisibleAtMaxHeight = 10;

  let fusePromise = (async () => {
    // Get inputs from the CLI
    const inputValues = await app.input<ItemData | string[]>();
    // If they are not objects (e.g. if they are headless CSV rows),
    // assume the first item in each row is the key
    if (inputValues.length > 0 && Array.isArray(inputValues[0])) {
      for (let i = 0; i < inputValues.length; i++) {
        const row = inputValues[i] as string[];
        inputValues[i] = { key: row[0] };
      }
    }
    allItems = inputValues as ItemData[];
    items = allItems;
    return new Fuse(items, {
      keys: ["key"],
      isCaseSensitive: app.options.case_sensitive as boolean,
      includeMatches: app.options.highlight_matches as boolean,
    });
  })();

  function cursorUp() {
    cursorIndex = util.wrappingShift(cursorIndex, -1, 0, items.length - 1);
  }
  function cursorDown() {
    cursorIndex = util.wrappingShift(cursorIndex, 1, 0, items.length - 1);
  }

  keymap.set("ctrl+j", cursorDown);
  keymap.set("arrowdown", cursorDown);
  keymap.set("ctrl+k", cursorUp);
  keymap.set("arrowup", cursorUp);
  keymap.set("ctrl+l", () => {
    selectedItems = [];
  });
  keymap.set("escape", app.close);
  keymap.set("ctrl+d", app.close);
  keymap.set("tab", () => {
    selectedItems = util.toggleSet(items[cursorIndex], selectedItems);
  });
  keymap.set("enter", () => {
    submit(items[cursorIndex], selectedItems);
  });
</script>

<main class="text-xl bg-transparent dark:text-white">
  <div
    class="flex flex-col max-h-screen w-full rounded-xl bg-gray-200/80 dark:bg-gray-900/80"
  >
    {#await fusePromise}
      <p>Loading...</p>
    {:then fuse}
      <!-- svelte-ignore a11y_autofocus -->
      <label class="w-full p-0 h-14 align-middle" for="search">
        <SearchIcon />
        <input
          name="search"
          class="w-5/6 h-full p-0 text-3xl outline-none placeholder:text-gray-800 dark:placeholder:text-gray-300"
          type="text"
          autocomplete="off"
          placeholder={app.options.placeholder as string}
          onfocusin={(e: FocusEvent) => (e.target as HTMLInputElement).select()}
          oninput={(e: Event) => {
            cursorIndex = 0;
            const query = (e.target as HTMLInputElement).value;
            if (query) {
              items = fuse.search(query).map((r) => {
                const matches = r.matches || [];
                const matchIndices = matches[0]
                  ? matches[0].indices
                  : undefined;
                return Object.assign(r.item, { matchIndices });
              });
            } else {
              for (const i of allItems) {
                i.matchIndices = undefined;
              }
              items = allItems;
            }
          }}
          autofocus
        />
      </label>
      <div class="flex flex-col h-full w-full overflow-y-scroll">
        {#each items as item, i}
          <Item
            index={i}
            data={item}
            selected={selectedItems.includes(item)}
            underCursor={i == cursorIndex}
            lastItem={i == itemsVisibleAtMaxHeight - 1}
            bind:selectedItems
          />
        {/each}
      </div>
    {:catch error}
      <p>Something went wrong: {error.message}</p>
    {/await}
  </div>
</main>
