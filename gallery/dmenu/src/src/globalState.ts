import { App } from "@polymenu/client";
import { Keymap } from "@polymenu/client/keymap";

export const keymap = new Keymap();
export const app = await App.fromFetchedOptions();
