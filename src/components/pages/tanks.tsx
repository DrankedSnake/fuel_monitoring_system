import "./tanks.css";

import { invoke } from "@tauri-apps/api";
import { Show, createResource } from "solid-js";

import Title from "../title";
import Table from "../table/table";


const getTanks = async () => {
    return await invoke("get_tanks");
}

export default function Tanks(){
    const [tanks] = createResource(getTanks);

    return (
        <div class="screen-container">
            <Title value="Tanks"/>
            <Show when={tanks()} fallback={<p>Loading...</p>}>
                <Table records={tanks()}/>
            </Show>
            <div class="operations-bar">
                <button>Add tank</button>
            </div>
        </div>
    )
}