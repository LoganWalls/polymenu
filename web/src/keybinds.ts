import { Action, appState } from "./state.svelte"


const keybinds = new Map<string, Action>();
keybinds.set("tab", Action.ToggleSelection);
keybinds.set("j+ctrl", Action.CursorNext);
keybinds.set("k+ctrl", Action.CursorPrevious);
keybinds.set("l+ctrl", Action.ClearSelection);
keybinds.set("enter", Action.Submit);
keybinds.set("escape", Action.Close);
keybinds.set("d+ctrl", Action.Close);
keybinds.set("c+ctrl", Action.Close);

function eventToKey(event: KeyboardEvent): string {
  const result = [event.key.toLowerCase()];
  if (event.shiftKey) {
    result.push("shift");
  }
  if (event.altKey) {
    result.push("alt");
  }
  if (event.ctrlKey) {
    result.push("ctrl");
  }
  if (event.metaKey) {
    result.push("super");
  }
  return result.join("+");
}

document.addEventListener(
  "keydown",
  (event) => {
    const action = keybinds.get(eventToKey(event));
    if (action !== undefined) {
      if (["tab", "escape", "enter"].includes(event.key.toLowerCase())) {
        event.preventDefault()
      }
      appState.executeAction(action);
    }
  },
  false,
);
