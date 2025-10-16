import { App } from "polymenu";
import { Keymap } from "polymenu/keymap";

export const keymap = new Keymap();
export const app = await App.fromFetchedOptions();
