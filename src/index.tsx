import { render } from "solid-js/web";

import "./styles.css";
import App from "./App";


const root = document.getElementById("root") as HTMLElement

render(() => (
        <App />
    ), root
);
