import "./tanks.css";

import { invoke } from "@tauri-apps/api";
import { Show, createEffect, createResource, createSignal } from "solid-js";

import Title from "../title";
import Table from "../table/table";
import AddRecordModal from "../addRecordModal";


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
            <AddRecordModal buttonText="Add tank" title="Add tank">
                <div>
                    <label for="tank_name">Tank name</label>
                    <input type="text" name="tank_name" id="tank_name" />
                </div>
                <div>
                    <label for="available_volume">Available volume</label>
                    <input type="number" id="available_volume" name="tank_volume" />
                </div>
                <div>
                    <label for="current_volume">Current Volume</label>
                <input type="text" id="current_volume" name="current_volume" />
                </div>
            </AddRecordModal>
        </div>
    )
}