/**
* A utility for managing keymaps
*
* @module keymap
* */

export type KeymapFn = () => void | Promise<void> | undefined;

export class Keymap {
  mode: string | null = null;
  map: Map<string | null, Map<string, KeymapFn>> = new Map();

  constructor() {
    document.addEventListener(
      "keydown",
      (event) => {
        const action = this.modeMapFor(this.mode).get(this.eventToKey(event));
        if (action !== undefined) {
          if (["tab", "escape", "enter"].includes(event.key.toLowerCase())) {
            event.preventDefault()
          }
          action();
        }
      },
      false,
    );
  }

  setMode = (mode: string | null) => {
    this.mode = mode;
  }

  modeMapFor = (mode: string | null): Map<string, KeymapFn> => {
    let map = this.map.get(mode);
    if (map === undefined) {
      map = new Map();
      this.map.set(mode, map);
    }
    return map
  }

  set = (lhs: string, rhs: KeymapFn, mode: string | null = null) => {
    const map = this.modeMapFor(mode);
    const lhsParts = lhs.toLowerCase().split("+");
    lhsParts.sort();
    const key = lhsParts.join("+");
    if (map.has(key)) {
      console.warn("Duplicate keymap detected for:", lhs);
    }
    map.set(key, rhs);
  }

  eventToKey = (event: KeyboardEvent): string => {
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
    result.sort();
    return result.join("+");
  }

}
