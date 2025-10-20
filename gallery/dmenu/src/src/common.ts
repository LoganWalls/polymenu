import { app } from "./globalState";

export interface ItemData {
  key: string;
  value?: string;
  icon?: string;
  matchIndices?: number[];
}

export function submit(itemUnderCusor: ItemData, selectedItems: ItemData[]) {
  if (
    !selectedItems.reduce(
      (found, current) => found || current.key === itemUnderCusor.key,
      false,
    )
  ) {
    selectedItems.push(itemUnderCusor);
  }
  app.print(selectedItems.map((item) => item.value || item.key));
  app.close();
}
