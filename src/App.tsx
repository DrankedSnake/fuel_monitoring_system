import { Route, Routes } from "@solidjs/router";
import "./App.css";
import { NavigationItems } from "./data";
import { For } from "solid-js";
import { NavigationBar } from "./components";


export default function App() {
  return (
    <div>
      <div id="container">
        <NavigationBar />
          <Routes>
            <For each={Object.values(NavigationItems())}>
            {
              (item) => (<Route path={item.item.path} component={item.item.component}/>)
            }
            </For>
          </Routes>
      </div>
    </div>
  )
}
