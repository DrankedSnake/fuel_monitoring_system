import { render } from "solid-js/web";

import "./styles.css";
import App from "./App";
import { Router } from "@solidjs/router";


const root = document.getElementById("root") as HTMLElement

render(() => (
    <Router>
        <App />
    </Router>
    ), root
);
