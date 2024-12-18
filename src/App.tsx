import { Route, Router } from "@solidjs/router";
import "./App.css";
import { NavigationItems } from "./data";
import { For } from "solid-js";
import { NavigationBar } from "./components";


export default function App() {
  return (
    <div>
      <div id="container">
        <NavigationBar />
          <Router>
            <For each={Object.values(NavigationItems())}>
            {
              (item) => {return (<Route path={item.item.path} component={item.item.component}/>)}
            }
            </For>
          </Router>
      </div>
    </div>
  )
}
