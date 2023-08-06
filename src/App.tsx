import { Route, Routes } from "@solidjs/router";
import "./App.css";

import {Differences, TankVolumeProfile, NavigationBar, Tankers, Tanks, Vessels} from "./components";


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
           <Route path="/tank_volume_profile" component={TankVolumeProfile}/>
           <Route path="/vessels" component={Vessels}/>
        </Routes>
      </div>
    </div>
  )
}
