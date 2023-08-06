import { Show, createResource, createSignal } from "solid-js"
import Title from "../title"
import Table from "../table/table"
import AddRecordModal from "../addRecordModal"
import UploadFileModal from "../uploadFileModal"
import { invoke } from "@tauri-apps/api"
import { createStore } from "solid-js/store"


const getVessels = async () => {
    return await invoke("get_vessels");
};


const addTrim = async () => {

};

export default function TankVolumeProfile(){
    const [activeVessel, setActiveVessel] = createSignal("");
    const [activeTank, setActiveTank] = createSignal("")

    const getTanks = async (id: string) => {
        if (activeVessel()){
            return await invoke("get_tanks", {"vesselId": id});
        }    
    };
    const getTankVolumeProfiles = async (vesselId: string, tankId: string)=> {
        if (activeVessel() && activeTank()){
            return await invoke("get_tank_volume_profiles", {"vesselId": vesselId, "tankId": tankId})
        }
    };
    const [tanks] = createResource(activeVessel, getTanks);
    const [vessels] = createResource(getVessels);

 
    const [trims] = createResource(activeVessel, activeTank, getTankVolumeProfiles)

    const [form, setForm] = createStore(
        {
            "tank_id": "",
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
        await addTrim(form);
    };


    return (
        <div class="screen-container">
            <Title value="Trim"/>

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

            <select name="tanks" id="tanks" onInput={
                (e)=>{
                    setActiveTank(e.currentTarget.value);
                    setForm({tank_id: activeTank()});
                }}
            >
                <option value="none">Select tank</option>
                <Show when={tanks()}>
                    <For each={tanks()}>
                        {
                            (tank) => (
                                <option value={tank.id}>{tank.name}</option>
                            )
                        }
                    </For>
                </Show>
            </select>

            <Show when={trims()} fallback={<p>Loading...</p>}>
                <Table records={trims()} headers={["name", "volume"]}/>
            </Show>

            <AddRecordModal buttonText="Add trim" title="Add trim">
                <div>
                    <label for="value">Value</label>
                    <input type="text" name="value" id="value" />
                </div>
            </AddRecordModal>
            <UploadFileModal buttonText="Upload from csv file" title="Upload trims">
                <input type="file" id="trimFile" name="filename"/>
            </UploadFileModal>
        </div>
    )
}