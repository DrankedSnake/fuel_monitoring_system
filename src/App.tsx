import { Route, Routes } from "@solidjs/router";
import "./App.css";

import {Differences, Trim, NavigationBar, Tankers, Tanks} from "./components";


export default function App() {
  return (
    <div>
      <div id="container">
        <NavigationBar />
        <Routes>
           <Route path="/" component={Differences}/>
           <Route path="/tankers" component={Tankers}/>
           <Route path="/differences" component={Differences}/>
           <Route path="/tanks" component={Tanks}/>
           <Route path="/trim" component={Trim}/>
        </Routes>
      </div>
    </div>
  )
}
