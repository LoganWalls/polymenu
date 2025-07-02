import type { ItemData } from "./app";

export enum Action {
  CursorPrevious,
  CursorNext,
  ToggleSelection,
  ClearSelection,
  Submit,
  Close,
}

class AppState {
  items: ItemData[] = $state([]);
  currentItem: ItemData = $derived.by(() => this.items[this.cursorIndex])
  cursorIndex: number = $state(0);
  selectedIds: number[] = $state([]);

  updateItems = async (query: string) => {
    const myRequest = new Request("fuzzy-match", {
      headers: {
        "Content-Type": "application/json",
      },
      method: "POST",
      body: JSON.stringify({ query }),
    });
    const response = await window.fetch(myRequest);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    const items = await response.json();
    this.cursorIndex = 0;
    this.items = items;
  }

  submit = async (selectedIds: number[]) => {
    const myRequest = new Request("submit", {
      headers: {
        "Content-Type": "application/json",
      },
      method: "POST",
      body: JSON.stringify({ selectedIds }),
    });
    const response = await window.fetch(myRequest);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
  }

  executeAction = (action: Action) => {
    switch (action) {
      case Action.CursorPrevious:
        this.cursorIndex--;
        if (this.cursorIndex < 0) {
          this.cursorIndex = this.items.length - 1;
        }
        break;

      case Action.CursorNext:
        this.cursorIndex++;
        if (this.cursorIndex > this.items.length - 1) {
          this.cursorIndex = 0;
        }
        break;

      case Action.ToggleSelection:
        if (this.selectedIds.includes(this.currentItem.id)) {
          this.selectedIds = this.selectedIds.filter((id) => id !== this.currentItem.id);
        } else {
          this.selectedIds.push(this.currentItem.id);
        }
        break;

      case Action.ClearSelection:
        this.selectedIds = [];
        break;

      case Action.Submit:
        if (this.selectedIds.length == 0 && !this.selectedIds.includes(this.currentItem.id)) {
          this.selectedIds.push(this.currentItem.id);
        }
        this.submit(this.selectedIds)
        break;

      case Action.Close:
        this.submit([])
        break;

      default:
        console.error("Unrecognized action type: ", action)
        break;
    }
  }
}

export const appState = new AppState();
