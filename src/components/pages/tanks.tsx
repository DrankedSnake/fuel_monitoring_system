import "./tanks.css";

import { invoke } from "@tauri-apps/api";
import { For, Show, createResource, createSignal } from "solid-js";

import Title from "../title";
import Table from "../table/table";
import AddRecordModal from "../addRecordModal";
import { createStore } from "solid-js/store";


const getVessels = async () => {
    return await invoke("get_vessels");
};

export default function Tanks(){
    const [activeVessel, setActiveVessel] = createSignal("");
    const getTanks = async (id: string) => {
        if (activeVessel()){
            return await invoke("get_tanks", {"vesselId": id});
        }    
    }
    const [vessels] = createResource(getVessels);
    const [tanks, {refetch, mutate}] = createResource(activeVessel, getTanks);
 

    const addTank = async (tank: any) => {
        if (activeVessel().length != 0){
            await invoke("add_tank", {"tank": tank});
            refetch(activeVessel());
        }
    }
    const [form, setForm] = createStore(
        {
            "tank_name": "",
            "available_volume": 0.0,
            "current_volume": 0.0,
            "vessel_id": "",
        }
    );

    const updateFormField = (fieldName: string) => (event: Event) => {
        const inputElement = event.currentTarget as HTMLInputElement;
        setForm({
          [fieldName]: inputElement.value
        });
      };

    const submitForm = async () => {
        await addTank(form);
    };

    return (
        <div class="screen-container">
            <Title value="Tanks"/>
            <select name="vessels" id="vessels" onInput={
                (e)=>{
                    setActiveVessel(e.currentTarget.value);
                    setForm({vessel_id: activeVessel()});
                }}
            >
                <option value="none">Select vessel</option>
                <Show when={vessels()}>
                    <For each={vessels()}>
                        {
                            (vessel) => (
                                <option value={vessel.id}>{vessel.name}</option>
                            )
                        }
                    </For>
                </Show>
            </select>

            <Show when={tanks()} fallback={<p>Loading...</p>}>
                <Table records={tanks()} headers={["name", "current_volume", "available_volume"]}/>
            </Show>

            <AddRecordModal buttonText="Add tank" title="Add tank" add_record_callback={submitForm}>
                <div>
                    <label for="tank_name">Tank name</label>
                    <input type="text" name="tank_name" id="tank_name" onChange={updateFormField("tank_name")}/>
                </div>
                <div>
                    <label for="available_volume">Available volume</label>
                    <input type="number" id="available_volume" name="available_volume" onChange={updateFormField("available_volume")}/>
                </div>
                <div>
                    <label for="current_volume">Current Volume</label>
                <input type="text" id="current_volume" name="current_volume" onChange={updateFormField("current_volume")}/>
                </div>
            </AddRecordModal>
        </div>
    )
}