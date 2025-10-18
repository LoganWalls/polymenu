<script lang="ts">
  import { app, keymap } from "./globalState";
  import * as util from "polymenu/util";

  import Fuse from "fuse.js";
  import SearchIcon from "./lib/SearchIcon.svelte";
  import Item, { type ItemData } from "./lib/Item.svelte";

  let selectedItems: ItemData[] = $state([]);
  let cursorIndex = $state(0);
  let items: ItemData[] = $state([]);
  let allItems: ItemData[] = $state([]);
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
    });
  })();

  keymap.set("enter", () => {
    if (selectedItems.length == 0) {
      selectedItems.push(items[cursorIndex]);
    }
    app.print(selectedItems.map((item) => item.key));
  });
  keymap.set("tab", () => {
    selectedItems = util.toggleSet(items[cursorIndex], selectedItems);
  });
  keymap.set("ctrl+j", () => {
    cursorIndex = util.wrappingShift(cursorIndex, 1, 0, items.length - 1);
  });
  keymap.set("ctrl+k", () => {
    cursorIndex = util.wrappingShift(cursorIndex, -1, 0, items.length - 1);
  });
  keymap.set("ctrl+l", () => {
    selectedItems = [];
  });
  keymap.set("ctrl+h", () =>
    app.runCommand("say_anything", { message: "Watermellon!" }).then(app.print),
  );
  keymap.set("escape", app.close);
  keymap.set("ctrl+d", app.close);
</script>

<main class="text-xl bg-transparent dark:text-white">
  <div
    class="max-h-screen w-lg rounded-xl flex flex-col items-center gap-5 bg-gray-200/80 dark:bg-gray-900/80"
  >
    {#await fusePromise}
      <p>Loading...</p>
    {:then fuse}
      <!-- svelte-ignore a11y_autofocus -->
      <label
        class="w-full p-0 h-14 border-b-1 border-b-gray-600 dark:border-b-gray-300 align-middle"
        for="search"
      >
        <SearchIcon />
        <input
          name="search"
          class="w-5/6 h-full p-0 text-3xl outline-none placeholder:text-gray-800 dark:placeholder:text-gray-300"
          type="text"
          autocomplete="off"
          placeholder={app.options.placeholder as string}
          onfocusin={(e: FocusEvent) => (e.target as HTMLInputElement).select()}
          oninput={(e: Event) => {
            const query = (e.target as HTMLInputElement).value;
            if (query) {
              items = fuse.search(query).map((r) => r.item);
            } else {
              items = allItems;
            }
          }}
          autofocus
        />
      </label>
      <div
        class="flex flex-col items-center p-1 gap-2 h-full overflow-y-scroll"
      >
        {#each items as item, i}
          <Item
            index={i}
            data={item}
            selected={selectedItems.includes(item)}
            underCursor={i == cursorIndex}
          />
        {/each}
      </div>
    {:catch error}
      <p>Something went wrong: {error.message}</p>
    {/await}
  </div>
</main>
