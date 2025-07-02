import { mount } from "svelte"
import "./appMain.css"
import AppMain from "./AppMain.svelte"

const appMain = mount(AppMain, {
  target: document.getElementById("appMain")!,
})

export default appMain
